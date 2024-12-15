use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize)]
struct CustomError {
    message: String,
    status_code: u16,
}

pub fn validate_input<T: Validate>(input: &T) -> Result<(), HttpResponse> {
    match input.validate() {
        Ok(_) => return Ok(()),
        Err(message) => {
            return Err(HttpResponse::BadRequest().json(CustomError {
                message: message.to_string(),
                status_code: 400,
            }));
        }
    };
}