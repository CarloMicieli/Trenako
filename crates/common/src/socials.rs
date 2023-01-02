use http::Uri;
use sqlx::Type;
use std::fmt;
use std::fmt::Formatter;
use std::str;
use thiserror::Error;

/// the social profiles for an organization
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Socials {
    /// the facebook handler
    pub facebook: Option<Handler>,
    /// the instagram handler
    pub instagram: Option<Handler>,
    /// the linkedin handler
    pub linkedin: Option<Handler>,
    /// the twitter handler
    pub twitter: Option<Handler>,
    /// the youtube handler
    pub youtube: Option<Handler>,
}

impl Socials {
    /// the Facebook handler
    pub fn facebook(&self) -> Option<&Handler> {
        self.facebook.as_ref()
    }

    /// the Instagram handler
    pub fn instagram(&self) -> Option<&Handler> {
        self.instagram.as_ref()
    }

    /// the Linkedin handler
    pub fn linkedin(&self) -> Option<&Handler> {
        self.linkedin.as_ref()
    }

    /// the Twitter handler
    pub fn twitter(&self) -> Option<&Handler> {
        self.twitter.as_ref()
    }

    /// the Youtube handler
    pub fn youtube(&self) -> Option<&Handler> {
        self.youtube.as_ref()
    }

    /// Returns a socials builder
    pub fn builder() -> SocialsBuilder {
        SocialsBuilder::default()
    }
}

#[derive(Default)]
pub struct SocialsBuilder {
    facebook: Option<Result<Handler, SocialHandlerError>>,
    instagram: Option<Result<Handler, SocialHandlerError>>,
    linkedin: Option<Result<Handler, SocialHandlerError>>,
    twitter: Option<Result<Handler, SocialHandlerError>>,
    youtube: Option<Result<Handler, SocialHandlerError>>,
}

impl SocialsBuilder {
    /// with a facebook handler
    pub fn facebook(mut self, facebook_handler: &str) -> SocialsBuilder {
        self.facebook = Some(Handler::try_from(facebook_handler));
        self
    }

    /// with an instagram handler
    pub fn instagram(mut self, instagram_handler: &str) -> SocialsBuilder {
        self.instagram = Some(Handler::try_from(instagram_handler));
        self
    }

    /// with a linkedin handler
    pub fn linkedin(mut self, linkedin_handler: &str) -> SocialsBuilder {
        self.linkedin = Some(Handler::try_from(linkedin_handler));
        self
    }

    /// with a twitter handler
    pub fn twitter(mut self, twitter_handler: &str) -> SocialsBuilder {
        self.twitter = Some(Handler::try_from(twitter_handler));
        self
    }

    /// with a youtube handler
    pub fn youtube(mut self, youtube_handler: &str) -> SocialsBuilder {
        self.youtube = Some(Handler::try_from(youtube_handler));
        self
    }

    pub fn build(self) -> Result<Socials, SocialsBuilderError> {
        let facebook = if let Some(f) = self.facebook { Some(f?) } else { None };
        let instagram = if let Some(i) = self.instagram { Some(i?) } else { None };
        let linkedin = if let Some(l) = self.linkedin { Some(l?) } else { None };
        let twitter = if let Some(t) = self.twitter { Some(t?) } else { None };
        let youtube = if let Some(y) = self.youtube { Some(y?) } else { None };

        Ok(Socials {
            facebook,
            instagram,
            linkedin,
            twitter,
            youtube,
        })
    }
}

#[derive(Debug, Eq, PartialEq, Error)]
pub enum SocialsBuilderError {
    #[error("invalid social handler(s)")]
    InvalidHandlers(#[from] SocialHandlerError),
}

/// A social network handler.
///
/// the value must be URL encoded.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, Type)]
#[sqlx(transparent)]
pub struct Handler(String);

impl Handler {
    /// Create a new social network handler
    pub fn new(value: &str) -> Self {
        Handler::try_from(value).expect("invalid social handler")
    }
}

impl TryFrom<&str> for Handler {
    type Error = SocialHandlerError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(SocialHandlerError::InvalidSocialHandler)
        } else {
            match value.parse::<Uri>() {
                Ok(_) => Ok(Handler(String::from(value))),
                Err(_) => Err(SocialHandlerError::InvalidUri),
            }
        }
    }
}

impl fmt::Display for Handler {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Eq, PartialEq, Error)]
pub enum SocialHandlerError {
    #[error("invalid social handler")]
    InvalidSocialHandler,
    #[error("invalid uri")]
    InvalidUri,
}

#[cfg(test)]
mod test {
    use super::*;

    mod socials {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;

        #[rstest]
        #[case("my_handler", Ok(Handler::new("my_handler")))]
        #[case("& invalid uri", Err(SocialHandlerError::InvalidUri))]
        fn it_should_validate_social_handler_input(
            #[case] input: &str,
            #[case] expected: Result<Handler, SocialHandlerError>,
        ) {
            let handler = Handler::try_from(input);
            assert_eq!(expected, handler);
        }

        #[test]
        fn it_should_create_new_social_handlers() {
            let handler = Handler::try_from("my_handler").unwrap();
            assert_eq!("my_handler", handler.to_string());
        }

        #[test]
        fn it_should_build_socials_value() {
            let social = Socials::builder()
                .facebook("facebook_user")
                .instagram("instagram_user")
                .linkedin("linkedin_user")
                .twitter("twitter_user")
                .youtube("youtube_user")
                .build()
                .unwrap();

            assert_eq!(&Handler("facebook_user".to_string()), social.facebook().unwrap());
            assert_eq!(&Handler("instagram_user".to_string()), social.instagram().unwrap());
            assert_eq!(&Handler("linkedin_user".to_string()), social.linkedin().unwrap());
            assert_eq!(&Handler("twitter_user".to_string()), social.twitter().unwrap());
            assert_eq!(&Handler("youtube_user".to_string()), social.youtube().unwrap());
        }
    }
}
