use sqlx::Type;
use std::str;
use strum_macros;
use strum_macros::{Display, EnumString};

/// It represents the service level for a passenger cars, like first or second class.
/// Values of service level can also include multiple service levels, like mixed first
/// and second class.
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString, Display, Serialize, Deserialize, Type)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[sqlx(type_name = "service_level", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ServiceLevel {
    /// first class only
    FirstClass,
    /// second class only
    SecondClass,
    /// third class only
    ThirdClass,
    /// mixed first and second class
    FirstAndSecondClass,
    /// mixed first, second and third class
    FirstSecondAndThirdClass,
    /// mixed second and third class
    SecondAndThirdClass,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod service_levels {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use strum::ParseError;

        #[rstest]
        #[case("FIRST_CLASS", Ok(ServiceLevel::FirstClass))]
        #[case("SECOND_CLASS", Ok(ServiceLevel::SecondClass))]
        #[case("FIRST_AND_SECOND_CLASS", Ok(ServiceLevel::FirstAndSecondClass))]
        #[case("FIRST_SECOND_AND_THIRD_CLASS", Ok(ServiceLevel::FirstSecondAndThirdClass))]
        #[case("SECOND_AND_THIRD_CLASS", Ok(ServiceLevel::SecondAndThirdClass))]
        fn it_should_parse_service_levels(#[case] input: &str, #[case] expected: Result<ServiceLevel, ParseError>) {
            assert_eq!(expected, input.parse::<ServiceLevel>());
        }

        #[rstest]
        #[case(ServiceLevel::FirstClass, "FIRST_CLASS")]
        #[case(ServiceLevel::SecondClass, "SECOND_CLASS")]
        #[case(ServiceLevel::FirstAndSecondClass, "FIRST_AND_SECOND_CLASS")]
        #[case(ServiceLevel::FirstSecondAndThirdClass, "FIRST_SECOND_AND_THIRD_CLASS")]
        #[case(ServiceLevel::SecondAndThirdClass, "SECOND_AND_THIRD_CLASS")]
        fn it_should_display_service_levels(#[case] input: ServiceLevel, #[case] expected: &str) {
            assert_eq!(expected, input.to_string());
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
