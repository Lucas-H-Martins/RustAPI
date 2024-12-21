use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum CustomErrors {
    #[error("internal error")]
    InternalError,
    #[error("environment file not found")]
    EnvFileNotFound(String),
    // #[error("too many requests; retry after {1} seconds: {0}")]
    // TooManyRequests(String, u64),
}
