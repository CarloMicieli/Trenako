use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use strum_macros;
use strum_macros::{Display, EnumString};

#[derive(Debug, Eq, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct TechSpecs {
    minimum_radius: Option<Radius>,
    coupling: Option<Coupling>,
    flywheel_fitted: FeatureFlag,
    close_couplers: FeatureFlag,
    metal_body: FeatureFlag,
    interior_lights: FeatureFlag,
    lights: FeatureFlag,
    spring_buffers: FeatureFlag,
    digital_shunting_coupling: FeatureFlag,
}

impl TechSpecs {
    pub fn minimum_radius(&self) -> Option<Radius> {
        self.minimum_radius
    }

    pub fn coupling(&self) -> Option<Coupling> {
        self.coupling
    }

    pub fn flywheel_fitted(&self) -> FeatureFlag {
        self.flywheel_fitted
    }

    pub fn close_couplers(&self) -> FeatureFlag {
        self.close_couplers
    }

    pub fn metal_body(&self) -> FeatureFlag {
        self.metal_body
    }

    pub fn interior_lights(&self) -> FeatureFlag {
        self.interior_lights
    }

    pub fn lights(&self) -> FeatureFlag {
        self.lights
    }

    pub fn spring_buffers(&self) -> FeatureFlag {
        self.spring_buffers
    }

    pub fn digital_shunting_coupling(&self) -> FeatureFlag {
        self.digital_shunting_coupling
    }

    pub fn builder() -> TechSpecsBuilder {
        TechSpecsBuilder::default()
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct TechSpecsBuilder {
    minimum_radius: Option<Radius>,
    coupling: Option<Coupling>,
    flywheel_fitted: FeatureFlag,
    close_couplers: FeatureFlag,
    metal_body: FeatureFlag,
    interior_lights: FeatureFlag,
    lights: FeatureFlag,
    spring_buffers: FeatureFlag,
    digital_shunting_coupling: FeatureFlag,
}

impl TechSpecsBuilder {
    /// Add the minimum radius to the tech specifications
    pub fn with_minimum_radius(mut self, radius: Radius) -> Self {
        self.minimum_radius = Some(radius);
        self
    }

    /// Add the coupling to the tech specifications
    pub fn with_coupling(mut self, coupling: Coupling) -> Self {
        self.coupling = Some(coupling);
        self
    }

    pub fn with_flywheel_fitted(mut self) -> Self {
        self.flywheel_fitted = FeatureFlag::Yes;
        self
    }

    pub fn with_close_couplers(mut self) -> Self {
        self.close_couplers = FeatureFlag::Yes;
        self
    }

    pub fn with_metal_body(mut self) -> Self {
        self.metal_body = FeatureFlag::Yes;
        self
    }

    pub fn with_interior_lights(mut self) -> Self {
        self.interior_lights = FeatureFlag::Yes;
        self
    }

    pub fn with_lights(mut self) -> Self {
        self.lights = FeatureFlag::Yes;
        self
    }

    pub fn with_spring_buffers(mut self) -> Self {
        self.spring_buffers = FeatureFlag::Yes;
        self
    }

    pub fn with_digital_shunting_coupling(mut self) -> Self {
        self.digital_shunting_coupling = FeatureFlag::Yes;
        self
    }

    pub fn build(self) -> TechSpecs {
        TechSpecs {
            minimum_radius: self.minimum_radius,
            coupling: self.coupling,
            flywheel_fitted: self.flywheel_fitted,
            close_couplers: self.close_couplers,
            metal_body: self.metal_body,
            interior_lights: self.interior_lights,
            lights: self.lights,
            spring_buffers: self.spring_buffers,
            digital_shunting_coupling: self.digital_shunting_coupling,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString, Display, Serialize, Deserialize)]
#[strum(ascii_case_insensitive)]
pub enum Coupling {
    #[strum(serialize = "none")]
    None,

    /// Receptacle for Replaceable Coupling Heads in Scales TT and N
    #[strum(serialize = "nem_355")]
    Nem355,

    /// Coupler Head for Scale N
    #[strum(serialize = "nem_356")]
    Nem356,

    /// Coupler Head for Scale N
    #[strum(serialize = "nem_357")]
    Nem357,

    /// Coupler Head for Scale TT
    #[strum(serialize = "nem_359")]
    Nem359,

    /// Standard Coupling for Scale H0
    #[strum(serialize = "nem_360")]
    Nem360,

    /// NEM shaft 362 with close coupling mechanism
    #[strum(serialize = "nem_362")]
    Nem362,

    /// Coupler Head for Scale 0
    #[strum(serialize = "nem_365")]
    Nem365,
}

impl Default for Coupling {
    fn default() -> Self {
        Coupling::None
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum FeatureFlag {
    Yes,
    No,
}

impl Default for FeatureFlag {
    fn default() -> Self {
        FeatureFlag::No
    }
}

/// Minimum drivable radius
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct Radius(Decimal);

impl Radius {
    pub fn new(value: f32) -> Option<Radius> {
        if value.is_sign_negative() {
            None
        } else {
            let v = Decimal::from_f32(value)?;
            Some(Radius(v))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod tech_specs {
        use super::*;
        use pretty_assertions::assert_eq;
        use rust_decimal_macros::dec;

        #[test]
        fn it_should_create_tech_specs() {
            let tech_specs = TechSpecs::builder()
                .with_coupling(Coupling::Nem362)
                .with_minimum_radius(Radius(dec!(360)))
                .build();

            assert_eq!(Some(Coupling::Nem362), tech_specs.coupling());
            assert_eq!(Some(Radius(dec!(360))), tech_specs.minimum_radius());
        }
    }
}
