use diesel::{PgConnection, r2d2::ConnectionManager};
use diesel::r2d2;
use actix_web::{App, HttpServer, web};
use handlers::{get_note_by_id, create_note};

mod handlers;
mod models;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new().data(pool.clone())
        .service(
            web::scope("/api/v1")
                .service(get_note_by_id)
                .service(create_note)
        )
    })
    .bind("localhost:8080")?
    .run()
    .await
}
