use crate::catalog::scales::routes;
use crate::web::queries::{to_http_response, to_response_error, QueryResponseError};
use actix_web::{web, HttpResponse};
use catalog::scales::queries::find_scale_by_id::find_scale_by_id;
use catalog::scales::scale_id::ScaleId;
use common::unit_of_work::postgres::PgDatabase;
use db::catalog::scales::repositories::ScalesRepository;
use sqlx::PgPool;
use tracing_actix_web::RequestId;

pub async fn handle(
    request_id: RequestId,
    scale_id: web::Path<ScaleId>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, QueryResponseError> {
    let database = PgDatabase::new(&db_pool);
    let repo = ScalesRepository;

    let result = find_scale_by_id(&scale_id, repo, database).await;

    result.map(to_http_response).map_err(|why| {
        let path = format!("{}/{}", routes::SCALE_ROOT_API, scale_id);
        to_response_error(*request_id, why, &path)
    })
}
