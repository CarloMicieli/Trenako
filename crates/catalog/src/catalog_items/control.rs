use strum_macros;
use strum_macros::{Display, EnumString};

/// The control method for this railway model.
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
#[strum(ascii_case_insensitive)]
pub enum Control {
    /// The model can be fitted with a dcc decoder.
    DccReady,

    /// The model has a dcc decoder installed.
    Dcc,

    /// The model has a dcc decoder installed with the sound module.
    DccSound,

    NoDcc,
}

/// NMRA and NEM Connectors for digital control (DCC)
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString, Display)]
#[strum(ascii_case_insensitive)]
pub enum DccInterface {
    #[strum(serialize = "nem_651")]
    Nem651,
    #[strum(serialize = "nem_652")]
    Nem652,
    #[strum(serialize = "plux_8")]
    Plux8,
    #[strum(serialize = "plux_16")]
    Plux16,
    #[strum(serialize = "plux_22")]
    Plux22,
    #[strum(serialize = "next_18")]
    Next18,
    #[strum(serialize = "mtc_21")]
    Mtc21,
}

impl Control {
    pub fn with_decoder(&self) -> bool {
        *self == Control::Dcc || *self == Control::DccSound
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod controls {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_parse_string_as_controls() {
            let c = "DCC_READY".parse::<Control>();
            assert!(c.is_ok());
            assert_eq!(c.unwrap(), Control::DccReady);
        }

        #[test]
        fn it_should_fail_to_parse_invalid_value_as_controls() {
            let blank = "".parse::<Control>();
            assert!(blank.is_err());

            let invalid = "invalid".parse::<Control>();
            assert!(invalid.is_err());
        }

        #[test]
        fn it_should_display_controls() {
            let c = Control::DccReady;
            assert_eq!("dcc_ready", c.to_string());
        }
    }

    mod dcc_interfaces {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_parse_string_as_dcc_interfaces() {
            let dcc = "NEM_652".parse::<DccInterface>();
            assert!(dcc.is_ok());
            assert_eq!(dcc.unwrap(), DccInterface::Nem652);
        }

        #[test]
        fn it_should_fail_to_parse_invalid_string_as_dcc_interfaces() {
            let blank = "".parse::<DccInterface>();
            assert!(blank.is_err());

            let invalid = "invalid".parse::<DccInterface>();
            assert!(invalid.is_err());
        }

        #[test]
        fn it_should_display_dcc_interfaces() {
            let dcc = DccInterface::Nem652;
            assert_eq!("nem_652", dcc.to_string());
        }
    }
}
