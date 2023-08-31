use actix_web::{get, post, HttpResponse, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[cfg(test)]
pub mod integration_tests {
    use super::*;
    use actix_web::{http::header::ContentType, test, App};

    #[actix_web::test]
    async fn test_index_get() {
        let app = test::init_service(App::new().service(index)).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
        let bytes = actix_http::body::to_bytes(res.into_body()).await.unwrap();
        assert!(std::str::from_utf8(&bytes).unwrap().contains("Hey there!"));
    }
}
