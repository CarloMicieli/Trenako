use itertools::Itertools;
use serde::de::{Unexpected, Visitor};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::fmt::Formatter;
use std::str;
use std::str::FromStr;

/// It represents the service level for a passenger cars, like first or second class.
/// Values of service level can also include multiple service levels, like mixed first
/// and second class.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ServiceLevel {
    FirstClass,
    SecondClass,
    ThirdClass,
    FirstAndSecondClass,
    FirstSecondAndThirdClass,
    SecondAndThirdClass,
}

impl ServiceLevel {
    const FIRST_CLASS: &'static str = "1cl";
    const SECOND_CLASS: &'static str = "2cl";
    const THIRD_CLASS: &'static str = "3cl";
}

impl Serialize for ServiceLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct ServiceLevelVisitor;

impl<'de> Visitor<'de> for ServiceLevelVisitor {
    type Value = ServiceLevel;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "the input is not a valid service level")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if let Ok(dd) = ServiceLevel::from_str(s) {
            Ok(dd)
        } else {
            Err(de::Error::invalid_value(Unexpected::Str(s), &self))
        }
    }
}

impl<'de> Deserialize<'de> for ServiceLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ServiceLevelVisitor)
    }
}

impl fmt::Display for ServiceLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ServiceLevel::FirstClass => {
                write!(f, "{}", ServiceLevel::FIRST_CLASS)
            }
            ServiceLevel::SecondClass => {
                write!(f, "{}", ServiceLevel::SECOND_CLASS)
            }
            ServiceLevel::ThirdClass => {
                write!(f, "{}", ServiceLevel::THIRD_CLASS)
            }
            ServiceLevel::FirstAndSecondClass => {
                write!(f, "{}/{}", ServiceLevel::FIRST_CLASS, ServiceLevel::SECOND_CLASS)
            }
            ServiceLevel::FirstSecondAndThirdClass => write!(
                f,
                "{}/{}/{}",
                ServiceLevel::FIRST_CLASS,
                ServiceLevel::SECOND_CLASS,
                ServiceLevel::THIRD_CLASS
            ),
            ServiceLevel::SecondAndThirdClass => {
                write!(f, "{}/{}", ServiceLevel::SECOND_CLASS, ServiceLevel::THIRD_CLASS)
            }
        }
    }
}

impl str::FromStr for ServiceLevel {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("item number cannot be blank");
        }

        let service_level;
        if s.contains('/') {
            let tokens: Vec<&str> = s.split_terminator('/').sorted().dedup().collect();

            if tokens.len() == 2 {
                let first = tokens[0];
                let second = tokens[1];
                if first == ServiceLevel::FIRST_CLASS && second == ServiceLevel::SECOND_CLASS {
                    service_level = ServiceLevel::FirstAndSecondClass;
                } else if first == ServiceLevel::SECOND_CLASS && second == ServiceLevel::THIRD_CLASS {
                    service_level = ServiceLevel::SecondAndThirdClass;
                } else {
                    return Err("Invalid mixed service level");
                }
            } else if tokens.len() == 3 {
                let first = tokens[0];
                let second = tokens[1];
                let third = tokens[2];

                if first == ServiceLevel::FIRST_CLASS
                    && second == ServiceLevel::SECOND_CLASS
                    && third == ServiceLevel::THIRD_CLASS
                {
                    service_level = ServiceLevel::FirstSecondAndThirdClass;
                } else {
                    return Err("Invalid mixed service level");
                }
            } else {
                return Err("Invalid mixed service level: max number of values is 3");
            }
        } else {
            service_level = match s {
                ServiceLevel::FIRST_CLASS => ServiceLevel::FirstClass,
                ServiceLevel::SECOND_CLASS => ServiceLevel::SecondClass,
                ServiceLevel::THIRD_CLASS => ServiceLevel::ThirdClass,
                _ => return Err("Wrong value for service level"),
            };
        }
        Ok(service_level)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod service_levels {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;

        #[test]
        fn it_should_convert_string_slices_to_service_levels() {
            let service_level = "1cl".parse::<ServiceLevel>();
            assert!(service_level.is_ok());
            assert_eq!(service_level.unwrap(), ServiceLevel::FirstClass);
        }

        #[test]
        fn it_should_convert_string_slices_to_mixed_service_levels() {
            let service_level = "1cl/2cl/3cl/2cl".parse::<ServiceLevel>();
            assert!(service_level.is_ok());
            assert_eq!(service_level.unwrap(), ServiceLevel::FirstSecondAndThirdClass);
        }

        #[test]
        fn it_should_fail_to_convert_invalid_values_to_service_levels() {
            let empty_string = "".parse::<ServiceLevel>();
            assert!(empty_string.is_err());

            let invalid_value = "aaaa".parse::<ServiceLevel>();
            assert!(invalid_value.is_err());
        }

        #[test]
        fn it_should_fail_to_convert_string_slices_to_mixed_service_levels() {
            let wrong = "1cl/2cl/4cl/2cl".parse::<ServiceLevel>();
            assert!(wrong.is_err());
        }

        #[test]
        fn it_should_display_service_level_values() {
            assert_eq!("1cl", format!("{}", ServiceLevel::FirstClass));
            assert_eq!("1cl/2cl", format!("{}", ServiceLevel::FirstAndSecondClass));
        }

        #[rstest]
        #[case(ServiceLevel::FirstClass, r#""1cl""#)]
        #[case(ServiceLevel::SecondClass, r#""2cl""#)]
        #[case(ServiceLevel::FirstAndSecondClass, r#""1cl/2cl""#)]
        #[case(ServiceLevel::FirstSecondAndThirdClass, r#""1cl/2cl/3cl""#)]
        #[case(ServiceLevel::SecondAndThirdClass, r#""2cl/3cl""#)]
        fn it_should_serialize_service_levels(#[case] input: ServiceLevel, #[case] expected: &str) {
            let result = serde_json::to_string(&input).unwrap();
            assert_eq!(expected, result);
        }

        #[rstest]
        #[case(ServiceLevel::FirstClass)]
        #[case(ServiceLevel::SecondClass)]
        #[case(ServiceLevel::FirstAndSecondClass)]
        #[case(ServiceLevel::FirstSecondAndThirdClass)]
        #[case(ServiceLevel::SecondAndThirdClass)]
        fn it_should_serialize_and_deserialize_delivery_dates(#[case] input: ServiceLevel) {
            let test_struct = TestStruct { input };

            let json = serde_json::json!(test_struct);

            let result: serde_json::Result<TestStruct> = serde_json::from_str(&json.to_string());
            assert_eq!(test_struct, result.unwrap());
        }

        #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
        pub struct TestStruct {
            pub input: ServiceLevel,
        }
    }
}
