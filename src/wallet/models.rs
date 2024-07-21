use serde::Deserialize;
use sqlx::types::{BigDecimal, Uuid};

#[derive(Deserialize, Debug, Clone)]
pub struct CreateMember {
    pub name: String,
    pub discord_id: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DepositAmountFromDiscord {
    pub from_discord_id: String,
    pub target_discord_id: String,
    pub amount: f64,
}

#[derive(Deserialize, sqlx::FromRow, Debug, Clone)]
pub struct MinimalMemberWallet {
    #[allow(dead_code)]
    pub id: Uuid,
    #[allow(dead_code)]
    pub name: String,
    #[allow(dead_code)]
    pub wallet_id: Uuid,
    pub discord_id: String,
    pub amount: BigDecimal,
}

#[derive(Deserialize, sqlx::FromRow, Debug, Clone)]
pub struct Wallet {
    pub id: Uuid,
    pub amount: BigDecimal,
    pub member_id: Uuid,
}
