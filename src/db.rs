use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub type ConnectionPool = Pool<Postgres>;

pub async fn get_pool(connection_uri: &str) -> ConnectionPool {
    PgPoolOptions::new()
        .connect(connection_uri)
        .await
        .expect("Error building connection pool")
}
