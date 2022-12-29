use actix_web::{web, HttpResponse, Responder};
use catalog::railways::railway_request::RailwayRequest;
use sqlx::PgPool;

pub async fn get_railway_by_id(railway_id: web::Path<String>, _db_pool: web::Data<PgPool>) -> impl Responder {
    println!("{}", railway_id);
    HttpResponse::Ok()
}

pub async fn get_all_railways(_db_pool: web::Data<PgPool>) -> impl Responder {
    HttpResponse::Ok()
}

pub async fn delete_railway(railway_id: web::Path<String>, _db_pool: web::Data<PgPool>) -> impl Responder {
    println!("{}", railway_id);
    HttpResponse::Ok()
}

pub async fn put_railway(
    railway_id: web::Path<String>,
    request: web::Json<RailwayRequest>,
    _db_pool: web::Data<PgPool>,
) -> impl Responder {
    println!("{}", railway_id);
    println!("{:?}", request);
    HttpResponse::Ok()
}

pub async fn post_railway(request: web::Json<RailwayRequest>, _db_pool: web::Data<PgPool>) -> impl Responder {
    println!("{:?}", request);
    HttpResponse::Ok()
}
