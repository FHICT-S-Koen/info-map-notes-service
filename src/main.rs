#[macro_use]
extern crate diesel;

use diesel::{PgConnection, r2d2, r2d2::ConnectionManager};
use diesel::prelude::*;
use actix_web::{dev::ServiceRequest, App, HttpServer, web, Error};
use handlers::{get_notes, get_note_by_id, add_note, delete_note};

mod errors;
mod handlers;
mod models;
mod schema;
mod auth;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(validator);
        App::new()
            .wrap(auth)
            .data(pool.clone())
            .route("/api/v1", web::get().to(get_notes))
            .route("/api/v1/{id}", web::get().to(get_note_by_id))
            .route("/api/v1", web::post().to(add_note))
            .route("/api/v1/{id}", web::delete().to(delete_note))
    })
    .bind("localhost:8080")?
    .run()
    .await
}

use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;

async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.get_ref().clone())
        .unwrap_or_else(Default::default);
    match auth::validate_token(credentials.token()) {
        Ok(res) => {
            if res {
                Ok(req)
            } else {
                Err(AuthenticationError::from(config).into())
            }
        }
        Err(_) => Err(AuthenticationError::from(config).into()),
    }
}
