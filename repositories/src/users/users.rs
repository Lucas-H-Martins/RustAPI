use async_trait::async_trait;
use commons::error::CustomErrors;
use models::users::{UserCreateRequest, UserDB};
use sqlx::{query_as, Pool, Postgres};
use tracing::error;

use super::UserRepository;

pub struct UsersRepositorioImpl {
    pool: Pool<Postgres>,
}

impl UsersRepositorioImpl {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UsersRepositorioImpl {
    async fn create_user(&self, user_infos: &UserCreateRequest) -> Result<UserDB, CustomErrors> {
        let query = r#"
        INSERT INTO users (name, email)
        VALUES ($1, $2)
        RETURNING id, name, email, created_at, updated_at
    "#;
        let result = query_as::<_, UserDB>(query)
            .bind(&user_infos.name)
            .bind(&user_infos.email)
            .fetch_one(&self.pool)
            .await;

        match result {
            Ok(user) => Ok(user),
            Err(err) => {
                error!(
                    "failed to create ser for user: {:?} error:{}",
                    user_infos,
                    err.to_string()
                );

                Err(CustomErrors::DatabaseError(err.to_string()))
            }
        }
    }
}
