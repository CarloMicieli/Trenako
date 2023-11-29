//! the brand status

use sqlx::Type;
use strum_macros;
use strum_macros::{Display, EnumString};

/// The current status for a model railway company
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, EnumString, Display, Type)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[sqlx(type_name = "brand_status", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BrandStatus {
    /// the brand is active
    Active,

    /// the brand is out of business
    OutOfBusiness,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod brand_statuses {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use strum::ParseError;

        #[rstest]
        #[case("ACTIVE", Ok(BrandStatus::Active))]
        #[case("OUT_OF_BUSINESS", Ok(BrandStatus::OutOfBusiness))]
        #[case("invalid", Err(ParseError::VariantNotFound))]
        fn it_should_parse_brand_statuses(#[case] input: &str, #[case] expected: Result<BrandStatus, ParseError>) {
            let status = input.parse::<BrandStatus>();
            assert_eq!(expected, status);
        }

        #[rstest]
        #[case(BrandStatus::Active, "ACTIVE")]
        #[case(BrandStatus::OutOfBusiness, "OUT_OF_BUSINESS")]
        fn it_should_display_brand_status(#[case] input: BrandStatus, #[case] expected: &str) {
            assert_eq!(expected, input.to_string());
        }
    }
}
