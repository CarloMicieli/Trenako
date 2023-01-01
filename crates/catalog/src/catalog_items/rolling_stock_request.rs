use crate::catalog_items::category::{
    ElectricMultipleUnitType, FreightCarType, LocomotiveType, PassengerCarType, RailcarType,
};
use crate::catalog_items::control::{Control, DccInterface};
use crate::catalog_items::length_over_buffer::LengthOverBuffer;
use crate::catalog_items::technical_specifications::TechnicalSpecifications;

/// It represents a request to create / modify a rolling stock
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(tag = "category")]
pub enum RollingStockRequest {
    /// an electric multiple unit rolling stock
    ElectricMultipleUnitRequest {
        /// the railway name for this rolling stock
        railway: String,
        /// the epoch for this rolling stock
        epoch: String,
        /// the livery description
        livery: Option<String>,
        /// the overall length
        length_over_buffer: Option<LengthOverBuffer>,
        /// the technical specifications
        technical_specifications: Option<TechnicalSpecifications>,
        /// the electric multiple unit type name
        type_name: String,
        /// the identification marking for this electric multiple unit
        road_number: Option<String>,
        /// the prototype series information
        series: Option<String>,
        /// the depot name
        depot: Option<String>,
        /// the electric multiple unit type
        electric_multiple_unit_type: ElectricMultipleUnitType,
        /// the dcc interface
        dcc_interface: Option<DccInterface>,
        /// the control
        control: Option<Control>,
        /// indicate whether the rolling stock has a motor or not
        is_dummy: bool,
    },
    /// a freight car rolling stock
    FreightCarRequest {
        /// the railway name for this rolling stock
        railway: String,
        /// the epoch for this rolling stock
        epoch: String,
        /// the livery description
        livery: Option<String>,
        /// the overall length
        length_over_buffer: Option<LengthOverBuffer>,
        /// the technical specifications
        technical_specifications: Option<TechnicalSpecifications>,
        /// the freight car type name
        type_name: String,
        /// the identification marking for this freight car
        road_number: Option<String>,
        /// the freight car type
        freight_car_type: Option<FreightCarType>,
    },
    /// a locomotive rolling stock
    LocomotiveRequest {
        /// the railway name for this rolling stock
        railway: String,
        /// the epoch for this rolling stock
        epoch: String,
        /// the livery description
        livery: Option<String>,
        /// the overall length
        length_over_buffer: Option<LengthOverBuffer>,
        /// the technical specifications
        technical_specifications: Option<TechnicalSpecifications>,
        /// the class of locomotives. The class is a group of locomotives built to a common design,
        /// typically for a single railroad or railway
        class_name: String,
        /// the identification marking for this locomotive
        road_number: String,
        /// the prototype series information
        series: Option<String>,
        /// the depot name
        depot: Option<String>,
        /// the locomotive type
        locomotive_type: LocomotiveType,
        /// the dcc interface
        dcc_interface: Option<DccInterface>,
        /// the control
        control: Option<Control>,
        /// indicate whether the rolling stock has a motor or not
        is_dummy: bool,
    },
    /// a passenger car rolling stock
    PassengerCarRequest {
        /// the railway name for this rolling stock
        railway: String,
        /// the epoch for this rolling stock
        epoch: String,
        /// the livery description
        livery: Option<String>,
        /// the overall length
        length_over_buffer: Option<LengthOverBuffer>,
        /// the technical specifications
        technical_specifications: Option<TechnicalSpecifications>,
        /// the passenger car type name
        type_name: String,
        /// the identification marking for this passenger car
        road_number: Option<String>,
        /// the passenger car type
        passenger_car_type: Option<PassengerCarType>,
        /// the travel class for this passenger car. Passenger cars can have multiple service
        /// levels (ie, '1st/2nd')
        service_level: Option<String>,
    },
    /// a railcar rolling stock
    RailcarRequest {
        /// the railway name for this rolling stock
        railway: String,
        /// the epoch for this rolling stock
        epoch: String,
        /// the livery description
        livery: Option<String>,
        /// the overall length
        length_over_buffer: Option<LengthOverBuffer>,
        /// the technical specifications
        technical_specifications: Option<TechnicalSpecifications>,
        /// the railcar type name
        type_name: String,
        /// the identification marking for this railcar
        road_number: Option<String>,
        /// the railcar series
        series: Option<String>,
        /// the depot name
        depot: Option<String>,
        /// the railcar type
        railcar_type: RailcarType,
        /// the dcc interface
        dcc_interface: Option<DccInterface>,
        /// the control
        control: Option<Control>,
        /// indicate whether the rolling stock has a motor or not
        is_dummy: bool,
    },
}
