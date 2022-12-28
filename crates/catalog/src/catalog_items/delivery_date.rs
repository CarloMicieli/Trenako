use serde::de::{Unexpected, Visitor};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::fmt::Formatter;
use std::str;
use std::str::FromStr;
use thiserror::Error;

pub type Quarter = u8;
pub type Year = i32;

/// A delivery date for a catalog item (either by quarter or just year).
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DeliveryDate {
    ByYear(Year),
    ByQuarter(Year, Quarter),
}

impl DeliveryDate {
    /// Creates a new delivery date without the quarter
    pub fn by_year(year: Year) -> Self {
        DeliveryDate::ByYear(year)
    }

    /// Creates a new delivery date with the quarter information
    pub fn by_quarter(year: Year, quarter: Quarter) -> Self {
        DeliveryDate::ByQuarter(year, quarter)
    }

    pub fn year(&self) -> Year {
        match self {
            DeliveryDate::ByQuarter(y, _) => *y,
            DeliveryDate::ByYear(y) => *y,
        }
    }

    pub fn quarter(&self) -> Option<Quarter> {
        match self {
            DeliveryDate::ByQuarter(_, q) => Some(*q),
            DeliveryDate::ByYear(_) => None,
        }
    }

    fn parse_year(s: &str) -> Result<Year, DeliveryDateParseError> {
        let year = s
            .parse::<Year>()
            .map_err(|_| DeliveryDateParseError::InvalidYearValue)?;
        if year < 1900 && year >= 2999 {
            return Err(DeliveryDateParseError::InvalidYearValue);
        }

        Ok(year)
    }

    fn parse_quarter(s: &str) -> Result<Quarter, DeliveryDateParseError> {
        if s.len() != 2 {
            return Err(DeliveryDateParseError::InvalidQuarterValue);
        }

        let quarter = s[1..]
            .parse::<Quarter>()
            .map_err(|_| DeliveryDateParseError::InvalidQuarterValue)?;
        if quarter < 1 && quarter >= 4 {
            return Err(DeliveryDateParseError::InvalidQuarterValue);
        }

        Ok(quarter)
    }
}

impl str::FromStr for DeliveryDate {
    type Err = DeliveryDateParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(DeliveryDateParseError::EmptyValue);
        }

        if s.contains('/') {
            let tokens: Vec<&str> = s.split_terminator('/').collect();
            if tokens.len() != 2 {
                return Err(DeliveryDateParseError::InvalidByQuarterValue);
            }

            let year = DeliveryDate::parse_year(tokens[0])?;
            let quarter = DeliveryDate::parse_quarter(tokens[1])?;
            Ok(DeliveryDate::ByQuarter(year, quarter))
        } else {
            let year = DeliveryDate::parse_year(s)?;
            Ok(DeliveryDate::ByYear(year))
        }
    }
}

impl fmt::Display for DeliveryDate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DeliveryDate::ByQuarter(y, q) => write!(f, "{}/Q{}", y, q),
            DeliveryDate::ByYear(y) => write!(f, "{}", y),
        }
    }
}

impl Serialize for DeliveryDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct DeliveryDateVisitor;

impl<'de> Visitor<'de> for DeliveryDateVisitor {
    type Value = DeliveryDate;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "the input is not a valid delivery date")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if let Ok(dd) = DeliveryDate::from_str(s) {
            Ok(dd)
        } else {
            Err(de::Error::invalid_value(Unexpected::Str(s), &self))
        }
    }
}

impl<'de> Deserialize<'de> for DeliveryDate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(DeliveryDateVisitor)
    }
}

#[derive(Debug, Error)]
pub enum DeliveryDateParseError {
    #[error("Delivery date cannot be empty")]
    EmptyValue,
    #[error("Invalid delivery date by quarter")]
    InvalidByQuarterValue,
    #[error("Delivery date year component is not valid")]
    InvalidYearValue,
    #[error("Delivery date quarter component is not valid")]
    InvalidQuarterValue,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod delivery_dates {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;

        #[test]
        fn it_should_parse_string_as_delivery_dates() {
            let dd1 = "2020/Q1".parse::<DeliveryDate>();
            let dd2 = "2020".parse::<DeliveryDate>();

            assert!(dd1.is_ok());

            let dd1_val = dd1.unwrap();
            assert_eq!(2020, dd1_val.year());
            assert_eq!(Some(1), dd1_val.quarter());

            assert!(dd2.is_ok());

            let dd2_val = dd2.unwrap();
            assert_eq!(2020, dd2_val.year());
            assert_eq!(None, dd2_val.quarter());
        }

        #[test]
        fn it_should_produce_string_representations_from_delivery_dates() {
            let dd1 = "2020/Q1".parse::<DeliveryDate>().unwrap();
            let dd2 = "2020".parse::<DeliveryDate>().unwrap();

            assert_eq!("2020/Q1", dd1.to_string());
            assert_eq!("2020", dd2.to_string());
        }

        #[rstest]
        #[case("2020/Q1", r#""2020/Q1""#)]
        #[case("2020", r#""2020""#)]
        fn it_should_serialize_delivery_dates(#[case] input: &str, #[case] expected: &str) {
            let dd1 = input.parse::<DeliveryDate>().unwrap();
            let result = serde_json::to_string(&dd1).unwrap();
            assert_eq!(expected, result);
        }

        #[test]
        fn it_should_deserialize_delivery_dates() {
            let test_struct = TestStruct {
                just_year: DeliveryDate::ByYear(2022),
                with_quarter: DeliveryDate::ByQuarter(2022, 3),
            };

            let json = serde_json::json!(test_struct);

            let result: serde_json::Result<TestStruct> = serde_json::from_str(&json.to_string());
            assert_eq!(test_struct, result.unwrap());
        }

        #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
        pub struct TestStruct {
            pub just_year: DeliveryDate,
            pub with_quarter: DeliveryDate,
        }
    }
}
