use sqlx::Type;
use std::fmt;
use std::fmt::Formatter;
use std::str;
use std::str::FromStr;
use thiserror::Error;

/// It represents a mail address
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Type)]
#[sqlx(transparent)]
pub struct MailAddress(String);

impl MailAddress {
    pub fn new(mail_address: &str) -> Self {
        MailAddress::from_str(mail_address).unwrap()
    }
}

#[derive(Debug, Eq, PartialEq, Error)]
pub enum MailAddressParsingError {
    #[error("invalid email address")]
    InvalidMailAddress,
}

impl fmt::Display for MailAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl str::FromStr for MailAddress {
    type Err = MailAddressParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(MailAddressParsingError::InvalidMailAddress)
        } else {
            Ok(MailAddress(String::from(s)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod mail_addresses {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_mail_addresses() {
            let mail_address = MailAddress::from_str("mail@mail.com").unwrap();
            assert_eq!("mail@mail.com", mail_address.to_string());
        }
    }
}
