use super::ratio::Ratio;
use super::scale_gauge::Gauge;
use super::scale_id::ScaleId;
use super::standard::Standard;
use common::metadata::Metadata;
use std::collections::HashSet;
use std::fmt;

/// Rail transport modelling uses a variety of scales (ratio between the real world and the model)
/// to ensure scale models look correct when placed next to each other.
///
/// Model railway scales are standardized worldwide by many organizations and hobbyist groups.
/// Some of the scales are recognized globally, while others are less widespread and, in many cases,
/// virtually unknown outside their circle of origin. Scales may be expressed as a numeric ratio
/// (e.g. 1/87 or 1:87) or as letters defined in rail transport modelling standards
/// (e.g. HO, OO, N, O, G, TT and Z.) The majority of commercial model railway equipment manufacturers
/// base their offerings on Normen Europäischer Modellbahnen (NEM) or
/// National Model Railroad Association (NMRA) standards in most popular scales.
#[derive(Debug)]
pub struct Scale {
    scale_id: ScaleId,
    name: String,
    description: Option<String>,
    ratio: Ratio,
    gauge: Gauge,
    standards: HashSet<Standard>,
    metadata: Metadata,
}

impl Scale {
    /// Create a new Scale
    pub fn new(
        scale_id: ScaleId,
        name: &str,
        description: Option<&str>,
        ratio: Ratio,
        gauge: Gauge,
        standards: HashSet<Standard>,
        metadata: Metadata,
    ) -> Self {
        Scale {
            scale_id,
            name: String::from(name),
            description: description.map(String::from),
            ratio,
            gauge,
            standards,
            metadata,
        }
    }

    /// The unique identifier for this Scale
    pub fn scale_id(&self) -> &ScaleId {
        &self.scale_id
    }

    /// The scale name, typically letters defined in rail transport modelling standards
    /// (e.g. HO, OO, N, O, G, TT and Z.)
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The (optional) Scale description
    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// This Scale ratio between the real world and the model
    /// (e.g. 1/87 or 1:87)
    pub fn ratio(&self) -> &Ratio {
        &self.ratio
    }

    /// Returns the scale gauge
    pub fn gauge(&self) -> &Gauge {
        &self.gauge
    }

    /// Returns the standards set for this scale
    pub fn standards(&self) -> &HashSet<Standard> {
        &self.standards
    }

    /// Returns the scale metadata
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", &self.name, &self.ratio)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod scales {
        use super::data::scale_h0;
        use super::*;
        use crate::common::TrackGauge;
        use chrono::{DateTime, Utc};
        use pretty_assertions::assert_eq;
        use rust_decimal::Decimal;
        use rust_decimal_macros::dec;

        #[test]
        fn it_should_create_new_scales() {
            let now: DateTime<Utc> = Utc::now();
            let id = ScaleId::new("H0");
            let ratio = Ratio::try_from(Decimal::from(87)).unwrap();
            let gauge = Gauge::from_millimeters(TrackGauge::Standard, dec!(16.5));

            let scale = Scale::new(
                id.clone(),
                "H0",
                Some("Scale H0"),
                ratio.clone(),
                gauge.clone(),
                HashSet::new(),
                Metadata::created_at(now),
            );
            assert_eq!(id, scale.scale_id);
            assert_eq!("H0", scale.name);
            assert_eq!(Some("Scale H0".to_string()), scale.description);
            assert_eq!(ratio, scale.ratio);
            assert_eq!(gauge, scale.gauge);
            assert_eq!(HashSet::new(), scale.standards);
            assert_eq!(Metadata::created_at(now), scale.metadata);
        }

        #[test]
        fn it_should_display_scales() {
            let scale = scale_h0();
            assert_eq!("H0 (1:87)", scale.to_string());
        }
    }

    mod data {
        use super::*;
        use crate::common::TrackGauge;
        use chrono::Utc;
        use rust_decimal::Decimal;
        use rust_decimal_macros::dec;

        pub fn scale_h0() -> Scale {
            let gauge = Gauge::from_millimeters(TrackGauge::Standard, dec!(16.5));
            Scale::new(
                ScaleId::new("H0"),
                "H0",
                Some("Scale H0"),
                Ratio::try_from(Decimal::from(87)).unwrap(),
                gauge,
                HashSet::new(),
                Metadata::created_at(Utc::now()),
            )
        }
    }
}
