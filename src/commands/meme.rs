use serenity::all::{CommandInteraction, Context, CreateCommand, CreateEmbed};

use crate::{errors::CustomError, meme};

pub async fn run(_: &Context, _: &CommandInteraction) -> Result<CreateEmbed, CustomError> {
    meme::create_meme_embeb().await
}

pub fn register() -> CreateCommand {
    CreateCommand::new("meme").description("Obtener un meme random")
}
