use chrono::NaiveDateTime;



#[derive(sqlx::FromRow)]
#[sqlx(rename_all = "camelCase")]
pub struct Project {
   pub id: String,
   pub title: String,
   pub description: String,
   pub owner_name: String,

   pub created_at: NaiveDateTime,
   pub updated_at: NaiveDateTime
}


#[derive(Debug, Clone)]
pub struct CreateProject {
    pub title: String,
    pub description: String,
    pub owner_name: String,
}
