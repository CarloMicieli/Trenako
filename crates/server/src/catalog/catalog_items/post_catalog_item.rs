use async_trait::async_trait;
use catalog::brands::brand_id::BrandId;
use catalog::catalog_items::availability_status::AvailabilityStatus;
use catalog::catalog_items::catalog_item_id::CatalogItemId;
use catalog::catalog_items::category::Category;
use catalog::catalog_items::commands::new_catalog_item::Result;
use catalog::catalog_items::commands::new_catalog_item::{
    NewCatalogItemCommand, NewCatalogItemRepository, NewRollingStockCommand, NewRollingStockRepository,
};
use catalog::catalog_items::power_method::PowerMethod;
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
    ) -> Result<bool> {
        Ok(false)
    }

    async fn insert(&self, new_item: &NewCatalogItemCommand, unit_of_work: &mut PgUnitOfWork<'db>) -> Result<()> {
        let catalog_item_id = &new_item.catalog_item_id;
        let request = &new_item.payload;
        let metadata = &new_item.metadata;

        let brand_id = &request.brand_id;
        let scale_id = &request.scale_id;

        sqlx::query!(
            r#"INSERT INTO catalog_items (
                catalog_item_id,
                brand_id,
                item_number,
                scale_id,
                category,
                description_it,
                details_it,
                power_method,
                delivery_date,
                availability_status,
                count,
                created_at,
                version
            )
            VALUES (
                $1, $2, $3, $4, $5, $6,
                $7, $8, $9, $10, $11, $12, $13
            )"#,
            catalog_item_id as &CatalogItemId,
            brand_id as &BrandId,
            request.item_number.value(),
            scale_id as &ScaleId,
            request.category as Category,
            request.description.italian(),
            request.details.italian(),
            request.power_method as PowerMethod,
            request.delivery_date.as_ref().map(|x| x.to_string()),
            request.availability_status.as_ref() as Option<&AvailabilityStatus>,
            request.count,
            metadata.created(),
            metadata.version() as i32
        )
        .execute(&mut unit_of_work.transaction)
        .await?;

        Ok(())
    }

    async fn brand_exists(&self, _brand_id: &BrandId, _unit_of_work: &mut PgUnitOfWork<'db>) -> Result<bool> {
        Ok(true)
    }

    async fn scale_exists(&self, _scale_id: &ScaleId, _unit_of_work: &mut PgUnitOfWork<'db>) -> Result<bool> {
        Ok(true)
    }
}

#[async_trait]
impl<'db> NewRollingStockRepository<'db, PgUnitOfWork<'db>> for PgNewRollingStockRepository {
    async fn insert(&self, _new_item: &NewRollingStockCommand, _unit_of_work: &mut PgUnitOfWork<'db>) -> Result<()> {
        todo!()
    }

    async fn railway_exists(&self, _railway_id: &RailwayId, _unit_of_work: &mut PgUnitOfWork<'db>) -> Result<bool> {
        todo!()
    }
}
