use crate::common::TrackGauge;
use common::length::Length;
use common::measure_units::MeasureUnit;
use common::measure_units::MeasureUnit::Millimeters;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::de::{MapAccess, SeqAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::cmp::Ordering;
use std::{cmp, fmt};
use thiserror::Error;

/// It represents the track gauge information for a modelling scale
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Gauge {
    /// the distance between the rails in millimeters
    pub millimeters: Length,
    /// the distance between the rails in inches
    pub inches: Length,
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
            (mm, inches) if !Millimeters.same_as(mm, MeasureUnit::Inches, inches) => Err(GaugeError::DifferentValues),
            (_, _) => Ok(Gauge {
                millimeters: Length::Millimeters(millimeters),
                inches: Length::Inches(inches),
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
    pub fn millimeters(&self) -> Length {
        self.millimeters
    }

    /// the distance between the rails in inches
    pub fn inches(&self) -> Length {
        self.inches
    }

    /// the track gauge
    pub fn track_gauge(&self) -> TrackGauge {
        self.track_gauge
    }

    /// The gauge for the HO scale
    pub const H0: Gauge = Gauge {
        track_gauge: TrackGauge::Standard,
        millimeters: Length::Millimeters(dec!(16.5)),
        inches: Length::Inches(dec!(0.65)),
    };

    /// The gauge for the N scale
    pub const N: Gauge = Gauge {
        track_gauge: TrackGauge::Standard,
        millimeters: Length::Millimeters(dec!(9.0)),
        inches: Length::Inches(dec!(0.354)),
    };
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

impl Serialize for Gauge {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Gauge", 2)?;
        state.serialize_field("track_gauge", &self.track_gauge)?;
        state.serialize_field("millimeters", &self.millimeters.quantity())?;
        state.serialize_field("inches", &self.inches.quantity())?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Gauge {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Inches,
            Millimeters,
            TrackGauge,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`inches` or `millimeters` or `track_gauge`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "inches" => Ok(Field::Inches),
                            "millimeters" => Ok(Field::Millimeters),
                            "track_gauge" => Ok(Field::TrackGauge),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct GaugeVisitor;

        impl<'de> Visitor<'de> for GaugeVisitor {
            type Value = Gauge;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Gauge")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Gauge, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let track_gauge = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let inches = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let millimeters = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(2, &self))?;
                Ok(Gauge::new(track_gauge, millimeters, inches).unwrap())
            }

            fn visit_map<V>(self, mut map: V) -> Result<Gauge, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut inches = None;
                let mut millimeters = None;
                let mut track_gauge = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Inches => {
                            if inches.is_some() {
                                return Err(de::Error::duplicate_field("inches"));
                            }
                            inches = Some(map.next_value()?);
                        }
                        Field::Millimeters => {
                            if millimeters.is_some() {
                                return Err(de::Error::duplicate_field("millimeters"));
                            }
                            millimeters = Some(map.next_value()?);
                        }
                        Field::TrackGauge => {
                            if track_gauge.is_some() {
                                return Err(de::Error::duplicate_field("track_gauge"));
                            }
                            track_gauge = Some(map.next_value()?);
                        }
                    }
                }
                let inches = inches.ok_or_else(|| de::Error::missing_field("inches"))?;
                let millimeters = millimeters.ok_or_else(|| de::Error::missing_field("millimeters"))?;
                let track_gauge = track_gauge.ok_or_else(|| de::Error::missing_field("track_gauge"))?;
                Ok(Gauge::new(track_gauge, millimeters, inches).unwrap())
            }
        }

        const FIELDS: &[&str] = &["inches", "millimeters", "track_gauge"];
        deserializer.deserialize_struct("Gauge", FIELDS, GaugeVisitor)
    }
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
            assert_eq!(dec!(16.5), gauge.millimeters().quantity());
            assert_eq!(dec!(0.65), gauge.inches().quantity());
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
            assert_eq!(dec!(16.5), gauge.millimeters().quantity());
            assert_eq!(dec!(0.64960665), gauge.inches().quantity());
        }

        #[test]
        fn it_should_create_scale_gauge_from_inches() {
            let gauge = Gauge::from_inches(TrackGauge::Standard, dec!(0.65)).expect("invalid scale gauge");
            assert_eq!(TrackGauge::Standard, gauge.track_gauge());
            assert_eq!(dec!(16.510), gauge.millimeters().quantity());
            assert_eq!(dec!(0.65), gauge.inches().quantity());
        }

        #[test]
        fn it_should_compare_two_gauges() {
            let gauge1 = Gauge::from_millimeters(TrackGauge::Standard, dec!(16.5)).expect("invalid scale gauge");
            let gauge2 = Gauge::from_millimeters(TrackGauge::Standard, dec!(9.0)).expect("invalid scale gauge");

            assert!(gauge1 > gauge2);
            assert!(gauge2 < gauge1);
        }

        #[test]
        fn it_should_serialize_scale_gauges_as_json() {
            let value = TestStruct {
                gauge: Gauge::new(TrackGauge::Standard, dec!(16.5), dec!(0.65)).expect("invalid scale gauge"),
            };

            let json = serde_json::to_string(&value).expect("invalid JSON value");

            assert_eq!(
                r#"{"gauge":{"track_gauge":"STANDARD","millimeters":"16.5","inches":"0.65"}}"#,
                json
            );
        }

        #[test]
        fn it_should_deserialize_scale_gauges_from_json() {
            let json = r#"{"gauge":{"track_gauge":"STANDARD","millimeters":"16.5","inches":"0.65"}}"#;
            let test_struct: TestStruct = serde_json::from_str(json).expect("Invalid test struct");

            assert_eq!(TrackGauge::Standard, test_struct.gauge.track_gauge);
            assert_eq!(dec!(16.5), test_struct.gauge.millimeters.quantity());
            assert_eq!(dec!(0.65), test_struct.gauge.inches.quantity());
        }

        #[derive(Serialize, Deserialize)]
        struct TestStruct {
            gauge: Gauge,
        }
    }
}
