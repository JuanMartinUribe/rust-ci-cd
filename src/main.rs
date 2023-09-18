use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8081))?
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
    
        // Create request object
        let req = test::TestRequest::with_uri("/test").to_request();
    
        // Execute application
        let res = app.call(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

}