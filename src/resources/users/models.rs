use serde::{Deserialize, Serialize};
use validator::Validate;
// all data models implement here
#[derive(Debug, Deserialize, Validate,Serialize)]
pub struct UserRequest {
    #[validate(length(min = 3))]
    name: String,

    #[validate(email)]
    email: String,

    #[validate(range(min = 13))]
    age: u32,
}
