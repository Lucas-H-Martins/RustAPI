mod docs;
mod errors;
mod infrastructure;
mod middlewares;
mod resources;
use std::io;

use actix_web::{middleware, App, HttpServer};

use infrastructure::{configure_env, configure_logger};
use resources::users::routes::user_routes;
// use routes::users::{create_user, welcome};
use tracing::info;

use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> io::Result<()> {
    // configure env file
    let _ = configure_env();
    // configure logger
    let _ = configure_logger();

    //init api
    info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            // enable automatic response compression - usually register this first
            .wrap(middleware::Compress::default())
            // // enable logger - always register Actix Web Logger middleware last
            .wrap(middleware::Logger::default())
            .configure(user_routes)
            // Serve the OpenAPI
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", docs::open_api::build_openapi()),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}
