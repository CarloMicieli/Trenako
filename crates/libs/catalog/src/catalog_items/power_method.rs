//! the rolling stock power method

use sqlx::Type;
use strum_macros;
use strum_macros::{Display, EnumString};

/// The power methods for the model.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, EnumString, Display, Type)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[sqlx(type_name = "power_method", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PowerMethod {
    /// Alternating current (Maerklin).
    #[serde(rename = "AC")]
    AC,

    /// Direct current.
    #[serde(rename = "DC")]
    DC,

    /// Trix Express was the main model train product range of the Trix of Nuremberg.
    /// The original system used 14V AC power, hanged to 14V DC in 1953 and used the third rail
    /// system until 2003 when the last models were produced.
    #[serde(rename = "TRIX_EXPRESS")]
    TrixExpress,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod power_methods {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use strum::ParseError;

        #[rstest]
        #[case("AC", Ok(PowerMethod::AC))]
        #[case("DC", Ok(PowerMethod::DC))]
        #[case("TRIX_EXPRESS", Ok(PowerMethod::TrixExpress))]
        fn it_should_parse_string_as_power_methods(
            #[case] input: &str,
            #[case] expected: Result<PowerMethod, ParseError>,
        ) {
            let power_method = input.parse::<PowerMethod>();
            assert_eq!(expected, power_method);
        }

        #[rstest]
        #[case(PowerMethod::AC, "AC")]
        #[case(PowerMethod::DC, "DC")]
        #[case(PowerMethod::TrixExpress, "TRIX_EXPRESS")]
        fn it_should_display_power_methods(#[case] input: PowerMethod, #[case] expected: &str) {
            assert_eq!(expected, input.to_string());
        }
    }
}
