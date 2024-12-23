use std::collections::HashMap;

use async_trait::async_trait;
use reqwest::Error;
use serde::{Deserialize, Serialize};

use crate::{
    api::CoinService,
    coins::{CoinID, CoinPrice, CoinResponse, PriceCurrency},
};

#[derive(Debug, Serialize, Deserialize)]
struct PricePairs {
    usd: f64,
    eur: f64,
    ars: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeckoCoinPrice {
    #[serde(flatten)]
    coins: HashMap<String, PricePairs>,
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

pub struct CoinGeckoService {
    api_key: String,
}

impl CoinGeckoService {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: String::from(api_key),
        }
    }
}

#[async_trait]
impl CoinService for CoinGeckoService {
    async fn get_coin_price(&self, coin_id: CoinID) -> Result<CoinResponse, Error> {
        let url = format!(
            "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies={}",
            coin_id,
            format_args!(
                "{},{},{}",
                PriceCurrency::USD,
                PriceCurrency::ARS,
                PriceCurrency::EUR,
            )
        );

        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .header("accept", "application/json")
            .header("x-cg-demo-api-key", &self.api_key)
            .send()
            .await?;

        let price_data = response.json::<GeckoCoinPrice>().await?;
        Ok(price_data.into())
    }
}
