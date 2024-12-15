use utoipa::OpenApi;

use crate::resources::users::docs::UserApiDoc;

#[derive(OpenApi)]
#[openapi(info(title = "RustAPI example", version = "1.0.0"))]
pub struct BaseApiDoc;

pub fn build_openapi() -> utoipa::openapi::OpenApi {
    let mut openapi = BaseApiDoc::openapi();

    // merge all docs here
    openapi.merge(UserApiDoc::openapi());

    return openapi
}
