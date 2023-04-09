//! the rolling stock control

use sqlx::Type;
use strum_macros;
use strum_macros::{Display, EnumString};

/// The control method for this railway model.
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString, Display, Serialize, Deserialize, Type)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[sqlx(type_name = "control", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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

/// The NMRA and NEM Connectors for digital control (DCC)
///
/// # Description
/// The NMRA and NEM adopted standard mechanical and electrical interfaces to connect Multifunction
/// Decoders to a locomotive's electrical system. These plugs and sockets make it simpler to install
/// a decoder into a suitably equipped locomotive.
///
/// In many cases a blanking plug must be removed before installing the decoder. If a locomotive
/// is not DCC-Ready it will lack an interface and must use a Hardwired Decoder or a drop-in
/// replacement DCC control board (if available) for that specific model.
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString, Display, Serialize, Deserialize, Type)]
#[strum(ascii_case_insensitive)]
#[sqlx(type_name = "dcc_interface")]
pub enum DccInterface {
    /// 6 Pin standard mechanical and electrical interfaces (NMRA Small)
    #[serde(rename = "NEM_651")]
    #[strum(serialize = "NEM_651")]
    #[sqlx(rename = "NEM_651")]
    Nem651,

    /// 8 Pin standard mechanical and electrical interfaces (NMRA Medium)
    #[serde(rename = "NEM_652")]
    #[strum(serialize = "NEM_652")]
    #[sqlx(rename = "NEM_652")]
    Nem652,

    /// 4 Pin standard mechanical and electrical interfaces (NMRA Large)
    #[serde(rename = "NEM_654")]
    #[strum(serialize = "NEM_654")]
    #[sqlx(rename = "NEM_654")]
    Nem654,

    /// The PluX8 connector consists of two rows of 4 pins.
    #[serde(rename = "PLUX_8")]
    #[strum(serialize = "PLUX_8")]
    #[sqlx(rename = "PLUX_8")]
    Plux8,

    #[serde(rename = "PLUX_12")]
    #[strum(serialize = "PLUX_12")]
    #[sqlx(rename = "PLUX_12")]
    Plux12,

    /// The PluX16 connector consists of two rows of 8 pins.
    #[serde(rename = "PLUX_16")]
    #[strum(serialize = "PLUX_16")]
    #[sqlx(rename = "PLUX_16")]
    Plux16,

    /// The PluX22 connector consists of two rows of 11 pins.
    #[serde(rename = "PLUX_22")]
    #[strum(serialize = "PLUX_22")]
    #[sqlx(rename = "PLUX_22")]
    Plux22,

    /// standard connector for extremely tight applications, such as TT and N scale locomotives (NEM 662)
    #[serde(rename = "NEXT_18")]
    #[strum(serialize = "NEXT_18")]
    #[sqlx(rename = "NEXT_18")]
    Next18,

    #[serde(rename = "NEXT_18_S")]
    #[strum(serialize = "NEXT_18_S")]
    #[sqlx(rename = "NEXT_18_S")]
    Next18S,

    /// 21MTC Connector interface is a standard adopted by both the NMRA and NEM (NEM 660).
    /// Its name comes from 21 pin Marklin/Trix Connector, developed by Marklin and ESU.
    #[serde(rename = "MTC_21")]
    #[strum(serialize = "MTC_21")]
    #[sqlx(rename = "MTC_21")]
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
        use rstest::rstest;
        use strum::ParseError;

        #[rstest]
        #[case("DCC", Ok(Control::Dcc))]
        #[case("DCC_READY", Ok(Control::DccReady))]
        #[case("DCC_SOUND", Ok(Control::DccSound))]
        #[case("NO_DCC", Ok(Control::NoDcc))]
        #[case("invalid", Err(ParseError::VariantNotFound))]
        fn it_should_parse_string_as_controls(#[case] input: &str, #[case] expected: Result<Control, ParseError>) {
            let c = input.parse::<Control>();
            assert_eq!(expected, c);
        }

        #[rstest]
        #[case(Control::Dcc, "DCC")]
        #[case(Control::DccReady, "DCC_READY")]
        #[case(Control::DccSound, "DCC_SOUND")]
        #[case(Control::NoDcc, "NO_DCC")]
        fn it_should_display_controls(#[case] input: Control, #[case] expected: &str) {
            assert_eq!(expected, input.to_string());
        }

        #[rstest]
        #[case(Control::Dcc, true)]
        #[case(Control::DccReady, false)]
        #[case(Control::DccSound, true)]
        #[case(Control::NoDcc, false)]
        fn it_should_check_for_the_dcc_decoder_presence(#[case] input: Control, #[case] expected: bool) {
            assert_eq!(expected, input.with_decoder());
        }
    }

    mod dcc_interfaces {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use strum::ParseError;

        #[rstest]
        #[case("NEM_651", Ok(DccInterface::Nem651))]
        #[case("NEM_652", Ok(DccInterface::Nem652))]
        #[case("NEM_654", Ok(DccInterface::Nem654))]
        #[case("PLUX_8", Ok(DccInterface::Plux8))]
        #[case("PLUX_12", Ok(DccInterface::Plux12))]
        #[case("PLUX_16", Ok(DccInterface::Plux16))]
        #[case("PLUX_22", Ok(DccInterface::Plux22))]
        #[case("NEXT_18", Ok(DccInterface::Next18))]
        #[case("NEXT_18_S", Ok(DccInterface::Next18S))]
        #[case("MTC_21", Ok(DccInterface::Mtc21))]
        #[case("invalid", Err(ParseError::VariantNotFound))]
        fn it_should_parse_string_as_dcc_interfaces(
            #[case] input: &str,
            #[case] expected: Result<DccInterface, ParseError>,
        ) {
            let dcc = input.parse::<DccInterface>();
            assert_eq!(expected, dcc);
        }

        #[rstest]
        #[case(DccInterface::Nem651, "NEM_651")]
        #[case(DccInterface::Nem652, "NEM_652")]
        #[case(DccInterface::Nem654, "NEM_654")]
        #[case(DccInterface::Plux8, "PLUX_8")]
        #[case(DccInterface::Plux12, "PLUX_12")]
        #[case(DccInterface::Plux16, "PLUX_16")]
        #[case(DccInterface::Plux22, "PLUX_22")]
        #[case(DccInterface::Next18, "NEXT_18")]
        #[case(DccInterface::Next18S, "NEXT_18_S")]
        #[case(DccInterface::Mtc21, "MTC_21")]
        fn it_should_display_dcc_interfaces(#[case] input: DccInterface, #[case] expected: &str) {
            assert_eq!(expected, input.to_string());
        }
    }
}
