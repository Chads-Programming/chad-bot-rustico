use ab_glyph::{FontRef, InvalidFont, PxScale};
use image::{GenericImage, GenericImageView, ImageBuffer, ImageError, Pixel, Rgba};

const FONT_SIZE: PxScale = PxScale { x: 36., y: 36. };

#[derive(Debug, Clone)]
enum Colors {
    White,
}

#[derive(Debug)]
pub enum GenerateImageError {
    FontError(String),
    ImageError(String),
}

impl From<InvalidFont> for GenerateImageError {
    fn from(value: InvalidFont) -> Self {
        Self::FontError(value.to_string())
    }
}

impl From<ImageError> for GenerateImageError {
    fn from(value: ImageError) -> Self {
        Self::ImageError(value.to_string())
    }
}

pub fn generate(
    avatar: &[u8],
    background: &str,
    fullname: String,
    position_number: usize,
    out_path: &str,
    regular_font: &[u8],
    bold_font: &[u8],
) -> Result<(), GenerateImageError> {
    let loaded_avatar = image::load_from_memory(avatar)?;
    let regular_font = FontRef::try_from_slice(regular_font)?;

    let bold_font = FontRef::try_from_slice(bold_font)?;

    let mut background = image::open(background)?;

    let (w, _) = background.dimensions();

    let avatar = loaded_avatar.resize(256, 256, image::imageops::Lanczos3);
    let avatar = rounded_image(&avatar, Colors::White);

    image::imageops::overlay(&mut background, &avatar, 412, 87);

    let w_msg = format!("Bienvenido {}", fullname);

    let (t1_x, _t1_y) = imageproc::drawing::text_size(FONT_SIZE, &bold_font, &w_msg);

    imageproc::drawing::draw_text_mut(
        &mut background,
        Rgba([255, 255, 255, 255]),
        ((w / 2) - (t1_x / 2)) as i32,
        429,
        FONT_SIZE,
        &bold_font,
        &w_msg,
    );

    let n_msg = format!("Eres el chad: {}", position_number);
    let (t2_x, _t2_y) = imageproc::drawing::text_size(FONT_SIZE, &regular_font, &n_msg);

    imageproc::drawing::draw_text_mut(
        &mut background,
        Rgba([255, 255, 255, 255]),
        ((w / 2) - (t2_x / 2)) as i32,
        488,
        FONT_SIZE,
        &regular_font,
        &n_msg,
    );

    background.save(out_path)?;

    Ok(())
}

impl From<Colors> for Rgba<u8> {
    fn from(value: Colors) -> Self {
        match value {
            Colors::White => Rgba([255, 255, 255, 255]),
        }
    }
}

// Code based from cangrebot: https://github.com/RustLangES/cangrebot/blob/main/crates/gen_welcome/src/lib.rs
fn rounded_image<I: GenericImageView<Pixel = Rgba<u8>>>(
    avatar: &I,
    border_color: Colors,
) -> impl GenericImage<Pixel = Rgba<u8>> {
    let (width, height) = avatar.dimensions();
    let radius = width as f32 / 2.0;
    let mut mask = ImageBuffer::new(width, height);
    let avatar_center = (width as f32 / 2.0, height as f32 / 2.0);

    for (x, y, pixel) in mask.enumerate_pixels_mut() {
        let dx = x as f32 - avatar_center.0 + 0.5;
        let dy = y as f32 - avatar_center.1 + 0.5;
        if dx.powi(2) + dy.powi(2) <= radius.powi(2) {
            *pixel = border_color.clone().into();
        } else {
            *pixel = Rgba([0, 0, 0, 0]);
        }
    }

    ImageBuffer::from_fn(width, height, |x, y| {
        let mask_pixel = mask.get_pixel(x, y).0[3];
        let avatar_pixel = avatar.get_pixel(x, y);
        if mask_pixel > 0 {
            avatar_pixel
        } else {
            avatar_pixel.map_with_alpha(|f| f, |_| 0)
        }
    })
}
