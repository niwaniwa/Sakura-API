use actix_web::{App, HttpServer};

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| App::new())
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
