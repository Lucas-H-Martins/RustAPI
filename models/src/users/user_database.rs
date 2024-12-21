use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, FromRow};
#[derive(Default, Serialize, Deserialize, FromRow)]
pub struct UserDB {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
