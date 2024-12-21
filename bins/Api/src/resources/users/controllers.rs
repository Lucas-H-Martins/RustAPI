use actix_web::{
    web::{Data, Json},
    HttpResponse, Responder,
};
use models::users::{UserCreateRequest, UserCreateResponse};
use tracing::error;

use crate::middlewares::validate_input;

use super::UserServices;

#[utoipa::path(
    post,
    path = "/v1/user",
    tag = "users",
    request_body = UserCreateRequest,
    responses(
        (status = 201, description = "User created successfully", body = UserCreateResponse),
        (status = 400, description = "Validation error")
    )
)]
pub async fn create_user(
    body: Json<UserCreateRequest>,
    services: Data<dyn UserServices>,
) -> impl Responder {
    let user_infos = body.into_inner();

    if let Err(err) = validate_input(&user_infos) {
        return HttpResponse::from(err);
    }

    //call here to service
    match services.create_user(&user_infos).await {
        Ok(user) => return HttpResponse::Created().json(user),
        Err(err) => {
            error!("failed to process service err:{}", err);
            return HttpResponse::from(err);
        }
    };
}
