use actix_web::{web, HttpResponse, Responder};

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("REST endpoint is working!")
}
