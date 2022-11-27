use strum_macros;
use strum_macros::{Display, EnumString};

/// The power methods for the model.
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString, Display)]
#[strum(ascii_case_insensitive)]
pub enum PowerMethod {
    /// Direct current.
    DC,

    /// Alternating current (Maerklin).
    AC,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod power_methods {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_parse_string_as_power_methods() {
            let pm = "AC".parse::<PowerMethod>();
            assert!(pm.is_ok());
            assert_eq!("AC", pm.unwrap().to_string());
        }
    }
}
