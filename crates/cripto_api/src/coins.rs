use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(strum_macros::Display)]
pub enum CoinID {
    #[strum(serialize = "bitcoin")]
    Bitcoin,
    #[strum(serialize = "solana")]
    Solana,
    #[strum(serialize = "usual")]
    Usual,
    #[strum(serialize = "ripple")]
    XRP,
    #[strum(serialize = "polkadot")]
    Polkadot,
    #[strum(serialize = "pepe")]
    Pepe,
    #[strum(serialize = "doge")]
    Doge,
}

#[derive(strum_macros::Display, Debug)]
pub enum PriceCurrency {
    #[strum(serialize = "usd")]
    USD,
    #[strum(serialize = "eur")]
    EUR,
    #[strum(serialize = "ars")]
    ARS,
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
                        currency: PriceCurrency::ARS,
                        value: price.ars,
                    },
                    CoinPrice {
                        currency: PriceCurrency::EUR,
                        value: price.eur,
                    },
                ],
            };
        }

        Self::default()
    }
}
