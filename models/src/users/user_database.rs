use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct UserDB {
    name: String,
    email: String,
    age: u32,
    created_at: String,
    updated_at: String,
    deleted_at: String,
}
