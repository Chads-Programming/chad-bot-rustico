// use chrono::NaiveDateTime;
use serde::Deserialize;
// use sqlx::types::Uuid;

#[derive(Deserialize, sqlx::FromRow, Debug, Clone)]
pub struct Project {
    // pub id: Uuid,
    pub title: String,
    pub description: String,
    pub owner_name: String,
    //    pub created_at: NaiveDateTime,
    //    pub updated_at: NaiveDateTime
}

#[derive(Debug, Clone)]
pub struct CreateProject {
    pub title: String,
    pub description: String,
    pub owner_name: String,
}

#[derive(Debug, Clone)]
pub struct FetchProjectsResponse {
    pub data: Vec<Project>,
    pub total: i64,
}
