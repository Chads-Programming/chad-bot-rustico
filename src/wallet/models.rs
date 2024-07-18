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
pub struct Member {
    pub id: Uuid,
    pub name: String,
    pub discord_id: String,
}

#[derive(Deserialize, sqlx::FromRow, Debug, Clone)]
pub struct Wallet {
    pub id: Uuid,
    pub amount: BigDecimal,
    pub member_id: Uuid,
}
