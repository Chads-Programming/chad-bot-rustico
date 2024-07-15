use serenity::all::{Ban, CommandInteraction, Context, CreateCommand};

fn ban_message(ban: Ban) -> String {
    let user = ban.user.name;
    let reason = ban
        .reason
        .unwrap_or("No se proporciono una razón".to_string());

    format!("**pibardo:** `{user}`\n**razón:** `{reason}`")
}

pub async fn run(ctx: &Context, command: &CommandInteraction) -> String {
    let guild_id = command.guild_id.unwrap();
    let http = &ctx.http;

    match http.get_guild(guild_id).await {
        Ok(guild) => {
            let bans = guild.bans(http, None, None).await.unwrap();
            let mut ban_info = bans
                .into_iter()
                .map(move |ban| ban_message(ban.clone()))
                .collect::<Vec<String>>();

            ban_info.push("🦊 🚬".to_string());
            ban_info.join("\n\n")
        }
        Err(_) => "Ha ocurrido un error".to_string(),
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("bans_info")
        .description("Devuelve un detalle de los miembros baneados del servidor 🌍")
}
