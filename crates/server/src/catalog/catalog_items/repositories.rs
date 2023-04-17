use anyhow::Context;
use async_trait::async_trait;
use catalog::brands::brand_id::BrandId;
use catalog::catalog_items::availability_status::AvailabilityStatus;
use catalog::catalog_items::catalog_item_id::CatalogItemId;
use catalog::catalog_items::category::Category;
use catalog::catalog_items::category::{
    ElectricMultipleUnitType, FreightCarType, LocomotiveType, PassengerCarType, RailcarType, RollingStockCategory,
};
use catalog::catalog_items::commands::new_catalog_item::{NewCatalogItemCommand, NewRollingStockCommand};
use catalog::catalog_items::commands::repositories::{CatalogItemRepository, RollingStockRepository};
use catalog::catalog_items::control::{Control, DccInterface};
use catalog::catalog_items::power_method::PowerMethod;
use catalog::catalog_items::rolling_stock_id::RollingStockId;
use catalog::catalog_items::service_level::ServiceLevel;
use catalog::catalog_items::technical_specifications::{CouplingSocket, FeatureFlag};
use catalog::railways::railway_id::RailwayId;
use catalog::scales::scale_id::ScaleId;
use common::unit_of_work::postgres::PgUnitOfWork;

pub struct PgCatalogItemRepository;
pub struct PgRollingStockRepository;

#[async_trait]
impl<'db> CatalogItemRepository<'db, PgUnitOfWork<'db>> for PgCatalogItemRepository {
    async fn exists(
        &self,
        catalog_item_id: &CatalogItemId,
        unit_of_work: &mut PgUnitOfWork<'db>,
    ) -> Result<bool, anyhow::Error> {
        let result = sqlx::query!(
            "SELECT catalog_item_id FROM catalog_items WHERE catalog_item_id = $1 LIMIT 1",
            catalog_item_id as &CatalogItemId
        )
        .fetch_optional(&mut unit_of_work.transaction)
        .await
        .context("A database failure was encountered while trying to check for the catalog item existence.")?;

        Ok(result.is_some())
    }

    async fn insert(
        &self,
        new_item: &NewCatalogItemCommand,
        unit_of_work: &mut PgUnitOfWork<'db>,
    ) -> Result<(), anyhow::Error> {
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
                description_en,
                description_it,
                details_en,
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
                $7, $8, $9, $10, $11, $12, 
                $13, $14, $15
            )"#,
            catalog_item_id as &CatalogItemId,
            brand_id as &BrandId,
            request.item_number.value(),
            scale_id as &ScaleId,
            request.category as Category,
            request.description.english(),
            request.description.italian(),
            request.details.english(),
            request.details.italian(),
            request.power_method as PowerMethod,
            request.delivery_date.as_ref().map(|x| x.to_string()),
            request.availability_status as Option<AvailabilityStatus>,
            request.count,
            metadata.created(),
            metadata.version() as i32
        )
        .execute(&mut unit_of_work.transaction)
        .await
        .context("A database failure was encountered while trying to store a catalog item.")?;

        Ok(())
    }

    async fn brand_exists(
        &self,
        brand_id: &BrandId,
        unit_of_work: &mut PgUnitOfWork<'db>,
    ) -> Result<bool, anyhow::Error> {
        let result = sqlx::query!("SELECT brand_id FROM brands WHERE brand_id = $1 LIMIT 1", brand_id)
            .fetch_optional(&mut unit_of_work.transaction)
            .await
            .context("A database failure was encountered while trying to check for brand existence.")?;

        Ok(result.is_some())
    }

    async fn scale_exists(
        &self,
        scale_id: &ScaleId,
        unit_of_work: &mut PgUnitOfWork<'db>,
    ) -> Result<bool, anyhow::Error> {
        let result = sqlx::query!("SELECT scale_id FROM scales WHERE scale_id = $1 LIMIT 1", scale_id)
            .fetch_optional(&mut unit_of_work.transaction)
            .await
            .context("A database failure was encountered while trying to check for scale existence.")?;

        Ok(result.is_some())
    }
}

#[async_trait]
impl<'db> RollingStockRepository<'db, PgUnitOfWork<'db>> for PgRollingStockRepository {
    async fn insert(
        &self,
        new_item: &NewRollingStockCommand,
        unit_of_work: &mut PgUnitOfWork<'db>,
    ) -> Result<(), anyhow::Error> {
        let request = &new_item.payload;
        let catalog_item_id = &new_item.catalog_item_id;
        let rolling_stock_id = &new_item.rolling_stock_id;
        let railway_id = &new_item.railway_id;

        sqlx::query!(
            r#"INSERT INTO rolling_stocks (
                        rolling_stock_id,
                        catalog_item_id,
                        railway_id,
                        rolling_stock_category,
                        epoch,
                        livery,
                        length_over_buffers_mm,
                        length_over_buffers_in,
                        type_name,
                        road_number,
                        series,
                        depot,
                        dcc_interface,
                        control,
                        electric_multiple_unit_type,
                        freight_car_type,
                        locomotive_type,
                        passenger_car_type,
                        railcar_type,
                        service_level,
                        is_dummy,
                        minimum_radius,
                        coupling_socket,
                        close_couplers,
                        digital_shunting_coupling,
                        flywheel_fitted,
                        metal_body,
                        interior_lights,
                        lights,
                        spring_buffers
                    )
                    VALUES (
                        $1, $2, $3, $4, $5, $6,
                        $7, $8, $9, $10, $11, $12, 
                        $13, $14, $15, $16, $17, $18,
                        $19, $20, $21, $22, $23, $24, 
                        $25, $26, $27, $28, $29, $30
                    )"#,
            rolling_stock_id as &RollingStockId,
            catalog_item_id as &CatalogItemId,
            railway_id as &RailwayId,
            request.category.expect("rolling stock category is mandatory") as RollingStockCategory,
            request
                .epoch
                .as_ref()
                .expect("rolling stock epoch is mandatory")
                .to_string(),
            request.livery,
            request.length_over_buffers_mm.map(|x| x.quantity()),
            request.length_over_buffers_in.map(|x| x.quantity()),
            request.type_name,
            request.road_number,
            request.series,
            request.depot,
            request.dcc_interface as Option<DccInterface>,
            request.control as Option<Control>,
            request.electric_multiple_unit_type as Option<ElectricMultipleUnitType>,
            request.freight_car_type as Option<FreightCarType>,
            request.locomotive_type as Option<LocomotiveType>,
            request.passenger_car_type as Option<PassengerCarType>,
            request.railcar_type as Option<RailcarType>,
            request.service_level as Option<ServiceLevel>,
            request.is_dummy,
            request.minimum_radius.map(|x| x.value().quantity()),
            request.coupling_socket as Option<CouplingSocket>,
            request.close_couplers as Option<FeatureFlag>,
            request.digital_shunting_coupling as Option<FeatureFlag>,
            request.flywheel_fitted as Option<FeatureFlag>,
            request.metal_body as Option<FeatureFlag>,
            request.interior_lights as Option<FeatureFlag>,
            request.lights as Option<FeatureFlag>,
            request.spring_buffers as Option<FeatureFlag>
        )
        .execute(&mut unit_of_work.transaction)
        .await
        .context("A database failure was encountered while trying to store a rolling stock.")?;

        Ok(())
    }

    async fn railway_exists(
        &self,
        railway_id: &RailwayId,
        unit_of_work: &mut PgUnitOfWork<'db>,
    ) -> Result<bool, anyhow::Error> {
        let result = sqlx::query!(
            "SELECT railway_id FROM railways WHERE railway_id = $1 LIMIT 1",
            railway_id
        )
        .fetch_optional(&mut unit_of_work.transaction)
        .await
        .context("A database failure was encountered while trying to check for a railway existence.")?;

        Ok(result.is_some())
    }
}
