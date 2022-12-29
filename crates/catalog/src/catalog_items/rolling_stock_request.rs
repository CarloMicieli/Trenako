use crate::catalog_items::category::{
    ElectricMultipleUnitType, FreightCarType, LocomotiveType, PassengerCarType, RailcarType,
};
use crate::catalog_items::control::{Control, DccInterface};
use crate::catalog_items::length_over_buffer::LengthOverBuffer;
use crate::catalog_items::technical_specifications::TechnicalSpecifications;

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(tag = "category")]
pub enum RollingStockRequest {
    ElectricMultipleUnitRequest {
        railway: String,
        epoch: String,
        livery: Option<String>,
        length_over_buffer: Option<LengthOverBuffer>,
        technical_specifications: Option<TechnicalSpecifications>,
        type_name: String,
        road_number: Option<String>,
        series: Option<String>,
        depot: Option<String>,
        electric_multiple_unit_type: ElectricMultipleUnitType,
        dcc_interface: Option<DccInterface>,
        control: Option<Control>,
        is_dummy: bool,
    },
    FreightCarRequest {
        railway: String,
        epoch: String,
        livery: Option<String>,
        length_over_buffer: Option<LengthOverBuffer>,
        technical_specifications: Option<TechnicalSpecifications>,
        type_name: String,
        road_number: Option<String>,
        freight_car_type: Option<FreightCarType>,
    },
    LocomotiveRequest {
        railway: String,
        epoch: String,
        livery: Option<String>,
        length_over_buffer: Option<LengthOverBuffer>,
        technical_specifications: Option<TechnicalSpecifications>,
        class_name: String,
        road_number: String,
        series: Option<String>,
        depot: Option<String>,
        locomotive_type: LocomotiveType,
        dcc_interface: Option<DccInterface>,
        control: Option<Control>,
        is_dummy: bool,
    },
    PassengerCarRequest {
        railway: String,
        epoch: String,
        livery: Option<String>,
        length_over_buffer: Option<LengthOverBuffer>,
        technical_specifications: Option<TechnicalSpecifications>,
        type_name: String,
        road_number: Option<String>,
        passenger_car_type: Option<PassengerCarType>,
        service_level: Option<String>,
    },
    RailcarRequest {
        railway: String,
        epoch: String,
        livery: Option<String>,
        length_over_buffer: Option<LengthOverBuffer>,
        technical_specifications: Option<TechnicalSpecifications>,
        type_name: String,
        road_number: Option<String>,
        series: Option<String>,
        depot: Option<String>,
        railcar_type: RailcarType,
        dcc_interface: Option<DccInterface>,
        control: Option<Control>,
        is_dummy: bool,
    },
}
