use meme_generator::{self};
use serenity::all::{CommandInteraction, Context, CreateCommand, CreateEmbed};
use tracing::error;

use crate::errors::CustomError;

pub async fn run(_: &Context, _: &CommandInteraction) -> Result<CreateEmbed, CustomError> {
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

pub fn register() -> CreateCommand {
    CreateCommand::new("meme").description("Obtener un meme random")
}
