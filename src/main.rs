use actix_web :: {get, App, HttpResponse, HttpServer, Responder};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(health_check)
    })
        .bind(("127.0.0.1", 8080))
        .expect("Port taken")
        .run()
        .await
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}