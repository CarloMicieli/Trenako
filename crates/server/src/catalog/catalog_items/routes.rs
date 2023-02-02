use crate::catalog::catalog_items::handlers;
use actix_web::web;

pub const CATALOG_ITEM_ROOT_API: &str = "/api/catalog-items";

pub fn configure_catalog_items_routes(cfg: &mut web::ServiceConfig) {
    #[rustfmt::skip]
    cfg.service(
    web::scope(CATALOG_ITEM_ROOT_API)
        .service(
            web::resource("")
                .route(web::post().to(handlers::post_catalog_item))
        )
        .service(
            web::scope("/{catalog_item_id}")
                .service(
                    web::resource("")
                        .route(web::delete().to(handlers::delete_catalog_item))
                        .route(web::get().to(handlers::get_catalog_item_by_id))
                        .route(web::put().to(handlers::put_catalog_item))
                )
                .service(
                    web::scope("/rolling-stocks")
                        .service(
                            web::resource("")
                                .route(web::post().to(handlers::post_rolling_stock))
                        )
                        .service(
                            web::resource("/{rolling_stock_id}")
                                .route(web::delete().to(handlers::delete_rolling_stock))
                                .route(web::get().to(handlers::get_rolling_stock_by_id))
                                .route(web::put().to(handlers::put_rolling_stock))
                        )
                )
        )
    );
}
