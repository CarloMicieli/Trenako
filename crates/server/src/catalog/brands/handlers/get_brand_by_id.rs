use crate::catalog::brands::repositories::PgBrandRepository;
use crate::catalog::brands::routes;
use crate::web::queries::{to_http_response, to_response_error, QueryResponseError};
use actix_web::{web, HttpResponse};
use async_trait::async_trait;
use catalog::brands::brand::Brand;
use catalog::brands::brand_id::BrandId;
use catalog::brands::queries::brand_row::BrandRow;
use common::queries::single_result::{ByIdCriteria, SingleResultQuery};
use common::unit_of_work::postgres::{PgDatabase, PgUnitOfWork};
use sqlx::PgPool;
use tracing_actix_web::RequestId;

pub async fn handle(
    request_id: RequestId,
    brand_id: web::Path<BrandId>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, QueryResponseError> {
    let database = PgDatabase::new(&db_pool);
    let criteria: ByIdCriteria<BrandId> = ByIdCriteria::new(&brand_id);
    let result = PgSingleBrandResultQuery::execute(&criteria, database).await;

    result.map(to_http_response).map_err(|why| {
        let path = format!("{}/{}", routes::BRAND_ROOT_API, brand_id);
        to_response_error(*request_id, why, &path)
    })
}

struct PgSingleBrandResultQuery;

#[async_trait]
impl<'db> SingleResultQuery<'db, PgUnitOfWork<'db>, PgDatabase<'db>, PgBrandRepository> for PgSingleBrandResultQuery {
    type Id = BrandId;
    type RowType = BrandRow;
    type Output = Brand;
}
