use crate::catalog::brands::post_brands;
use actix_web::{web, HttpResponse, Responder};
use catalog::brands::brand_id::BrandId;
use catalog::brands::brand_request::BrandRequest;
use sqlx::PgPool;

pub const BRAND_ROOT_API: &str = "/api/brands";

pub fn configure_brand_routes(cfg: &mut web::ServiceConfig) {
    #[rustfmt::skip]
    cfg.service(
    web::scope(BRAND_ROOT_API)
        .service(
            web::resource("")
                .route(web::get().to(get_all_brands))
                .route(web::post().to(post_brands::handler))
        )
        .service(
            web::resource("/{brand}")
                .route(web::delete().to(delete_brand))
                .route(web::get().to(get_brand_by_id))
                .route(web::put().to(put_brand))
        )
    );
}

async fn get_brand_by_id(brand_id: web::Path<BrandId>, _db_pool: web::Data<PgPool>) -> impl Responder {
    println!("{}", brand_id);
    HttpResponse::Ok()
}

async fn get_all_brands(_db_pool: web::Data<PgPool>) -> impl Responder {
    HttpResponse::Ok()
}

async fn delete_brand(brand_id: web::Path<BrandId>, _db_pool: web::Data<PgPool>) -> impl Responder {
    println!("{}", brand_id);
    HttpResponse::Ok()
}

async fn put_brand(
    brand_id: web::Path<BrandId>,
    request: web::Json<BrandRequest>,
    _db_pool: web::Data<PgPool>,
) -> impl Responder {
    println!("{}", brand_id);
    println!("{:?}", request);
    HttpResponse::Ok()
}
