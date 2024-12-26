use cripto_api::{api::CoinService, coins::CoinID};
use num_format::{Locale, ToFormattedString};
use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateCommandOption, CreateEmbed, ResolvedOption,
    ResolvedValue,
};

use std::str::FromStr;

use crate::{errors::CustomError, state::SharedState};

const COINS: [CoinID; 7] = [
    CoinID::Bitcoin,
    CoinID::Solana,
    CoinID::XRP,
    CoinID::Pepe,
    CoinID::Doge,
    CoinID::Polkadot,
    CoinID::Usual,
];

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
            let mut embed = CreateEmbed::new().title(price_detail.name.to_uppercase());

            for price in price_detail.prices.into_iter() {
                let formated_price = if price.value >= 1.0 {
                    let integer_price = price.value.floor() as i64;
                    let decimal_price_part = format!("{:.3}", (price.value % integer_price as f64));
                    let formated_integer = integer_price.to_formatted_string(&Locale::en);

                    format!("{}.{}", formated_integer, &decimal_price_part[2..])
                } else {
                    price.value.to_string()
                };

                embed = embed.field(
                    price.currency.to_string().to_uppercase(),
                    formated_price,
                    true,
                )
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
    let command = CreateCommand::new("crypto_prices")
        .description("Mostrar el precio actual de una cripto divisa");

    let mut option = CreateCommandOption::new(
        serenity::all::CommandOptionType::String,
        "coin",
        "El nombre de la modena",
    )
    .required(true);

    for coin in COINS.into_iter() {
        let coin_name = coin.to_string();

        option = option.add_string_choice(coin_name.to_uppercase(), coin_name);
    }

    command.add_option(option)
}
