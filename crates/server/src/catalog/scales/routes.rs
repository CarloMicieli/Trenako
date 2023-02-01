use crate::catalog::scales::post_scales::PgNewScaleRepository;
use crate::web::problem_detail::ProblemDetail;
use actix_web::http::header::LOCATION;
use actix_web::{web, HttpResponse, Responder};
use catalog::scales::commands::new_scales::{create_new_scale, ScaleCreationError};
use catalog::scales::scale_id::ScaleId;
use catalog::scales::scale_request::ScaleRequest;
use common::unit_of_work::postgres::PgDatabase;
use sqlx::PgPool;
use tracing_actix_web::RequestId;

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

async fn get_scale_by_id(_scale_id: web::Path<ScaleId>, _db_pool: web::Data<PgPool>) -> impl Responder {
    HttpResponse::Ok()
}

async fn get_all_scales(_db_pool: web::Data<PgPool>) -> impl Responder {
    HttpResponse::Ok()
}

async fn delete_scale(_scale_id: web::Path<ScaleId>, _db_pool: web::Data<PgPool>) -> impl Responder {
    HttpResponse::Ok()
}

async fn put_scale(
    _scale_id: web::Path<ScaleId>,
    _request: web::Json<ScaleRequest>,
    _db_pool: web::Data<PgPool>,
) -> impl Responder {
    HttpResponse::Ok()
}

async fn post_scale(
    request_id: RequestId,
    request: web::Json<ScaleRequest>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let repo = PgNewScaleRepository;
    let database = PgDatabase::new(&db_pool);

    let result = create_new_scale(request.0, repo, database).await;
    match result {
        Ok(created) => {
            let location = format!("{}/{}", SCALE_ROOT_API, created.scale_id);
            HttpResponse::Created().insert_header((LOCATION, location)).finish()
        }
        Err(why) => match why {
            ScaleCreationError::ScaleAlreadyExists(_) => HttpResponse::Conflict().finish(),
            _ => {
                tracing::error!("{:?}", why);
                ProblemDetail::from_error(*request_id, &why.to_string()).to_response()
            }
        },
    }
}
