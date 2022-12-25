use strum_macros;
use strum_macros::{Display, EnumString};

/// The different kinds for railway models brands
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString, Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
pub enum BrandType {
    /// These manufactures produce models using the die casting method
    Industrial,

    /// These manufacturers produce models which are made of brass or similar alloys.
    ///
    /// They are usually more expensive than the industrial series due to the limited
    /// production quantities and the "hand made" nature of the production
    BrassModels,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod brand_types {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use strum::ParseError;

        #[rstest]
        #[case("BRASS_MODELS", Ok(BrandType::BrassModels))]
        #[case("INDUSTRIAL", Ok(BrandType::Industrial))]
        #[case("invalid", Err(ParseError::VariantNotFound))]
        fn it_should_parse_brand_types(#[case] input: &str, #[case] expected: Result<BrandType, ParseError>) {
            let brand_type = input.parse::<BrandType>();
            assert_eq!(expected, brand_type);
        }

        #[rstest]
        #[case(BrandType::BrassModels, "BRASS_MODELS")]
        #[case(BrandType::Industrial, "INDUSTRIAL")]
        fn it_should_display_brand_status(#[case] input: BrandType, #[case] expected: &str) {
            assert_eq!(expected, input.to_string());
        }
    }
}
