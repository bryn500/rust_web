// // use crate::cache::add_cache;
// use crate::config::CONFIG;
// use crate::routes::routes;
// // use actix_cors::Cors;
// use actix_web::{middleware::Logger, App, HttpServer};
// use listenfd::ListenFd;

// pub async fn server() -> std::io::Result<()> {
//     dotenv::dotenv().ok();
//     env_logger::init();

//     let mut listenfd = ListenFd::from_env();
//     let mut server = HttpServer::new(move || {
//         App::new()
//             // .configure(add_cache)
//             .wrap(Logger::default())
//             .configure(routes)
//     });

//     server = if let Some(l) = listenfd.take_tcp_listener(0)? {
//         server.listen(l)?
//     } else {
//         server.bind(&CONFIG.server)?
//     };

//     server.run().await
// }

// // use actix_web::{web, App, HttpServer};
// // mod routes;
// // use routes::*;

// // #[actix_web::main]
// // async fn main() -> std::io::Result<()> {
// //     HttpServer::new(|| {
// //         App::new()
// //             .service(index)
// //             .service(echo)
// //             .route("/hey", web::get().to(manual_hello))
// //     })
// //     .bind(("127.0.0.1", 8080))?
// //     .run()
// //     .await
// // }
