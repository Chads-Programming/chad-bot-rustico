use std::sync::Arc;
use crate::{db::ConnectionPool, errors::CustomError};

use super::models::CreateProject;

pub struct ProjectRepository {
    conn: Arc<ConnectionPool>,
}


impl ProjectRepository {
    pub fn new(conn: Arc<ConnectionPool>) -> Self {
        Self {
            conn,
        }
    }

    pub async fn create(&self, create_project: &CreateProject) -> Result<bool, CustomError> {
        let conn = &*self.conn;

        let result =
            sqlx::query("INSERT INTO PROJECTS (title, description, owner_name) values ($1, $2, $3)")
                .bind(create_project.title.clone())
                .bind(create_project.description.clone())
                .bind(create_project.owner_name.clone())
                .execute(conn)
                .await;

        if result.is_err() {
            println!("Error on create project {:?}", result.err());

            return Err(CustomError::InternalError(format!(
                "Error on create project: {0}",
                create_project.title
            )));
        }

        Ok(true)
    }

}