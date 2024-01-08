use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use env_logger;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "trace");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .service(hello)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}