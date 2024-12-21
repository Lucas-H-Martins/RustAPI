use actix_web::HttpResponse;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum CustomErrors {
    #[error("internal error")]
    InternalError,
    #[error("environment file not found")]
    EnvFileNotFound(String),
    #[error("error querying database")]
    DatabaseError(String),
    #[error("validation error: {0}")]
    ValidationError(String),
    #[error("resource not found: {0}")]
    NotFound(String),
    #[error("unauthorized")]
    Unauthorized,
}

impl From<CustomErrors> for HttpResponse {
    fn from(error: CustomErrors) -> Self {
        match error {
            CustomErrors::InternalError => {
                HttpResponse::InternalServerError().json("Internal server error")
            }
            CustomErrors::EnvFileNotFound(message) => {
                HttpResponse::InternalServerError().json(format!("Environment file not found: {}", message))
            }
            CustomErrors::DatabaseError(message) => {
                HttpResponse::InternalServerError().json(format!("Database error: {}", message))
            }
            CustomErrors::ValidationError(message) => {
                HttpResponse::BadRequest().json(format!("Validation error: {}", message))
            }
            CustomErrors::NotFound(resource) => {
                HttpResponse::NotFound().json(format!("Resource not found: {}", resource))
            }
            CustomErrors::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized access"),
        }
    }
}
