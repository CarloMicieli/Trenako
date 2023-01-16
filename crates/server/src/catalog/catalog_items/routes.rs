use actix_web::{web, HttpResponse, Responder};
use catalog::catalog_items::catalog_item_id::CatalogItemId;
use catalog::catalog_items::catalog_item_request::CatalogItemRequest;
use catalog::catalog_items::rolling_stock_id::RollingStockId;
use catalog::catalog_items::rolling_stock_request::RollingStockRequest;
use sqlx::PgPool;
use web::Data;

pub fn configure_catalog_items_routes(cfg: &mut web::ServiceConfig) {
    #[rustfmt::skip]
    cfg.service(
        web::scope("/api")
        .service(
            web::scope("/catalog-items")
                .service(
                    web::resource("")
                        .route(web::post().to(post_catalog_item))
                )
                .service(
                    web::scope("/{catalog_item_id}")
                        .service(
                            web::resource("")
                                .route(web::delete().to(delete_catalog_item))
                                .route(web::get().to(get_catalog_item_by_id))
                                .route(web::put().to(put_catalog_item))
                        )
                        .service(
                            web::scope("/rolling-stocks")
                                .service(
                                    web::resource("")
                                        .route(web::get().to(get_rolling_stock_by_id))
                                        .route(web::post().to(post_rolling_stock))
                                )
                                .service(
                                    web::resource("/{rolling_stock_id}")
                                        .route(web::delete().to(delete_rolling_stock))
                                        .route(web::get().to(get_rolling_stock_by_id))
                                        .route(web::put().to(put_rolling_stock))
                                )
                        )
                )
            )
    );
}

async fn get_catalog_item_by_id(catalog_item_id: web::Path<CatalogItemId>, _db_pool: Data<PgPool>) -> impl Responder {
    println!("{}", catalog_item_id);
    HttpResponse::Ok()
}

async fn delete_catalog_item(catalog_item_id: web::Path<CatalogItemId>, _db_pool: Data<PgPool>) -> impl Responder {
    println!("{}", catalog_item_id);
    HttpResponse::Ok()
}

async fn put_catalog_item(
    catalog_item_id: web::Path<CatalogItemId>,
    request: web::Json<CatalogItemRequest>,
    _db_pool: Data<PgPool>,
) -> impl Responder {
    println!("{}", catalog_item_id);
    println!("{:?}", request);
    HttpResponse::Ok()
}

async fn post_catalog_item(request: web::Json<CatalogItemRequest>, _db_pool: Data<PgPool>) -> impl Responder {
    println!("{:?}", request);
    HttpResponse::Ok()
}

async fn post_rolling_stock(
    catalog_item_id: web::Path<CatalogItemId>,
    request: web::Json<RollingStockRequest>,
    _db_pool: Data<PgPool>,
) -> impl Responder {
    println!("{}", catalog_item_id);
    println!("{:?}", request);
    HttpResponse::Ok()
}

async fn get_rolling_stock_by_id(
    catalog_item_id: web::Path<CatalogItemId>,
    rolling_stock_id: web::Path<RollingStockId>,
    _db_pool: Data<PgPool>,
) -> impl Responder {
    println!("{}", catalog_item_id);
    println!("{}", rolling_stock_id);
    HttpResponse::Ok()
}

async fn delete_rolling_stock(
    catalog_item_id: web::Path<CatalogItemId>,
    rolling_stock_id: web::Path<RollingStockId>,
    _db_pool: Data<PgPool>,
) -> impl Responder {
    println!("{}", catalog_item_id);
    println!("{}", rolling_stock_id);
    HttpResponse::Ok()
}

async fn put_rolling_stock(
    catalog_item_id: web::Path<CatalogItemId>,
    rolling_stock_id: web::Path<RollingStockId>,
    request: web::Json<RollingStockRequest>,
    _db_pool: Data<PgPool>,
) -> impl Responder {
    println!("{}", catalog_item_id);
    println!("{}", rolling_stock_id);
    println!("{:?}", request);
    HttpResponse::Ok()
}
