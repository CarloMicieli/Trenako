use crate::catalog::brands::handlers;
use actix_web::web;

pub const BRAND_ROOT_API: &str = "/api/brands";

pub fn configure_brand_routes(cfg: &mut web::ServiceConfig) {
    #[rustfmt::skip]
    cfg.service(
    web::scope(BRAND_ROOT_API)
        .service(
            web::resource("")
                .route(web::get().to(handlers::get_all_brands))
                .route(web::post().to(handlers::post_brand))
        )
        .service(
            web::resource("/{brand}")
                .route(web::delete().to(handlers::delete_brand))
                .route(web::get().to(handlers::get_brand_by_id))
                .route(web::put().to(handlers::put_brand))
        )
    );
}
