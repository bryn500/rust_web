use actix_web::{middleware, App, HttpServer};
mod routes;
use routes::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    log::info!("starting HTTP server at http://localhost:8080");
    log::info!("Current working directory: {:?}", std::env::current_dir());

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
            .service(echo)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
