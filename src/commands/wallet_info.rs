use serenity::all::{CommandInteraction, Context};
use serenity::builder::CreateCommand;
use tracing::log::error;

use crate::errors::CustomError;
use crate::state::SharedState;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> String {
    let data = ctx.data.read().await;
    let state = &data.get::<SharedState>().unwrap();
    let wallet_service = &state.wallet_service;

    let member = interaction.user.clone();

    let wallet_result = wallet_service
        .find_wallet_by_discord_id(&member.id.to_string())
        .await;

    if let Ok(wallet) = wallet_result {
        return format!(
            "Estimado: `{}` este el status de su wallet: \n **saldo:** `{} chad coins`\n\n🦊🚬",
            member.name, wallet.amount
        );
    }

    let err = wallet_result.unwrap_err();

    match err {
        CustomError::NotFound(err) => {
            error!("{err:?}");

            "No tienes una wallet".to_string()
        }
        CustomError::InternalError(err) => {
            error!("{err:?}");

            "Ha ocurrido un error interno".to_string()
        }
        _ => "Ha occurrido algun error intentalo más tarde".to_string(),
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("wallet_info").description("Consultar la información de la wallet")
}
