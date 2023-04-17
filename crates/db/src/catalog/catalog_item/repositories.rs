use crate::catalog::catalog_item::catalog_item_row::CatalogItemRow;
use crate::catalog::catalog_item::rolling_stock_row::RollingStockRow;
use anyhow::Context;
use async_trait::async_trait;
use catalog::brands::brand_id::BrandId;
use catalog::catalog_items::availability_status::AvailabilityStatus;
use catalog::catalog_items::catalog_item::CatalogItem;
use catalog::catalog_items::catalog_item_id::CatalogItemId;
use catalog::catalog_items::category::{
    Category, ElectricMultipleUnitType, FreightCarType, LocomotiveType, PassengerCarType, RailcarType,
    RollingStockCategory,
};
use catalog::catalog_items::control::{Control, DccInterface};
use catalog::catalog_items::power_method::PowerMethod;
use catalog::catalog_items::queries::find_catalog_item_by_id::{
    FindCatalogItemByIdRepository, FindRollingStocksByCatalogItemIdRepository,
};
use catalog::catalog_items::rolling_stock::RollingStock;
use catalog::catalog_items::rolling_stock_id::RollingStockId;
use catalog::catalog_items::service_level::ServiceLevel;
use catalog::catalog_items::technical_specifications::{CouplingSocket, FeatureFlag};
use catalog::railways::railway_id::RailwayId;
use catalog::scales::scale_id::ScaleId;
use common::queries::converters::ToOutputConverter;
use common::queries::errors::DatabaseError;
use common::unit_of_work::postgres::PgUnitOfWork;

#[derive(Debug)]
pub struct CatalogItemsRepository;

#[async_trait]
impl<'db> FindCatalogItemByIdRepository<'db, PgUnitOfWork<'db>> for CatalogItemsRepository {
    async fn find_by_id(
        &self,
        catalog_item_id: &CatalogItemId,
        unit_of_work: &mut PgUnitOfWork<'db>,
    ) -> Result<Option<CatalogItem>, DatabaseError> {
        let result = sqlx::query_as!(
            CatalogItemRow,
            r#"SELECT
                c.catalog_item_id as "catalog_item_id: CatalogItemId",
                c.item_number,
                c.brand_id as "brand_id: BrandId",
                b.name as brand_display,
                c.scale_id as "scale_id: ScaleId",
                s.name as scale_display,
                c.category as "category: Category",
                c.power_method as "power_method: PowerMethod",
                c.description_en,
                c.description_it,
                c.details_en,
                c.details_it,
                c.delivery_date,
                c.availability_status as "availability_status: AvailabilityStatus",
                c.count,
                c.created_at,
                c.last_modified_at,
                c.version
            FROM catalog_items AS c
            JOIN brands AS b
              ON c.brand_id = b.brand_id
            JOIN scales AS s
              ON s.scale_id = c.scale_id
            WHERE c.catalog_item_id = $1 "#,
            catalog_item_id as &CatalogItemId
        )
        .fetch_optional(&mut unit_of_work.transaction)
        .await
        .context("A database failure was encountered while trying to fetch a catalog item.")?;

        result.map(row_to_catalog_item).transpose()
    }
}

fn row_to_catalog_item(row: CatalogItemRow) -> Result<CatalogItem, DatabaseError> {
    row.to_output().map_err(DatabaseError::ConversionError)
}

#[async_trait]
impl<'db> FindRollingStocksByCatalogItemIdRepository<'db, PgUnitOfWork<'db>> for CatalogItemsRepository {
    async fn find_rolling_stocks_by_id(
        &self,
        catalog_item_id: &CatalogItemId,
        unit_of_work: &mut PgUnitOfWork<'db>,
    ) -> Result<Vec<RollingStock>, DatabaseError> {
        let rolling_stocks: Vec<RollingStockRow> = sqlx::query_as!(
            RollingStockRow,
            r#"SELECT 
                rs.rolling_stock_id as "rolling_stock_id: RollingStockId",
                rs.catalog_item_id as "catalog_item_id: CatalogItemId",
                rs.railway_id as "railway_id: RailwayId",
                r.name as railway_label, 
                rs.rolling_stock_category as "rolling_stock_category: RollingStockCategory",
                rs.epoch,
                rs.livery,
                rs.length_over_buffers_mm,
                rs.length_over_buffers_in,
                rs.type_name,
                rs.road_number,
                rs.series,
                rs.depot,
                rs.dcc_interface as "dcc_interface: DccInterface",
                rs.control as "control: Control",
                rs.electric_multiple_unit_type as "electric_multiple_unit_type: ElectricMultipleUnitType",
                rs.freight_car_type as "freight_car_type: FreightCarType",
                rs.locomotive_type as "locomotive_type: LocomotiveType",
                rs.passenger_car_type as "passenger_car_type: PassengerCarType",
                rs.railcar_type as "railcar_type: RailcarType",
                rs.service_level as "service_level: ServiceLevel",
                rs.is_dummy,
                rs.minimum_radius,
                rs.coupling_socket as "coupling_socket: CouplingSocket",
                rs.close_couplers as "close_couplers: FeatureFlag",
                rs.digital_shunting_coupling as "digital_shunting_coupling: FeatureFlag",
                rs.flywheel_fitted as "flywheel_fitted: FeatureFlag",
                rs.metal_body as "metal_body: FeatureFlag",
                rs.interior_lights as "interior_lights: FeatureFlag",
                rs.lights as "lights: FeatureFlag",
                rs.spring_buffers as "spring_buffers: FeatureFlag"
            FROM rolling_stocks AS rs
            JOIN railways AS r
              ON r.railway_id = rs.railway_id
            WHERE rs.catalog_item_id = $1"#,
            catalog_item_id as &CatalogItemId
        )
        .fetch_all(&mut unit_of_work.transaction)
        .await
        .expect("A database failure was encountered while trying to fetch the rolling stock(s).");

        let mut result = Vec::with_capacity(rolling_stocks.len());

        for rolling_stock in rolling_stocks.into_iter() {
            let rs = rolling_stock.to_output().map_err(DatabaseError::ConversionError)?;
            result.push(rs);
        }

        Ok(result)
    }
}
