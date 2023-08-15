//! the rolling stock row definition

use catalog::catalog_items::catalog_item_id::CatalogItemId;
use catalog::catalog_items::category::{
    ElectricMultipleUnitType, FreightCarType, LocomotiveType, PassengerCarType, RailcarType, RollingStockCategory,
};
use catalog::catalog_items::control::{Control, DccInterface};
use catalog::catalog_items::rolling_stock_id::RollingStockId;
use catalog::catalog_items::service_level::ServiceLevel;
use catalog::catalog_items::technical_specifications::{CouplingSocket, FeatureFlag};
use catalog::railways::railway_id::RailwayId;
use rust_decimal::Decimal;

/// It represents the rolling stock row definition
#[derive(Debug, Clone)]
pub struct RollingStockRow {
    pub rolling_stock_id: RollingStockId,
    pub catalog_item_id: CatalogItemId,
    pub railway_id: RailwayId,
    pub railway_label: String,
    pub rolling_stock_category: RollingStockCategory,
    pub epoch: String,
    pub livery: Option<String>,
    pub length_over_buffers_mm: Option<Decimal>,
    pub length_over_buffers_in: Option<Decimal>,
    pub type_name: String,
    pub road_number: Option<String>,
    pub series: Option<String>,
    pub depot: Option<String>,
    pub dcc_interface: Option<DccInterface>,
    pub control: Option<Control>,
    pub electric_multiple_unit_type: Option<ElectricMultipleUnitType>,
    pub freight_car_type: Option<FreightCarType>,
    pub locomotive_type: Option<LocomotiveType>,
    pub passenger_car_type: Option<PassengerCarType>,
    pub railcar_type: Option<RailcarType>,
    pub service_level: Option<ServiceLevel>,
    pub is_dummy: Option<bool>,
    pub minimum_radius: Option<Decimal>,
    pub coupling_socket: Option<CouplingSocket>,
    pub close_couplers: Option<FeatureFlag>,
    pub digital_shunting_coupling: Option<FeatureFlag>,
    pub flywheel_fitted: Option<FeatureFlag>,
    pub metal_body: Option<FeatureFlag>,
    pub interior_lights: Option<FeatureFlag>,
    pub lights: Option<FeatureFlag>,
    pub spring_buffers: Option<FeatureFlag>,
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[allow(dead_code)]
    pub fn new_rolling_stock_row(catalog_item_id: CatalogItemId, type_name: &str, railway: &str) -> RollingStockRow {
        RollingStockRow {
            rolling_stock_id: Default::default(),
            catalog_item_id,
            railway_id: RailwayId::new(railway),
            railway_label: String::from(railway),
            rolling_stock_category: RollingStockCategory::FreightCar,
            epoch: "IV".to_string(),
            livery: None,
            length_over_buffers_mm: None,
            length_over_buffers_in: None,
            type_name: String::from(type_name),
            road_number: None,
            series: None,
            depot: None,
            dcc_interface: None,
            control: None,
            electric_multiple_unit_type: None,
            freight_car_type: None,
            locomotive_type: None,
            passenger_car_type: None,
            railcar_type: None,
            service_level: None,
            is_dummy: None,
            minimum_radius: None,
            coupling_socket: None,
            close_couplers: Some(FeatureFlag::NotApplicable),
            digital_shunting_coupling: Some(FeatureFlag::NotApplicable),
            flywheel_fitted: Some(FeatureFlag::NotApplicable),
            metal_body: Some(FeatureFlag::NotApplicable),
            interior_lights: Some(FeatureFlag::NotApplicable),
            lights: Some(FeatureFlag::NotApplicable),
            spring_buffers: Some(FeatureFlag::NotApplicable),
        }
    }
}
