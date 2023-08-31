use actix_web::{middleware, web, App, HttpServer};

mod routes;
mod template;
use routes::*;
use template::TEMPLATES;

use actix_files as fs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    log::info!("starting HTTP server at http://localhost:8080");
    log::info!("Current working directory: {:?}", std::env::current_dir());

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(TEMPLATES.clone()))
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .service(
                fs::Files::new("/static", "static/")
                    .show_files_listing()
                    .use_last_modified(true),
            )
            .service(index)
            .service(old)
            .service(test)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
