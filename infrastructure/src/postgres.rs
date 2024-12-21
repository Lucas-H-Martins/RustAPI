use async_trait::async_trait;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tracing::info;

use crate::Config;
// todo 
// for high level software need a abstration for another tipes of database, i am not using this, but another abstration here is needed.
#[async_trait]
pub trait Database {
    fn pool(&self) -> &Pool<Postgres>;

    async fn connect(config: Config) -> Result<Pool<Postgres>, sqlx::Error>
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

    async fn connect(config: Config) -> Result<Pool<Postgres>, sqlx::Error> {
        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            config.database.user,
            config.database.pass,
            config.database.host,
            config.database.port,
            config.database.db
        );
        info!(
            "Connecting in database with host {} on port {}.",
            config.database.host, config.database.port
        );

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        Ok(pool)
    }
}
