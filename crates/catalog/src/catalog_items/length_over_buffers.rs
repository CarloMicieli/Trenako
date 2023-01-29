use common::length::Length;
use common::measure_units::MeasureUnit;
use rust_decimal::Decimal;
use serde::de::{MapAccess, SeqAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use thiserror::Error;

/// The rail vehicle measurement method expressed as the length over buffers
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct LengthOverBuffers {
    /// the overall length in inches
    pub inches: Option<Length>,
    /// the overall length in millimeters
    pub millimeters: Option<Length>,
}

impl LengthOverBuffers {
    /// Creates a new length over buffers value
    pub fn new(inches: Option<Decimal>, millimeters: Option<Decimal>) -> Result<Self, LengthOverBuffersError> {
        match (inches, millimeters) {
            (Some(inches), _) if inches.is_sign_negative() || inches.is_zero() => {
                Err(LengthOverBuffersError::NonPositiveValue)
            }
            (_, Some(mm)) if mm.is_sign_negative() || mm.is_zero() => Err(LengthOverBuffersError::NonPositiveValue),
            (Some(inches), Some(mm)) if !MeasureUnit::Millimeters.same_as(mm, MeasureUnit::Inches, inches) => {
                Err(LengthOverBuffersError::DifferentValues)
            }
            _ => {
                let inches = inches.map(Length::Inches);
                let millimeters = millimeters.map(Length::Millimeters);
                Ok(LengthOverBuffers { inches, millimeters })
            }
        }
    }

    /// Creates a new length over buffers value in millimeters
    pub fn from_millimeters(millimeters: Length) -> Self {
        let inches = MeasureUnit::Millimeters
            .to(MeasureUnit::Inches)
            .convert(millimeters.quantity());
        LengthOverBuffers {
            inches: Some(Length::Inches(inches)),
            millimeters: Some(millimeters),
        }
    }

    /// Creates a new length over buffers value in inches
    pub fn from_inches(inches: Length) -> Self {
        let millimeters = MeasureUnit::Inches
            .to(MeasureUnit::Millimeters)
            .convert(inches.quantity());
        LengthOverBuffers {
            inches: Some(inches),
            millimeters: Some(Length::Millimeters(millimeters)),
        }
    }

    /// the length over buffers value in inches
    pub fn inches(&self) -> Option<&Length> {
        self.inches.as_ref()
    }

    /// the length over buffers value in millimeters
    pub fn millimeters(&self) -> Option<&Length> {
        self.millimeters.as_ref()
    }
}

#[derive(Debug, PartialEq, Error)]
pub enum LengthOverBuffersError {
    #[error("the value in millimeters is not matching the one in inches")]
    DifferentValues,
    #[error("The length over buffers must be positive")]
    NonPositiveValue,
}

impl Serialize for LengthOverBuffers {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("LengthOverBuffers", 2)?;
        state.serialize_field("inches", &self.inches.map(|inches| inches.quantity()))?;
        state.serialize_field("millimeters", &self.millimeters.map(|mm| mm.quantity()))?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for LengthOverBuffers {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Inches,
            Millimeters,
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
                        formatter.write_str("`inches` or `millimeters`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "inches" => Ok(Field::Inches),
                            "millimeters" => Ok(Field::Millimeters),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct LengthOverBuffersVisitor;

        impl<'de> Visitor<'de> for LengthOverBuffersVisitor {
            type Value = LengthOverBuffers;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct LengthOverBuffers")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<LengthOverBuffers, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let inches = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let millimeters = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(LengthOverBuffers::new(inches, millimeters).unwrap())
            }

            fn visit_map<V>(self, mut map: V) -> Result<LengthOverBuffers, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut inches = None;
                let mut millimeters = None;
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
                    }
                }
                let inches = inches.ok_or_else(|| de::Error::missing_field("inches"))?;
                let millimeters = millimeters.ok_or_else(|| de::Error::missing_field("millimeters"))?;
                Ok(LengthOverBuffers::new(inches, millimeters).unwrap())
            }
        }

        const FIELDS: &[&str] = &["inches", "millimeters"];
        deserializer.deserialize_struct("LengthOverBuffers", FIELDS, LengthOverBuffersVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod length_over_buffer_tests {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use rust_decimal_macros::dec;

        #[rstest]
        #[case(None, None, Ok(LengthOverBuffers { inches: None, millimeters: None}))]
        #[case(Some(dec!(0.0)), Some(dec!(0.0)), Err(LengthOverBuffersError::NonPositiveValue))]
        #[case(Some(dec!(-0.65)), Some(dec!(-16.5)), Err(LengthOverBuffersError::NonPositiveValue))]
        #[case(Some(dec!(0.65)), Some(dec!(16.3)), Err(LengthOverBuffersError::DifferentValues))]
        fn it_should_create_new_length_over_buffers_values(
            #[case] inches: Option<Decimal>,
            #[case] millimeters: Option<Decimal>,
            #[case] expected: Result<LengthOverBuffers, LengthOverBuffersError>,
        ) {
            let result = LengthOverBuffers::new(inches, millimeters);
            assert_eq!(expected, result);
        }

        #[test]
        fn it_should_create_new_length_over_buffer_from_inches() {
            let inches = Length::Inches(dec!(42));
            let lob = LengthOverBuffers::from_inches(inches);
            assert_eq!(Some(&inches), lob.inches());
            assert_eq!(Some(&Length::Millimeters(dec!(1066.8))), lob.millimeters());
        }

        #[test]
        fn it_should_create_new_length_over_buffer_from_millimeters() {
            let millimeters = Length::Millimeters(dec!(42));
            let lob = LengthOverBuffers::from_millimeters(millimeters);
            assert_eq!(Some(&millimeters), lob.millimeters());
            assert_eq!(Some(&Length::Inches(dec!(1.6535442))), lob.inches());
        }

        #[test]
        fn it_should_serialize_as_json() {
            let inches = dec!(0.65);
            let millimeters = dec!(16.5);
            let value = TestStruct {
                length_over_buffers: LengthOverBuffers::new(Some(inches), Some(millimeters))
                    .expect("invalid length over buffers"),
            };

            let json = serde_json::to_string(&value).expect("invalid JSON value");

            let expected = r#"{"length_over_buffers":{"inches":"0.65","millimeters":"16.5"}}"#;
            assert_eq!(expected, json);
        }

        #[test]
        fn it_should_deserialize_from_json() {
            let inches = dec!(0.65);
            let millimeters = dec!(16.5);

            let json = r#"{"length_over_buffers":{"inches":"0.65","millimeters":"16.5"}}"#;

            let test_struct: TestStruct = serde_json::from_str(json).expect("Invalid test struct");

            assert_eq!(
                Some(inches),
                test_struct.length_over_buffers.inches.map(|l| l.quantity())
            );
            assert_eq!(
                Some(millimeters),
                test_struct.length_over_buffers.millimeters.map(|l| l.quantity())
            );
        }

        #[derive(Serialize, Deserialize)]
        struct TestStruct {
            length_over_buffers: LengthOverBuffers,
        }
    }
}
