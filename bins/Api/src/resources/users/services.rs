// todo logic to process request where
use async_trait::async_trait;
use std::sync::Arc;
use tracing::error;

use commons::error::CustomErrors;
use models::users::{UserCreateRequest, UserCreateResponse};
use repositories::users::UserRepository;

use super::UserServices;

pub struct UserServicesImpl {
    user_repository: Arc<dyn UserRepository>,
}

impl UserServicesImpl {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl UserServices for UserServicesImpl {
    async fn create_user(
        &self,
        user_infos: &UserCreateRequest,
    ) -> Result<UserCreateResponse, CustomErrors> {
        let user = match self.user_repository.create_user(&user_infos).await {
            Ok(user) => user,
            Err(err) => {
                //
                error!(
                    "failed to create user for user_infos: {:?} error:{}",
                    &user_infos, &err
                );
                return Err(err);
            }
        };

        todo!()
    }
}
