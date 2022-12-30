use sqlx::Type;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Socials {
    facebook: Option<Handler>,
    instagram: Option<Handler>,
    linkedin: Option<Handler>,
    twitter: Option<Handler>,
    youtube: Option<Handler>,
}

impl Socials {
    /// The Facebook handler
    pub fn facebook(&self) -> Option<&Handler> {
        self.facebook.as_ref()
    }

    /// The Instagram handler
    pub fn instagram(&self) -> Option<&Handler> {
        self.instagram.as_ref()
    }

    /// The Linkedin handler
    pub fn linkedin(&self) -> Option<&Handler> {
        self.linkedin.as_ref()
    }

    /// The Twitter handler
    pub fn twitter(&self) -> Option<&Handler> {
        self.twitter.as_ref()
    }

    /// The Youtube handler
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
    facebook: Option<Handler>,
    instagram: Option<Handler>,
    linkedin: Option<Handler>,
    twitter: Option<Handler>,
    youtube: Option<Handler>,
}

impl SocialsBuilder {
    pub fn facebook(mut self, facebook_handler: &str) -> SocialsBuilder {
        self.facebook = Handler::try_from(facebook_handler).ok();
        self
    }

    pub fn instagram(mut self, instagram_handler: &str) -> SocialsBuilder {
        self.instagram = Handler::try_from(instagram_handler).ok();
        self
    }

    pub fn linkedin(mut self, linkedin_handler: &str) -> SocialsBuilder {
        self.linkedin = Handler::try_from(linkedin_handler).ok();
        self
    }

    pub fn twitter(mut self, twitter_handler: &str) -> SocialsBuilder {
        self.twitter = Handler::try_from(twitter_handler).ok();
        self
    }

    pub fn youtube(mut self, youtube_handler: &str) -> SocialsBuilder {
        self.youtube = Handler::try_from(youtube_handler).ok();
        self
    }

    pub fn build(self) -> Socials {
        Socials {
            facebook: self.facebook,
            instagram: self.instagram,
            linkedin: self.linkedin,
            twitter: self.twitter,
            youtube: self.youtube,
        }
    }
}

/// A social network handler.
///
/// The value must be URL encoded.
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
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(())
        } else {
            Ok(Handler(String::from(value)))
        }
    }
}

impl fmt::Display for Handler {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod socials {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_new_social_handlers() {
            let handler = Handler::try_from("my_handler").unwrap();
            assert_eq!("my_handler", handler.to_string());
        }

        #[test]
        fn it_should_create_socials_value() {
            let social = Socials::builder()
                .facebook("facebook_user")
                .instagram("instagram_user")
                .linkedin("linkedin_user")
                .twitter("twitter_user")
                .youtube("youtube_user")
                .build();

            assert_eq!(&Handler("facebook_user".to_string()), social.facebook().unwrap());
            assert_eq!(&Handler("instagram_user".to_string()), social.instagram().unwrap());
            assert_eq!(&Handler("linkedin_user".to_string()), social.linkedin().unwrap());
            assert_eq!(&Handler("twitter_user".to_string()), social.twitter().unwrap());
            assert_eq!(&Handler("youtube_user".to_string()), social.youtube().unwrap());
        }
    }
}
