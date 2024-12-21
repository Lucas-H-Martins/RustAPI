use async_trait::async_trait;
use commons::error::CustomErrors;
use models::users::{UserCreateRequest, UserDB};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user_infos: &UserCreateRequest) -> Result<UserDB, CustomErrors>;
}
