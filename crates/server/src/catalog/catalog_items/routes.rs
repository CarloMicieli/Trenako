use crate::catalog::catalog_items::post_catalog_item::{PgNewCatalogItemRepository, PgNewRollingStockRepository};
use crate::web::problem_detail::ProblemDetail;
use actix_web::http::header::LOCATION;
use actix_web::{web, HttpResponse, Responder};
use catalog::catalog_items::catalog_item_id::CatalogItemId;
use catalog::catalog_items::catalog_item_request::CatalogItemRequest;
use catalog::catalog_items::commands::new_catalog_item::create_new_catalog_item;
use catalog::catalog_items::rolling_stock_id::RollingStockId;
use catalog::catalog_items::rolling_stock_request::RollingStockRequest;
use common::unit_of_work::postgres::PgDatabase;
use sqlx::PgPool;
use tracing_actix_web::RequestId;
use web::Data;

pub const CATALOG_ITEM_ROOT_API: &str = "/api/catalog-items";

pub fn configure_catalog_items_routes(cfg: &mut web::ServiceConfig) {
    #[rustfmt::skip]
    cfg.service(
    web::scope(CATALOG_ITEM_ROOT_API)
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

async fn post_catalog_item(
    request_id: RequestId,
    request: web::Json<CatalogItemRequest>,
    db_pool: Data<PgPool>,
) -> impl Responder {
    let repo = PgNewCatalogItemRepository;
    let rr_repo = PgNewRollingStockRepository;
    let database = PgDatabase::new(&db_pool);

    let result = create_new_catalog_item(request.0, repo, rr_repo, database).await;
    match result {
        Ok(created) => {
            let location = format!("{}/{}", CATALOG_ITEM_ROOT_API, created.catalog_item_id);
            HttpResponse::Created().insert_header((LOCATION, location)).finish()
        }
        Err(why) => {
            tracing::error!("{:?}", why);
            ProblemDetail::from_error(*request_id, &why.to_string()).to_response()
        }
    }
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
