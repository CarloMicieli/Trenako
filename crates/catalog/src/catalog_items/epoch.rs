use itertools::Itertools;
use serde::de::{Unexpected, Visitor};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::fmt::Formatter;
use std::str;
use std::str::FromStr;
use thiserror::Error;

/// The model railway industry adopted an 'Era', or 'Epoch' system; the idea being to group models
/// into a defined time bracket, so that locomotives, coaching and wagon stock could be reasonably
/// grouped together.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[allow(non_snake_case)]
pub enum Epoch {
    I,
    II,
    IIa,
    IIb,
    III,
    IIIa,
    IIIb,
    IIIc,
    IV,
    IVa,
    IVb,
    V,
    Va,
    Vb,
    Vm,
    VI,
    Multiple(Box<Epoch>, Box<Epoch>),
}

impl str::FromStr for Epoch {
    type Err = EpochParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(EpochParseError::BlankValue);
        }

        if s.contains('/') {
            let tokens: Vec<&str> = s.split_terminator('/').sorted().dedup().collect();
            if tokens.len() == 2 {
                let first = Epoch::parse_str(tokens[0])?;
                let second = Epoch::parse_str(tokens[1])?;
                Ok(Epoch::Multiple(Box::new(first), Box::new(second)))
            } else {
                Err(EpochParseError::InvalidNumberOfValues)
            }
        } else {
            Epoch::parse_str(s)
        }
    }
}

#[derive(Error, Debug)]
pub enum EpochParseError {
    #[error("Epoch value cannot be blank")]
    BlankValue,
    #[error("Invalid number of elements for epoch values")]
    InvalidNumberOfValues,
    #[error("Invalid value for epoch")]
    InvalidValue,
}

impl Epoch {
    // Helper method to parse just the simple value
    fn parse_str(value: &str) -> Result<Self, EpochParseError> {
        match value {
            "I" => Ok(Epoch::I),
            "II" => Ok(Epoch::II),
            "IIa" => Ok(Epoch::IIa),
            "IIb" => Ok(Epoch::IIb),
            "III" => Ok(Epoch::III),
            "IIIa" => Ok(Epoch::IIIa),
            "IIIb" => Ok(Epoch::IIIb),
            "IIIc" => Ok(Epoch::IIIc),
            "IV" => Ok(Epoch::IV),
            "IVa" => Ok(Epoch::IVa),
            "IVb" => Ok(Epoch::IVb),
            "V" => Ok(Epoch::V),
            "Va" => Ok(Epoch::Va),
            "Vb" => Ok(Epoch::Vb),
            "Vm" => Ok(Epoch::Vm),
            "VI" => Ok(Epoch::VI),
            _ => Err(EpochParseError::InvalidValue),
        }
    }
}

impl fmt::Display for Epoch {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Epoch::Multiple(ep1, ep2) => write!(f, "{}/{}", &ep1, &ep2),
            _ => write!(f, "{self:?}"),
        }
    }
}

impl Serialize for Epoch {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct EpochVisitor;

impl<'de> Visitor<'de> for EpochVisitor {
    type Value = Epoch;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "the input is not a valid epoch")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if let Ok(dd) = Epoch::from_str(s) {
            Ok(dd)
        } else {
            Err(de::Error::invalid_value(Unexpected::Str(s), &self))
        }
    }
}

impl<'de> Deserialize<'de> for Epoch {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(EpochVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod epochs {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;

        #[test]
        fn it_should_convert_string_slices_to_epochs() {
            let epoch = "I".parse::<Epoch>();
            assert!(epoch.is_ok());
            assert_eq!(epoch.unwrap(), Epoch::I);
        }

        #[test]
        fn it_should_convert_string_slices_to_mixed_epochs() {
            let epoch = "I/II".parse::<Epoch>();
            assert!(epoch.is_ok());
            assert_eq!(epoch.unwrap(), Epoch::Multiple(Box::new(Epoch::I), Box::new(Epoch::II)));
        }

        #[test]
        fn it_should_fail_to_convert_invalid_values_to_epochs() {
            let empty_epoch = "".parse::<Epoch>();
            assert!(empty_epoch.is_err());

            let invalid_epoch = "invalid".parse::<Epoch>();
            assert!(invalid_epoch.is_err());
        }

        #[test]
        #[allow(non_snake_case)]
        fn it_should_display_epoch_values() {
            let epoch_I_II = Epoch::Multiple(Box::new(Epoch::I), Box::new(Epoch::II));
            let epoch_IVa = Epoch::IVa;

            assert_eq!("I/II", epoch_I_II.to_string());
            assert_eq!("IVa", epoch_IVa.to_string());
        }

        #[rstest]
        #[case(Epoch::I, r#""I""#)]
        #[case(Epoch::II, r#""II""#)]
        #[case(Epoch::IIa, r#""IIa""#)]
        #[case(Epoch::IIb, r#""IIb""#)]
        #[case(Epoch::III, r#""III""#)]
        #[case(Epoch::IIIa, r#""IIIa""#)]
        #[case(Epoch::IIIb, r#""IIIb""#)]
        #[case(Epoch::IIIc, r#""IIIc""#)]
        #[case(Epoch::IV, r#""IV""#)]
        #[case(Epoch::IVa, r#""IVa""#)]
        #[case(Epoch::IVb, r#""IVb""#)]
        #[case(Epoch::V, r#""V""#)]
        #[case(Epoch::Va, r#""Va""#)]
        #[case(Epoch::Vb, r#""Vb""#)]
        #[case(Epoch::Vm, r#""Vm""#)]
        #[case(Epoch::VI, r#""VI""#)]
        #[case(Epoch::Multiple(Box::new(Epoch::IV), Box::new(Epoch::V)), r#""IV/V""#)]
        fn it_should_serialize_epochs(#[case] input: Epoch, #[case] expected: &str) {
            let result = serde_json::to_string(&input).unwrap();
            assert_eq!(expected, result);
        }

        #[rstest]
        #[case(Epoch::I)]
        #[case(Epoch::II)]
        #[case(Epoch::IIa)]
        #[case(Epoch::IIb)]
        #[case(Epoch::III)]
        #[case(Epoch::IIIa)]
        #[case(Epoch::IIIb)]
        #[case(Epoch::IIIc)]
        #[case(Epoch::IV)]
        #[case(Epoch::IVa)]
        #[case(Epoch::IVb)]
        #[case(Epoch::V)]
        #[case(Epoch::Va)]
        #[case(Epoch::Vb)]
        #[case(Epoch::Vm)]
        #[case(Epoch::VI)]
        fn it_should_deserialize_delivery_epochs(#[case] input: Epoch) {
            let test_struct = TestStruct { input };

            let json = serde_json::json!(test_struct);

            let result: serde_json::Result<TestStruct> = serde_json::from_str(&json.to_string());
            assert_eq!(test_struct, result.unwrap());
        }

        #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
        pub struct TestStruct {
            pub input: Epoch,
        }
    }
}
