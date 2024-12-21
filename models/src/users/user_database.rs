use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Default, Serialize, Deserialize,FromRow)]
pub struct UserDB {
    pub id: String,
    pub name: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
}
