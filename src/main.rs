use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hola Mundo!!!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
#[cfg(test)]
mod tests {
    use actix_web::{test, web, App, HttpResponse, http::StatusCode};
    use actix_service::Service;

    #[actix_web::test]
    async fn test_init_service() {
        let app = test::init_service(
            App::new()
                .service(web::resource("/test").to(|| async { "OK" }))
        ).await;
    
        let req = test::TestRequest::with_uri("/test").to_request();
    
        let res = app.call(req).await.unwrap();
        let status_code = res.status().to_string();
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

}