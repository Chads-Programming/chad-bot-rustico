#[derive(Debug)]
pub struct CoinPrice {
    pub currency: PriceCurrency,
    pub value: f64,
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
