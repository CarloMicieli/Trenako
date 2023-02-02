use crate::catalog::railways::handlers;
use actix_web::web;

pub const RAILWAY_ROOT_API: &str = "/api/railways";

pub fn configure_railway_routes(cfg: &mut web::ServiceConfig) {
    #[rustfmt::skip]
    cfg.service(
    web::scope(RAILWAY_ROOT_API)
        .service(
            web::resource("")
                .route(web::get().to(handlers::get_all_railways))
                .route(web::post().to(handlers::post_railway))
        )
        .service(
            web::resource("/{railway}")
                .route(web::delete().to(handlers::delete_railway))
                .route(web::get().to(handlers::get_railway_by_id))
                .route(web::put().to(handlers::put_railway))
        )
    );
}
