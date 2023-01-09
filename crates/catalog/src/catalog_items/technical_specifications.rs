use common::length::Length;
use rust_decimal::Decimal;
use sqlx::Type;
use std::fmt;
use std::fmt::Formatter;
use strum_macros;
use strum_macros::{Display, EnumString};
use thiserror::Error;

/// It represents the coupling configuration for a rolling stock.
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Coupling {
    /// the rolling stock coupling socket
    pub socket: CouplingSocket,
    /// the rolling stock has a close coupling mechanism
    pub close_couplers: FeatureFlag,
    /// the rolling stock has a digital shunting couplers mechanism
    pub digital_shunting: FeatureFlag,
}

impl Coupling {
    /// Creates a new rolling stock coupling configuration
    pub fn new(socket: CouplingSocket, close_couplers: FeatureFlag, digital_shunting: FeatureFlag) -> Self {
        Coupling {
            socket,
            close_couplers,
            digital_shunting,
        }
    }

    /// Creates a new rolling stock close coupling configuration with a given coupling socket
    pub fn with_close_couplers(socket: CouplingSocket) -> Self {
        Coupling {
            socket,
            close_couplers: FeatureFlag::Yes,
            digital_shunting: FeatureFlag::No,
        }
    }

    /// the coupling socket if present
    pub fn socket(&self) -> CouplingSocket {
        self.socket
    }

    /// true if the coupling configuration include a mechanism to reduce the gaps between two
    /// rolling stocks; false otherwise
    pub fn close_couplers(&self) -> FeatureFlag {
        self.close_couplers
    }

    /// true if the coupling configuration implements digital control functionalities,
    /// false otherwise  
    pub fn digital_shunting(&self) -> FeatureFlag {
        self.digital_shunting
    }
}

/// The NEM coupling socket standards
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, EnumString, Display, Type)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[sqlx(type_name = "socket_type", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CouplingSocket {
    #[strum(serialize = "NONE")]
    None,

    /// Receptacle for Replaceable Coupling Heads in Scales TT and N
    #[strum(serialize = "NEM_355")]
    Nem355,

    /// Coupler Head for Scale N
    #[strum(serialize = "NEM_356")]
    Nem356,

    /// Coupler Head for Scale N
    #[strum(serialize = "NEM_357")]
    Nem357,

    /// Coupler Head for Scale TT
    #[strum(serialize = "NEM_359")]
    Nem359,

    /// Standard Coupling for Scale H0
    #[strum(serialize = "NEM_360")]
    Nem360,

    /// NEM shaft 362 with close coupling mechanism
    #[strum(serialize = "NEM_362")]
    Nem362,

    /// Coupler Head for Scale 0
    #[strum(serialize = "NEM_365")]
    Nem365,
}

impl Default for CouplingSocket {
    fn default() -> Self {
        CouplingSocket::None
    }
}

/// The technical specification data for a rolling stock model
#[derive(Debug, Eq, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct TechnicalSpecifications {
    /// the minimum drivable radius
    pub minimum_radius: Option<Radius>,
    /// the coupling
    pub coupling: Option<Coupling>,
    /// has a flywheel fitted
    pub flywheel_fitted: FeatureFlag,
    /// has metal body
    pub metal_body: FeatureFlag,
    /// has interior lighting
    pub interior_lights: FeatureFlag,
    /// has lights
    pub lights: FeatureFlag,
    /// has spring buffers
    pub spring_buffers: FeatureFlag,
}

impl TechnicalSpecifications {
    /// the minimum radius
    pub fn minimum_radius(&self) -> Option<Radius> {
        self.minimum_radius
    }

    /// the coupling shaft standard
    pub fn coupling(&self) -> Option<Coupling> {
        self.coupling
    }

    /// with flywheel fitted
    pub fn flywheel_fitted(&self) -> FeatureFlag {
        self.flywheel_fitted
    }

    /// with metal body
    pub fn metal_body(&self) -> FeatureFlag {
        self.metal_body
    }

    /// with interior lights
    pub fn interior_lights(&self) -> FeatureFlag {
        self.interior_lights
    }

    /// with headlights
    pub fn lights(&self) -> FeatureFlag {
        self.lights
    }

    /// with spring buffers
    pub fn spring_buffers(&self) -> FeatureFlag {
        self.spring_buffers
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TechnicalSpecificationsBuilder {
    minimum_radius: Option<Radius>,
    coupling: Option<Coupling>,
    flywheel_fitted: FeatureFlag,
    metal_body: FeatureFlag,
    interior_lights: FeatureFlag,
    lights: FeatureFlag,
    spring_buffers: FeatureFlag,
}

impl TechnicalSpecificationsBuilder {
    /// with the minimum radius
    pub fn with_minimum_radius(mut self, radius: Radius) -> Self {
        self.minimum_radius = Some(radius);
        self
    }

    /// with the coupling shaft
    pub fn with_coupling(mut self, coupling: Coupling) -> Self {
        self.coupling = Some(coupling);
        self
    }

    /// with flywheel fitted
    pub fn with_flywheel_fitted(mut self) -> Self {
        self.flywheel_fitted = FeatureFlag::Yes;
        self
    }

    /// with metal body
    pub fn with_metal_body(mut self) -> Self {
        self.metal_body = FeatureFlag::Yes;
        self
    }

    /// with interior lights
    pub fn with_interior_lights(mut self) -> Self {
        self.interior_lights = FeatureFlag::Yes;
        self
    }

    /// with headlights
    pub fn with_lights(mut self) -> Self {
        self.lights = FeatureFlag::Yes;
        self
    }

    /// with spring buffers
    pub fn with_spring_buffers(mut self) -> Self {
        self.spring_buffers = FeatureFlag::Yes;
        self
    }

    pub fn build(self) -> TechnicalSpecifications {
        TechnicalSpecifications {
            minimum_radius: self.minimum_radius,
            coupling: self.coupling,
            flywheel_fitted: self.flywheel_fitted,
            metal_body: self.metal_body,
            interior_lights: self.interior_lights,
            lights: self.lights,
            spring_buffers: self.spring_buffers,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize, EnumString, Display, Type)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[sqlx(type_name = "feature_flag", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FeatureFlag {
    Yes,
    No,
    NotApplicable,
}

impl Default for FeatureFlag {
    fn default() -> Self {
        FeatureFlag::NotApplicable
    }
}

/// The minimum drivable radius
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize, Type)]
#[sqlx(transparent)]
pub struct Radius(Length);

impl Radius {
    /// Returns a drivable radius expressed in millimeters
    pub fn from_millimeters(value: Decimal) -> Result<Self, RadiusError> {
        if value.is_sign_positive() {
            Ok(Radius(Length::Millimeters(value)))
        } else {
            Err(RadiusError::NegativeRadius)
        }
    }

    /// Returns the value for this radius
    pub fn value(&self) -> Length {
        self.0
    }
}

impl fmt::Display for Radius {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Eq, PartialEq, Error)]
pub enum RadiusError {
    #[error("radius cannot be negative")]
    NegativeRadius,
}

#[cfg(test)]
mod test {
    use super::*;

    mod sockets {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use strum::ParseError;

        #[rstest]
        #[case("NONE", Ok(CouplingSocket::None))]
        #[case("NEM_355", Ok(CouplingSocket::Nem355))]
        #[case("NEM_356", Ok(CouplingSocket::Nem356))]
        #[case("NEM_357", Ok(CouplingSocket::Nem357))]
        #[case("NEM_359", Ok(CouplingSocket::Nem359))]
        #[case("NEM_360", Ok(CouplingSocket::Nem360))]
        #[case("NEM_362", Ok(CouplingSocket::Nem362))]
        #[case("NEM_365", Ok(CouplingSocket::Nem365))]
        #[case("invalid", Err(ParseError::VariantNotFound))]
        fn it_should_parse_strings_as_couplings(
            #[case] input: &str,
            #[case] expected: Result<CouplingSocket, ParseError>,
        ) {
            let coupling = input.parse::<CouplingSocket>();
            assert_eq!(expected, coupling);
        }

        #[rstest]
        #[case(CouplingSocket::None, "NONE")]
        #[case(CouplingSocket::Nem355, "NEM_355")]
        #[case(CouplingSocket::Nem356, "NEM_356")]
        #[case(CouplingSocket::Nem357, "NEM_357")]
        #[case(CouplingSocket::Nem359, "NEM_359")]
        #[case(CouplingSocket::Nem360, "NEM_360")]
        #[case(CouplingSocket::Nem362, "NEM_362")]
        #[case(CouplingSocket::Nem365, "NEM_365")]
        fn it_should_display_couplings(#[case] input: CouplingSocket, #[case] expected: &str) {
            assert_eq!(expected, input.to_string());
        }
    }

    mod feature_flags {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use strum::ParseError;

        #[rstest]
        #[case("YES", Ok(FeatureFlag::Yes))]
        #[case("NO", Ok(FeatureFlag::No))]
        #[case("NOT_APPLICABLE", Ok(FeatureFlag::NotApplicable))]
        #[case("invalid", Err(ParseError::VariantNotFound))]
        fn it_should_parse_strings_as_feature_flags(
            #[case] input: &str,
            #[case] expected: Result<FeatureFlag, ParseError>,
        ) {
            let flag = input.parse::<FeatureFlag>();
            assert_eq!(expected, flag);
        }

        #[rstest]
        #[case(FeatureFlag::Yes, "YES")]
        #[case(FeatureFlag::No, "NO")]
        #[case(FeatureFlag::NotApplicable, "NOT_APPLICABLE")]
        fn it_should_display_feature_flags(#[case] input: FeatureFlag, #[case] expected: &str) {
            assert_eq!(expected, input.to_string());
        }
    }

    mod radius {
        use super::*;
        use common::measure_units::MeasureUnit;
        use pretty_assertions::assert_eq;
        use rust_decimal_macros::dec;

        #[test]
        fn it_should_create_a_new_radius_in_millimeters() {
            let radius = Radius::from_millimeters(dec!(360.0)).expect("unable to create the radius");
            assert_eq!(Length::new(dec!(360), MeasureUnit::Millimeters), radius.value());
        }

        #[test]
        fn it_should_fail_to_create_negative_radius() {
            let result = Radius::from_millimeters(dec!(-1.0));
            assert_eq!(Err(RadiusError::NegativeRadius), result);
        }

        #[test]
        fn it_should_display_a_radius() {
            let radius = Radius::from_millimeters(dec!(360.0)).unwrap();
            assert_eq!("360.0 mm", radius.to_string());
        }
    }

    mod technical_specifications {
        use super::*;
        use pretty_assertions::assert_eq;
        use rust_decimal_macros::dec;

        #[test]
        fn it_should_create_tech_specs() {
            let coupling = Coupling::new(CouplingSocket::Nem362, FeatureFlag::Yes, FeatureFlag::No);

            let radius = Radius::from_millimeters(dec!(360)).unwrap();
            let tech_specs = TechnicalSpecificationsBuilder::default()
                .with_coupling(coupling)
                .with_metal_body()
                .with_minimum_radius(radius)
                .with_interior_lights()
                .with_lights()
                .with_spring_buffers()
                .with_flywheel_fitted()
                .build();

            assert_eq!(Some(coupling), tech_specs.coupling());
            assert_eq!(Some(radius), tech_specs.minimum_radius());
            assert_eq!(FeatureFlag::Yes, tech_specs.metal_body());
            assert_eq!(FeatureFlag::Yes, tech_specs.interior_lights());
            assert_eq!(FeatureFlag::Yes, tech_specs.lights());
            assert_eq!(FeatureFlag::Yes, tech_specs.spring_buffers());
            assert_eq!(FeatureFlag::Yes, tech_specs.flywheel_fitted());
        }
    }
}
