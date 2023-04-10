use crate::catalog::catalog_items::repositories::{PgCatalogItemRepository, PgRollingStockRepository};
use crate::catalog::catalog_items::routes;
use crate::web::queries::{to_http_response, to_response_error, QueryResponseError};
use actix_web::{web, HttpResponse};
use async_trait::async_trait;
use catalog::catalog_items::catalog_item::CatalogItem;
use catalog::catalog_items::catalog_item_id::CatalogItemId;
use catalog::catalog_items::queries::catalog_item_row::CatalogItemRow;
use catalog::catalog_items::queries::rolling_stock_row::RollingStockRow;
use catalog::catalog_items::rolling_stock::RollingStock;
use common::queries::aggregate::Aggregate;
use common::queries::single_result::{ByIdCriteria, SingleResultWithChildrenQuery};
use common::unit_of_work::postgres::{PgDatabase, PgUnitOfWork};
use sqlx::PgPool;
use tracing_actix_web::RequestId;

pub async fn handle(
    request_id: RequestId,
    catalog_item_id: web::Path<CatalogItemId>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, QueryResponseError> {
    let database = PgDatabase::new(&db_pool);
    let criteria: ByIdCriteria<CatalogItemId> = ByIdCriteria::new(&catalog_item_id);
    let result = GetCatalogItemByIdQuery::execute(&criteria, database).await;

    result.map(to_http_response).map_err(|why| {
        let path = format!("{}/{}", routes::CATALOG_ITEM_ROOT_API, catalog_item_id);
        to_response_error(*request_id, why, &path)
    })
}

struct GetCatalogItemByIdQuery;

#[async_trait]
impl<'db>
    SingleResultWithChildrenQuery<
        'db,
        PgUnitOfWork<'db>,
        PgDatabase<'db>,
        PgCatalogItemRepository,
        PgRollingStockRepository,
    > for GetCatalogItemByIdQuery
{
    type Id = CatalogItemId;
    type RowType = CatalogItemRow;
    type ChildRowType = RollingStockRow;
    type RootOutput = CatalogItem;
    type ChildOutput = RollingStock;
    type A = CatalogItemAggregate;
}

struct CatalogItemAggregate;

impl Aggregate for CatalogItemAggregate {
    type Id = CatalogItemId;
    type RootRowType = CatalogItemRow;
    type ChildRowType = RollingStockRow;
    type RootOutput = CatalogItem;
    type ChildOutput = RollingStock;
}
