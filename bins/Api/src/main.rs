mod docs;
mod middlewares;
mod resources;
use std::{
    io::{Error, ErrorKind},
    sync::Arc,
};

use actix_web::{middleware, web, App, HttpServer};
use infrastructure::{Config, Database, PostgresDB};
use repositories::users::UsersRepositorioImpl;
use resources::users::{routes::user_routes, services::UserServicesImpl, UserServices};
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
        Ok(pool) => pool,
        Err(err) => {
            tracing::error!("Error to configure database: {}", err);
            return Err(Error::new(ErrorKind::Other, err));
        }
    };

    // repositorys
    let user_repository = Arc::new(UsersRepositorioImpl::new(db_con));
    // sevices
    let user_service: Arc<dyn UserServices> = Arc::new(UserServicesImpl::new(user_repository));

    //init api
    info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            //inject all dependences using macro app_data
            .app_data(web::Data::new(user_service.clone()))
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
