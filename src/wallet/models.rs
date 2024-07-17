use serde::Deserialize;
use sqlx::types::Uuid;

#[derive(Deserialize, Debug, Clone)]
pub struct CreateMember {
    pub name: String,
    pub discord_id: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DepositAmount {
    pub from_discord_id: String,
    pub discord_id: String,
    pub ammount: String,
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
    pub amount: i64,
    pub member_id: Uuid,
}
