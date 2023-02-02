use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

pub async fn handle(_db_pool: web::Data<PgPool>) -> impl Responder {
    HttpResponse::Ok()
}
