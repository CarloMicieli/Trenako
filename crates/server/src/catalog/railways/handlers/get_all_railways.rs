use crate::catalog::railways::routes;
use crate::web::queries::{to_response_error, QueryResponseError};
use actix_web::{web, HttpResponse};
use catalog::railways::queries::find_all_railways::find_all_railways;
use catalog::railways::railway::Railway;
use common::queries::pagination::PageRequest;
use common::unit_of_work::postgres::PgDatabase;
use db::catalog::railways::repositories::RailwaysRepository;
use sqlx::PgPool;
use tracing_actix_web::RequestId;

pub async fn handle(
    request_id: RequestId,
    _page_request: web::Query<PageRequest>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, QueryResponseError> {
    let database = PgDatabase::new(&db_pool);
    let repo = RailwaysRepository;

    let results = find_all_railways(repo, database).await;
    results
        .map(to_http_response)
        .map_err(|why| to_response_error(*request_id, why, routes::RAILWAY_ROOT_API))
}

fn to_http_response(results: Vec<Railway>) -> HttpResponse {
    HttpResponse::Ok().json(results)
}
