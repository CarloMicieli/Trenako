use sqlx::Type;
use strum_macros;
use strum_macros::{Display, EnumString};

/// The enumeration of the model categories.
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString, Display, Serialize, Deserialize, Type)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[sqlx(type_name = "catalog_item_category", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Category {
    /// The steam locomotives category
    Locomotives,

    /// The train sets category
    TrainSets,

    /// The train sets category
    StarterSets,

    /// The freight cars category
    FreightCars,

    /// The passenger cars category
    PassengerCars,

    /// The electric multiple units category
    ElectricMultipleUnits,

    /// The railcars category
    Railcars,
}

/// The different kind of freight cars
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString, Display, Serialize, Deserialize, Type)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[sqlx(type_name = "freight_car_type", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FreightCarType {
    AutoTransportCars,
    BrakeWagon,
    ContainerCars,
    CoveredFreightCars,
    DeepWellFlatCars,
    DumpCars,
    Gondola,
    HeavyGoodsWagons,
    HingedCoverWagons,
    HopperWagon,
    RefrigeratorCars,
    SiloContainerCars,
    SlideTarpaulinWagon,
    SlidingWallBoxcars,
    SpecialTransport,
    StakeWagons,
    SwingRoofWagon,
    TankCars,
    TelescopeHoodWagons,
}

/// The different kinds of locomotives
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString, Display, Serialize, Deserialize, Type)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[sqlx(type_name = "locomotive_type", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LocomotiveType {
    /// The steam locomotives category
    SteamLocomotive,

    /// The diesel locomotives category
    DieselLocomotive,

    /// The electric locomotives category
    ElectricLocomotive,
}

/// The types for passenger car rolling stocks
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString, Display, Serialize, Deserialize, Type)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[sqlx(type_name = "passenger_car_type", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PassengerCarType {
    /// The baggage car is a car that was normally placed between the train's motive power and the
    /// remainder of the passenger train. The car's interior is normally wide open and is used to
    /// carry passengers' checked baggage.
    BaggageCar,

    /// A combine car is a type of railroad car which combines sections for both passengers and freight
    CombineCar,

    /// "closed" coaches or "compartment" cars have a side corridor to connect individual compartments
    /// along the body of the train, each with two rows of seats facing each other.
    CompartmentCoach,

    /// A dining car (or diner) is used to serve meals to the passengers.
    DiningCar,

    /// A double-decker coach, or bilevel car, is a type of rail car that has two levels of passenger
    /// accommodation, as opposed to one, increasing passenger capacity
    DoubleDecker,

    /// A driving trailer is a purpose-built control car railway vehicle that allows the driver
    /// to operate with a locomotive in push-pull formation from the opposite end of a train
    DrivingTrailer,

    /// Lounge cars carry a bar and public seating.
    Lounge,

    /// The observation car almost always operated as the last car in a passenger train, in US
    /// practice. Its interior could include features of a coach, lounge, diner, or sleeper. The
    /// main spotting feature was at the tail end of the car.
    Observation,

    /// An "open coach" has a central aisle; the car's interior is often filled with row upon row of
    /// seats as in a passenger airliner.
    OpenCoach,

    /// A railway post office is a railroad car that was normally operated in passenger service
    /// as a means to sort mail en route, in order to speed delivery.
    RailwayPostOffice,

    ///Often called "sleepers" or "Pullman cars", these cars provide sleeping arrangements for
    ///passengers travelling at night. Early models were divided into sections, where coach
    /// seating converted at night into semi-private berths.
    SleepingCar,
}

/// The cars that form a complete EMU set can usually be separated by function into four types:
/// power car, motor car, driving car, and trailer car.
///
/// Each car can have more than one function, such as a motor-driving car or power-driving car.
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString, Display, Serialize, Deserialize, Type)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[sqlx(type_name = "electric_multiple_unit_type", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ElectricMultipleUnitType {
    /// Driving cars are similar to a cab car, containing a driver's cab for controlling the train.
    /// An EMU will usually have two driving cars at its outer ends.
    DrivingCar,

    /// High-speed rail is a type of rail system that runs significantly faster than traditional
    /// rail, using an integrated system of specialised rolling stock and dedicated tracks.
    HighSpeedTrain,

    /// Motor cars carry the traction motors to move the train, and are often combined with the
    /// power car to avoid high-voltage inter-car connections.
    MotorCar,

    /// A power car carries the necessary equipment to draw power from the electrified
    /// infrastructure, such as pickup shoes for third rail systems and pantographs for
    /// overhead systems, and transformers.
    PowerCar,

    /// Trailer cars are any cars (sometimes semi-permanently coupled) that carry little or no
    /// traction or power related equipment, and are similar to passenger cars in a
    /// locomotive-hauled train.
    TrailerCar,

    /// A trainset is working as whole unit
    TrainSet,
}

/// The types for railcar rolling stocks
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString, Display, Serialize, Deserialize, Type)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[sqlx(type_name = "railcar_type", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RailcarType {
    PowerCar,
    TrailerCar,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod categories {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use strum::ParseError;

        #[rstest]
        #[case("ELECTRIC_MULTIPLE_UNITS", Ok(Category::ElectricMultipleUnits))]
        #[case("FREIGHT_CARS", Ok(Category::FreightCars))]
        #[case("LOCOMOTIVES", Ok(Category::Locomotives))]
        #[case("PASSENGER_CARS", Ok(Category::PassengerCars))]
        #[case("RAILCARS", Ok(Category::Railcars))]
        #[case("STARTER_SETS", Ok(Category::StarterSets))]
        #[case("TRAIN_SETS", Ok(Category::TrainSets))]
        #[case("invalid", Err(ParseError::VariantNotFound))]
        fn it_should_parse_categories(#[case] input: &str, #[case] expected: Result<Category, ParseError>) {
            let category = input.parse::<Category>();
            assert_eq!(expected, category);
        }

        #[rstest]
        #[case(Category::ElectricMultipleUnits, "ELECTRIC_MULTIPLE_UNITS")]
        #[case(Category::FreightCars, "FREIGHT_CARS")]
        #[case(Category::Locomotives, "LOCOMOTIVES")]
        #[case(Category::PassengerCars, "PASSENGER_CARS")]
        #[case(Category::Railcars, "RAILCARS")]
        #[case(Category::StarterSets, "STARTER_SETS")]
        #[case(Category::TrainSets, "TRAIN_SETS")]
        fn it_should_display_categories(#[case] input: Category, #[case] expected: &str) {
            assert_eq!(expected.to_string(), input.to_string());
        }
    }

    mod electric_multiple_unit_types {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use strum::ParseError;

        #[rstest]
        #[case("DRIVING_CAR", Ok(ElectricMultipleUnitType::DrivingCar))]
        #[case("HIGH_SPEED_TRAIN", Ok(ElectricMultipleUnitType::HighSpeedTrain))]
        #[case("MOTOR_CAR", Ok(ElectricMultipleUnitType::MotorCar))]
        #[case("POWER_CAR", Ok(ElectricMultipleUnitType::PowerCar))]
        #[case("TRAILER_CAR", Ok(ElectricMultipleUnitType::TrailerCar))]
        #[case("TRAIN_SET", Ok(ElectricMultipleUnitType::TrainSet))]
        #[case("invalid", Err(ParseError::VariantNotFound))]
        fn it_should_parse_electric_multiple_unit_types(
            #[case] input: &str,
            #[case] expected: Result<ElectricMultipleUnitType, ParseError>,
        ) {
            let emu_type = input.parse::<ElectricMultipleUnitType>();
            assert_eq!(expected, emu_type);
        }

        #[rstest]
        #[case(ElectricMultipleUnitType::DrivingCar, "DRIVING_CAR")]
        #[case(ElectricMultipleUnitType::HighSpeedTrain, "HIGH_SPEED_TRAIN")]
        #[case(ElectricMultipleUnitType::MotorCar, "MOTOR_CAR")]
        #[case(ElectricMultipleUnitType::PowerCar, "POWER_CAR")]
        #[case(ElectricMultipleUnitType::TrailerCar, "TRAILER_CAR")]
        #[case(ElectricMultipleUnitType::TrainSet, "TRAIN_SET")]
        fn it_should_display_electric_multiple_unit_types(
            #[case] input: ElectricMultipleUnitType,
            #[case] expected: &str,
        ) {
            assert_eq!(expected.to_string(), input.to_string());
        }
    }

    mod freight_car_types {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use strum::ParseError;

        #[rstest]
        #[case("AUTO_TRANSPORT_CARS", Ok(FreightCarType::AutoTransportCars))]
        #[case("BRAKE_WAGON", Ok(FreightCarType::BrakeWagon))]
        #[case("CONTAINER_CARS", Ok(FreightCarType::ContainerCars))]
        #[case("COVERED_FREIGHT_CARS", Ok(FreightCarType::CoveredFreightCars))]
        #[case("DEEP_WELL_FLAT_CARS", Ok(FreightCarType::DeepWellFlatCars))]
        #[case("DUMP_CARS", Ok(FreightCarType::DumpCars))]
        #[case("GONDOLA", Ok(FreightCarType::Gondola))]
        #[case("HEAVY_GOODS_WAGONS", Ok(FreightCarType::HeavyGoodsWagons))]
        #[case("HINGED_COVER_WAGONS", Ok(FreightCarType::HingedCoverWagons))]
        #[case("HOPPER_WAGON", Ok(FreightCarType::HopperWagon))]
        #[case("REFRIGERATOR_CARS", Ok(FreightCarType::RefrigeratorCars))]
        #[case("SILO_CONTAINER_CARS", Ok(FreightCarType::SiloContainerCars))]
        #[case("SLIDE_TARPAULIN_WAGON", Ok(FreightCarType::SlideTarpaulinWagon))]
        #[case("SLIDING_WALL_BOXCARS", Ok(FreightCarType::SlidingWallBoxcars))]
        #[case("SPECIAL_TRANSPORT", Ok(FreightCarType::SpecialTransport))]
        #[case("STAKE_WAGONS", Ok(FreightCarType::StakeWagons))]
        #[case("SWING_ROOF_WAGON", Ok(FreightCarType::SwingRoofWagon))]
        #[case("TANK_CARS", Ok(FreightCarType::TankCars))]
        #[case("TELESCOPE_HOOD_WAGONS", Ok(FreightCarType::TelescopeHoodWagons))]
        #[case("invalid", Err(ParseError::VariantNotFound))]
        fn it_should_parse_freight_car_types(
            #[case] input: &str,
            #[case] expected: Result<FreightCarType, ParseError>,
        ) {
            let freight_car_type = input.parse::<FreightCarType>();
            assert_eq!(expected, freight_car_type);
        }

        #[rstest]
        #[case(FreightCarType::AutoTransportCars, "AUTO_TRANSPORT_CARS")]
        #[case(FreightCarType::BrakeWagon, "BRAKE_WAGON")]
        #[case(FreightCarType::ContainerCars, "CONTAINER_CARS")]
        #[case(FreightCarType::CoveredFreightCars, "COVERED_FREIGHT_CARS")]
        #[case(FreightCarType::DeepWellFlatCars, "DEEP_WELL_FLAT_CARS")]
        #[case(FreightCarType::DumpCars, "DUMP_CARS")]
        #[case(FreightCarType::Gondola, "GONDOLA")]
        #[case(FreightCarType::HeavyGoodsWagons, "HEAVY_GOODS_WAGONS")]
        #[case(FreightCarType::HingedCoverWagons, "HINGED_COVER_WAGONS")]
        #[case(FreightCarType::HopperWagon, "HOPPER_WAGON")]
        #[case(FreightCarType::RefrigeratorCars, "REFRIGERATOR_CARS")]
        #[case(FreightCarType::SiloContainerCars, "SILO_CONTAINER_CARS")]
        #[case(FreightCarType::SlideTarpaulinWagon, "SLIDE_TARPAULIN_WAGON")]
        #[case(FreightCarType::SlidingWallBoxcars, "SLIDING_WALL_BOXCARS")]
        #[case(FreightCarType::SpecialTransport, "SPECIAL_TRANSPORT")]
        #[case(FreightCarType::StakeWagons, "STAKE_WAGONS")]
        #[case(FreightCarType::SwingRoofWagon, "SWING_ROOF_WAGON")]
        #[case(FreightCarType::TankCars, "TANK_CARS")]
        #[case(FreightCarType::TelescopeHoodWagons, "TELESCOPE_HOOD_WAGONS")]
        fn it_should_display_freight_car_types(#[case] input: FreightCarType, #[case] expected: &str) {
            assert_eq!(expected.to_string(), input.to_string());
        }
    }

    mod locomotive_types {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use strum::ParseError;

        #[rstest]
        #[case("DIESEL_LOCOMOTIVE", Ok(LocomotiveType::DieselLocomotive))]
        #[case("ELECTRIC_LOCOMOTIVE", Ok(LocomotiveType::ElectricLocomotive))]
        #[case("STEAM_LOCOMOTIVE", Ok(LocomotiveType::SteamLocomotive))]
        #[case("invalid", Err(ParseError::VariantNotFound))]
        fn it_should_parse_locomotive_types(#[case] input: &str, #[case] expected: Result<LocomotiveType, ParseError>) {
            let locomotive_type = input.parse::<LocomotiveType>();
            assert_eq!(expected, locomotive_type);
        }

        #[rstest]
        #[case(LocomotiveType::DieselLocomotive, "DIESEL_LOCOMOTIVE")]
        #[case(LocomotiveType::ElectricLocomotive, "ELECTRIC_LOCOMOTIVE")]
        #[case(LocomotiveType::SteamLocomotive, "STEAM_LOCOMOTIVE")]
        fn it_should_display_locomotive_types(#[case] input: LocomotiveType, #[case] expected: &str) {
            assert_eq!(expected.to_string(), input.to_string());
        }
    }

    mod passenger_car_types {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use strum::ParseError;

        #[rstest]
        #[case("BAGGAGE_CAR", Ok(PassengerCarType::BaggageCar))]
        #[case("COMBINE_CAR", Ok(PassengerCarType::CombineCar))]
        #[case("COMPARTMENT_COACH", Ok(PassengerCarType::CompartmentCoach))]
        #[case("DINING_CAR", Ok(PassengerCarType::DiningCar))]
        #[case("DOUBLE_DECKER", Ok(PassengerCarType::DoubleDecker))]
        #[case("DRIVING_TRAILER", Ok(PassengerCarType::DrivingTrailer))]
        #[case("LOUNGE", Ok(PassengerCarType::Lounge))]
        #[case("OBSERVATION", Ok(PassengerCarType::Observation))]
        #[case("OPEN_COACH", Ok(PassengerCarType::OpenCoach))]
        #[case("RAILWAY_POST_OFFICE", Ok(PassengerCarType::RailwayPostOffice))]
        #[case("SLEEPING_CAR", Ok(PassengerCarType::SleepingCar))]
        #[case("invalid", Err(ParseError::VariantNotFound))]
        fn it_should_parse_passenger_car_types(
            #[case] input: &str,
            #[case] expected: Result<PassengerCarType, ParseError>,
        ) {
            let pc_type = input.parse::<PassengerCarType>();
            assert_eq!(expected, pc_type);
        }

        #[rstest]
        #[case(PassengerCarType::BaggageCar, "BAGGAGE_CAR")]
        #[case(PassengerCarType::CombineCar, "COMBINE_CAR")]
        #[case(PassengerCarType::CompartmentCoach, "COMPARTMENT_COACH")]
        #[case(PassengerCarType::DiningCar, "DINING_CAR")]
        #[case(PassengerCarType::DoubleDecker, "DOUBLE_DECKER")]
        #[case(PassengerCarType::DrivingTrailer, "DRIVING_TRAILER")]
        #[case(PassengerCarType::Lounge, "LOUNGE")]
        #[case(PassengerCarType::Observation, "OBSERVATION")]
        #[case(PassengerCarType::OpenCoach, "OPEN_COACH")]
        #[case(PassengerCarType::RailwayPostOffice, "RAILWAY_POST_OFFICE")]
        #[case(PassengerCarType::SleepingCar, "SLEEPING_CAR")]
        fn it_should_display_passenger_car_types(#[case] input: PassengerCarType, #[case] expected: &str) {
            assert_eq!(expected.to_string(), input.to_string());
        }
    }

    mod railcar_types {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use strum::ParseError;

        #[rstest]
        #[case("POWER_CAR", Ok(RailcarType::PowerCar))]
        #[case("TRAILER_CAR", Ok(RailcarType::TrailerCar))]
        #[case("invalid", Err(ParseError::VariantNotFound))]
        fn it_should_parse_railcar_types(#[case] input: &str, #[case] expected: Result<RailcarType, ParseError>) {
            let railcar_type = input.parse::<RailcarType>();
            assert_eq!(expected, railcar_type);
        }

        #[rstest]
        #[case(RailcarType::PowerCar, "POWER_CAR")]
        #[case(RailcarType::TrailerCar, "TRAILER_CAR")]
        fn it_should_display_railcar_types(#[case] input: RailcarType, #[case] expected: &str) {
            assert_eq!(expected.to_string(), input.to_string());
        }
    }
}
