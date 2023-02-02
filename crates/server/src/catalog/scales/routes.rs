use crate::catalog::scales::handlers;
use actix_web::web;

pub const SCALE_ROOT_API: &str = "/api/scales";

pub fn configure_scale_routes(cfg: &mut web::ServiceConfig) {
    # [rustfmt::skip]
    cfg.service(
        web::scope(SCALE_ROOT_API)
            .service(
                web::resource("")
                    .route(web::get().to(handlers::get_all_scales))
                    .route(web::post().to(handlers::post_scale))
            )
            .service(
                web::resource("/{scale}")
                    .route(web::delete().to(handlers::delete_scale))
                    .route(web::get().to(handlers::get_scale_by_id))
                    .route(web::put().to(handlers::put_scale))
            )
    );
}
