//! the module includes everything related to website urls

use sqlx::Type;
use std::borrow::Cow;
use std::fmt;
use std::fmt::Formatter;
use std::str;
use std::str::FromStr;
use thiserror::Error;
use url::Url;
use validator::{ValidateLength, ValidateUrl, ValidationError};

/// It represents a website url
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Type)]
#[sqlx(transparent)]
pub struct WebsiteUrl(String);

pub fn validate_website_url(input: &WebsiteUrl) -> Result<(), ValidationError> {
    if input.0.validate_url() {
        Ok(())
    } else {
        let mut error = ValidationError::new("url");
        error.add_param(Cow::from("value"), &input.0);
        Err(error)
    }
}

pub fn validate_website_url_length(input: &WebsiteUrl) -> Result<(), ValidationError> {
    if input.0.validate_length(None, Some(100), None) {
        Ok(())
    } else {
        let mut error = ValidationError::new("length");
        error.add_param(Cow::from("max"), &Some(100));
        error.add_param(Cow::from("value"), &input.0);
        Err(error)
    }
}

impl WebsiteUrl {
    /// Create a new website url
    pub fn new(value: &str) -> WebsiteUrl {
        WebsiteUrl::from_str(value).unwrap()
    }
}

impl fmt::Display for WebsiteUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl str::FromStr for WebsiteUrl {
    type Err = WebsiteUrlParsingError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let url = Url::parse(value)?;
        Ok(WebsiteUrl(url.to_string()))
    }
}

#[derive(Debug, Eq, PartialEq, Error)]
pub enum WebsiteUrlParsingError {
    #[error("invalid website url")]
    InvalidWebsiteUrl(#[from] url::ParseError),
}

#[cfg(test)]
mod tests {
    use super::*;

    mod website_urls {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_convert_str_to_website_urls() {
            let result = WebsiteUrl::from_str("http://www.website.com");
            assert!(result.is_ok());
            assert_eq!("http://www.website.com/", result.unwrap().to_string());
        }
    }

    mod website_urls_validation {
        use super::*;
        use crate::test_helpers::random_str;

        #[test]
        fn it_should_validate_website_url() {
            let input = WebsiteUrl(String::from("invalid url"));

            let result = validate_website_url(&input);

            assert!(result.is_err());

            let error = result.unwrap_err();
            assert_eq!(error.code, "url");
            assert_eq!(error.params["value"], String::from("invalid url"));
        }

        #[test]
        fn it_should_validate_website_url_length() {
            let website_url = random_str(101);
            let input = WebsiteUrl(website_url.clone());

            let result = validate_website_url_length(&input);

            assert!(result.is_err());

            let error = result.unwrap_err();
            assert_eq!(error.code, "length");
            assert_eq!(error.params["value"], website_url);
            assert_eq!(error.params["max"], 100);
        }
    }
}
