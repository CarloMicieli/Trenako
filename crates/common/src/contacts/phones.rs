use sqlx::Type;
use std::fmt;
use std::fmt::Formatter;
use std::str;
use std::str::FromStr;
use thiserror::Error;

/// It represents a phone number
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Type)]
#[sqlx(transparent)]
pub struct PhoneNumber(String);

impl PhoneNumber {
    pub fn new(phone: &str) -> Self {
        PhoneNumber::from_str(phone).unwrap()
    }
}

impl fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl str::FromStr for PhoneNumber {
    type Err = PhoneParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(PhoneParsingError::InvalidPhone)
        } else {
            Ok(PhoneNumber(String::from(s)))
        }
    }
}

#[derive(Debug, Eq, PartialEq, Error)]
pub enum PhoneParsingError {
    #[error("invalid phone number")]
    InvalidPhone,
}

#[cfg(test)]
mod tests {
    use super::*;
    mod phone_numbers {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_phone_numbers() {
            let phone_number = PhoneNumber::from_str("555-123456").unwrap();
            assert_eq!("555-123456", phone_number.to_string());
        }
    }
}
