use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub struct CoinPrice {
    pub currency: PriceCurrency,
    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeckoCoinPrice {
    #[serde(flatten)]
    coins: HashMap<String, PricePairs>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PricePairs {
    usd: f64,
    eur: f64,
    ars: f64,
}

pub struct CoinResponse {
    pub name: String,
    pub prices: Vec<CoinPrice>,
}

impl Default for CoinResponse {
    fn default() -> Self {
        Self {
            name: "Empty coin".to_string(),
            prices: vec![],
        }
    }
}

pub enum CoinID {
    BITCOIN,
    SOLANA,
    USUAL,
    XRP,
    DOT,
}

#[derive(strum_macros::Display)]
pub enum PriceCurrency {
    #[strum(serialize = "usd")]
    USD,
    #[strum(serialize = "eur")]
    EUR,
    #[strum(serialize = "ars")]
    ARS,
}

impl CoinID {
    pub fn to_str(&self) -> String {
        let str_coind_id = match self {
            CoinID::BITCOIN => "bitcoin",
            CoinID::SOLANA => "sol",
            CoinID::USUAL => "usual",
            CoinID::XRP => "xrp",
            CoinID::DOT => "dto",
        };

        String::from(str_coind_id)
    }
}

impl From<GeckoCoinPrice> for CoinResponse {
    fn from(gecko_price: GeckoCoinPrice) -> Self {
        if let Some((coin, price)) = gecko_price.coins.iter().next() {
            return Self {
                name: coin.clone(),
                prices: vec![
                    CoinPrice {
                        currency: PriceCurrency::USD,
                        value: price.usd,
                    },
                    CoinPrice {
                        currency: PriceCurrency::EUR,
                        value: price.ars,
                    },
                    CoinPrice {
                        currency: PriceCurrency::ARS,
                        value: price.eur,
                    },
                ],
            };
        }

        Self::default()
    }
}
