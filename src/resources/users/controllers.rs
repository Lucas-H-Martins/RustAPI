use actix_web::{web::Json, HttpResponse, Responder};

use crate::{
    middlewares::validate::validate_input,
    resources::users::models::{UserRequest, UserResponse},
};

#[utoipa::path(
    post,
    path = "/v1/user",
    request_body = UserRequest,
    responses(
        (status = 201, description = "User created successfully", body = UserResponse),
        (status = 400, description = "Validation error")
    )
)]
pub async fn create_user(body: Json<UserRequest>) -> impl Responder {
    let user = body.into_inner();

    if let Err(response) = validate_input(&user) {
        return response;
    }

    HttpResponse::Created().json(user)
}
