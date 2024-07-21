use super::models::{CreateMember, DepositAmountFromDiscord, MinimalMemberWallet, Wallet};
use crate::{db::ConnectionPool, errors::CustomError};
use bigdecimal::{BigDecimal, FromPrimitive};
use sqlx::types::Uuid;
use sqlx::{self};
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

        if let Ok(existing_member) = already_exist_member_result {
            return Err(CustomError::AlreadyMemberExists(format!(
                "Member was registerd with same discord account id: {}",
                existing_member.member_id
            )));
        }

        let query_result: (Uuid,) =
            sqlx::query_as("SELECT * FROM create_member_with_wallet($1, $2)")
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

        let deposit_amount_parsed = BigDecimal::from_f64(deposit_amount.amount).unwrap();

        let has_enougth_amount = from_wallet.amount >= deposit_amount_parsed;

        if !has_enougth_amount {
            return Err(CustomError::OutOfFunds(
                "User has not enought founds".to_string(),
            ));
        }

        let new_target_amount = target_wallet.amount + deposit_amount_parsed.clone();
        let new_from_amount = from_wallet.amount - deposit_amount_parsed.clone();

        sqlx::query("Update WALLET set amount=$1 where member_id=$2")
            .bind(new_target_amount)
            .bind(target_wallet.id)
            .execute(conn)
            .await?;

        sqlx::query("Update WALLET set amount=$1 where member_id=$2")
            .bind(new_from_amount)
            .bind(from_wallet.id)
            .execute(conn)
            .await?;

        tx.commit().await?;

        Ok(true)
    }

    pub async fn find_wallet_by_discord_id(&self, discord_id: &str) -> Result<Wallet, CustomError> {
        let conn = &*self.conn;

        let result = sqlx::query_as::<_, Wallet>(
            "Select * from public.WALLET W JOIN public.MEMBER M ON M.id = W.member_id where M.discord_id=$1",
        )
        .bind(discord_id)
        .fetch_one(conn)
        .await?;

        Ok(result)
    }

    pub async fn find_members_sort_by_wallet_amount(
        &self,
    ) -> Result<Vec<MinimalMemberWallet>, CustomError> {
        let conn = &*self.conn;
        let result = sqlx::query_as::<_, MinimalMemberWallet>(
            "Select M.id, W.id as wallet_id, M.name, amount from public.WALLET W INNER JOIN public.MEMBER M ON M.id = W.member_id ORDER BY W.amount DESC LIMIT 10",
        )
        .fetch_all(conn)
        .await?;

        Ok(result)
    }
}
