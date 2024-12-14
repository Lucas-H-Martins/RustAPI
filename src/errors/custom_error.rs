use thiserror::Error;
#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum Errors {
    #[error("environment file not found")]
    EnvFileNotFound(String),
}