use crate::coins::{CoinID, CoinResponse};
use async_trait::async_trait;
use reqwest::Error;

#[async_trait]
pub trait CoinService {
    async fn get_coin_price(&self, coin_id: CoinID) -> Result<CoinResponse, Error>;
}
