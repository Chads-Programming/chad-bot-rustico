use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommandOption, ResolvedOption,
    ResolvedValue,
};
use serenity::builder::CreateCommand;
use tracing::log::error;

use crate::errors::CustomError;
use crate::state::SharedState;
use crate::wallet::models::DepositAmountFromDiscord;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<String, String> {
    let data = ctx.data.read().await;
    let state = &data.get::<SharedState>().unwrap();
    let wallet_service = &state.wallet_service;

    let donator_member_discord = interaction.user.clone();
    let options = &interaction.data.options().clone();

    let query = {
        let option_user = if let Some(ResolvedOption {
            value: ResolvedValue::User(user, _),
            ..
        }) = options.first()
        {
            Some(user)
        } else {
            None
        };

        let option_amount = if let Some(ResolvedOption {
            value: ResolvedValue::Number(amount),
            ..
        }) = options.get(1)
        {
            Some(amount)
        } else {
            None
        };

        let option_note = if let Some(ResolvedOption {
            value: ResolvedValue::String(note),
            ..
        }) = options.get(2)
        {
            Some(note)
        } else {
            None
        };

        println!("{option_note:?}");

        (option_user, option_amount, option_note)
    };

    if query.0.is_none() {
        return Err("Por favor proporcione un usuario".to_string());
    }

    if query.1.is_none() {
        return Err("Por favor proporcione un usuario".to_string());
    }

    let target_user = query.0.unwrap();
    let amount = *query.1.unwrap();

    if target_user.id.to_string() == donator_member_discord.id.to_string() {
        return Err("No te puedes donar a ti mismo 😾".to_string());
    }

    if amount <= 0.0 {
        return Err("El monto debe ser mayor que 0 pibe".to_string());
    }

    let donate_result = wallet_service
        .deposit_amount_from_discord(&DepositAmountFromDiscord {
            from_discord_id: donator_member_discord.id.to_string(),
            target_discord_id: target_user.id.to_string(),
            amount,
        })
        .await;

    if donate_result.is_ok() {
        let mut message = format!(
            "\n*<@{}>* a dado **{amount}** chad coins a *<@{}>*",
            donator_member_discord.id, target_user.id
        );

        if let Some(note) = query.2 {
            message = format!("{message} por: `{note}`")
        }

        return Ok(format!("{message} \n\n🦊🚬"));
    }

    let err = donate_result.unwrap_err();

    match err {
        CustomError::FetchError(_) => {
            error!("{err:?}");

            Err("Ha ocurrido un error al recuperar los datos".to_string())
        }
        CustomError::OutOfFunds(err) => {
            error!("{err:?}");

            Err(
                "No tienes los suficientes fondos para realizar esta transacción pibardo"
                    .to_string(),
            )
        }
        CustomError::NotFound(err) => {
            error!("{err:?}");

            Err("Parece que uno de los participantes en esta transacción no se encuentra registrado"
                .to_string())
        }
        _ => Err("Ha occurrido algun error interno intentalo más tarde".to_string()),
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("donate_coins")
        .description("Donar chad coins a un determinado chad")
        .add_option(
            CreateCommandOption::new(CommandOptionType::User, "user", "El chad a donar")
                .required(true),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Number,
                "amount",
                "El monto a depositar (debes tener suficientes chad coins)",
            )
            .required(true),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "note",
                "Una nota/razón sobre la transacción",
            )
            .required(false),
        )
}
