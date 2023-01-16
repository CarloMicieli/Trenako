use actix_web::{web, HttpResponse, Responder};
use catalog::railways::railway_id::RailwayId;
use catalog::railways::railway_request::RailwayRequest;
use sqlx::PgPool;

pub fn configure_railway_routes(cfg: &mut web::ServiceConfig) {
    #[rustfmt::skip]
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/railways")
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

async fn post_railway(request: web::Json<RailwayRequest>, _db_pool: web::Data<PgPool>) -> impl Responder {
    println!("{:?}", request);
    HttpResponse::Ok()
}
