use infrastructure::{Config, Database, PostgresDB};
use std::{
    io::{Error, ErrorKind},
    sync::Arc,
};
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = match Config::from_env() {
        Ok(config) => config,
        Err(err) => {
            tracing::error!("Error to load configs: {}", err);
            return Err(Error::new(ErrorKind::Other, err));
        }
    };

    let db_con = match PostgresDB::connect(config).await {
        Ok(pool) => Arc::new(pool),
        Err(err) => {
            tracing::error!("Error to configure database: {}", err);
            return Err(Error::new(ErrorKind::Other, err));
        }
    };

    match sqlx::migrate!("src/migrations/").run(&*db_con).await {
        Ok(_) => {
            info!("migrations sucessfull executed");
        }
        Err(err) => {
            error!("migrations are not executed err: {}", err)
        }
    }

    Ok(())
}
