use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;
use url::Url;

/// It represents a mail address
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MailAddress(String);

impl MailAddress {
    pub fn new(mail_address: &str) -> Self {
        MailAddress::from_str(mail_address).unwrap()
    }
}

impl fmt::Display for MailAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for MailAddress {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MailAddress(String::from(s)))
    }
}

/// It represents a phone number
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PhoneNumber(String);

impl PhoneNumber {
    pub fn new(phone: &str) -> Self {
        PhoneNumber::from_str(phone).unwrap()
    }
}

impl fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for PhoneNumber {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(PhoneNumber(String::from(s)))
    }
}

/// It represents a website url
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct WebsiteUrl(Url);

impl WebsiteUrl {
    /// Create a new website url
    pub fn new(value: &str) -> WebsiteUrl {
        let url: Url = Url::parse(value).expect("invalid url");
        WebsiteUrl(url)
    }
}

impl fmt::Display for WebsiteUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for WebsiteUrl {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let url = Url::parse(value).map_err(|_e| ())?;
        Ok(WebsiteUrl(url))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ContactInfo {
    email: Option<MailAddress>,
    website_url: Option<WebsiteUrl>,
    phone: Option<PhoneNumber>,
}

impl ContactInfo {
    pub fn new(email: Option<MailAddress>, website_url: Option<WebsiteUrl>, phone: Option<PhoneNumber>) -> Self {
        ContactInfo {
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

            let contact_info = ContactInfo::new(Some(email.clone()), Some(website_url.clone()), Some(phone.clone()));

            assert_eq!(Some(&email), contact_info.email());
            assert_eq!(Some(&phone), contact_info.phone());
            assert_eq!(Some(&website_url), contact_info.website_url());
        }
    }

    mod mail_addresses {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_mail_addresses() {
            let mail_address = MailAddress::from_str("mail@mail.com").unwrap();
            assert_eq!("mail@mail.com", mail_address.to_string());
        }
    }

    mod phone_numbers {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_phone_numbers() {
            let phone_number = PhoneNumber::from_str("555-123456").unwrap();
            assert_eq!("555-123456", phone_number.to_string());
        }
    }

    mod website_urls {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_convert_str_to_website_urls() {
            let result: Result<WebsiteUrl, ()> = "http://www.website.com".try_into();
            assert!(result.is_ok());
            assert_eq!("http://www.website.com/", result.unwrap().to_string());
        }
    }
}
