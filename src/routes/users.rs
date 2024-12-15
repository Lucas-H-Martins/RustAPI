use actix_web::{post, web::Json, HttpResponse, Responder};

use crate::{middlewares::validate::validate_input, models::users::UserRequest};

#[post("/v1/user")]
pub async fn create_user(body: Json<UserRequest>) -> impl Responder {
    let user = body.into_inner();

    if let Err(response) = validate_input(&user) {
        return response;
    }

    HttpResponse::Created().json(user)
}
