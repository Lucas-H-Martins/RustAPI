use actix_web::{web::Json, HttpResponse, Responder};
use models::users::{UserCreateRequest, UserCreateResponse};

use crate::middlewares::validate::validate_input;

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
pub async fn create_user(body: Json<UserCreateRequest>) -> impl Responder {
    let user = body.into_inner();

    if let Err(response) = validate_input(&user) {
        return response;
    }

    HttpResponse::Created().json(user)
}
