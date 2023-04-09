//! the catalog item availability status

use sqlx::Type;
use strum_macros;
use strum_macros::{Display, EnumString};

/// The availability status for a catalog item
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString, Display, Serialize, Deserialize, Type)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[sqlx(type_name = "availability_status", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AvailabilityStatus {
    /// the catalog item is just announced, hence not yet available
    Announced,

    /// the catalog item is available
    Available,

    /// the catalog item is discontinued
    Discontinued,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod availability_status {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use strum::ParseError;

        #[rstest]
        #[case("ANNOUNCED", Ok(AvailabilityStatus::Announced))]
        #[case("AVAILABLE", Ok(AvailabilityStatus::Available))]
        #[case("DISCONTINUED", Ok(AvailabilityStatus::Discontinued))]
        fn it_should_parse_string_as_availability_status(
            #[case] input: &str,
            #[case] expected: Result<AvailabilityStatus, ParseError>,
        ) {
            let result = input.parse::<AvailabilityStatus>();
            assert_eq!(expected, result);
        }

        #[rstest]
        #[case(AvailabilityStatus::Announced, "ANNOUNCED")]
        #[case(AvailabilityStatus::Available, "AVAILABLE")]
        #[case(AvailabilityStatus::Discontinued, "DISCONTINUED")]
        fn it_should_display_dcc_interfaces(#[case] input: AvailabilityStatus, #[case] expected: &str) {
            assert_eq!(expected, input.to_string());
        }
    }
}
