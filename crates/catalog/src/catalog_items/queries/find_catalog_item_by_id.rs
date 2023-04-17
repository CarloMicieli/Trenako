use crate::catalog_items::catalog_item::CatalogItem;
use crate::catalog_items::catalog_item_id::CatalogItemId;
use crate::catalog_items::rolling_stock::RollingStock;
use async_trait::async_trait;
use common::queries::errors::DatabaseError;
use common::queries::single_result::QueryError;
use common::unit_of_work::{Database, UnitOfWork};

pub async fn find_catalog_item_by_id<'db, U, Repo, DB>(
    catalog_item_id: &CatalogItemId,
    repo: Repo,
    db: DB,
) -> Result<CatalogItem, QueryError>
where
    U: UnitOfWork<'db>,
    Repo: FindCatalogItemByIdRepository<'db, U> + FindRollingStocksByCatalogItemIdRepository<'db, U>,
    DB: Database<'db, U>,
{
    let mut unit_of_work = db.begin().await?;

    let catalog_item = repo.find_by_id(catalog_item_id, &mut unit_of_work).await?;

    let result = if let Some(mut catalog_item) = catalog_item {
        catalog_item.rolling_stocks = repo
            .find_rolling_stocks_by_id(catalog_item_id, &mut unit_of_work)
            .await?;
        Ok(catalog_item)
    } else {
        Err(QueryError::EmptyResultSet)
    };

    unit_of_work.commit().await?;

    result
}

#[async_trait]
pub trait FindCatalogItemByIdRepository<'db, U: UnitOfWork<'db>> {
    async fn find_by_id(
        &self,
        catalog_item_id: &CatalogItemId,
        unit_of_work: &mut U,
    ) -> Result<Option<CatalogItem>, DatabaseError>;
}

#[async_trait]
pub trait FindRollingStocksByCatalogItemIdRepository<'db, U: UnitOfWork<'db>> {
    async fn find_rolling_stocks_by_id(
        &self,
        catalog_item_id: &CatalogItemId,
        unit_of_work: &mut U,
    ) -> Result<Vec<RollingStock>, DatabaseError>;
}
