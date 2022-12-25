use itertools::Itertools;
use std::fmt;
use std::fmt::Formatter;
use std::str;
use thiserror::Error;

/// The model railway industry adopted an 'Era', or 'Epoch' system; the idea being to group models
/// into a defined time bracket, so that locomotives, coaching and wagon stock could be reasonably
/// grouped together.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub enum Epoch {
    I,
    II,
    IIa,
    IIb,
    III,
    IIIa,
    IIIb,
    IV,
    IVa,
    IVb,
    V,
    Va,
    Vb,
    Vm,
    VI,
    Multiple(Box<Epoch>, Box<Epoch>),
}

impl str::FromStr for Epoch {
    type Err = EpochParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(EpochParseError::BlankValue);
        }

        if s.contains('/') {
            let tokens: Vec<&str> = s.split_terminator('/').sorted().dedup().collect();
            if tokens.len() == 2 {
                let first = Epoch::parse_str(tokens[0])?;
                let second = Epoch::parse_str(tokens[1])?;
                Ok(Epoch::Multiple(Box::new(first), Box::new(second)))
            } else {
                Err(EpochParseError::InvalidNumberOfValues)
            }
        } else {
            Epoch::parse_str(s)
        }
    }
}

#[derive(Error, Debug)]
pub enum EpochParseError {
    #[error("Epoch value cannot be blank")]
    BlankValue,
    #[error("Invalid number of elements for epoch values")]
    InvalidNumberOfValues,
    #[error("Invalid value for epoch")]
    InvalidValue,
}

impl Epoch {
    // Helper method to parse just the simple value
    fn parse_str(value: &str) -> Result<Self, EpochParseError> {
        match value {
            "I" => Ok(Epoch::I),
            "II" => Ok(Epoch::II),
            "IIa" => Ok(Epoch::IIa),
            "IIb" => Ok(Epoch::IIb),
            "III" => Ok(Epoch::III),
            "IIIa" => Ok(Epoch::IIIa),
            "IIIb" => Ok(Epoch::IIIb),
            "IV" => Ok(Epoch::IV),
            "IVa" => Ok(Epoch::IVa),
            "IVb" => Ok(Epoch::IVb),
            "V" => Ok(Epoch::V),
            "Va" => Ok(Epoch::Va),
            "Vb" => Ok(Epoch::Vb),
            "Vm" => Ok(Epoch::Vm),
            "VI" => Ok(Epoch::VI),
            _ => Err(EpochParseError::InvalidValue),
        }
    }
}

impl fmt::Display for Epoch {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Epoch::Multiple(ep1, ep2) => write!(f, "{}/{}", &ep1, &ep2),
            _ => write!(f, "{:?}", self),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod epochs {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_convert_string_slices_to_epochs() {
            let epoch = "I".parse::<Epoch>();
            assert!(epoch.is_ok());
            assert_eq!(epoch.unwrap(), Epoch::I);
        }

        #[test]
        fn it_should_convert_string_slices_to_mixed_epochs() {
            let epoch = "I/II".parse::<Epoch>();
            assert!(epoch.is_ok());
            assert_eq!(epoch.unwrap(), Epoch::Multiple(Box::new(Epoch::I), Box::new(Epoch::II)));
        }

        #[test]
        fn it_should_fail_to_convert_invalid_values_to_epochs() {
            let empty_epoch = "".parse::<Epoch>();
            assert!(empty_epoch.is_err());

            let invalid_epoch = "invalid".parse::<Epoch>();
            assert!(invalid_epoch.is_err());
        }

        #[test]
        #[allow(non_snake_case)]
        fn it_should_display_epoch_values() {
            let epoch_I_II = Epoch::Multiple(Box::new(Epoch::I), Box::new(Epoch::II));
            let epoch_IVa = Epoch::IVa;

            assert_eq!("I/II", epoch_I_II.to_string());
            assert_eq!("IVa", epoch_IVa.to_string());
        }
    }
}
