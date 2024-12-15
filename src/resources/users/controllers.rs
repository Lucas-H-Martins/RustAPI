use actix_web::{web::Json, HttpResponse, Responder};

use crate::{middlewares::validate::validate_input, resources::users::models::UserRequest};

// receive the request wherem sample validations and call service

pub async fn create_user(body: Json<UserRequest>) -> impl Responder {
    let user = body.into_inner();

    if let Err(response) = validate_input(&user) {
        return response;
    }

    HttpResponse::Created().json(user)
}

pub async fn list_user(body: Json<UserRequest>) -> impl Responder {
    let user = body.into_inner();

    if let Err(response) = validate_input(&user) {
        return response;
    }

    HttpResponse::Created().json(user)
}
