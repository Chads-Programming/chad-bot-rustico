use super::models::CreateMember;
use crate::{db::ConnectionPool, errors::CustomError};
use std::sync::Arc;

pub struct BankService {
    conn: Arc<ConnectionPool>,
}

impl BankService {
    async fn register_member(&self, create_member: &CreateMember) -> Result<bool, CustomError> {
        let conn = &*self.conn;

        let result = sqlx::query("call create_member_with_wallet(?, ?)")
            .bind(create_member.name.clone())
            .bind(create_member.discord_id.clone())
            .execute(conn)
            .await;

        if result.is_err() {
            println!("Error on register member {:?}", result.err());

            return Err(CustomError::CreationError(format!(
                "Error on create member: {0}",
                create_member.name
            )));
        }

        Ok(true)
    }

    async fn deposit_amount(&self, create_member: &CreateMember) -> Result<bool, CustomError> {
        let conn = &*self.conn;

        let result = sqlx::query("call create_member_with_wallet(?, ?)")
            .bind(create_member.name.clone())
            .bind(create_member.discord_id.clone())
            .execute(conn)
            .await;

        if result.is_err() {
            println!("Error on register member {:?}", result.err());

            return Err(CustomError::CreationError(format!(
                "Error on create member: {0}",
                create_member.name
            )));
        }

        Ok(true)
    }
}
