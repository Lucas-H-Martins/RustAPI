use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
// all data models implement here
#[derive(Debug, Deserialize, Validate, Serialize, ToSchema)]
pub struct UserCreateRequest {
    #[validate(length(min = 3))]
    name: String,
    #[validate(email)]
    email: String,
    #[validate(range(min = 13))]
    age: u32,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UserCreateResponse {
    id: i32,
    name: String,
    email: String,
    age: u32,
    created_at: String,
}
