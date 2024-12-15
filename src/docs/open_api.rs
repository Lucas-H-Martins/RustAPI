use utoipa::OpenApi;

use crate::resources::users::{controllers as user_ctl, models as user_model};

#[derive(OpenApi)]
#[openapi(
    paths(user_ctl::create_user),
    components(schemas(user_model::UserRequest, user_model::UserResponse)),
    info(title = "RustAPI", version = "1.0.0")
)]
pub struct ApiDoc;
