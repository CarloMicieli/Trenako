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

    /// The model has no dcc support (like no standard decoder plug)
    NoDcc,
}

/// NMRA and NEM Connectors for digital control (DCC)
///
/// The NMRA and NEM adopted standard mechanical and electrical interfaces to connect Multifunction
/// Decoders to a locomotive's electrical system. These plugs and sockets make it simpler to install
/// a decoder into a suitably equipped locomotive.
///
/// In many cases a blanking plug must be removed before installing the decoder. If a locomotive
/// is not DCC-Ready it will lack an interface and must use a Hardwired Decoder or a drop-in
/// replacement DCC control board (if available) for that specific model.
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString, Display)]
#[strum(ascii_case_insensitive)]
pub enum DccInterface {
    /// 6 Pin standard mechanical and electrical interfaces (NMRA Small)
    #[strum(serialize = "nem_651")]
    Nem651,

    /// 8 Pin standard mechanical and electrical interfaces (NMRA Medium)
    #[strum(serialize = "nem_652")]
    Nem652,

    /// 4 Pin standard mechanical and electrical interfaces (NMRA Large)
    #[strum(serialize = "nem_654")]
    Nem654,

    /// The PluX8 connector consists of two rows of 4 pins.
    #[strum(serialize = "plux_8")]
    Plux8,

    #[strum(serialize = "plux_12")]
    Plux12,

    /// The PluX16 connector consists of two rows of 8 pins.
    #[strum(serialize = "plux_16")]
    Plux16,

    /// The PluX22 connector consists of two rows of 11 pins.
    #[strum(serialize = "plux_22")]
    Plux22,

    /// standard connector for extremely tight applications, such as TT and N scale locomotives (NEM 662)
    #[strum(serialize = "next_18")]
    Next18,

    #[strum(serialize = "next_18_s")]
    Next18S,

    /// 21MTC Connector interface is a standard adopted by both the NMRA and NEM (NEM 660).
    /// Its name comes from 21 pin Marklin/Trix Connector, developed by Marklin and ESU.
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
