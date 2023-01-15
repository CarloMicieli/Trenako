use actix_web::{web, HttpResponse, Responder};
use catalog::catalog_items::catalog_item_id::CatalogItemId;
use catalog::catalog_items::catalog_item_request::CatalogItemRequest;
use catalog::catalog_items::rolling_stock_id::RollingStockId;
use catalog::catalog_items::rolling_stock_request::RollingStockRequest;
use sqlx::PgPool;

pub async fn get_catalog_item_by_id(
    catalog_item_id: web::Path<CatalogItemId>,
    _db_pool: web::Data<PgPool>,
) -> impl Responder {
    println!("{}", catalog_item_id);
    HttpResponse::Ok()
}

pub async fn get_all_catalog_items(_db_pool: web::Data<PgPool>) -> impl Responder {
    HttpResponse::Ok()
}

pub async fn delete_catalog_item(
    catalog_item_id: web::Path<CatalogItemId>,
    _db_pool: web::Data<PgPool>,
) -> impl Responder {
    println!("{}", catalog_item_id);
    HttpResponse::Ok()
}

pub async fn put_catalog_item(
    catalog_item_id: web::Path<CatalogItemId>,
    request: web::Json<CatalogItemRequest>,
    _db_pool: web::Data<PgPool>,
) -> impl Responder {
    println!("{}", catalog_item_id);
    println!("{:?}", request);
    HttpResponse::Ok()
}

pub async fn post_catalog_item(request: web::Json<CatalogItemRequest>, _db_pool: web::Data<PgPool>) -> impl Responder {
    println!("{:?}", request);
    HttpResponse::Ok()
}

pub async fn post_rolling_stock(
    catalog_item_id: web::Path<CatalogItemId>,
    request: web::Json<RollingStockRequest>,
    _db_pool: web::Data<PgPool>,
) -> impl Responder {
    println!("{}", catalog_item_id);
    println!("{:?}", request);
    HttpResponse::Ok()
}

pub async fn get_rolling_stock_by_id(
    catalog_item_id: web::Path<CatalogItemId>,
    rolling_stock_id: web::Path<RollingStockId>,
    _db_pool: web::Data<PgPool>,
) -> impl Responder {
    println!("{}", catalog_item_id);
    println!("{}", rolling_stock_id);
    HttpResponse::Ok()
}

pub async fn get_all_rolling_stocks(
    catalog_item_id: web::Path<CatalogItemId>,
    _db_pool: web::Data<PgPool>,
) -> impl Responder {
    println!("{}", catalog_item_id);
    HttpResponse::Ok()
}

pub async fn delete_rolling_stock(
    catalog_item_id: web::Path<CatalogItemId>,
    rolling_stock_id: web::Path<RollingStockId>,
    _db_pool: web::Data<PgPool>,
) -> impl Responder {
    println!("{}", catalog_item_id);
    println!("{}", rolling_stock_id);
    HttpResponse::Ok()
}

pub async fn put_rolling_stock(
    catalog_item_id: web::Path<CatalogItemId>,
    rolling_stock_id: web::Path<RollingStockId>,
    request: web::Json<RollingStockRequest>,
    _db_pool: web::Data<PgPool>,
) -> impl Responder {
    println!("{}", catalog_item_id);
    println!("{}", rolling_stock_id);
    println!("{:?}", request);
    HttpResponse::Ok()
}
