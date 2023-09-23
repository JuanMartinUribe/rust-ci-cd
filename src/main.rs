use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hola mundo!!!")
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
    use actix_web::{test, web, App, http::StatusCode};
    use actix_service::Service;

    #[actix_web::test]
    async fn test_init_service() {
        let app = test::init_service(
            App::new()
                .service(web::resource("/test").to(|| async { "OK" }))
        ).await;
    
        let req = test::TestRequest::with_uri("/test").to_request();
    
        let res = app.call(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

}