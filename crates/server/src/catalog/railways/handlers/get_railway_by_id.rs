use crate::catalog::railways::routes;
use crate::web::queries::{to_http_response, to_response_error, QueryResponseError};
use actix_web::{web, HttpResponse};
use catalog::railways::queries::find_railway_by_id::find_railway_by_id;
use catalog::railways::railway_id::RailwayId;
use common::unit_of_work::postgres::PgDatabase;
use db::catalog::railways::repositories::RailwaysRepository;
use sqlx::PgPool;
use tracing_actix_web::RequestId;

pub async fn handle(
    request_id: RequestId,
    railway_id: web::Path<RailwayId>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, QueryResponseError> {
    let database = PgDatabase::new(&db_pool);
    let repo = RailwaysRepository;

    let result = find_railway_by_id(&railway_id, repo, database).await;

    result.map(to_http_response).map_err(|why| {
        let path = format!("{}/{}", routes::RAILWAY_ROOT_API, railway_id);
        to_response_error(*request_id, why, &path)
    })
}
