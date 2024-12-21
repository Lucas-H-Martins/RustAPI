use utoipa::OpenApi;

use crate::resources::users::{controllers as user_ctl, models as user_model};

#[derive(OpenApi)]
#[openapi(
    paths(user_ctl::create_user),
    components(schemas(user_model::UserCreateRequest, user_model::UserCreateResponse)),
    tags(
        (name = "users", description = "User endpoints."),
      ),
    info(title = "RustAPI Example", version = "1.0.0")
)]
pub struct UserApiDoc;
