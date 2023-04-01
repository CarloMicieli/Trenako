use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

pub async fn handler(db_pool: web::Data<PgPool>) -> impl Responder {
    let is_database_connected = sqlx::query("SELECT 1").fetch_one(&**db_pool).await.is_ok();

    if is_database_connected {
        HttpResponse::Ok()
    } else {
        HttpResponse::ServiceUnavailable()
    }
}
