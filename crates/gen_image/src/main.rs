use std::{fs::File, io::Read};

pub fn main() {
    let position_number = 12;

    let mut args = std::env::args().skip(1);

    let Some(background) = args.next() else {
        panic!("background path")
    };

    let font_regular = {
        let path = args.next().unwrap();

        let mut f = File::open(path).unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).expect("buffer overflow");

        buffer
    };

    let font_bold = {
        let path = args.next().unwrap();

        let mut f = File::open(path).unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).expect("buffer overflow");

        buffer
    };

    let avatar = {
        let path = args.next().unwrap();

        let mut f = File::open(path).unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).expect("buffer overflow");

        buffer
    };

    if let Err(err) = gen_image::generate(
        &avatar,
        &background,
        "Peronist man".to_string(),
        position_number,
        "test_welcome.png",
        &font_regular,
        &font_bold,
    ) {
        panic!("{err:?}");
    }
}
