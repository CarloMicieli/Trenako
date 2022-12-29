use actix_web::{web, HttpResponse, Responder};
use catalog::brands::brand_request::BrandRequest;
use sqlx::PgPool;

pub async fn get_brand_by_id(brand_id: web::Path<String>, _db_pool: web::Data<PgPool>) -> impl Responder {
    println!("{}", brand_id);
    HttpResponse::Ok()
}

pub async fn get_all_brands(_db_pool: web::Data<PgPool>) -> impl Responder {
    HttpResponse::Ok()
}

pub async fn delete_brand(brand_id: web::Path<String>, _db_pool: web::Data<PgPool>) -> impl Responder {
    println!("{}", brand_id);
    HttpResponse::Ok()
}

pub async fn put_brand(
    brand_id: web::Path<String>,
    request: web::Json<BrandRequest>,
    _db_pool: web::Data<PgPool>,
) -> impl Responder {
    println!("{}", brand_id);
    println!("{:?}", request);
    HttpResponse::Ok()
}

pub async fn post_brand(request: web::Json<BrandRequest>, _db_pool: web::Data<PgPool>) -> impl Responder {
    println!("{:?}", request);
    HttpResponse::Ok()
}
