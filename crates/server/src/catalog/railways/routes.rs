use crate::catalog::railways::post_railways::PgNewRailwayRepository;
use crate::web::problem_detail::ProblemDetail;
use actix_web::{web, HttpResponse, Responder};
use catalog::railways::commands::new_railways::create_new_railway;
use catalog::railways::railway_id::RailwayId;
use catalog::railways::railway_request::RailwayRequest;
use sqlx::PgPool;
use tracing_actix_web::RequestId;

pub const RAILWAY_ROOT_API: &str = "/api/railways";

pub fn configure_railway_routes(cfg: &mut web::ServiceConfig) {
    #[rustfmt::skip]
    cfg.service(
    web::scope(RAILWAY_ROOT_API)
        .service(
            web::resource("")
                .route(web::get().to(get_all_railways))
                .route(web::post().to(post_railway))
        )
        .service(
            web::resource("/{railway}")
                .route(web::delete().to(delete_railway))
                .route(web::get().to(get_railway_by_id))
                .route(web::put().to(put_railway))
        )
    );
}

async fn get_railway_by_id(railway_id: web::Path<RailwayId>, _db_pool: web::Data<PgPool>) -> impl Responder {
    println!("{}", railway_id);
    HttpResponse::Ok()
}

async fn get_all_railways(_db_pool: web::Data<PgPool>) -> impl Responder {
    HttpResponse::Ok()
}

async fn delete_railway(railway_id: web::Path<RailwayId>, _db_pool: web::Data<PgPool>) -> impl Responder {
    println!("{}", railway_id);
    HttpResponse::Ok()
}

async fn put_railway(
    railway_id: web::Path<RailwayId>,
    request: web::Json<RailwayRequest>,
    _db_pool: web::Data<PgPool>,
) -> impl Responder {
    println!("{}", railway_id);
    println!("{:?}", request);
    HttpResponse::Ok()
}

async fn post_railway(
    request_id: RequestId,
    request: web::Json<RailwayRequest>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let repo = PgNewRailwayRepository::new(&db_pool);
    let result = create_new_railway(request.0, repo).await;
    match result {
        Ok(created) => {
            let location = format!("{}/{}", RAILWAY_ROOT_API, created.railway_id);
            HttpResponse::Created().insert_header(("Location", location)).finish()
        }
        Err(why) => {
            tracing::error!("{:?}", why);
            ProblemDetail::from_error(*request_id, &why.to_string()).to_response()
        }
    }
}
