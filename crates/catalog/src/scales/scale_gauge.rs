use crate::common::TrackGauge;
use common::measure_units::MeasureUnit;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::cmp;
use std::cmp::Ordering;
use thiserror::Error;

/// It represents the track gauge information for a modelling scale
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct Gauge {
    /// the distance between the rails in millimeters
    pub millimeters: Decimal,
    /// the distance between the rails in inches
    pub inches: Decimal,
    /// the track gauge
    pub track_gauge: TrackGauge,
}

impl Gauge {
    /// Creates a new scale gauge
    pub fn new(track_gauge: TrackGauge, millimeters: Decimal, inches: Decimal) -> Result<Self, GaugeError> {
        match (millimeters, inches) {
            (mm, _) if mm.is_sign_negative() || mm.is_zero() => {
                Err(GaugeError::NegativeRailsDistance(mm, MeasureUnit::Millimeters))
            }
            (_, inches) if inches.is_sign_negative() || inches.is_zero() => {
                Err(GaugeError::NegativeRailsDistance(inches, MeasureUnit::Inches))
            }
            (mm, inches) if different_value_after_conversion(mm, inches) => Err(GaugeError::DifferentValues),
            (_, _) => Ok(Gauge {
                millimeters,
                inches,
                track_gauge,
            }),
        }
    }

    /// Creates a new scale gauge from the distance between the rails in inches
    pub fn from_inches(track_gauge: TrackGauge, inches: Decimal) -> Result<Self, GaugeError> {
        let millimeters = MeasureUnit::Inches.to(MeasureUnit::Millimeters).convert(inches);
        Gauge::new(track_gauge, millimeters, inches)
    }

    /// Creates a new scale gauge from the distance between the rails in millimeters
    pub fn from_millimeters(track_gauge: TrackGauge, millimeters: Decimal) -> Result<Self, GaugeError> {
        let inches = MeasureUnit::Millimeters.to(MeasureUnit::Inches).convert(millimeters);
        Gauge::new(track_gauge, millimeters, inches)
    }

    /// the distance between the rails in millimeters
    pub fn millimeters(&self) -> Decimal {
        self.millimeters
    }

    /// the distance between the rails in inches
    pub fn inches(&self) -> Decimal {
        self.inches
    }

    /// the track gauge
    pub fn track_gauge(&self) -> TrackGauge {
        self.track_gauge
    }

    /// The gauge for the HO scale
    pub const H0: Gauge = Gauge {
        track_gauge: TrackGauge::Standard,
        millimeters: dec!(16.5),
        inches: dec!(0.65),
    };

    /// The gauge for the N scale
    pub const N: Gauge = Gauge {
        track_gauge: TrackGauge::Standard,
        millimeters: dec!(9.0),
        inches: dec!(0.354),
    };
}

fn different_value_after_conversion(millimeters: Decimal, inches: Decimal) -> bool {
    let millimeters_converted = MeasureUnit::Inches.to(MeasureUnit::Millimeters).convert(inches);
    let diff = millimeters_converted - millimeters;
    Decimal::abs(&diff) > dec!(0.1)
}

impl cmp::PartialOrd for Gauge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.millimeters.partial_cmp(&other.millimeters)
    }
}

#[derive(Debug, PartialEq, Error)]
pub enum GaugeError {
    #[error("the distance between rails must be positive ({0} {1})")]
    NegativeRailsDistance(Decimal, MeasureUnit),
    #[error("the value in millimeters is not matching the one in inches")]
    DifferentValues,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod scale_gauges {
        use super::*;
        use crate::common::TrackGauge;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use rust_decimal_macros::dec;

        #[test]
        fn it_should_create_new_scale_gauges() {
            let millimeters = dec!(16.5);
            let inches = dec!(0.65);
            let gauge = Gauge::new(TrackGauge::Standard, millimeters, inches).expect("Invalid scale gauge");

            assert_eq!(TrackGauge::Standard, gauge.track_gauge());
            assert_eq!(dec!(16.5), gauge.millimeters());
            assert_eq!(dec!(0.65), gauge.inches());
        }

        #[rstest]
        #[case(dec!(-16.5), dec!(-0.65), Err(GaugeError::NegativeRailsDistance(dec!(-16.5), MeasureUnit::Millimeters)))]
        #[case(dec!(0.0), dec!(0.0), Err(GaugeError::NegativeRailsDistance(dec!(0.0), MeasureUnit::Millimeters)))]
        fn it_should_validate_the_input_creating_scale_gauge(
            #[case] millimeters: Decimal,
            #[case] inches: Decimal,
            #[case] expected: Result<Gauge, GaugeError>,
        ) {
            let result = Gauge::new(TrackGauge::Standard, millimeters, inches);
            assert_eq!(expected, result);
        }

        #[rstest]
        #[case(dec!(16.5), dec!(0.75), Err(GaugeError::DifferentValues))]
        fn it_should_validate_the_input_creating_scale_gauge_are_the_same_after_conversion(
            #[case] millimeters: Decimal,
            #[case] inches: Decimal,
            #[case] expected: Result<Gauge, GaugeError>,
        ) {
            let result = Gauge::new(TrackGauge::Standard, millimeters, inches);
            assert_eq!(expected, result);
        }

        #[test]
        fn it_should_create_scale_gauge_from_millimeters() {
            let gauge = Gauge::from_millimeters(TrackGauge::Standard, dec!(16.5)).expect("invalid scale gauge");
            assert_eq!(TrackGauge::Standard, gauge.track_gauge());
            assert_eq!(dec!(16.5), gauge.millimeters());
            assert_eq!(dec!(0.64960665), gauge.inches());
        }

        #[test]
        fn it_should_create_scale_gauge_from_inches() {
            let gauge = Gauge::from_inches(TrackGauge::Standard, dec!(0.65)).expect("invalid scale gauge");
            assert_eq!(TrackGauge::Standard, gauge.track_gauge());
            assert_eq!(dec!(16.510), gauge.millimeters());
            assert_eq!(dec!(0.65), gauge.inches());
        }

        #[test]
        fn it_should_compare_two_gauges() {
            let gauge1 = Gauge::from_millimeters(TrackGauge::Standard, dec!(16.5)).expect("invalid scale gauge");
            let gauge2 = Gauge::from_millimeters(TrackGauge::Standard, dec!(9.0)).expect("invalid scale gauge");

            assert!(gauge1 > gauge2);
            assert!(gauge2 < gauge1);
        }
    }
}
