use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
// all data models implement here
#[derive(Debug, Deserialize, Validate, Serialize, ToSchema)]
pub struct UserCreateRequest {
    #[validate(length(min = 3))]
    pub name: String,
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UserCreateResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
}
