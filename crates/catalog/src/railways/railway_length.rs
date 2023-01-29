use common::length::Length;
use common::measure_units::MeasureUnit;
use rust_decimal::Decimal;
use serde::de::{MapAccess, SeqAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::fmt::{Display, Formatter};
use thiserror::Error;

/// The overall length of tracks (in km and miles) operated by a railway company
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct RailwayLength {
    /// the total railway network in kilometers
    pub kilometers: Length,
    /// the total railway network in miles
    pub miles: Length,
}

impl RailwayLength {
    /// Creates a new railway length
    pub fn new(kilometers: Decimal, miles: Decimal) -> Self {
        RailwayLength {
            kilometers: Length::Kilometers(kilometers),
            miles: Length::Miles(miles),
        }
    }

    /// Creates a new railway length from the kilometers value
    pub fn of_kilometers(kilometers: Decimal) -> Self {
        let miles = MeasureUnit::Kilometers.to(MeasureUnit::Miles).convert(kilometers);
        RailwayLength::new(kilometers, miles)
    }

    /// Creates a new railway length from the miles value
    pub fn of_miles(miles: Decimal) -> Self {
        let kilometers = MeasureUnit::Miles.to(MeasureUnit::Kilometers).convert(miles);
        RailwayLength::new(kilometers, miles)
    }

    /// Returns the length of track in Kilometers
    pub fn kilometers(&self) -> Length {
        self.kilometers
    }

    /// Returns the length of track in Miles
    pub fn miles(&self) -> Length {
        self.miles
    }
}

#[derive(Debug, PartialEq, Error)]
pub enum RailwayLengthError {
    #[error("the value in kilometers is not matching the one in miles")]
    DifferentValues,
    #[error("The length over buffers must be positive")]
    NonPositiveValue,
}

impl Display for RailwayLength {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "kilometers: {}, miles: {}", self.kilometers, self.miles)
    }
}

impl Serialize for RailwayLength {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("RailwayLength", 2)?;
        state.serialize_field("kilometers", &self.kilometers.quantity())?;
        state.serialize_field("miles", &self.miles.quantity())?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for RailwayLength {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Kilometers,
            Miles,
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
                        formatter.write_str("`kilometers` or `miles`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "kilometers" => Ok(Field::Kilometers),
                            "miles" => Ok(Field::Miles),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct RailwayLengthVisitor;

        impl<'de> Visitor<'de> for RailwayLengthVisitor {
            type Value = RailwayLength;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct RailwayLength")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<RailwayLength, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let inches = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let millimeters = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(RailwayLength::new(inches, millimeters))
            }

            fn visit_map<V>(self, mut map: V) -> Result<RailwayLength, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut kilometers = None;
                let mut miles = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Kilometers => {
                            if kilometers.is_some() {
                                return Err(de::Error::duplicate_field("kilometers"));
                            }
                            kilometers = Some(map.next_value()?);
                        }
                        Field::Miles => {
                            if miles.is_some() {
                                return Err(de::Error::duplicate_field("miles"));
                            }
                            miles = Some(map.next_value()?);
                        }
                    }
                }
                let kilometers = kilometers.ok_or_else(|| de::Error::missing_field("kilometers"))?;
                let miles = miles.ok_or_else(|| de::Error::missing_field("miles"))?;
                Ok(RailwayLength::new(kilometers, miles))
            }
        }

        const FIELDS: &[&str] = &["kilometers", "miles"];
        deserializer.deserialize_struct("RailwayLength", FIELDS, RailwayLengthVisitor)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod railway_lengths {
        use super::*;
        use pretty_assertions::assert_eq;
        use rust_decimal_macros::dec;

        #[test]
        fn it_should_create_new_railway_lengths() {
            let miles = Length::Miles(dec!(100));
            let kilometers = Length::Kilometers(dec!(100));
            let len = RailwayLength::new(dec!(100), dec!(100));
            assert_eq!(miles, len.miles());
            assert_eq!(kilometers, len.kilometers());
        }

        #[test]
        fn it_should_display_a_railway_length_value() {
            let miles = dec!(100);
            let kilometers = dec!(100);
            let len = RailwayLength::new(kilometers, miles);
            assert_eq!("kilometers: 100 km, miles: 100 mi", len.to_string());
        }

        #[test]
        fn it_should_create_a_railway_length_from_kilometers() {
            let kilometers = dec!(100);
            let length = RailwayLength::of_kilometers(kilometers);

            assert_eq!(Length::Kilometers(kilometers), length.kilometers());
            assert_eq!(Length::Miles(dec!(62.137100)), length.miles());
        }

        #[test]
        fn it_should_create_a_railway_length_from_miles() {
            let miles = dec!(100);
            let length = RailwayLength::of_miles(miles);

            assert_eq!(Length::Kilometers(dec!(160.93400)), length.kilometers());
            assert_eq!(Length::Miles(miles), length.miles());
        }

        #[test]
        fn it_should_serialize_railway_lengths_as_json() {
            let value = TestStruct {
                railway_length: RailwayLength::new(dec!(100), dec!(62.1)),
            };

            let json = serde_json::to_string(&value).expect("invalid JSON value");

            assert_eq!(r#"{"railway_length":{"kilometers":"100","miles":"62.1"}}"#, json);
        }

        #[test]
        fn it_should_deserialize_railway_lengths_from_json() {
            let json = r#"{"railway_length":{"kilometers":100,"miles":62.1}}"#;
            let test_struct: TestStruct = serde_json::from_str(json).expect("Invalid test struct");

            assert_eq!(Length::Kilometers(dec!(100)), test_struct.railway_length.kilometers());
            assert_eq!(Length::Miles(dec!(62.1)), test_struct.railway_length.miles());
        }

        #[derive(Serialize, Deserialize)]
        struct TestStruct {
            railway_length: RailwayLength,
        }
    }
}
