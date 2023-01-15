use actix_web::{web, HttpResponse, Responder};
use catalog::scales::scale_id::ScaleId;
use catalog::scales::scale_request::ScaleRequest;
use sqlx::PgPool;

pub async fn get_scale_by_id(scale_id: web::Path<ScaleId>, _db_pool: web::Data<PgPool>) -> impl Responder {
    println!("{}", scale_id);
    HttpResponse::Ok()
}

pub async fn get_all_scales(_db_pool: web::Data<PgPool>) -> impl Responder {
    HttpResponse::Ok()
}

pub async fn delete_scale(scale_id: web::Path<ScaleId>, _db_pool: web::Data<PgPool>) -> impl Responder {
    println!("{}", scale_id);
    HttpResponse::Ok()
}

pub async fn put_scale(
    scale_id: web::Path<ScaleId>,
    request: web::Json<ScaleRequest>,
    _db_pool: web::Data<PgPool>,
) -> impl Responder {
    println!("{}", scale_id);
    println!("{:?}", request);
    HttpResponse::Ok()
}

pub async fn post_scale(request: web::Json<ScaleRequest>, _db_pool: web::Data<PgPool>) -> impl Responder {
    println!("{:?}", request);
    HttpResponse::Ok()
}
