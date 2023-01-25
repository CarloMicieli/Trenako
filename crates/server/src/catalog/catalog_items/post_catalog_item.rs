use async_trait::async_trait;
use catalog::brands::brand_id::BrandId;
use catalog::catalog_items::catalog_item_id::CatalogItemId;
use catalog::catalog_items::commands::new_catalog_item::{
    NewCatalogItemCommand, NewCatalogItemRepository, NewRollingStockCommand, NewRollingStockRepository,
};
use catalog::railways::railway_id::RailwayId;
use catalog::scales::scale_id::ScaleId;
use common::unit_of_work::postgres::PgUnitOfWork;

pub struct PgNewCatalogItemRepository;
pub struct PgNewRollingStockRepository;

#[async_trait]
impl<'db> NewCatalogItemRepository<'db, PgUnitOfWork<'db>> for PgNewCatalogItemRepository {
    async fn exists_already(
        &self,
        _catalog_item_id: &CatalogItemId,
        _unit_of_work: &mut PgUnitOfWork<'db>,
    ) -> catalog::catalog_items::commands::new_catalog_item::Result<bool> {
        todo!()
    }

    async fn insert(
        &self,
        _new_item: &NewCatalogItemCommand,
        _unit_of_work: &mut PgUnitOfWork<'db>,
    ) -> catalog::catalog_items::commands::new_catalog_item::Result<()> {
        todo!()
    }

    async fn brand_exists(
        &self,
        _brand_id: &BrandId,
        _unit_of_work: &mut PgUnitOfWork<'db>,
    ) -> catalog::catalog_items::commands::new_catalog_item::Result<bool> {
        todo!()
    }

    async fn scale_exists(
        &self,
        _scale_id: &ScaleId,
        _unit_of_work: &mut PgUnitOfWork<'db>,
    ) -> catalog::catalog_items::commands::new_catalog_item::Result<bool> {
        todo!()
    }
}

#[async_trait]
impl<'db> NewRollingStockRepository<'db, PgUnitOfWork<'db>> for PgNewRollingStockRepository {
    async fn insert(
        &self,
        _new_item: &NewRollingStockCommand,
        _unit_of_work: &mut PgUnitOfWork<'db>,
    ) -> catalog::catalog_items::commands::new_catalog_item::Result<()> {
        todo!()
    }

    async fn railway_exists(
        &self,
        _railway_id: &RailwayId,
        _unit_of_work: &mut PgUnitOfWork<'db>,
    ) -> catalog::catalog_items::commands::new_catalog_item::Result<bool> {
        todo!()
    }
}
