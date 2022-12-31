use sqlx::Type;
use std::fmt;
use std::fmt::Formatter;
use std::str;
use std::str::FromStr;
use thiserror::Error;
use url::Url;

/// It represents a website url
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Type)]
#[sqlx(transparent)]
pub struct WebsiteUrl(Url);

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
        Ok(WebsiteUrl(url))
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
}
