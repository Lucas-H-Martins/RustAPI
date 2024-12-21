use std::sync::Arc;

use async_trait::async_trait;
use commons::error::CustomErrors;
use models::users::{UserCreateRequest, UserDB};
use sqlx::{Pool, Postgres};

use super::UserRepository;

pub struct UsersRepositorioImpl {
    pool: Arc<Pool<Postgres>>,
}

impl UsersRepositorioImpl {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Arc<Self> {
        Arc::new(Self { pool })
    }
}

#[async_trait]
impl UserRepository for UsersRepositorioImpl {
    async fn create_user(&self, user_infos: &UserCreateRequest) -> Result<UserDB, CustomErrors> {
        todo!()
    }
}
