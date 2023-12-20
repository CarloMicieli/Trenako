//! the rolling stock command request

use crate::catalog_items::category::{
    ElectricMultipleUnitType, FreightCarType, LocomotiveType, PassengerCarType, RailcarType, RollingStockCategory,
};
use crate::catalog_items::control::{Control, DccInterface};
use crate::catalog_items::length_over_buffers::LengthOverBuffers;
use crate::catalog_items::rolling_stock_request::validators::{
    validate_electric_multiple_unit, validate_freight_car, validate_locomotive, validate_passenger_car,
    validate_railcar,
};
use crate::catalog_items::service_level::ServiceLevel;
use crate::catalog_items::technical_specifications::TechnicalSpecifications;
use common::validation::Validator;
use validator::{Validate, ValidationErrors};

/// It represents a request to create / modify a rolling stock
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "category")]
pub enum RollingStockRequest {
    /// an electric multiple unit rolling stock
    #[serde(rename = "ELECTRIC_MULTIPLE_UNIT")]
    ElectricMultipleUnitRequest {
        /// the railway name for this rolling stock
        railway: String,
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

    /// Returns the rolling stock livery
    pub fn livery(&self) -> Option<&String> {
        match self {
            RollingStockRequest::ElectricMultipleUnitRequest { livery, .. } => livery.as_ref(),
            RollingStockRequest::FreightCarRequest { livery, .. } => livery.as_ref(),
            RollingStockRequest::LocomotiveRequest { livery, .. } => livery.as_ref(),
            RollingStockRequest::PassengerCarRequest { livery, .. } => livery.as_ref(),
            RollingStockRequest::RailcarRequest { livery, .. } => livery.as_ref(),
        }
    }

    /// Returns the railway for this rolling stock request
    pub fn railway(&self) -> &String {
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

impl Validate for RollingStockRequest {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut validator = Validator::new();

        validator.validate_length("railway", Some(2), Some(50), self.railway());
        validator.validate_length_optional("livery", None, Some(50), self.livery());

        match self {
            RollingStockRequest::LocomotiveRequest { .. } => validate_locomotive(&mut validator, self),
            RollingStockRequest::FreightCarRequest { .. } => validate_freight_car(&mut validator, self),
            RollingStockRequest::PassengerCarRequest { .. } => validate_passenger_car(&mut validator, self),
            RollingStockRequest::RailcarRequest { .. } => validate_railcar(&mut validator, self),
            RollingStockRequest::ElectricMultipleUnitRequest { .. } => {
                validate_electric_multiple_unit(&mut validator, self)
            }
        };

        let nested_result = self
            .technical_specifications()
            .map(|it| it.validate())
            .unwrap_or(Ok(()));

        validator = validator.add_nested("technical_specifications", nested_result);

        validator.into()
    }
}

mod validators {
    use crate::catalog_items::rolling_stock_request::RollingStockRequest;
    use common::validation::Validator;

    pub fn validate_freight_car(validator: &mut Validator, input: &RollingStockRequest) {
        if let RollingStockRequest::FreightCarRequest {
            type_name, road_number, ..
        } = input
        {
            validator.validate_length("type_name", Some(3), Some(25), type_name);
            validator.validate_length_optional("road_number", None, Some(50), road_number.as_ref());
        }
    }

    pub fn validate_passenger_car(validator: &mut Validator, input: &RollingStockRequest) {
        if let RollingStockRequest::PassengerCarRequest {
            type_name,
            road_number,
            series,
            ..
        } = input
        {
            validator.validate_length("type_name", Some(3), Some(25), type_name);
            validator.validate_length_optional("road_number", None, Some(50), road_number.as_ref());
            validator.validate_length_optional("series", None, Some(50), series.as_ref());
        }
    }

    pub fn validate_railcar(validator: &mut Validator, input: &RollingStockRequest) {
        if let RollingStockRequest::RailcarRequest {
            type_name,
            road_number,
            series,
            depot,
            ..
        } = input
        {
            validator.validate_length("type_name", Some(3), Some(15), type_name);
            validator.validate_length_optional("road_number", None, Some(50), road_number.as_ref());
            validator.validate_length_optional("series", None, Some(50), series.as_ref());
            validator.validate_length_optional("depot", None, Some(100), depot.as_ref());
        }
    }

    pub fn validate_electric_multiple_unit(validator: &mut Validator, input: &RollingStockRequest) {
        if let RollingStockRequest::ElectricMultipleUnitRequest {
            type_name,
            road_number,
            series,
            depot,
            ..
        } = input
        {
            validator.validate_length("type_name", Some(3), Some(15), type_name);
            validator.validate_length_optional("road_number", None, Some(50), road_number.as_ref());
            validator.validate_length_optional("series", None, Some(50), series.as_ref());
            validator.validate_length_optional("depot", None, Some(100), depot.as_ref());
        }
    }

    pub fn validate_locomotive(validator: &mut Validator, input: &RollingStockRequest) {
        if let RollingStockRequest::LocomotiveRequest {
            class_name,
            road_number,
            series,
            depot,
            ..
        } = input
        {
            validator.validate_length("class_name", Some(3), Some(15), class_name);
            validator.validate_length("road_number", Some(3), Some(50), road_number);
            validator.validate_length_optional("series", None, Some(50), series.as_ref());
            validator.validate_length_optional("depot", None, Some(100), depot.as_ref());
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod rolling_stock_requests {
        use super::data::{
            electric_multiple_unit_request, freight_car_request, locomotive_request, passenger_car_request,
            railcar_request,
        };
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
        #[case(electric_multiple_unit_request(), Some(String::from("castano/isabella")))]
        #[case(freight_car_request(), Some(String::from("castano/isabella")))]
        #[case(locomotive_request(), Some(String::from("castano/isabella")))]
        #[case(passenger_car_request(), Some(String::from("castano/isabella")))]
        #[case(railcar_request(), Some(String::from("castano/isabella")))]
        fn it_should_return_the_livery(#[case] request: RollingStockRequest, #[case] expected: Option<String>) {
            assert_eq!(expected.as_ref(), request.livery());
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

        fn technical_specifications() -> Option<TechnicalSpecifications> {
            Some(TechnicalSpecifications::default())
        }
    }

    mod electric_multiple_unit_requests_validation {
        use crate::catalog_items::category::ElectricMultipleUnitType;
        use crate::catalog_items::rolling_stock_request::RollingStockRequest;
        use crate::test_helpers::random_str;
        use rstest::rstest;
        use validator::Validate;

        #[test]
        fn it_should_validate_electric_multiple_units() {
            let request = new_request_with(None, None, None, None, None, None);
            let result = request.validate();
            assert!(result.is_ok());
        }

        #[rstest]
        #[case(random_str(0))]
        #[case(random_str(1))]
        #[case(random_str(51))]
        fn it_should_validate_electric_multiple_unit_railway_name(#[case] input: String) {
            let request = new_request_with(Some(&input), None, None, None, None, None);

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert_eq!(1, errors.len());
            assert!(errors.contains_key("railway"));
            assert_eq!(errors["railway"].len(), 1);
            assert_eq!(errors["railway"][0].code, "length");
            assert_eq!(errors["railway"][0].params["value"], input);
            assert_eq!(errors["railway"][0].params["min"], 2);
            assert_eq!(errors["railway"][0].params["max"], 50);
        }

        #[rstest]
        #[case(random_str(0))]
        #[case(random_str(1))]
        #[case(random_str(2))]
        #[case(random_str(51))]
        fn it_should_validate_electric_multiple_unit_type_name(#[case] input: String) {
            let request = new_request_with(None, Some(&input), None, None, None, None);

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert_eq!(1, errors.len());
            assert!(errors.contains_key("type_name"));
            assert_eq!(errors["type_name"].len(), 1);
            assert_eq!(errors["type_name"][0].code, "length");
            assert_eq!(errors["type_name"][0].params["value"], input);
            assert_eq!(errors["type_name"][0].params["min"], 3);
            assert_eq!(errors["type_name"][0].params["max"], 15);
        }

        #[rstest]
        #[case(random_str(51))]
        fn it_should_validate_electric_multiple_unit_road_number(#[case] input: String) {
            let request = new_request_with(None, None, Some(&input), None, None, None);

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert_eq!(1, errors.len());
            assert!(errors.contains_key("road_number"));
            assert_eq!(errors["road_number"].len(), 1);
            assert_eq!(errors["road_number"][0].code, "length");
            assert_eq!(errors["road_number"][0].params["value"], input);
            assert_eq!(errors["road_number"][0].params["max"], 50);
        }

        #[rstest]
        #[case(random_str(101))]
        fn it_should_validate_electric_multiple_unit_depot(#[case] input: String) {
            let request = new_request_with(None, None, None, None, Some(&input), None);

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert_eq!(1, errors.len());
            assert!(errors.contains_key("depot"));
            assert_eq!(errors["depot"].len(), 1);
            assert_eq!(errors["depot"][0].code, "length");
            assert_eq!(errors["depot"][0].params["value"], input);
            assert_eq!(errors["depot"][0].params["max"], 100);
        }

        #[rstest]
        #[case(random_str(51))]
        fn it_should_validate_electric_multiple_unit_series(#[case] input: String) {
            let request = new_request_with(None, None, None, Some(&input), None, None);

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert_eq!(1, errors.len());
            assert!(errors.contains_key("series"));
            assert_eq!(errors["series"].len(), 1);
            assert_eq!(errors["series"][0].code, "length");
            assert_eq!(errors["series"][0].params["value"], input);
            assert_eq!(errors["series"][0].params["max"], 50);
        }

        #[rstest]
        #[case(random_str(51))]
        fn it_should_validate_electric_multiple_unit_livery(#[case] input: String) {
            let request = new_request_with(None, None, None, None, None, Some(&input));

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert_eq!(1, errors.len());
            assert!(errors.contains_key("livery"));
            assert_eq!(errors["livery"].len(), 1);
            assert_eq!(errors["livery"][0].code, "length");
            assert_eq!(errors["livery"][0].params["value"], input);
            assert_eq!(errors["livery"][0].params["max"], 50);
        }

        fn new_request_with(
            railway: Option<&String>,
            type_name: Option<&String>,
            road_number: Option<&String>,
            series: Option<&String>,
            depot: Option<&String>,
            livery: Option<&String>,
        ) -> RollingStockRequest {
            let railway: String = railway.map(String::to_string).unwrap_or(String::from("FS"));
            let type_name: String = type_name.map(String::to_string).unwrap_or(String::from("111"));

            RollingStockRequest::ElectricMultipleUnitRequest {
                railway,
                livery: livery.map(String::to_string),
                length_over_buffers: None,
                technical_specifications: None,
                type_name,
                road_number: road_number.map(String::to_string),
                series: series.map(String::to_string),
                depot: depot.map(String::to_string),
                electric_multiple_unit_type: ElectricMultipleUnitType::DrivingCar,
                dcc_interface: None,
                control: None,
                is_dummy: false,
            }
        }
    }

    mod freight_car_requests_validation {
        use crate::catalog_items::category::FreightCarType;
        use crate::catalog_items::rolling_stock_request::RollingStockRequest;
        use crate::test_helpers::random_str;
        use rstest::rstest;
        use validator::Validate;

        #[test]
        fn it_should_validate_freight_cars() {
            let request = new_request_with(None, None, None, None);
            let result = request.validate();
            assert!(result.is_ok());
        }

        #[rstest]
        #[case(random_str(0))]
        #[case(random_str(1))]
        #[case(random_str(51))]
        fn it_should_validate_freight_car_railway_name(#[case] input: String) {
            let request = new_request_with(Some(&input), None, None, None);

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert_eq!(1, errors.len());
            assert!(errors.contains_key("railway"));
            assert_eq!(errors["railway"].len(), 1);
            assert_eq!(errors["railway"][0].code, "length");
            assert_eq!(errors["railway"][0].params["value"], input);
            assert_eq!(errors["railway"][0].params["min"], 2);
            assert_eq!(errors["railway"][0].params["max"], 50);
        }

        #[rstest]
        #[case(random_str(1))]
        #[case(random_str(2))]
        #[case(random_str(51))]
        fn it_should_validate_freight_car_type_name(#[case] input: String) {
            let request = new_request_with(None, Some(&input), None, None);

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert_eq!(1, errors.len());
            assert!(errors.contains_key("type_name"));
            assert_eq!(errors["type_name"].len(), 1);
            assert_eq!(errors["type_name"][0].code, "length");
            assert_eq!(errors["type_name"][0].params["value"], input);
            assert_eq!(errors["type_name"][0].params["min"], 3);
            assert_eq!(errors["type_name"][0].params["max"], 25);
        }

        #[rstest]
        #[case(random_str(51))]
        fn it_should_validate_freight_car_road_number(#[case] input: String) {
            let request = new_request_with(None, None, Some(&input), None);

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert_eq!(1, errors.len());
            assert!(errors.contains_key("road_number"));
            assert_eq!(errors["road_number"].len(), 1);
            assert_eq!(errors["road_number"][0].code, "length");
            assert_eq!(errors["road_number"][0].params["value"], input);
            assert_eq!(errors["road_number"][0].params["max"], 50);
        }

        #[rstest]
        #[case(random_str(51))]
        fn it_should_validate_freight_car_livery(#[case] input: String) {
            let request = new_request_with(None, None, None, Some(&input));

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert_eq!(1, errors.len());
            assert!(errors.contains_key("livery"));
            assert_eq!(errors["livery"].len(), 1);
            assert_eq!(errors["livery"][0].code, "length");
            assert_eq!(errors["livery"][0].params["value"], input);
            assert_eq!(errors["livery"][0].params["max"], 50);
        }

        fn new_request_with(
            railway: Option<&String>,
            type_name: Option<&String>,
            road_number: Option<&String>,
            livery: Option<&String>,
        ) -> RollingStockRequest {
            let railway: String = railway.map(String::to_string).unwrap_or(String::from("FS"));
            let type_name: String = type_name.map(String::to_string).unwrap_or(String::from("111"));

            RollingStockRequest::FreightCarRequest {
                railway,
                livery: livery.map(String::to_string),
                length_over_buffers: None,
                technical_specifications: None,
                type_name,
                road_number: road_number.map(String::to_string),
                freight_car_type: Some(FreightCarType::CoveredFreightCars),
            }
        }
    }

    mod locomotive_requests_validation {
        use crate::catalog_items::category::LocomotiveType;
        use crate::catalog_items::rolling_stock_request::RollingStockRequest;
        use crate::test_helpers::random_str;
        use rstest::rstest;
        use validator::Validate;

        #[test]
        fn it_should_validate_locomotives() {
            let request = new_request_with(None, None, None, None, None, None);
            let result = request.validate();
            assert!(result.is_ok());
        }

        #[rstest]
        #[case(random_str(0))]
        #[case(random_str(1))]
        #[case(random_str(51))]
        fn it_should_validate_locomotive_railway_name(#[case] input: String) {
            let request = new_request_with(Some(&input), None, None, None, None, None);

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert_eq!(1, errors.len());
            assert!(errors.contains_key("railway"));
            assert_eq!(errors["railway"].len(), 1);
            assert_eq!(errors["railway"][0].code, "length");
            assert_eq!(errors["railway"][0].params["value"], input);
            assert_eq!(errors["railway"][0].params["min"], 2.0);
            assert_eq!(errors["railway"][0].params["max"], 50.0);
        }

        #[rstest]
        #[case(random_str(0))]
        #[case(random_str(1))]
        #[case(random_str(2))]
        #[case(random_str(51))]
        fn it_should_validate_locomotive_class_name(#[case] input: String) {
            let request = new_request_with(None, Some(&input), None, None, None, None);

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert_eq!(1, errors.len());
            assert!(errors.contains_key("class_name"));
            assert_eq!(errors["class_name"].len(), 1);
            assert_eq!(errors["class_name"][0].code, "length");
            assert_eq!(errors["class_name"][0].params["value"], input);
            assert_eq!(errors["class_name"][0].params["min"], 3.0);
            assert_eq!(errors["class_name"][0].params["max"], 15.0);
        }

        #[rstest]
        #[case(random_str(0))]
        #[case(random_str(1))]
        #[case(random_str(2))]
        #[case(random_str(51))]
        fn it_should_validate_locomotive_road_number(#[case] input: String) {
            let request = new_request_with(None, None, Some(&input), None, None, None);

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert_eq!(1, errors.len());
            assert!(errors.contains_key("road_number"));
            assert_eq!(errors["road_number"].len(), 1);
            assert_eq!(errors["road_number"][0].code, "length");
            assert_eq!(errors["road_number"][0].params["value"], input);
            assert_eq!(errors["road_number"][0].params["min"], 3.0);
            assert_eq!(errors["road_number"][0].params["max"], 50.0);
        }

        #[rstest]
        #[case(random_str(101))]
        fn it_should_validate_locomotive_depot(#[case] input: String) {
            let request = new_request_with(None, None, None, None, Some(&input), None);

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert_eq!(1, errors.len());
            assert!(errors.contains_key("depot"));
            assert_eq!(errors["depot"].len(), 1);
            assert_eq!(errors["depot"][0].code, "length");
            assert_eq!(errors["depot"][0].params["value"], input);
            assert_eq!(errors["depot"][0].params["max"], 100);
        }

        #[rstest]
        #[case(random_str(51))]
        fn it_should_validate_locomotive_series(#[case] input: String) {
            let request = new_request_with(None, None, None, Some(&input), None, None);

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert_eq!(1, errors.len());
            assert!(errors.contains_key("series"));
            assert_eq!(errors["series"].len(), 1);
            assert_eq!(errors["series"][0].code, "length");
            assert_eq!(errors["series"][0].params["value"], input);
            assert_eq!(errors["series"][0].params["max"], 50);
        }

        #[rstest]
        #[case(random_str(51))]
        fn it_should_validate_locomotive_livery(#[case] input: String) {
            let request = new_request_with(None, None, None, None, None, Some(&input));

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert_eq!(1, errors.len());
            assert!(errors.contains_key("livery"));
            assert_eq!(errors["livery"].len(), 1);
            assert_eq!(errors["livery"][0].code, "length");
            assert_eq!(errors["livery"][0].params["value"], input);
            assert_eq!(errors["livery"][0].params["max"], 50);
        }

        fn new_request_with(
            railway: Option<&String>,
            class_name: Option<&String>,
            road_number: Option<&String>,
            series: Option<&String>,
            depot: Option<&String>,
            livery: Option<&String>,
        ) -> RollingStockRequest {
            let railway: String = railway.map(String::to_string).unwrap_or(String::from("FS"));
            let class_name: String = class_name.map(String::to_string).unwrap_or(String::from("111"));
            let road_number: String = road_number.map(String::to_string).unwrap_or(String::from("222"));

            RollingStockRequest::LocomotiveRequest {
                railway,
                livery: livery.map(String::to_string),
                length_over_buffers: None,
                technical_specifications: None,
                class_name,
                road_number,
                series: series.map(String::to_string),
                depot: depot.map(String::to_string),
                locomotive_type: LocomotiveType::ElectricLocomotive,
                dcc_interface: None,
                control: None,
                is_dummy: false,
            }
        }
    }

    mod passenger_car_requests_validation {
        use crate::catalog_items::category::PassengerCarType;
        use crate::catalog_items::rolling_stock_request::RollingStockRequest;
        use crate::catalog_items::service_level::ServiceLevel;
        use crate::test_helpers::random_str;
        use rstest::rstest;
        use validator::Validate;

        #[test]
        fn it_should_validate_passenger_cars() {
            let request = new_request_with(None, None, None, None, None);
            let result = request.validate();
            assert!(result.is_ok());
        }

        #[rstest]
        #[case(random_str(0))]
        #[case(random_str(1))]
        #[case(random_str(51))]
        fn it_should_validate_passenger_car_railway_name(#[case] input: String) {
            let request = new_request_with(Some(&input), None, None, None, None);

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert_eq!(1, errors.len());
            assert!(errors.contains_key("railway"));
            assert_eq!(errors["railway"].len(), 1);
            assert_eq!(errors["railway"][0].code, "length");
            assert_eq!(errors["railway"][0].params["value"], input);
            assert_eq!(errors["railway"][0].params["min"], 2);
            assert_eq!(errors["railway"][0].params["max"], 50);
        }

        #[rstest]
        #[case(random_str(1))]
        #[case(random_str(2))]
        #[case(random_str(51))]
        fn it_should_validate_passenger_car_type_name(#[case] input: String) {
            let request = new_request_with(None, Some(&input), None, None, None);

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert_eq!(1, errors.len());
            assert!(errors.contains_key("type_name"));
            assert_eq!(errors["type_name"].len(), 1);
            assert_eq!(errors["type_name"][0].code, "length");
            assert_eq!(errors["type_name"][0].params["value"], input);
            assert_eq!(errors["type_name"][0].params["min"], 3);
            assert_eq!(errors["type_name"][0].params["max"], 25);
        }

        #[rstest]
        #[case(random_str(51))]
        fn it_should_validate_passenger_car_road_number(#[case] input: String) {
            let request = new_request_with(None, None, Some(&input), None, None);

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert_eq!(1, errors.len());
            assert!(errors.contains_key("road_number"));
            assert_eq!(errors["road_number"].len(), 1);
            assert_eq!(errors["road_number"][0].code, "length");
            assert_eq!(errors["road_number"][0].params["value"], input);
            assert_eq!(errors["road_number"][0].params["max"], 50);
        }

        #[rstest]
        #[case(random_str(51))]
        fn it_should_validate_passenger_car_series(#[case] input: String) {
            let request = new_request_with(None, None, None, Some(&input), None);

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert_eq!(1, errors.len());
            assert!(errors.contains_key("series"));
            assert_eq!(errors["series"].len(), 1);
            assert_eq!(errors["series"][0].code, "length");
            assert_eq!(errors["series"][0].params["value"], input);
            assert_eq!(errors["series"][0].params["max"], 50);
        }

        #[rstest]
        #[case(random_str(51))]
        fn it_should_validate_passenger_car_livery(#[case] input: String) {
            let request = new_request_with(None, None, None, None, Some(&input));

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert_eq!(1, errors.len());
            assert!(errors.contains_key("livery"));
            assert_eq!(errors["livery"].len(), 1);
            assert_eq!(errors["livery"][0].code, "length");
            assert_eq!(errors["livery"][0].params["value"], input);
            assert_eq!(errors["livery"][0].params["max"], 50);
        }

        fn new_request_with(
            railway: Option<&String>,
            type_name: Option<&String>,
            road_number: Option<&String>,
            series: Option<&String>,
            livery: Option<&String>,
        ) -> RollingStockRequest {
            let railway: String = railway.map(String::to_string).unwrap_or(String::from("FS"));
            let type_name: String = type_name.map(String::to_string).unwrap_or(String::from("111"));

            RollingStockRequest::PassengerCarRequest {
                railway,
                livery: livery.map(String::to_string),
                length_over_buffers: None,
                technical_specifications: None,
                type_name,
                road_number: road_number.map(String::to_string),
                series: series.map(String::to_string),
                passenger_car_type: Some(PassengerCarType::OpenCoach),
                service_level: Some(ServiceLevel::FirstClass),
            }
        }
    }

    mod railcar_requests_validation {
        use crate::catalog_items::category::RailcarType;
        use crate::catalog_items::rolling_stock_request::RollingStockRequest;
        use crate::test_helpers::random_str;
        use rstest::rstest;
        use validator::Validate;

        #[test]
        fn it_should_validate_railcars() {
            let request = new_request_with(None, None, None, None, None, None);
            let result = request.validate();
            assert!(result.is_ok());
        }

        #[rstest]
        #[case(random_str(0))]
        #[case(random_str(1))]
        #[case(random_str(51))]
        fn it_should_validate_railcar_railway_name(#[case] input: String) {
            let request = new_request_with(Some(&input), None, None, None, None, None);

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert_eq!(1, errors.len());
            assert!(errors.contains_key("railway"));
            assert_eq!(errors["railway"].len(), 1);
            assert_eq!(errors["railway"][0].code, "length");
            assert_eq!(errors["railway"][0].params["value"], input);
            assert_eq!(errors["railway"][0].params["min"], 2);
            assert_eq!(errors["railway"][0].params["max"], 50);
        }

        #[rstest]
        #[case(random_str(0))]
        #[case(random_str(1))]
        #[case(random_str(2))]
        #[case(random_str(51))]
        fn it_should_validate_railcar_type_name(#[case] input: String) {
            let request = new_request_with(None, Some(&input), None, None, None, None);

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert_eq!(1, errors.len());
            assert!(errors.contains_key("type_name"));
            assert_eq!(errors["type_name"].len(), 1);
            assert_eq!(errors["type_name"][0].code, "length");
            assert_eq!(errors["type_name"][0].params["value"], input);
            assert_eq!(errors["type_name"][0].params["min"], 3);
            assert_eq!(errors["type_name"][0].params["max"], 15);
        }

        #[rstest]
        #[case(random_str(51))]
        fn it_should_validate_railcar_road_number(#[case] input: String) {
            let request = new_request_with(None, None, Some(&input), None, None, None);

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert_eq!(1, errors.len());
            assert!(errors.contains_key("road_number"));
            assert_eq!(errors["road_number"].len(), 1);
            assert_eq!(errors["road_number"][0].code, "length");
            assert_eq!(errors["road_number"][0].params["value"], input);
            assert_eq!(errors["road_number"][0].params["max"], 50);
        }

        #[rstest]
        #[case(random_str(101))]
        fn it_should_validate_railcar_depot(#[case] input: String) {
            let request = new_request_with(None, None, None, None, Some(&input), None);

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert_eq!(1, errors.len());
            assert!(errors.contains_key("depot"));
            assert_eq!(errors["depot"].len(), 1);
            assert_eq!(errors["depot"][0].code, "length");
            assert_eq!(errors["depot"][0].params["value"], input);
            assert_eq!(errors["depot"][0].params["max"], 100);
        }

        #[rstest]
        #[case(random_str(51))]
        fn it_should_validate_railcar_series(#[case] input: String) {
            let request = new_request_with(None, None, None, Some(&input), None, None);

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert_eq!(1, errors.len());
            assert!(errors.contains_key("series"));
            assert_eq!(errors["series"].len(), 1);
            assert_eq!(errors["series"][0].code, "length");
            assert_eq!(errors["series"][0].params["value"], input);
            assert_eq!(errors["series"][0].params["max"], 50);
        }

        #[rstest]
        #[case(random_str(51))]
        fn it_should_validate_railcar_livery(#[case] input: String) {
            let request = new_request_with(None, None, None, None, None, Some(&input));

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert_eq!(1, errors.len());
            assert!(errors.contains_key("livery"));
            assert_eq!(errors["livery"].len(), 1);
            assert_eq!(errors["livery"][0].code, "length");
            assert_eq!(errors["livery"][0].params["value"], input);
            assert_eq!(errors["livery"][0].params["max"], 50);
        }

        fn new_request_with(
            railway: Option<&String>,
            type_name: Option<&String>,
            road_number: Option<&String>,
            series: Option<&String>,
            depot: Option<&String>,
            livery: Option<&String>,
        ) -> RollingStockRequest {
            let railway: String = railway.map(String::to_string).unwrap_or(String::from("FS"));
            let type_name: String = type_name.map(String::to_string).unwrap_or(String::from("111"));

            RollingStockRequest::RailcarRequest {
                railway,
                livery: livery.map(String::to_string),
                length_over_buffers: None,
                technical_specifications: None,
                type_name,
                road_number: road_number.map(String::to_string),
                series: series.map(String::to_string),
                depot: depot.map(String::to_string),
                railcar_type: RailcarType::PowerCar,
                dcc_interface: None,
                control: None,
                is_dummy: false,
            }
        }
    }
}

#[cfg(test)]
pub mod data {
    use crate::catalog_items::category::{ElectricMultipleUnitType, LocomotiveType, RailcarType};
    use crate::catalog_items::rolling_stock_request::RollingStockRequest;
    use crate::catalog_items::technical_specifications::TechnicalSpecifications;

    /// Creates a test value for an electric multiple unit request
    pub fn electric_multiple_unit_request() -> RollingStockRequest {
        RollingStockRequest::ElectricMultipleUnitRequest {
            railway: "FS".to_string(),
            livery: Some(String::from("castano/isabella")),
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

    /// Creates a test value for a freight car request
    pub fn freight_car_request() -> RollingStockRequest {
        RollingStockRequest::FreightCarRequest {
            railway: "FS".to_string(),
            livery: Some(String::from("castano/isabella")),
            length_over_buffers: None,
            technical_specifications: technical_specifications(),
            type_name: "111".to_string(),
            road_number: None,
            freight_car_type: None,
        }
    }

    /// Creates a test value for a locomotive request
    pub fn locomotive_request() -> RollingStockRequest {
        RollingStockRequest::LocomotiveRequest {
            railway: "FS".to_string(),
            livery: Some(String::from("castano/isabella")),
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

    /// Creates a test value for a passenger car request
    pub fn passenger_car_request() -> RollingStockRequest {
        RollingStockRequest::PassengerCarRequest {
            railway: "FS".to_string(),
            livery: Some(String::from("castano/isabella")),
            length_over_buffers: None,
            technical_specifications: technical_specifications(),
            type_name: "111".to_string(),
            road_number: None,
            series: None,
            passenger_car_type: None,
            service_level: None,
        }
    }

    /// Creates a test value for a railcar request
    pub fn railcar_request() -> RollingStockRequest {
        RollingStockRequest::RailcarRequest {
            railway: "FS".to_string(),
            livery: Some(String::from("castano/isabella")),
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
