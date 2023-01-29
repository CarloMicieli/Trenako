use crate::catalog_items::category::{
    ElectricMultipleUnitType, FreightCarType, LocomotiveType, PassengerCarType, RailcarType, RollingStockCategory,
};
use crate::catalog_items::control::{Control, DccInterface};
use crate::catalog_items::epoch::Epoch;
use crate::catalog_items::length_over_buffers::LengthOverBuffers;
use crate::catalog_items::service_level::ServiceLevel;
use crate::catalog_items::technical_specifications::TechnicalSpecifications;

/// It represents a request to create / modify a rolling stock
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(tag = "category")]
pub enum RollingStockRequest {
    /// an electric multiple unit rolling stock
    #[serde(rename = "ELECTRIC_MULTIPLE_UNIT")]
    ElectricMultipleUnitRequest {
        /// the railway name for this rolling stock
        railway: String,
        /// the epoch for this rolling stock
        epoch: Epoch,
        /// the livery description
        livery: Option<String>,
        /// the overall length
        length_over_buffers: Option<LengthOverBuffers>,
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
    #[serde(rename = "FREIGHT_CAR")]
    FreightCarRequest {
        /// the railway name for this rolling stock
        railway: String,
        /// the epoch for this rolling stock
        epoch: Epoch,
        /// the livery description
        livery: Option<String>,
        /// the overall length
        length_over_buffers: Option<LengthOverBuffers>,
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
    #[serde(rename = "LOCOMOTIVE")]
    LocomotiveRequest {
        /// the railway name for this rolling stock
        railway: String,
        /// the epoch for this rolling stock
        epoch: Epoch,
        /// the livery description
        livery: Option<String>,
        /// the overall length
        length_over_buffers: Option<LengthOverBuffers>,
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
    #[serde(rename = "PASSENGER_CAR")]
    PassengerCarRequest {
        /// the railway name for this rolling stock
        railway: String,
        /// the epoch for this rolling stock
        epoch: Epoch,
        /// the livery description
        livery: Option<String>,
        /// the overall length
        length_over_buffers: Option<LengthOverBuffers>,
        /// the technical specifications
        technical_specifications: Option<TechnicalSpecifications>,
        /// the passenger car type name
        type_name: String,
        /// the identification marking for this passenger car
        road_number: Option<String>,
        /// the prototype series information
        series: Option<String>,
        /// the passenger car type
        passenger_car_type: Option<PassengerCarType>,
        /// the travel class for this passenger car. Passenger cars can have multiple service
        /// levels (ie, '1st/2nd')
        service_level: Option<ServiceLevel>,
    },
    /// a railcar rolling stock
    #[serde(rename = "RAILCAR")]
    RailcarRequest {
        /// the railway name for this rolling stock
        railway: String,
        /// the epoch for this rolling stock
        epoch: Epoch,
        /// the livery description
        livery: Option<String>,
        /// the overall length
        length_over_buffers: Option<LengthOverBuffers>,
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

impl RollingStockRequest {
    /// Returns the rolling stock category
    pub fn category(&self) -> RollingStockCategory {
        match self {
            RollingStockRequest::ElectricMultipleUnitRequest { .. } => RollingStockCategory::ElectricMultipleUnit,
            RollingStockRequest::FreightCarRequest { .. } => RollingStockCategory::FreightCar,
            RollingStockRequest::LocomotiveRequest { .. } => RollingStockCategory::Locomotive,
            RollingStockRequest::PassengerCarRequest { .. } => RollingStockCategory::PassengerCar,
            RollingStockRequest::RailcarRequest { .. } => RollingStockCategory::Railcar,
        }
    }

    /// Returns the railway for this rolling stock request
    pub fn railway(&self) -> &str {
        match self {
            RollingStockRequest::ElectricMultipleUnitRequest { railway, .. } => railway,
            RollingStockRequest::FreightCarRequest { railway, .. } => railway,
            RollingStockRequest::LocomotiveRequest { railway, .. } => railway,
            RollingStockRequest::PassengerCarRequest { railway, .. } => railway,
            RollingStockRequest::RailcarRequest { railway, .. } => railway,
        }
    }

    /// Returns the rolling stock length over buffers
    pub fn length_over_buffers(&self) -> Option<&LengthOverBuffers> {
        match self {
            RollingStockRequest::ElectricMultipleUnitRequest {
                length_over_buffers, ..
            } => length_over_buffers.as_ref(),
            RollingStockRequest::FreightCarRequest {
                length_over_buffers, ..
            } => length_over_buffers.as_ref(),
            RollingStockRequest::LocomotiveRequest {
                length_over_buffers, ..
            } => length_over_buffers.as_ref(),
            RollingStockRequest::PassengerCarRequest {
                length_over_buffers, ..
            } => length_over_buffers.as_ref(),
            RollingStockRequest::RailcarRequest {
                length_over_buffers, ..
            } => length_over_buffers.as_ref(),
        }
    }

    /// Returns the technical specifications for this rolling stock request
    pub fn technical_specifications(&self) -> Option<&TechnicalSpecifications> {
        match self {
            RollingStockRequest::ElectricMultipleUnitRequest {
                technical_specifications,
                ..
            } => technical_specifications.as_ref(),
            RollingStockRequest::FreightCarRequest {
                technical_specifications,
                ..
            } => technical_specifications.as_ref(),
            RollingStockRequest::LocomotiveRequest {
                technical_specifications,
                ..
            } => technical_specifications.as_ref(),
            RollingStockRequest::PassengerCarRequest {
                technical_specifications,
                ..
            } => technical_specifications.as_ref(),
            RollingStockRequest::RailcarRequest {
                technical_specifications,
                ..
            } => technical_specifications.as_ref(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod rolling_stock_requests {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;

        #[rstest]
        #[case(electric_multiple_unit_request(), RollingStockCategory::ElectricMultipleUnit)]
        #[case(freight_car_request(), RollingStockCategory::FreightCar)]
        #[case(locomotive_request(), RollingStockCategory::Locomotive)]
        #[case(passenger_car_request(), RollingStockCategory::PassengerCar)]
        #[case(railcar_request(), RollingStockCategory::Railcar)]
        fn it_should_return_the_category(#[case] request: RollingStockRequest, #[case] expected: RollingStockCategory) {
            assert_eq!(expected, request.category());
        }

        #[rstest]
        #[case(electric_multiple_unit_request(), "FS")]
        #[case(freight_car_request(), "FS")]
        #[case(locomotive_request(), "FS")]
        #[case(passenger_car_request(), "FS")]
        #[case(railcar_request(), "FS")]
        fn it_should_return_the_railway(#[case] request: RollingStockRequest, #[case] expected: &str) {
            assert_eq!(expected, request.railway());
        }

        #[rstest]
        #[case(electric_multiple_unit_request())]
        #[case(freight_car_request())]
        #[case(locomotive_request())]
        #[case(passenger_car_request())]
        #[case(railcar_request())]
        fn it_should_return_the_technical_specifications(#[case] request: RollingStockRequest) {
            let expected = technical_specifications();
            assert_eq!(expected.as_ref(), request.technical_specifications());
        }

        fn electric_multiple_unit_request() -> RollingStockRequest {
            RollingStockRequest::ElectricMultipleUnitRequest {
                railway: "FS".to_string(),
                epoch: Epoch::IV,
                livery: None,
                length_over_buffers: None,
                technical_specifications: technical_specifications(),
                type_name: "111".to_string(),
                road_number: None,
                series: None,
                depot: None,
                electric_multiple_unit_type: ElectricMultipleUnitType::PowerCar,
                dcc_interface: None,
                control: None,
                is_dummy: false,
            }
        }

        fn freight_car_request() -> RollingStockRequest {
            RollingStockRequest::FreightCarRequest {
                railway: "FS".to_string(),
                epoch: Epoch::IV,
                livery: None,
                length_over_buffers: None,
                technical_specifications: technical_specifications(),
                type_name: "111".to_string(),
                road_number: None,
                freight_car_type: None,
            }
        }

        fn locomotive_request() -> RollingStockRequest {
            RollingStockRequest::LocomotiveRequest {
                railway: "FS".to_string(),
                epoch: Epoch::IV,
                livery: None,
                length_over_buffers: None,
                technical_specifications: technical_specifications(),
                class_name: "111".to_string(),
                road_number: "999".to_string(),
                series: None,
                depot: None,
                locomotive_type: LocomotiveType::ElectricLocomotive,
                dcc_interface: None,
                control: None,
                is_dummy: false,
            }
        }

        fn passenger_car_request() -> RollingStockRequest {
            RollingStockRequest::PassengerCarRequest {
                railway: "FS".to_string(),
                epoch: Epoch::IV,
                livery: None,
                length_over_buffers: None,
                technical_specifications: technical_specifications(),
                type_name: "111".to_string(),
                road_number: None,
                series: None,
                passenger_car_type: None,
                service_level: None,
            }
        }

        fn railcar_request() -> RollingStockRequest {
            RollingStockRequest::RailcarRequest {
                railway: "FS".to_string(),
                epoch: Epoch::IV,
                livery: None,
                length_over_buffers: None,
                technical_specifications: technical_specifications(),
                type_name: "111".to_string(),
                road_number: None,
                series: None,
                depot: None,
                railcar_type: RailcarType::PowerCar,
                dcc_interface: None,
                control: None,
                is_dummy: false,
            }
        }

        fn technical_specifications() -> Option<TechnicalSpecifications> {
            Some(TechnicalSpecifications::default())
        }
    }
}
