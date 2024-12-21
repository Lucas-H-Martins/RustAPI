use async_trait::async_trait;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;
use tracing::info;

#[async_trait]
pub trait Database {
    fn pool(&self) -> &Pool<Postgres>;

    async fn connect() -> Result<Self, sqlx::Error>
    where
        Self: Sized;
}

pub struct PostgresDB {
    pool: Pool<Postgres>,
}

#[async_trait]
impl Database for PostgresDB {
    fn pool(&self) -> &Pool<Postgres> {
        &self.pool
    }

    async fn connect() -> Result<Self, sqlx::Error> {
        let user = env::var("POSTGRES_USER").unwrap_or_else(|_| "postgres".to_string());
        let password = env::var("POSTGRES_PASS").unwrap_or_else(|_| "postgres".to_string());
        let host = env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = env::var("POSTGRES_PORT").unwrap_or_else(|_| "5432".to_string());
        let database = env::var("POSTGRES_DB").unwrap_or_else(|_| "postgres".to_string());

        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            user, password, host, port, database
        );
        info!(
            "Connecting in database with host {} on port {}.",
            host, port
        );

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        Ok(Self { pool })
    }
}
