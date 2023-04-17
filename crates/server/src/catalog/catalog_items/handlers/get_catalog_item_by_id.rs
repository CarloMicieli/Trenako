use crate::catalog::catalog_items::routes;
use crate::web::queries::{to_http_response, to_response_error, QueryResponseError};
use actix_web::{web, HttpResponse};
use catalog::catalog_items::catalog_item_id::CatalogItemId;
use catalog::catalog_items::queries::find_catalog_item_by_id::find_catalog_item_by_id;
use common::unit_of_work::postgres::PgDatabase;
use db::catalog::catalog_item::repositories::CatalogItemsRepository;
use sqlx::PgPool;
use tracing_actix_web::RequestId;

pub async fn handle(
    request_id: RequestId,
    catalog_item_id: web::Path<CatalogItemId>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, QueryResponseError> {
    let database = PgDatabase::new(&db_pool);
    let repo = CatalogItemsRepository;

    let result = find_catalog_item_by_id(&catalog_item_id, repo, database).await;

    result.map(to_http_response).map_err(|why| {
        let path = format!("{}/{}", routes::CATALOG_ITEM_ROOT_API, catalog_item_id);
        to_response_error(*request_id, why, &path)
    })
}
