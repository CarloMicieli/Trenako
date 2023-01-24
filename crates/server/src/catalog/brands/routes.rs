use crate::catalog::brands::post_brands::PgNewBrandRepository;
use crate::web::problem_detail::ProblemDetail;
use actix_web::http::header::LOCATION;
use actix_web::{web, HttpResponse, Responder};
use catalog::brands::brand_id::BrandId;
use catalog::brands::brand_request::BrandRequest;
use catalog::brands::commands::new_brand::create_new_brand;
use common::unit_of_work::postgres::PgDatabase;
use sqlx::PgPool;
use tracing_actix_web::RequestId;

const BRAND_ROOT_API: &str = "/api/brands";

pub fn configure_brand_routes(cfg: &mut web::ServiceConfig) {
    #[rustfmt::skip]
    cfg.service(
    web::scope(BRAND_ROOT_API)
        .service(
            web::resource("")
                .route(web::get().to(get_all_brands))
                .route(web::post().to(post_brand))
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

async fn post_brand(
    request_id: RequestId,
    request: web::Json<BrandRequest>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let repo = PgNewBrandRepository;
    let database = PgDatabase::new(&db_pool);

    let result = create_new_brand(request.0, repo, database).await;
    match result {
        Ok(created) => {
            let location = format!("{}/{}", BRAND_ROOT_API, created.brand_id);
            HttpResponse::Created().insert_header((LOCATION, location)).finish()
        }
        Err(why) => {
            tracing::error!("{:?}", why);
            ProblemDetail::from_error(*request_id, &why.to_string()).to_response()
        }
    }
}
