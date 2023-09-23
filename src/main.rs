use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!!!")
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
                .service(web::resource("/").to(|| async { "OK" }))
        ).await;
    
        // Create request object
        let req = test::TestRequest::with_uri("/test").to_request();
    
        // Execute application
        let res = app.call(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

}