use common::length::Length;
use rust_decimal::Decimal;
use sqlx::Type;
use strum_macros;
use strum_macros::{Display, EnumString};

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Coupling {
    socket: Socket,
    close_couplers: FeatureFlag,
    digital_shunting: FeatureFlag,
}

impl Coupling {
    pub fn new(socket: Socket, close_couplers: FeatureFlag, digital_shunting: FeatureFlag) -> Self {
        Coupling {
            socket,
            close_couplers,
            digital_shunting,
        }
    }

    pub fn with_close_couplers(socket: Socket) -> Self {
        Coupling {
            socket,
            close_couplers: FeatureFlag::Yes,
            digital_shunting: FeatureFlag::No,
        }
    }

    pub fn socket(&self) -> Socket {
        self.socket
    }

    pub fn close_couplers(&self) -> FeatureFlag {
        self.close_couplers
    }

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
pub enum Socket {
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

impl Default for Socket {
    fn default() -> Self {
        Socket::None
    }
}

/// The technical specification data for a rolling stock model
#[derive(Debug, Eq, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct TechnicalSpecifications {
    minimum_radius: Option<Radius>,
    coupling: Option<Coupling>,
    flywheel_fitted: FeatureFlag,
    metal_body: FeatureFlag,
    interior_lights: FeatureFlag,
    lights: FeatureFlag,
    spring_buffers: FeatureFlag,
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

    pub fn builder() -> TechnicalSpecificationsBuilder {
        TechnicalSpecificationsBuilder::default()
    }
}

#[derive(Default, Serialize, Deserialize)]
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
    NotAvailable,
}

impl Default for FeatureFlag {
    fn default() -> Self {
        FeatureFlag::No
    }
}

/// The minimum drivable radius
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize, Type)]
#[sqlx(transparent)]
pub struct Radius(Length);

impl Radius {
    pub fn of_millimeters(value: Decimal) -> Result<Self, &'static str> {
        if value.is_sign_positive() {
            Ok(Radius(Length::Millimeters(value)))
        } else {
            Err("negative radius")
        }
    }
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
        #[case("NONE", Ok(Socket::None))]
        #[case("NEM_355", Ok(Socket::Nem355))]
        #[case("NEM_356", Ok(Socket::Nem356))]
        #[case("NEM_357", Ok(Socket::Nem357))]
        #[case("NEM_359", Ok(Socket::Nem359))]
        #[case("NEM_360", Ok(Socket::Nem360))]
        #[case("NEM_362", Ok(Socket::Nem362))]
        #[case("NEM_365", Ok(Socket::Nem365))]
        #[case("invalid", Err(ParseError::VariantNotFound))]
        fn it_should_parse_strings_as_couplings(#[case] input: &str, #[case] expected: Result<Socket, ParseError>) {
            let coupling = input.parse::<Socket>();
            assert_eq!(expected, coupling);
        }

        #[rstest]
        #[case(Socket::None, "NONE")]
        #[case(Socket::Nem355, "NEM_355")]
        #[case(Socket::Nem356, "NEM_356")]
        #[case(Socket::Nem357, "NEM_357")]
        #[case(Socket::Nem359, "NEM_359")]
        #[case(Socket::Nem360, "NEM_360")]
        #[case(Socket::Nem362, "NEM_362")]
        #[case(Socket::Nem365, "NEM_365")]
        fn it_should_display_couplings(#[case] input: Socket, #[case] expected: &str) {
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
        #[case("NOT_AVAILABLE", Ok(FeatureFlag::NotAvailable))]
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
        #[case(FeatureFlag::NotAvailable, "NOT_AVAILABLE")]
        fn it_should_display_feature_flags(#[case] input: FeatureFlag, #[case] expected: &str) {
            assert_eq!(expected, input.to_string());
        }
    }

    mod technical_specifications {
        use super::*;
        use pretty_assertions::assert_eq;
        use rust_decimal_macros::dec;

        #[test]
        fn it_should_create_tech_specs() {
            let coupling = Coupling::new(Socket::Nem362, FeatureFlag::Yes, FeatureFlag::No);

            let radius = Radius::of_millimeters(dec!(360)).unwrap();
            let tech_specs = TechnicalSpecifications::builder()
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
