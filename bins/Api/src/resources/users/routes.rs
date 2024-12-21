use actix_web::web::{self, post, scope};

use super::controllers::create_user;

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(scope("/v1/user").route("", post().to(create_user)));
}
