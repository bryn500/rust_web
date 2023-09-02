use actix_web::{error, get, post, web, Error, HttpResponse, Responder, Result};
use reqwest::Error as ReqwestError;
use serde::{Deserialize, Serialize};
use std::path::Path as stdPath;
use std::sync::atomic::{AtomicUsize, Ordering};
use tera::{Context, Tera};

#[get("/")]
pub async fn index(tera: web::Data<Tera>) -> Result<impl Responder, Error> {
    log::info!("serving index");
    let mut context = Context::new();
    context.insert("title", "Home");
    let path = stdPath::new("templates/markdown/test.md");
    let result = markdown::file_to_html(path).map_err(error::ErrorInternalServerError)?;
    context.insert("markdown", &result);

    let template = tera.render("pages/home.html", &context).expect("Error");
    Ok(HttpResponse::Ok().body(template))
}

#[get("/old")]
pub async fn old() -> Result<impl Responder, Error> {
    log::info!("serving old");
    let path = stdPath::new("templates/markdown/test.md");

    let result = markdown::file_to_html(path).map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(result))

    // same thing using match
    // let result = markdown::file_to_html(path);
    // match result {
    //     Ok(value) => Ok(HttpResponse::Ok().body(value)),
    //     Err(error) => Err(error::ErrorInternalServerError(error)),
    // }
}

#[get("/test")]
async fn test(tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("title", "Test");
    context.insert("markdown", "Hello markdown");
    let template = tera.render("pages/home.html", &context).expect("Error");
    HttpResponse::Ok().body(template)
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[derive(Debug, Deserialize, Serialize)]
struct ApiResponse {
    message: String,
    field2: i32,
}

async fn fetch_api_data() -> Result<ApiResponse, ReqwestError> {
    let api_url = "http://127.0.0.1:9000";
    let response = reqwest::get(api_url).await?;
    let api_data: ApiResponse = response.json().await?;
    Ok(api_data)
}

#[get("/api-data")]
pub async fn get_api_data() -> impl Responder {
    match fetch_api_data().await {
        Ok(api_data) => HttpResponse::Ok().json(api_data),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn increment_counter() -> usize {
    // Increment the counter
    COUNTER.fetch_add(1, Ordering::SeqCst)
}

#[post("/counter")]
pub async fn counter(tera: web::Data<Tera>) -> Result<impl Responder, Error> {
    let count = increment_counter();
    let mut context = Context::new();
    context.insert("count", &count);
    let template = tera.render("partials/count.html", &context).expect("Error");
    Ok(HttpResponse::Ok().body(template))
}

#[cfg(test)]
pub mod integration_tests {
    use super::*;
    use crate::template::TEMPLATES;
    use actix_web::{http::header::ContentType, test, web, App};
    use std::path::Path;

    #[test]
    async fn test_markdown_from_file() {
        let path = Path::new("templates/markdown/test.md");
        let result = markdown::file_to_html(path);

        match result {
            Ok(value) => {
                // println!("Test: {0}", value);
                assert!(value.contains("<h1 id='test'>"));
            }
            Err(error) => {
                println!("Error: {}", error);
                assert_eq!(true, false);
            }
        }
    }

    #[actix_web::test]
    async fn test_index_get() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(TEMPLATES.clone()))
                .service(index),
        )
        .await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
        let bytes = actix_http::body::to_bytes(res.into_body()).await.unwrap();
        assert!(std::str::from_utf8(&bytes).unwrap().contains("<h1 id="));
    }

    #[actix_web::test]
    async fn test_manual_hello_get() {
        let app = test::init_service(App::new().route("/", web::get().to(manual_hello))).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
        let bytes = actix_http::body::to_bytes(res.into_body()).await.unwrap();
        assert!(std::str::from_utf8(&bytes).unwrap().contains("Hey there!"));
    }
}
