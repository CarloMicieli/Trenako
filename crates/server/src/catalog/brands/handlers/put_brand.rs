use actix_web::{web, HttpResponse, Responder};
use catalog::brands::brand_id::BrandId;
use catalog::brands::brand_request::BrandRequest;
use sqlx::PgPool;

pub async fn handle(
    _brand_id: web::Path<BrandId>,
    _request: web::Json<BrandRequest>,
    _db_pool: web::Data<PgPool>,
) -> impl Responder {
    HttpResponse::Ok()
}
