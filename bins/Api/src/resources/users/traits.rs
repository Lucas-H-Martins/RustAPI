use async_trait::async_trait;
use commons::error::CustomErrors;
use models::users::{UserCreateRequest, UserCreateResponse};

#[async_trait]
pub trait UserServices: Send + Sync {
    async fn create_user(
        &self,
        user_infos: &UserCreateRequest,
    ) -> Result<UserCreateResponse, CustomErrors>;
}
