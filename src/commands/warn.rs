use serenity::all::{CommandOptionType, Context, CreateCommand, CreateCommandOption, User};

use crate::gifs;

pub async fn run(ctx: &Context, user: &User) -> String {
    let target_user = user.clone();

    match target_user.create_dm_channel(&ctx.http).await {
        Ok(channel) => {
            let name = target_user.name;
            let image_url = gifs::WARN_CAT;
            let message = format!("\n**Quieto ahí pibardo!**\nEstimado: *{name}* se le informa educamente que **ha sido advertido** \n[hungry_cat]({image_url})");

            channel.say(&ctx.http, message).await.unwrap();

            format!("{name} ha sido advertido")
        }
        Err(_) => "Error en advertir al usuario".to_string(),
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("warn")
        .description("Send a warn message to user")
        .add_option(
            CreateCommandOption::new(CommandOptionType::User, "user", "The user to lookup")
                .required(true),
        )
}
