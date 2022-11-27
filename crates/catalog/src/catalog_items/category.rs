use strum_macros;
use strum_macros::{Display, EnumString};

/// The enumeration of the model categories.
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
#[strum(ascii_case_insensitive)]
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
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
#[strum(ascii_case_insensitive)]
pub enum FreightCarType {
    AutoTransportCars,
    BrakeWagon,
    ContainerCars,
    CoveredFreightCars,
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
    DeepWellFlatCars,
}

/// The different kinds of locomotives
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
#[strum(ascii_case_insensitive)]
pub enum LocomotiveType {
    /// The steam locomotives category
    SteamLocomotive,

    /// The diesel locomotives category
    DieselLocomotive,

    /// The electric locomotives category
    ElectricLocomotive,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
#[strum(ascii_case_insensitive)]
pub enum PassengerCarType {
    /// An "open coach" has a central aisle; the car's interior is often filled with row upon row of
    /// seats as in a passenger airliner.
    OpenCoach,

    /// "closed" coaches or "compartment" cars have a side corridor to connect individual compartments
    /// along the body of the train, each with two rows of seats facing each other.
    CompartmentCoach,

    /// A dining car (or diner) is used to serve meals to the passengers.
    DiningCar,

    /// Lounge cars carry a bar and public seating.
    Lounge,

    /// The observation car almost always operated as the last car in a passenger train, in US
    /// practice. Its interior could include features of a coach, lounge, diner, or sleeper. The
    /// main spotting feature was at the tail end of the car.
    Observation,

    ///Often called "sleepers" or "Pullman cars", these cars provide sleeping arrangements for
    ///passengers travelling at night. Early models were divided into sections, where coach
    /// seating converted at night into semi-private berths.
    SleepingCar,

    /// The baggage car is a car that was normally placed between the train's motive power and the
    /// remainder of the passenger train. The car's interior is normally wide open and is used to
    /// carry passengers' checked baggage.
    BaggageCar,

    DoubleDecker,

    CombineCar,

    DrivingTrailer,

    RailwayPostOffice,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
#[strum(ascii_case_insensitive)]
pub enum ElectricMultipleUnitType {
    PowerCar,
    TrailerCar,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
#[strum(ascii_case_insensitive)]
pub enum RailcarType {
    PowerCar,
    TrailerCar,
}
