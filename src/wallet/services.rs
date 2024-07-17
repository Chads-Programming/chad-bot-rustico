use sqlx::types::Uuid;

use super::models::{CreateMember, DepositAmountFromDiscord, Wallet};
use crate::{db::ConnectionPool, errors::CustomError};
use std::sync::Arc;

pub struct WalletService {
    conn: Arc<ConnectionPool>,
}

impl WalletService {
    pub fn new(conn: Arc<ConnectionPool>) -> Self {
        Self { conn }
    }

    pub async fn register_member(&self, create_member: &CreateMember) -> Result<Uuid, CustomError> {
        let conn = &*self.conn;

        let already_exist_member_result = self
            .find_wallet_by_discord_id(&create_member.discord_id)
            .await;

        if already_exist_member_result.is_ok() {
            return Err(CustomError::AlreadyMemberExists(
                "Member was registerd with same discord account id: {existing_member}".to_string(),
            ));
        }

        let query_result: (Uuid,) = sqlx::query_as("call create_member_with_wallet(?, ?)")
            .bind(create_member.name.clone())
            .bind(create_member.discord_id.clone())
            .fetch_one(conn)
            .await?;

        Ok(query_result.0)
    }

    pub async fn deposit_amount_from_discord(
        &self,
        deposit_amount: &DepositAmountFromDiscord,
    ) -> Result<bool, CustomError> {
        let conn = &*self.conn;

        let tx = conn.begin().await?;

        let from_wallet = self
            .find_wallet_by_discord_id(&deposit_amount.from_discord_id)
            .await?;

        let target_wallet = self
            .find_wallet_by_discord_id(&deposit_amount.target_discord_id)
            .await?;

        let has_enougth_amount = from_wallet.amount >= deposit_amount.amount;

        if !has_enougth_amount {
            return Err(CustomError::OutOfFunds(
                "User has not enought founds".to_string(),
            ));
        }

        let new_target_amount = target_wallet.amount + deposit_amount.amount;
        let new_from_amount = from_wallet.amount - deposit_amount.amount;

        sqlx::query("Update WALLET set amount=? where member_id=?")
            .bind(new_target_amount)
            .bind(target_wallet.id)
            .execute(conn)
            .await?;

        sqlx::query("Update WALLET set amount=? where member_id=?")
            .bind(new_from_amount)
            .bind(from_wallet.id)
            .execute(conn)
            .await?;

        tx.commit().await?;

        Ok(true)
    }

    async fn find_wallet_by_discord_id(&self, discord_id: &str) -> Result<Wallet, CustomError> {
        let conn = &*self.conn;

        let result = sqlx::query_as::<_, Wallet>(
            "Select * from WALLET inner join MEMBERS m where m.discord_id=?",
        )
        .bind(discord_id)
        .fetch_one(conn)
        .await?;

        Ok(result)
    }
}
