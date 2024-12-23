use crate::coins::{CoinID, CoinResponse, GeckoCoinPrice, PriceCurrency};
use reqwest::Error;

pub struct CoinService {
    api_key: String,
}

impl CoinService {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: String::from(api_key),
        }
    }

    pub async fn get_coin_price(&self, coin_id: CoinID) -> Result<CoinResponse, Error> {
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
