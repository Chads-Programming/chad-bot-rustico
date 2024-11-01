use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    ResolvedOption, ResolvedValue,
};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> String {
    let options = &interaction.data.options().clone();

    let option_user = if let Some(ResolvedOption {
        value: ResolvedValue::User(user, _),
        ..
    }) = options.first()
    {
        Some(user)
    } else {
        None
    };

    let reason_option = if let Some(ResolvedOption {
        value: ResolvedValue::String(reason),
        ..
    }) = options.get(1)
    {
        Some(*reason)
    } else {
        None
    };

    if option_user.is_none() {
        return "No se especifico un usuario".to_string();
    }

    let target_user = option_user.unwrap();

    match target_user.create_dm_channel(&ctx.http).await {
        Ok(channel) => {
            let name = target_user.name.clone();
            let base_message = format!("\n**Quieto ahí pibardo!**\nEstimado: *{name}* se le informa educamente que **ha sido advertido**");

            let message = if let Some(reason) = reason_option {
                format!("{base_message}\nRazón:`{reason}`")
            } else {
                base_message
            };

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
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "reason", "The reason of warn")
                .required(true),
        )
}
