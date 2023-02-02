use crate::catalog::railways::repositories::PgNewRailwayRepository;
use crate::catalog::railways::routes::RAILWAY_ROOT_API;
use crate::web::problem_detail::ProblemDetail;
use actix_web::http::header::LOCATION;
use actix_web::{web, HttpResponse, Responder};
use catalog::railways::commands::new_railways::{create_new_railway, RailwayCreationError};
use catalog::railways::railway_request::RailwayRequest;
use common::unit_of_work::postgres::PgDatabase;
use sqlx::PgPool;
use tracing_actix_web::RequestId;

pub async fn handle(
    request_id: RequestId,
    request: web::Json<RailwayRequest>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let repo = PgNewRailwayRepository;
    let database = PgDatabase::new(&db_pool);

    let result = create_new_railway(request.0, repo, database).await;
    match result {
        Ok(created) => {
            let location = format!("{}/{}", RAILWAY_ROOT_API, created.railway_id);
            HttpResponse::Created().insert_header((LOCATION, location)).finish()
        }
        Err(why) => match why {
            RailwayCreationError::RailwayAlreadyExists(_) => HttpResponse::Conflict().finish(),
            _ => {
                tracing::error!("{:?}", why);
                ProblemDetail::from_error(*request_id, &why.to_string()).to_response()
            }
        },
    }
}
