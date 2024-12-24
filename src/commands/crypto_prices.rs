use cripto_api::{api::CoinService, coins::CoinID};
use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateCommandOption, CreateEmbed, ResolvedOption,
    ResolvedValue,
};
use std::str::FromStr;

use crate::{errors::CustomError, state::SharedState};

pub async fn run(ctx: &Context, command: &CommandInteraction) -> Result<CreateEmbed, CustomError> {
    let data = ctx.data.read().await;
    let options = &command.data.options().clone();
    let state = &data.get::<SharedState>().unwrap();
    let coin_service = &state.coin_service;

    let coin_id_query = {
        let option_course = if let Some(ResolvedOption {
            value: ResolvedValue::String(coin_id),
            ..
        }) = options.first()
        {
            Some(coin_id)
        } else {
            None
        };

        option_course
    };

    if coin_id_query.is_none() {
        return Err(CustomError::BadArguments(
            "Coin id is mandatory".to_string(),
        ));
    }

    let coind_id = CoinID::from_str(coin_id_query.unwrap());

    if coind_id.is_err() {
        return Err(CustomError::BadArguments("Coin id is invalid".to_string()));
    }

    let price_result = coin_service.get_coin_price(coind_id.unwrap()).await;

    match price_result {
        Ok(price_detail) => Ok({
            let mut embed = CreateEmbed::new().title(price_detail.name);

            for price in price_detail.prices.into_iter() {
                embed = embed.field(price.currency.to_string(), price.value.to_string(), false)
            }

            embed
        }),
        Err(err) => Err(CustomError::FetchError(format!(
            "Error on fetching prices: {:?}",
            err
        ))),
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("crypto_prices")
        .description("Mostrar el precio actual de una cripto divisa")
        .add_option(
            CreateCommandOption::new(
                serenity::all::CommandOptionType::String,
                "coin",
                "El nombre de la modena",
            )
            .add_string_choice(CoinID::Bitcoin.to_string(), CoinID::Bitcoin.to_string())
            .add_string_choice(CoinID::Solana.to_string(), CoinID::Solana.to_string())
            .add_string_choice(CoinID::XRP.to_string(), CoinID::XRP.to_string())
            .add_string_choice(CoinID::Pepe.to_string(), CoinID::Pepe.to_string())
            .add_string_choice(CoinID::Doge.to_string(), CoinID::Doge.to_string())
            .add_string_choice(CoinID::Usual.to_string(), CoinID::Usual.to_string())
            .add_string_choice(CoinID::Polkadot.to_string(), CoinID::Polkadot.to_string())
            .required(true),
        )
}
