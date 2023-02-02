use actix_web::{web, HttpResponse, Responder};
use catalog::brands::brand_id::BrandId;
use sqlx::PgPool;

pub async fn handle(_brand_id: web::Path<BrandId>, _db_pool: web::Data<PgPool>) -> impl Responder {
    HttpResponse::Ok()
}
