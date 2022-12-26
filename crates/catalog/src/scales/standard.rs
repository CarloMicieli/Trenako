use strum_macros;
use strum_macros::{Display, EnumString};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, EnumString, Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
pub enum Standard {
    British,

    Japanese,

    /// NEM-standards are used by model railway industry and hobbyists in Europe.
    NEM,

    /// NMRA standards are used widely in North America and by certain special
    /// interest groups all over the world.
    NMRA,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod standards {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use strum::ParseError;

        #[rstest]
        #[case("BRITISH", Ok(Standard::British))]
        #[case("JAPANESE", Ok(Standard::Japanese))]
        #[case("NEM", Ok(Standard::NEM))]
        #[case("NMRA", Ok(Standard::NMRA))]
        #[case("invalid", Err(ParseError::VariantNotFound))]
        fn it_should_parse_strings_as_scale_standards(
            #[case] input: &str,
            #[case] expected: Result<Standard, ParseError>,
        ) {
            let standard = input.parse::<Standard>();
            assert_eq!(expected, standard);
        }

        #[rstest]
        #[case(Standard::British, "BRITISH")]
        #[case(Standard::Japanese, "JAPANESE")]
        #[case(Standard::NEM, "NEM")]
        #[case(Standard::NMRA, "NMRA")]
        fn it_should_display_scale_standards(#[case] input: Standard, #[case] expected: &str) {
            assert_eq!(expected, input.to_string());
        }
    }
}
