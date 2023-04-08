use crate::catalog::scales::repositories::PgScaleRepository;
use crate::catalog::scales::routes;
use crate::web::queries::{to_http_response, to_response_error, QueryResponseError};
use actix_web::{web, HttpResponse};
use async_trait::async_trait;
use catalog::scales::queries::scale_row::ScaleRow;
use catalog::scales::scale::Scale;
use catalog::scales::scale_id::ScaleId;
use common::queries::single_result::{ByIdCriteria, SingleResultQuery};
use common::unit_of_work::postgres::{PgDatabase, PgUnitOfWork};
use sqlx::PgPool;
use tracing_actix_web::RequestId;

pub async fn handle(
    request_id: RequestId,
    scale_id: web::Path<ScaleId>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, QueryResponseError> {
    let database = PgDatabase::new(&db_pool);
    let criteria: ByIdCriteria<ScaleId> = ByIdCriteria::new(&scale_id);
    let result = PgSingleScaleResultQuery::execute(&criteria, database).await;

    result.map(to_http_response).map_err(|why| {
        let path = format!("{}/{}", routes::SCALE_ROOT_API, scale_id);
        to_response_error(*request_id, why, &path)
    })
}

struct PgSingleScaleResultQuery;

#[async_trait]
impl<'db> SingleResultQuery<'db, PgUnitOfWork<'db>, PgDatabase<'db>, PgScaleRepository> for PgSingleScaleResultQuery {
    type Id = ScaleId;
    type RowType = ScaleRow;
    type Output = Scale;
}
