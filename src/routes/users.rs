use actix_web::{post, HttpRequest, HttpResponse, Responder};
// use actix_web_validator::Json;

#[post("/v1/user")]
pub async fn create_user(_: HttpRequest) -> impl Responder {
    HttpResponse::Created().json({})
}
