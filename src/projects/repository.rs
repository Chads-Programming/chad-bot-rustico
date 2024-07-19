use crate::{db::ConnectionPool, errors::CustomError};
use std::sync::Arc;

use super::models::{CreateProject, FetchProjectsResponse, Project};

pub struct ProjectRepository {
    conn: Arc<ConnectionPool>,
}

impl ProjectRepository {
    pub fn new(conn: Arc<ConnectionPool>) -> Self {
        Self { conn }
    }

    pub async fn create(&self, create_project: &CreateProject) -> Result<bool, CustomError> {
        let conn = &*self.conn;

        let result = sqlx::query(
            "INSERT INTO PROJECTS (title, description, owner_name) values ($1, $2, $3)",
        )
        .bind(create_project.title.clone())
        .bind(create_project.description.clone())
        .bind(create_project.owner_name.clone())
        .execute(conn)
        .await;

        if result.is_err() {
            return Err(CustomError::InternalError(format!(
                "Error on create project: {0}",
                create_project.title
            )));
        }

        Ok(true)
    }

    pub async fn find_all(&self) -> Result<FetchProjectsResponse, CustomError> {
        let conn = &*self.conn;

        let search_query = "SELECT * from public.PROJECTS";
        let count_query = "SELECT COUNT(id) from public.PROJECTS";

        let fecth_result = sqlx::query_as::<_, Project>(search_query)
            .fetch_all(conn)
            .await;

        let count_result = sqlx::query_scalar::<_, i64>(count_query)
            .fetch_one(conn)
            .await;

        if count_result.is_err() {
            println!("Db error: {:?}", count_result.err().unwrap());

            return Err(CustomError::FetchError(String::from("Error on fetching")));
        }

        if fecth_result.is_err() {
            println!("Db error: {:?}", fecth_result.err().unwrap());

            return Err(CustomError::InternalError(String::from(
                "Error on fetching",
            )));
        }

        let projects = fecth_result.unwrap();
        let count = count_result.unwrap();

        Ok(FetchProjectsResponse {
            data: projects,
            total: count,
        })
    }
}
