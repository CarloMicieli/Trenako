use crate::catalog::brands::routes;
use crate::web::queries::{to_http_response, to_response_error, QueryResponseError};
use actix_web::{web, HttpResponse};
use catalog::brands::brand_id::BrandId;
use catalog::brands::queries::find_brand_by_id::find_brand_by_id;
use common::unit_of_work::postgres::PgDatabase;
use db::catalog::brands::repositories::BrandsRepository;
use sqlx::PgPool;
use tracing_actix_web::RequestId;

pub async fn handle(
    request_id: RequestId,
    brand_id: web::Path<BrandId>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, QueryResponseError> {
    let database = PgDatabase::new(&db_pool);
    let repo = BrandsRepository;

    let result = find_brand_by_id(&brand_id, repo, database).await;

    result.map(to_http_response).map_err(|why| {
        let path = format!("{}/{}", routes::BRAND_ROOT_API, brand_id);
        to_response_error(*request_id, why, &path)
    })
}
