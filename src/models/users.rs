use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserRequest {
    #[validate(length(min = 3, message = "Name must be at least 3 characters long"))]
    name: String,
    #[validate(email(message = "Invalid email address"))]
    email: String,
    #[validate(range(min = 18, message = "Age must be at least 18"))]
    age: u8,
}