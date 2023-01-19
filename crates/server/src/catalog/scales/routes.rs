use actix_web::{web, HttpResponse, Responder};
use catalog::scales::scale_id::ScaleId;
use catalog::scales::scale_request::ScaleRequest;
use sqlx::PgPool;

pub const SCALE_ROOT_API: &str = "/api/scales";

pub fn configure_scale_routes(cfg: &mut web::ServiceConfig) {
    # [rustfmt::skip]
    cfg.service(
        web::scope(SCALE_ROOT_API)
            .service(
                web::resource("")
                    .route(web::get().to(get_all_scales))
                    .route(web::post().to(post_scale))
            )
            .service(
                web::resource("/{scale}")
                    .route(web::delete().to(delete_scale))
                    .route(web::get().to(get_scale_by_id))
                    .route(web::put().to(put_scale))
            )
    );
}

async fn get_scale_by_id(scale_id: web::Path<ScaleId>, _db_pool: web::Data<PgPool>) -> impl Responder {
    println!("{}", scale_id);
    HttpResponse::Ok()
}

async fn get_all_scales(_db_pool: web::Data<PgPool>) -> impl Responder {
    HttpResponse::Ok()
}

async fn delete_scale(scale_id: web::Path<ScaleId>, _db_pool: web::Data<PgPool>) -> impl Responder {
    println!("{}", scale_id);
    HttpResponse::Ok()
}

async fn put_scale(
    scale_id: web::Path<ScaleId>,
    request: web::Json<ScaleRequest>,
    _db_pool: web::Data<PgPool>,
) -> impl Responder {
    println!("{}", scale_id);
    println!("{:?}", request);
    HttpResponse::Ok()
}

async fn post_scale(request: web::Json<ScaleRequest>, _db_pool: web::Data<PgPool>) -> impl Responder {
    println!("{:?}", request);
    HttpResponse::Ok()
}
