use crate::catalog_items::category::{
    Category, ElectricMultipleUnitType, FreightCarType, LocomotiveType, PassengerCarType, RailcarType,
};
use crate::catalog_items::control::{Control, DccInterface};
use crate::catalog_items::epoch::Epoch;
use crate::catalog_items::length_over_buffer::LengthOverBuffer;
use crate::catalog_items::rolling_stock_id::RollingStockId;
use crate::catalog_items::service_level::ServiceLevel;
use crate::catalog_items::technical_specifications::TechnicalSpecifications;
use crate::railways::railway_id::RailwayId;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(tag = "category")]
pub enum RollingStock {
    ElectricMultipleUnit {
        id: RollingStockId,
        type_name: String,
        road_number: Option<String>,
        railway: RollingStockRailway,
        epoch: Epoch,
        electric_multiple_unit_type: ElectricMultipleUnitType,
        depot: Option<String>,
        livery: Option<String>,
        is_dummy: bool,
        length_over_buffer: Option<LengthOverBuffer>,
        control: Option<Control>,
        dcc_interface: Option<DccInterface>,
        tech_specs: Option<TechnicalSpecifications>,
    },
    FreightCar {
        id: RollingStockId,
        type_name: String,
        road_number: Option<String>,
        railway: RollingStockRailway,
        epoch: Epoch,
        freight_car_type: Option<FreightCarType>,
        livery: Option<String>,
        length_over_buffer: Option<LengthOverBuffer>,
        tech_specs: Option<TechnicalSpecifications>,
    },
    Locomotive {
        id: RollingStockId,
        class_name: String,
        road_number: String,
        series: Option<String>,
        railway: RollingStockRailway,
        epoch: Epoch,
        locomotive_type: LocomotiveType,
        depot: Option<String>,
        livery: Option<String>,
        is_dummy: bool,
        length_over_buffer: Option<LengthOverBuffer>,
        control: Option<Control>,
        dcc_interface: Option<DccInterface>,
        tech_specs: Option<TechnicalSpecifications>,
    },
    PassengerCar {
        id: RollingStockId,
        type_name: String,
        road_number: Option<String>,
        railway: RollingStockRailway,
        epoch: Epoch,
        passenger_car_type: Option<PassengerCarType>,
        service_level: Option<ServiceLevel>,
        livery: Option<String>,
        length_over_buffer: Option<LengthOverBuffer>,
        tech_specs: Option<TechnicalSpecifications>,
    },
    Railcar {
        id: RollingStockId,
        type_name: String,
        road_number: Option<String>,
        railway: RollingStockRailway,
        epoch: Epoch,
        railcar_type: RailcarType,
        depot: Option<String>,
        livery: Option<String>,
        is_dummy: bool,
        length_over_buffer: Option<LengthOverBuffer>,
        control: Option<Control>,
        dcc_interface: Option<DccInterface>,
        tech_specs: Option<TechnicalSpecifications>,
    },
}

impl RollingStock {
    /// Creates a new electric multiple unit rolling stock
    pub fn new_electric_multiple_unit(
        id: RollingStockId,
        type_name: &str,
        road_number: Option<&str>,
        railway: RollingStockRailway,
        epoch: Epoch,
        electric_multiple_unit_type: ElectricMultipleUnitType,
        depot: Option<&str>,
        livery: Option<&str>,
        is_dummy: bool,
        length_over_buffer: Option<LengthOverBuffer>,
        control: Option<Control>,
        dcc_interface: Option<DccInterface>,
        tech_specs: Option<TechnicalSpecifications>,
    ) -> Self {
        RollingStock::ElectricMultipleUnit {
            id,
            type_name: String::from(type_name),
            road_number: road_number.map(str::to_string),
            railway,
            epoch,
            electric_multiple_unit_type,
            depot: depot.map(str::to_string),
            livery: livery.map(str::to_string),
            is_dummy,
            length_over_buffer,
            control,
            dcc_interface,
            tech_specs,
        }
    }

    /// Creates a new locomotive rolling stock
    pub fn new_locomotive(
        id: RollingStockId,
        class_name: &str,
        road_number: &str,
        series: Option<&str>,
        railway: RollingStockRailway,
        epoch: Epoch,
        locomotive_type: LocomotiveType,
        depot: Option<&str>,
        livery: Option<&str>,
        is_dummy: bool,
        length_over_buffer: Option<LengthOverBuffer>,
        control: Option<Control>,
        dcc_interface: Option<DccInterface>,
        tech_specs: Option<TechnicalSpecifications>,
    ) -> Self {
        RollingStock::Locomotive {
            id,
            class_name: String::from(class_name),
            road_number: String::from(road_number),
            series: series.map(str::to_string),
            railway,
            epoch,
            locomotive_type,
            depot: depot.map(str::to_string),
            livery: livery.map(str::to_string),
            is_dummy,
            length_over_buffer,
            control,
            dcc_interface,
            tech_specs,
        }
    }

    /// Creates a new freight car rolling stock
    pub fn new_freight_car(
        id: RollingStockId,
        type_name: &str,
        road_number: Option<&str>,
        railway: RollingStockRailway,
        epoch: Epoch,
        freight_car_type: Option<FreightCarType>,
        livery: Option<&str>,
        length_over_buffer: Option<LengthOverBuffer>,
        tech_specs: Option<TechnicalSpecifications>,
    ) -> Self {
        RollingStock::FreightCar {
            id,
            type_name: String::from(type_name),
            road_number: road_number.map(str::to_string),
            railway,
            epoch,
            freight_car_type,
            livery: livery.map(str::to_string),
            length_over_buffer,
            tech_specs,
        }
    }

    /// Creates a new passenger car rolling stock
    pub fn new_passenger_car(
        id: RollingStockId,
        type_name: &str,
        road_number: Option<&str>,
        railway: RollingStockRailway,
        epoch: Epoch,
        passenger_car_type: Option<PassengerCarType>,
        service_level: Option<ServiceLevel>,
        livery: Option<&str>,
        length_over_buffer: Option<LengthOverBuffer>,
        tech_specs: Option<TechnicalSpecifications>,
    ) -> Self {
        RollingStock::PassengerCar {
            id,
            type_name: String::from(type_name),
            road_number: road_number.map(str::to_string),
            railway,
            epoch,
            passenger_car_type,
            service_level,
            livery: livery.map(str::to_string),
            length_over_buffer,
            tech_specs,
        }
    }

    /// Creates a new railcar rolling stock
    pub fn new_railcar(
        id: RollingStockId,
        type_name: &str,
        road_number: Option<&str>,
        railway: RollingStockRailway,
        epoch: Epoch,
        railcar_type: RailcarType,
        depot: Option<&str>,
        livery: Option<&str>,
        is_dummy: bool,
        length_over_buffer: Option<LengthOverBuffer>,
        control: Option<Control>,
        dcc_interface: Option<DccInterface>,
        tech_specs: Option<TechnicalSpecifications>,
    ) -> Self {
        RollingStock::Railcar {
            id,
            type_name: String::from(type_name),
            road_number: road_number.map(str::to_string),
            railway,
            epoch,
            railcar_type,
            depot: depot.map(str::to_string),
            livery: livery.map(str::to_string),
            is_dummy,
            length_over_buffer,
            control,
            dcc_interface,
            tech_specs,
        }
    }

    /// The category for this rolling stock
    pub fn category(&self) -> Category {
        match self {
            RollingStock::ElectricMultipleUnit { .. } => Category::ElectricMultipleUnits,
            RollingStock::Locomotive { .. } => Category::Locomotives,
            RollingStock::FreightCar { .. } => Category::FreightCars,
            RollingStock::PassengerCar { .. } => Category::PassengerCars,
            RollingStock::Railcar { .. } => Category::Railcars,
        }
    }

    /// The unique identifier for this rolling stock
    pub fn id(&self) -> RollingStockId {
        match self {
            RollingStock::ElectricMultipleUnit { id, .. } => *id,
            RollingStock::Locomotive { id, .. } => *id,
            RollingStock::FreightCar { id, .. } => *id,
            RollingStock::PassengerCar { id, .. } => *id,
            RollingStock::Railcar { id, .. } => *id,
        }
    }

    /// Return the epoch for this rolling stock
    pub fn epoch(&self) -> &Epoch {
        match self {
            RollingStock::ElectricMultipleUnit { epoch, .. } => epoch,
            RollingStock::Locomotive { epoch, .. } => epoch,
            RollingStock::FreightCar { epoch, .. } => epoch,
            RollingStock::PassengerCar { epoch, .. } => epoch,
            RollingStock::Railcar { epoch, .. } => epoch,
        }
    }

    /// Return the livery for this rolling stock
    pub fn livery(&self) -> Option<&str> {
        match self {
            RollingStock::ElectricMultipleUnit { livery, .. } => livery.as_deref(),
            RollingStock::Locomotive { livery, .. } => livery.as_deref(),
            RollingStock::FreightCar { livery, .. } => livery.as_deref(),
            RollingStock::PassengerCar { livery, .. } => livery.as_deref(),
            RollingStock::Railcar { livery, .. } => livery.as_deref(),
        }
    }

    /// Return the overall length for this rolling stock
    pub fn length_over_buffer(&self) -> Option<&LengthOverBuffer> {
        match self {
            RollingStock::ElectricMultipleUnit { length_over_buffer, .. } => length_over_buffer.as_ref(),
            RollingStock::Locomotive { length_over_buffer, .. } => length_over_buffer.as_ref(),
            RollingStock::FreightCar { length_over_buffer, .. } => length_over_buffer.as_ref(),
            RollingStock::PassengerCar { length_over_buffer, .. } => length_over_buffer.as_ref(),
            RollingStock::Railcar { length_over_buffer, .. } => length_over_buffer.as_ref(),
        }
    }

    pub fn railway(&self) -> &RollingStockRailway {
        match self {
            RollingStock::ElectricMultipleUnit { railway, .. } => railway,
            RollingStock::Locomotive { railway, .. } => railway,
            RollingStock::FreightCar { railway, .. } => railway,
            RollingStock::PassengerCar { railway, .. } => railway,
            RollingStock::Railcar { railway, .. } => railway,
        }
    }

    pub fn road_number(&self) -> Option<&str> {
        match self {
            RollingStock::ElectricMultipleUnit { road_number, .. } => road_number.as_deref(),
            RollingStock::Locomotive { road_number, .. } => Some(road_number),
            RollingStock::FreightCar { road_number, .. } => road_number.as_deref(),
            RollingStock::PassengerCar { road_number, .. } => road_number.as_deref(),
            RollingStock::Railcar { road_number, .. } => road_number.as_deref(),
        }
    }

    /// Returns the technical specification for this rolling stock
    pub fn technical_specifications(&self) -> Option<&TechnicalSpecifications> {
        match self {
            RollingStock::ElectricMultipleUnit { tech_specs, .. } => tech_specs.as_ref(),
            RollingStock::Locomotive { tech_specs, .. } => tech_specs.as_ref(),
            RollingStock::FreightCar { tech_specs, .. } => tech_specs.as_ref(),
            RollingStock::PassengerCar { tech_specs, .. } => tech_specs.as_ref(),
            RollingStock::Railcar { tech_specs, .. } => tech_specs.as_ref(),
        }
    }

    pub fn control(&self) -> Option<Control> {
        match self {
            RollingStock::ElectricMultipleUnit {
                control: Some(control), ..
            } => Some(*control),
            RollingStock::Locomotive {
                control: Some(control), ..
            } => Some(*control),
            RollingStock::Railcar {
                control: Some(control), ..
            } => Some(*control),
            _ => None,
        }
    }

    pub fn dcc_interface(&self) -> Option<DccInterface> {
        match self {
            RollingStock::ElectricMultipleUnit {
                dcc_interface: Some(dcc_interface),
                ..
            } => Some(*dcc_interface),
            RollingStock::Locomotive {
                dcc_interface: Some(dcc_interface),
                ..
            } => Some(*dcc_interface),
            RollingStock::Railcar {
                dcc_interface: Some(dcc_interface),
                ..
            } => Some(*dcc_interface),
            _ => None,
        }
    }

    pub fn with_decoder(&self) -> bool {
        match self {
            RollingStock::ElectricMultipleUnit {
                control: Some(control), ..
            } => control.with_decoder(),
            RollingStock::Locomotive {
                control: Some(control), ..
            } => control.with_decoder(),
            RollingStock::Railcar {
                control: Some(control), ..
            } => control.with_decoder(),
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub struct RollingStockRailway {
    railway_id: RailwayId,
    display: String,
}

impl RollingStockRailway {
    /// Creates a new railway with the display text.
    pub fn new(railway_id: RailwayId, display: &str) -> Self {
        RollingStockRailway {
            railway_id,
            display: display.to_owned(),
        }
    }

    /// Returns this railway unique identifier
    pub fn id(&self) -> &RailwayId {
        &self.railway_id
    }

    /// Returns this railway display text
    pub fn display_text(&self) -> &str {
        &self.display
    }
}

impl Display for RollingStockRailway {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.display)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod locomotives {
        use super::*;
        use crate::catalog_items::rolling_stock_id::RollingStockId;
        use crate::catalog_items::technical_specifications::{Coupling, Radius, Socket};
        use common::length::Length;
        use pretty_assertions::assert_eq;
        use rust_decimal_macros::dec;

        #[test]
        fn it_should_create_new_locomotives() {
            let id = RollingStockId::new();
            let length = LengthOverBuffer::from_millimeters(Length::Millimeters(dec!(210)));
            let fs = RollingStockRailway::new(RailwayId::new("fs"), "FS");

            let tech_specs = technical_specification();

            let locomotive = RollingStock::new_locomotive(
                id,
                "E.656",
                "E.656 077",
                Some("I serie"),
                fs.clone(),
                Epoch::IV,
                LocomotiveType::ElectricLocomotive,
                Some("Milano Centrale"),
                Some("blu/grigio"),
                false,
                Some(length),
                Some(Control::DccReady),
                Some(DccInterface::Nem652),
                Some(tech_specs.clone()),
            );

            assert_eq!(id, locomotive.id());
            assert_eq!(Category::Locomotives, locomotive.category());
            assert_eq!(&Epoch::IV, locomotive.epoch());
            assert_eq!(Some("blu/grigio"), locomotive.livery());
            assert_eq!(Some(&length), locomotive.length_over_buffer());
            assert_eq!(&fs, locomotive.railway());
            assert_eq!(Some("E.656 077"), locomotive.road_number());
            assert_eq!(Some(DccInterface::Nem652), locomotive.dcc_interface());
            assert_eq!(Some(Control::DccReady), locomotive.control());
            assert_eq!(Some(&tech_specs), locomotive.technical_specifications());
        }

        #[test]
        fn it_should_create_new_electric_multiple_units() {
            let id = RollingStockId::new();
            let length = LengthOverBuffer::from_millimeters(Length::Millimeters(dec!(303)));
            let fs = RollingStockRailway::new(RailwayId::new("fs"), "FS");

            let tech_specs = technical_specification();

            let power_car = RollingStock::new_electric_multiple_unit(
                id,
                "ALe 801",
                Some("ALe 801 003"),
                fs.clone(),
                Epoch::IV,
                ElectricMultipleUnitType::PowerCar,
                Some("Milano Centrale"),
                Some("livrea originale giallo/arancio"),
                false,
                Some(length),
                Some(Control::DccReady),
                Some(DccInterface::Nem652),
                Some(tech_specs.clone()),
            );

            assert_eq!(id, power_car.id());
            assert_eq!(Category::ElectricMultipleUnits, power_car.category());
            assert_eq!(&Epoch::IV, power_car.epoch());
            assert_eq!(Some("livrea originale giallo/arancio"), power_car.livery());
            assert_eq!(Some(&length), power_car.length_over_buffer());
            assert_eq!(&fs, power_car.railway());
            assert_eq!(Some("ALe 801 003"), power_car.road_number());
            assert_eq!(Some(DccInterface::Nem652), power_car.dcc_interface());
            assert_eq!(Some(Control::DccReady), power_car.control());
            assert_eq!(Some(&tech_specs), power_car.technical_specifications());
        }

        #[test]
        fn it_should_create_new_passenger_cars() {
            let id = RollingStockId::new();
            let length = LengthOverBuffer::from_millimeters(Length::Millimeters(dec!(303)));
            let fs = RollingStockRailway::new(RailwayId::new("fs"), "FS");

            let tech_specs = technical_specification();

            let passenger_car = RollingStock::new_passenger_car(
                id,
                "UIC-Z1",
                Some("61 83 19-90 105-3 A"),
                fs.clone(),
                Epoch::V,
                Some(PassengerCarType::CompartmentCoach),
                Some(ServiceLevel::FirstClass),
                Some("XMPR"),
                Some(length),
                Some(tech_specs.clone()),
            );

            assert_eq!(id, passenger_car.id());
            assert_eq!(Category::PassengerCars, passenger_car.category());
            assert_eq!(&Epoch::V, passenger_car.epoch());
            assert_eq!(Some("XMPR"), passenger_car.livery());
            assert_eq!(Some(&length), passenger_car.length_over_buffer());
            assert_eq!(&fs, passenger_car.railway());
            assert_eq!(Some("61 83 19-90 105-3 A"), passenger_car.road_number());
            assert_eq!(None, passenger_car.dcc_interface());
            assert_eq!(None, passenger_car.control());
            assert_eq!(Some(&tech_specs), passenger_car.technical_specifications());
        }

        #[test]
        fn it_should_create_new_railcars() {
            let id = RollingStockId::new();
            let length = LengthOverBuffer::from_millimeters(Length::Millimeters(dec!(303)));
            let fs = RollingStockRailway::new(RailwayId::new("fs"), "FS");

            let tech_specs = technical_specification();

            let power_car = RollingStock::new_railcar(
                id,
                "ALn 668",
                Some("ALn 668 1449"),
                fs.clone(),
                Epoch::IIIb,
                RailcarType::PowerCar,
                Some("Milano Centrale"),
                Some("verde lichene/giallo coloniale"),
                false,
                Some(length),
                Some(Control::DccReady),
                Some(DccInterface::Nem652),
                Some(tech_specs.clone()),
            );

            assert_eq!(id, power_car.id());
            assert_eq!(Category::Railcars, power_car.category());
            assert_eq!(&Epoch::IIIb, power_car.epoch());
            assert_eq!(Some("verde lichene/giallo coloniale"), power_car.livery());
            assert_eq!(Some(&length), power_car.length_over_buffer());
            assert_eq!(&fs, power_car.railway());
            assert_eq!(Some("ALn 668 1449"), power_car.road_number());
            assert_eq!(Some(DccInterface::Nem652), power_car.dcc_interface());
            assert_eq!(Some(Control::DccReady), power_car.control());
            assert_eq!(Some(&tech_specs), power_car.technical_specifications());
        }

        #[test]
        fn it_should_create_new_freight_cars() {
            let id = RollingStockId::new();
            let length = LengthOverBuffer::from_millimeters(Length::Millimeters(dec!(303)));
            let fs = RollingStockRailway::new(RailwayId::new("fs"), "FS");

            let tech_specs = technical_specification();

            let epoch = Epoch::Multiple(Box::new(Epoch::IV), Box::new(Epoch::V));

            let freight_car = RollingStock::new_freight_car(
                id,
                "Fals",
                Some("31 83 665 0 150-6"),
                fs.clone(),
                epoch.clone(),
                Some(FreightCarType::Gondola),
                Some("castano"),
                Some(length),
                Some(tech_specs.clone()),
            );

            assert_eq!(id, freight_car.id());
            assert_eq!(Category::FreightCars, freight_car.category());
            assert_eq!(&epoch, freight_car.epoch());
            assert_eq!(Some("castano"), freight_car.livery());
            assert_eq!(Some(&length), freight_car.length_over_buffer());
            assert_eq!(&fs, freight_car.railway());
            assert_eq!(Some("31 83 665 0 150-6"), freight_car.road_number());
            assert_eq!(None, freight_car.dcc_interface());
            assert_eq!(None, freight_car.control());
            assert_eq!(Some(&tech_specs), freight_car.technical_specifications());
        }

        fn technical_specification() -> TechnicalSpecifications {
            let radius = Radius::of_millimeters(dec!(360.0)).unwrap();
            let coupling = Coupling::with_close_couplers(Socket::Nem362);
            TechnicalSpecifications::builder()
                .with_coupling(coupling)
                .with_minimum_radius(radius)
                .build()
        }
    }
}
