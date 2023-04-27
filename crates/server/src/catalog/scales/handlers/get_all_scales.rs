use crate::catalog::scales::routes;
use crate::web::queries::{to_response_error, QueryResponseError};
use actix_web::{web, HttpResponse};
use catalog::scales::queries::find_all_scales::find_all_scales;
use catalog::scales::scale::Scale;
use common::queries::pagination::PageRequest;
use common::unit_of_work::postgres::PgDatabase;
use db::catalog::scales::repositories::ScalesRepository;
use sqlx::PgPool;
use tracing_actix_web::RequestId;

pub async fn handle(
    request_id: RequestId,
    _page_request: web::Query<PageRequest>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, QueryResponseError> {
    let database = PgDatabase::new(&db_pool);
    let repo = ScalesRepository;

    let results = find_all_scales(repo, database).await;

    results
        .map(to_http_response)
        .map_err(|why| to_response_error(*request_id, why, routes::SCALE_ROOT_API))
}

fn to_http_response(results: Vec<Scale>) -> HttpResponse {
    HttpResponse::Ok().json(results)
}
