use serenity::all::{CommandInteraction, Context};
use serenity::builder::CreateCommand;
use tracing::log::error;

use crate::errors::CustomError;
use crate::state::SharedState;
use crate::wallet::models::CreateMember;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> String {
    let data = ctx.data.read().await;
    let state = &data.get::<SharedState>().unwrap();
    let wallet_service = &state.wallet_service;

    let new_member = interaction.user.clone();

    let register_result = wallet_service
        .register_member(&CreateMember {
            name: new_member.name.clone(),
            discord_id: new_member.id.to_string(),
        })
        .await;

    if register_result.is_ok() {
        return format!("`{}` ha registrado su wallet \n\n🦊🚬", new_member.name);
    }

    let err = register_result.unwrap_err();

    match err {
        CustomError::FetchError(_) => {
            error!("{err:?}");

            "Ha ocurrido un error al recuperar los datos".to_string()
        }
        CustomError::AlreadyMemberExists(err) => {
            error!("{err:?}");

            "Ya tienes una wallet 🦊🚬".to_string()
        }
        CustomError::InternalError(err) => {
            error!("{err:?}");

            "Ha ocurrido un error interno".to_string()
        }
        _ => "Ha occurrido algun error intentalo más tarde".to_string(),
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("register_wallet")
        .description("Crear y registrar tu propia wallet (si no tienes una)")
}
