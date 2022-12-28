use crate::common::TrackGauge;
use common::measure_units::MeasureUnit;
use rust_decimal::Decimal;
use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct Gauge {
    track_gauge: TrackGauge,
    millimeters: Decimal,
    inches: Decimal,
}

impl Gauge {
    pub fn new(track_gauge: TrackGauge, millimeters: Decimal, inches: Decimal) -> Self {
        Gauge {
            track_gauge,
            millimeters,
            inches,
        }
    }

    pub fn from_inches(track_gauge: TrackGauge, inches: Decimal) -> Self {
        let millimeters = MeasureUnit::Inches.to(MeasureUnit::Millimeters).convert(inches);
        Gauge {
            track_gauge,
            millimeters,
            inches,
        }
    }

    pub fn from_millimeters(track_gauge: TrackGauge, millimeters: Decimal) -> Self {
        let inches = MeasureUnit::Millimeters.to(MeasureUnit::Inches).convert(millimeters);
        Gauge {
            track_gauge,
            millimeters,
            inches,
        }
    }

    /// The distance between the rails in millimeters
    pub fn millimeters(&self) -> Decimal {
        self.millimeters
    }

    /// The distance between the rails in inches
    pub fn inches(&self) -> Decimal {
        self.inches
    }

    /// The track gauge
    pub fn track_gauge(&self) -> TrackGauge {
        self.track_gauge
    }
}

impl PartialOrd for Gauge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.millimeters.partial_cmp(&other.millimeters)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod scale_gauges {
        use super::*;
        use crate::common::TrackGauge;
        use pretty_assertions::assert_eq;
        use rust_decimal_macros::dec;

        #[test]
        fn it_should_create_scale_gauge_from_millimeters() {
            let gauge = Gauge::from_millimeters(TrackGauge::Standard, dec!(16.5));
            assert_eq!(TrackGauge::Standard, gauge.track_gauge());
            assert_eq!(dec!(16.5), gauge.millimeters());
            assert_eq!(dec!(0.64960665), gauge.inches());
        }

        #[test]
        fn it_should_create_scale_gauge_from_inches() {
            let gauge = Gauge::from_inches(TrackGauge::Standard, dec!(0.65));
            assert_eq!(TrackGauge::Standard, gauge.track_gauge());
            assert_eq!(dec!(16.510), gauge.millimeters());
            assert_eq!(dec!(0.65), gauge.inches());
        }

        #[test]
        fn it_should_compare_two_gauges() {
            let gauge1 = Gauge::from_millimeters(TrackGauge::Standard, dec!(16.5));
            let gauge2 = Gauge::from_millimeters(TrackGauge::Standard, dec!(9.0));

            assert!(gauge1 > gauge2);
            assert!(gauge2 < gauge1);
        }
    }
}
