use crate::catalog_items::category::{
    Category, ElectricMultipleUnitType, FreightCarType, LocomotiveType, PassengerCarType, RailcarType,
};
use crate::catalog_items::control::{Control, DccInterface};
use crate::catalog_items::epoch::Epoch;
use crate::catalog_items::rolling_stock_id::RollingStockId;
use crate::catalog_items::service_level::ServiceLevel;
use crate::catalog_items::tech_specs::TechSpecs;
use crate::railways::railway_id::RailwayId;
use common::length::Length;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RollingStock {
    ElectricMultipleUnit {
        id: RollingStockId,
        type_name: String,
        road_number: Option<String>,
        railway: Railway,
        epoch: Epoch,
        category: ElectricMultipleUnitType,
        depot: Option<String>,
        livery: Option<String>,
        is_dummy: bool,
        length_over_buffer: Option<Length>,
        control: Option<Control>,
        dcc_interface: Option<DccInterface>,
        tech_specs: Option<TechSpecs>,
    },
    Locomotive {
        id: RollingStockId,
        class_name: String,
        road_number: String,
        series: Option<String>,
        railway: Railway,
        epoch: Epoch,
        category: LocomotiveType,
        depot: Option<String>,
        livery: Option<String>,
        is_dummy: bool,
        length_over_buffer: Option<Length>,
        control: Option<Control>,
        dcc_interface: Option<DccInterface>,
        tech_specs: Option<TechSpecs>,
    },
    FreightCar {
        id: RollingStockId,
        type_name: String,
        road_number: Option<String>,
        railway: Railway,
        epoch: Epoch,
        category: Option<FreightCarType>,
        livery: Option<String>,
        length_over_buffer: Option<Length>,
        tech_specs: Option<TechSpecs>,
    },
    PassengerCar {
        id: RollingStockId,
        type_name: String,
        road_number: Option<String>,
        railway: Railway,
        epoch: Epoch,
        category: Option<PassengerCarType>,
        service_level: Option<ServiceLevel>,
        livery: Option<String>,
        length_over_buffer: Option<Length>,
        tech_specs: Option<TechSpecs>,
    },
    Railcar {
        id: RollingStockId,
        type_name: String,
        road_number: Option<String>,
        railway: Railway,
        epoch: Epoch,
        category: Option<RailcarType>,
        depot: Option<String>,
        livery: Option<String>,
        is_dummy: bool,
        length_over_buffer: Option<Length>,
        control: Option<Control>,
        dcc_interface: Option<DccInterface>,
        tech_specs: Option<TechSpecs>,
    },
}

impl RollingStock {
    /// Creates a new electric multiple unit rolling stock
    pub fn new_electric_multiple_unit(
        id: RollingStockId,
        type_name: &str,
        road_number: Option<&str>,
        railway: Railway,
        epoch: Epoch,
        category: ElectricMultipleUnitType,
        depot: Option<&str>,
        livery: Option<&str>,
        is_dummy: bool,
        length_over_buffer: Option<Length>,
        control: Option<Control>,
        dcc_interface: Option<DccInterface>,
        tech_specs: Option<TechSpecs>,
    ) -> Self {
        RollingStock::ElectricMultipleUnit {
            id,
            type_name: String::from(type_name),
            road_number: road_number.map(str::to_string),
            railway,
            epoch,
            category,
            depot: depot.map(str::to_string),
            livery: livery.map(str::to_string),
            is_dummy,
            length_over_buffer,
            control,
            dcc_interface,
            tech_specs,
        }
    }

    pub fn new_locomotive(
        id: RollingStockId,
        class_name: &str,
        road_number: &str,
        series: Option<&str>,
        railway: Railway,
        epoch: Epoch,
        category: LocomotiveType,
        depot: Option<&str>,
        livery: Option<&str>,
        is_dummy: bool,
        length_over_buffer: Option<Length>,
        control: Option<Control>,
        dcc_interface: Option<DccInterface>,
        tech_specs: Option<TechSpecs>,
    ) -> Self {
        RollingStock::Locomotive {
            id,
            class_name: String::from(class_name),
            road_number: String::from(road_number),
            series: series.map(str::to_string),
            railway,
            epoch,
            category,
            depot: depot.map(str::to_string),
            livery: livery.map(str::to_string),
            is_dummy,
            length_over_buffer,
            control,
            dcc_interface,
            tech_specs,
        }
    }

    pub fn new_freight_car(
        id: RollingStockId,
        type_name: &str,
        road_number: Option<&str>,
        railway: Railway,
        epoch: Epoch,
        category: Option<FreightCarType>,
        livery: Option<&str>,
        length_over_buffer: Option<Length>,
        tech_specs: Option<TechSpecs>,
    ) -> Self {
        RollingStock::FreightCar {
            id,
            type_name: String::from(type_name),
            road_number: road_number.map(str::to_string),
            railway,
            epoch,
            category,
            livery: livery.map(str::to_string),
            length_over_buffer,
            tech_specs,
        }
    }

    pub fn new_passenger_car(
        id: RollingStockId,
        type_name: &str,
        road_number: Option<&str>,
        railway: Railway,
        epoch: Epoch,
        category: Option<PassengerCarType>,
        service_level: Option<ServiceLevel>,
        livery: Option<&str>,
        length_over_buffer: Option<Length>,
        tech_specs: Option<TechSpecs>,
    ) -> Self {
        RollingStock::PassengerCar {
            id,
            type_name: String::from(type_name),
            road_number: road_number.map(str::to_string),
            railway,
            epoch,
            category,
            service_level,
            livery: livery.map(str::to_string),
            length_over_buffer,
            tech_specs,
        }
    }

    pub fn new_railcar(
        id: RollingStockId,
        type_name: &str,
        road_number: Option<&str>,
        railway: Railway,
        epoch: Epoch,
        category: Option<RailcarType>,
        depot: Option<&str>,
        livery: Option<&str>,
        is_dummy: bool,
        length_over_buffer: Option<Length>,
        control: Option<Control>,
        dcc_interface: Option<DccInterface>,
        tech_specs: Option<TechSpecs>,
    ) -> Self {
        RollingStock::Railcar {
            id,
            type_name: String::from(type_name),
            road_number: road_number.map(str::to_string),
            railway,
            epoch,
            category,
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
    pub fn length_over_buffer(&self) -> Option<&Length> {
        match self {
            RollingStock::ElectricMultipleUnit { length_over_buffer, .. } => length_over_buffer.as_ref(),
            RollingStock::Locomotive { length_over_buffer, .. } => length_over_buffer.as_ref(),
            RollingStock::FreightCar { length_over_buffer, .. } => length_over_buffer.as_ref(),
            RollingStock::PassengerCar { length_over_buffer, .. } => length_over_buffer.as_ref(),
            RollingStock::Railcar { length_over_buffer, .. } => length_over_buffer.as_ref(),
        }
    }

    pub fn railway(&self) -> &Railway {
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

    pub fn tech_specs(&self) -> Option<&TechSpecs> {
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Railway {
    railway_id: RailwayId,
    name: String,
}

impl Railway {
    /// Creates a new railway with the given name.
    pub fn new(railway_id: RailwayId, name: &str) -> Self {
        Railway {
            railway_id,
            name: name.to_owned(),
        }
    }

    /// Returns this railway unique identifier
    pub fn id(&self) -> &RailwayId {
        &self.railway_id
    }

    /// Returns this railway name
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Display for Railway {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.name)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod locomotives {
        use super::*;
        use crate::catalog_items::rolling_stock_id::RollingStockId;
        use crate::catalog_items::tech_specs::{Coupling, Radius};
        use common::measure_units::MeasureUnit;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_new_locomotives() {
            let id = RollingStockId::new();
            let length = Length::new(210_f32, MeasureUnit::Millimeters).unwrap();
            let fs = Railway::new(RailwayId::new("fs"), "FS");

            let tech_specs = TechSpecs::builder()
                .with_coupling(Coupling::Nem362)
                .with_minimum_radius(Radius::new(360_f32).unwrap())
                .build();

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
            assert_eq!(Some(&tech_specs), locomotive.tech_specs());
        }
    }
}
