use std::env;

use crate::errors::Errors;

pub fn configure_env() -> Result<(), Errors> {
    let rust_env = env::var("RUST_ENV").unwrap_or_else(|_| "develop".to_string());

    let env_file = match rust_env.as_str() {
        "develop" => ".env.develop",
        "staging" => ".env.staging",
        "prod" => ".env.prod",
        _ => ".env.develop",
    };

    match dotenv::from_filename(env_file) {
        Ok(_) => Ok(()),
        Err(error) => Err(Errors::EnvFileNotFound(error.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{env, sync::Mutex};
    static TEST_MUTEX: Mutex<()> = Mutex::new(()); // lock process for not error on multithread

    #[test]
    fn test_configure_env_develop() {
        let _lock = TEST_MUTEX.lock().unwrap(); // lock test

        // arrange
        env::set_var("RUST_ENV", "develop");
        // act
        let result = configure_env();
        // result
        assert!(result.is_ok());
        assert_eq!(env::var("RUST_ENV").unwrap(), "develop");
    }

    #[test]
    fn test_configure_env_staging() {
        let _lock = TEST_MUTEX.lock().unwrap(); // lock test

        // arrange
        env::set_var("RUST_ENV", "staging");
        // act
        let result = configure_env();
        // result
        assert!(result.is_ok());
        assert_eq!(env::var("RUST_ENV").unwrap(), "staging");
    }

    #[test]
    fn test_configure_env_prod() {
        let _lock = TEST_MUTEX.lock().unwrap(); // lock test

        // arrange
        env::set_var("RUST_ENV", "prod");
        // act
        let result = configure_env();
        // result
        assert!(result.is_ok());
        assert_eq!(env::var("RUST_ENV").unwrap(), "prod");
    }

    #[test]
    fn test_configure_env_unrecognized() {
        let _lock = TEST_MUTEX.lock().unwrap(); // lock test

        // arrange
        env::set_var("RUST_ENV", "invalid_env");
        // act
        let result = configure_env();
        // result
        assert!(result.is_ok());
        assert_eq!(env::var("RUST_ENV").unwrap(), "invalid_env");
    }
}
