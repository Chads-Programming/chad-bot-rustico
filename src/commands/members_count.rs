use serenity::all::{CommandInteraction, Context};
use serenity::builder::CreateCommand;

pub async fn run(
    ctx: &Context,
    interaction: &CommandInteraction,
) -> Result<String, serenity::Error> {
    if interaction.guild_id.is_none() {
        return Err(serenity::Error::Other("Ha ocurrido un error al recuparar el guild"));
    }

    let guild_id = interaction.guild_id.unwrap();
    let mut count: usize = 0;
    
    if let Ok(member_list) = guild_id.members(&ctx.http, None, None).await {
        for member in member_list.iter() {
            if !member.user.bot {
                count += 1;
            }
        }
    }

    if count == 0 {
        return Err(serenity::Error::Other("Error al contar los miembros"));
    }

    let msg = format!("Miembros en el servidor: `{count}`");
    Ok(msg)
}

pub fn register() -> CreateCommand {
    CreateCommand::new("members_count")
        .description("Send a message with quantity of members in the server")
}
