use serenity::all::{CommandOptionType, Context, CreateCommand, CreateCommandOption, User};

pub async fn run(ctx: &Context, user: &User) -> String {
    let target_user = user.clone();

    match target_user.create_dm_channel(&ctx.http).await {
        Ok(channel) => {
            channel.say(&ctx.http, "**Quieto ahí pibardo!** \nhttps://tenor.com/view/luna-crunchycat-nerd-gif-9668924902045279367\n**Has sido advertido**".to_string()).await.unwrap();
            let name = target_user.name;

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
