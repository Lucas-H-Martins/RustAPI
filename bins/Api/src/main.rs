mod docs;
mod middlewares;
mod resources;
use std::{
    io::{Error, ErrorKind},
    sync::Arc,
};

use actix_web::{middleware, App, HttpServer};

use infrastructure::{Config, Database, PostgresDB};
use resources::users::routes::user_routes;

use tracing::info;

use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    let config = match Config::from_env() {
        Ok(config) => config,
        Err(err) => {
            tracing::error!("Error to load configs: {}", err);
            return Err(Error::new(ErrorKind::Other, err));
        }
    };

    let db_con = match PostgresDB::connect(config).await {
        Ok(pool) => Arc::new(pool),
        Err(err) => {
            tracing::error!("Error to configure database: {}", err);
            return Err(Error::new(ErrorKind::Other, err));
        }
    };

    //init api
    info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            //let here to call for routes of aplication (resourses)
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
