use actix_web::web;

use super::controllers::create_user;

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/v1/user").route("", web::post().to(create_user)));
}
