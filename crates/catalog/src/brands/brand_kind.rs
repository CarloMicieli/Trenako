//! the brand kinds

use sqlx::Type;
use strum_macros;
use strum_macros::{Display, EnumString};

/// The different kinds for railway models brands
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, EnumString, Display, Type, Default)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[sqlx(type_name = "brand_kind", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BrandKind {
    /// These manufacturers produce models which are made of brass or similar alloys.
    ///
    /// They are usually more expensive than the industrial series due to the limited
    /// production quantities and the "hand made" nature of the production
    BrassModels,

    /// These manufactures produce models using the die casting method
    #[default]
    Industrial,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod brand_kinds {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use strum::ParseError;

        #[rstest]
        #[case("BRASS_MODELS", Ok(BrandKind::BrassModels))]
        #[case("INDUSTRIAL", Ok(BrandKind::Industrial))]
        #[case("invalid", Err(ParseError::VariantNotFound))]
        fn it_should_parse_brand_kinds(#[case] input: &str, #[case] expected: Result<BrandKind, ParseError>) {
            let brand_kind = input.parse::<BrandKind>();
            assert_eq!(expected, brand_kind);
        }

        #[rstest]
        #[case(BrandKind::BrassModels, "BRASS_MODELS")]
        #[case(BrandKind::Industrial, "INDUSTRIAL")]
        fn it_should_display_brand_kinds(#[case] input: BrandKind, #[case] expected: &str) {
            assert_eq!(expected, input.to_string());
        }

        #[test]
        fn it_should_define_a_default_brand_kind() {
            let kind = BrandKind::default();
            assert_eq!(BrandKind::Industrial, kind);
        }
    }
}
