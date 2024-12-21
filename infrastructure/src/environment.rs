use std::env;

use commons::error::CustomErrors;
use tracing::{info, level_filters::LevelFilter};

use crate::models::{Database, Identitty};

#[derive(Debug, Default)]
pub struct Config {
    pub database: Database,
    pub identity: Identitty,
}

impl Config {
    pub fn from_env() -> Result<Self, CustomErrors> {
        configure_env()?;
        configure_logger()?;

        let mut config = Self::default();

        config.load_envs();

        Ok(config)
    }

    fn load_envs(&mut self) {
        //database
        self.database.user = env::var("POSTGRES_USER").unwrap_or("postgres".into());
        self.database.pass = env::var("POSTGRES_PASS").unwrap_or("postgres".into());
        self.database.host = env::var("POSTGRES_HOST").unwrap_or("localhost".into());
        self.database.port = env::var("POSTGRES_PORT").unwrap_or("5432".into());
        self.database.db = env::var("POSTGRES_DB").unwrap_or("postgres".into());
    }
}

fn configure_env() -> Result<(), CustomErrors> {
    let rust_env = env::var("RUST_ENV").unwrap_or_else(|_| "develop".to_string());

    let env_file = match rust_env.as_str() {
        "develop" => ".env.develop",
        "staging" => ".env.staging",
        "prod" => ".env.prod",
        _ => ".env.develop",
    };

    match dotenv::from_filename(env_file) {
        Ok(_) => Ok(()),
        Err(error) => Err(CustomErrors::EnvFileNotFound(error.to_string())),
    }
}

fn configure_logger() -> Result<(), CustomErrors> {
    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "trace".to_string());

    let level_filter = match log_level.to_lowercase().as_str() {
        "trace" => LevelFilter::TRACE,
        "debug" => LevelFilter::DEBUG,
        "info" => LevelFilter::INFO,
        "warn" => LevelFilter::WARN,
        "error" => LevelFilter::ERROR,
        _ => {
            eprintln!(
                "LOG_LEVEL invalid: '{}', using 'info' as default.",
                log_level
            );
            LevelFilter::DEBUG
        }
    };

    tracing_subscriber::fmt()
        .with_max_level(level_filter)
        .init();

    info!("running with log level {}", log_level);

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::{env, sync::Mutex};
//     static TEST_MUTEX: Mutex<()> = Mutex::new(()); // lock process for not error on multithread

//     #[test]
//     fn test_configure_env_develop() {
//         let _lock = TEST_MUTEX.lock().unwrap(); // lock test

//         // arrange
//         env::set_var("RUST_ENV", "develop");
//         // act
//         let result = configure_env();
//         // result
//         assert!(result.is_ok());
//         assert_eq!(env::var("RUST_ENV").unwrap(), "develop");
//     }

//     #[test]
//     fn test_configure_env_staging() {
//         let _lock = TEST_MUTEX.lock().unwrap(); // lock test

//         // arrange
//         env::set_var("RUST_ENV", "staging");
//         // act
//         let result = configure_env();
//         // result
//         assert!(result.is_ok());
//         assert_eq!(env::var("RUST_ENV").unwrap(), "staging");
//     }

//     #[test]
//     fn test_configure_env_prod() {
//         let _lock = TEST_MUTEX.lock().unwrap(); // lock test

//         // arrange
//         env::set_var("RUST_ENV", "prod");
//         // act
//         let result = configure_env();
//         // result
//         assert!(result.is_ok());
//         assert_eq!(env::var("RUST_ENV").unwrap(), "prod");
//     }

//     #[test]
//     fn test_configure_env_unrecognized() {
//         let _lock = TEST_MUTEX.lock().unwrap(); // lock test

//         // arrange
//         env::set_var("RUST_ENV", "invalid_env");
//         // act
//         let result = configure_env();
//         // result
//         assert!(result.is_ok());
//         assert_eq!(env::var("RUST_ENV").unwrap(), "invalid_env");
//     }
// }
