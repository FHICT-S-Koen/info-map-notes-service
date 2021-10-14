use actix_web::{get, post, HttpResponse, Responder};

#[get("/")]
pub async fn get_note_by_id() -> impl Responder {
    HttpResponse::Ok().body("note")
}

#[post("/")]
pub async fn create_note(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
