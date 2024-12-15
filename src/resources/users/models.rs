use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
// all data models implement here
#[derive(Debug, Deserialize, Validate, Serialize, ToSchema)]
pub struct UserRequest {
    #[validate(length(min = 3))]
    name: String,

    #[validate(email)]
    email: String,

    #[validate(range(min = 13))]
    age: u32,
}

#[derive(Debug, Deserialize, Validate, Serialize, ToSchema)]
pub struct UserResponse {
    #[validate(length(min = 3))]
    name: String,

    #[validate(email)]
    email: String,

    #[validate(range(min = 13))]
    age: u32,
}
