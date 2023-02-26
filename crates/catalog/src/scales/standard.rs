use sqlx::postgres::{PgHasArrayType, PgTypeInfo};
use sqlx::Type;
use strum_macros;
use strum_macros::{Display, EnumString};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, EnumString, Display, Type)]
#[sqlx(type_name = "scale_standard")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
pub enum Standard {
    #[serde(rename = "BRITISH")]
    #[sqlx(rename = "BRITISH")]
    British,

    #[serde(rename = "JAPANESE")]
    #[sqlx(rename = "JAPANESE")]
    Japanese,

    /// NEM-standards are used by model railway industry and hobbyists in Europe.
    #[serde(rename = "NEM")]
    #[sqlx(rename = "NEM")]
    NEM,

    /// NMRA standards are used widely in North America and by certain special
    /// interest groups all over the world.
    #[serde(rename = "NMRA")]
    #[sqlx(rename = "NMRA")]
    NMRA,
}

// See https://github.com/launchbadge/sqlx/issues/1004
impl PgHasArrayType for Standard {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_scale_standard")
    }
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
