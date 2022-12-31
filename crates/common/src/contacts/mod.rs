mod emails;
mod phones;
mod website_urls;

use crate::contacts::emails::MailAddressParsingError;
use crate::contacts::phones::PhoneParsingError;
use crate::contacts::website_urls::WebsiteUrlParsingError;
use std::str::FromStr;
use thiserror::Error;

pub use emails::MailAddress;
pub use phones::PhoneNumber;
pub use website_urls::WebsiteUrl;

/// The contact information provides the means to communicate with an organization.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct ContactInformation {
    /// the email address
    pub email: Option<MailAddress>,

    /// the phone number
    pub phone: Option<PhoneNumber>,

    /// the website url
    pub website_url: Option<WebsiteUrl>,
}

impl ContactInformation {
    pub fn new(email: Option<MailAddress>, website_url: Option<WebsiteUrl>, phone: Option<PhoneNumber>) -> Self {
        ContactInformation {
            email,
            website_url,
            phone,
        }
    }

    /// Returns the mail address for this contact info
    pub fn email(&self) -> Option<&MailAddress> {
        self.email.as_ref()
    }

    /// Returns the phone number for this contact info
    pub fn phone(&self) -> Option<&PhoneNumber> {
        self.phone.as_ref()
    }

    /// Returns the website url for this contact info
    pub fn website_url(&self) -> Option<&WebsiteUrl> {
        self.website_url.as_ref()
    }

    /// Creates a new contact info builder
    pub fn builder() -> ContactInformationBuilder {
        ContactInformationBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct ContactInformationBuilder {
    email: Option<String>,
    phone: Option<String>,
    website_url: Option<String>,
}

impl ContactInformationBuilder {
    /// Set the mail address
    pub fn email(mut self, mail_address: &str) -> Self {
        self.email = Some(mail_address).map(String::from);
        self
    }

    /// Set the phone number
    pub fn phone(mut self, phone: &str) -> Self {
        self.phone = Some(phone).map(String::from);
        self
    }

    /// Set the website url
    pub fn website_url(mut self, website_url: &str) -> Self {
        self.website_url = Some(website_url).map(String::from);
        self
    }

    pub fn build(self) -> Result<ContactInformation, ContactInformationError> {
        let email = if let Some(email) = self.email {
            Some(MailAddress::from_str(&email)?)
        } else {
            None
        };

        let phone = if let Some(phone) = self.phone {
            Some(PhoneNumber::from_str(&phone)?)
        } else {
            None
        };

        let website_url = if let Some(website_url) = self.website_url {
            Some(WebsiteUrl::from_str(&website_url)?)
        } else {
            None
        };

        Ok(ContactInformation {
            email,
            phone,
            website_url,
        })
    }
}

#[derive(Debug, Eq, PartialEq, Error)]
pub enum ContactInformationError {
    #[error("invalid mail address")]
    InvalidEmail(#[from] MailAddressParsingError),
    #[error("invalid phone number")]
    InvalidPhoneNumber(#[from] PhoneParsingError),
    #[error("invalid website url")]
    InvalidWebsiteUrl(#[from] WebsiteUrlParsingError),
}

#[cfg(test)]
mod tests {
    use super::*;

    mod contacts {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_contact_info() {
            let email = MailAddress::from_str("mail@mail.com").unwrap();
            let website_url = WebsiteUrl::new("http://www.website.com");
            let phone = PhoneNumber::from_str("+15551234").unwrap();

            let contact_info =
                ContactInformation::new(Some(email.clone()), Some(website_url.clone()), Some(phone.clone()));

            assert_eq!(Some(&email), contact_info.email());
            assert_eq!(Some(&phone), contact_info.phone());
            assert_eq!(Some(&website_url), contact_info.website_url());
        }
    }

    mod contact_info_builder {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;

        #[rstest]
        #[case(
            "",
            "+555 1234",
            "http://www.ebsite.com",
            Err(ContactInformationError::InvalidEmail(MailAddressParsingError::InvalidMailAddress))
        )]
        #[case(
            "mail@mail.com",
            "",
            "http://www.ebsite.com",
            Err(ContactInformationError::InvalidPhoneNumber(PhoneParsingError::InvalidPhone))
        )]
        #[case(
            "mail@mail.com",
            "+555 1234",
            "",
            Err(
                ContactInformationError::InvalidWebsiteUrl(WebsiteUrlParsingError::InvalidWebsiteUrl(
                    url::ParseError::RelativeUrlWithoutBase
                ))
            )
        )]
        fn it_should_validate_input_building_contact_info(
            #[case] email: &str,
            #[case] phone: &str,
            #[case] website_url: &str,
            #[case] expected: Result<ContactInformation, ContactInformationError>,
        ) {
            let result = ContactInformation::builder()
                .email(email)
                .phone(phone)
                .website_url(website_url)
                .build();
            assert_eq!(expected, result);
        }
    }
}
