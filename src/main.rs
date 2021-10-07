mod handlers;
use handlers::{hello, echo};
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
