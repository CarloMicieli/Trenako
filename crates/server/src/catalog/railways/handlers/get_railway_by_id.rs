use crate::catalog::railways::repositories::PgRailwayRepository;
use crate::catalog::railways::routes;
use crate::web::queries::{to_http_response, to_response_error, QueryResponseError};
use actix_web::{web, HttpResponse};
use async_trait::async_trait;
use catalog::railways::queries::railway_row::RailwayRow;
use catalog::railways::railway::Railway;
use catalog::railways::railway_id::RailwayId;
use common::queries::single_result::{ByIdCriteria, SingleResultQuery};
use common::unit_of_work::postgres::{PgDatabase, PgUnitOfWork};
use sqlx::PgPool;
use tracing_actix_web::RequestId;

pub async fn handle(
    request_id: RequestId,
    railway_id: web::Path<RailwayId>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, QueryResponseError> {
    let database = PgDatabase::new(&db_pool);
    let criteria: ByIdCriteria<RailwayId> = ByIdCriteria::new(&railway_id);
    let result = PgSingleRailwayResultQuery::execute(&criteria, database).await;

    result.map(to_http_response).map_err(|why| {
        let path = format!("{}/{}", routes::RAILWAY_ROOT_API, railway_id);
        to_response_error(*request_id, why, &path)
    })
}

struct PgSingleRailwayResultQuery;

#[async_trait]
impl<'db> SingleResultQuery<'db, PgUnitOfWork<'db>, PgDatabase<'db>, PgRailwayRepository>
    for PgSingleRailwayResultQuery
{
    type Id = RailwayId;
    type RowType = RailwayRow;
    type Output = Railway;
}
