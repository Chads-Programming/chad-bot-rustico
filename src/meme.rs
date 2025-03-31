use meme_generator::{self};

use serenity::all::CreateEmbed;
use tracing::error;

use crate::errors::CustomError;

pub async fn create_meme_embeb() -> Result<CreateEmbed, CustomError> {
    match meme_generator::api::get_random_meme_information().await {
        Ok(meme_response) => {
            let embed = CreateEmbed::new()
                .title(meme_response.title)
                .image(meme_response.preview.last().unwrap());

            Ok(embed)
        }
        Err(err) => {
            error!("{:?}", err);

            Err(CustomError::FetchError(
                "Error on download meme image".to_string(),
            ))
        }
    }
}
