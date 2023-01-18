use slug::slugify;
use sqlx::Type;
use std::ops;
use std::str;
use std::str::FromStr;
use std::{convert, fmt};
use thiserror::Error;

/// A SEO friendly string
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Serialize, Deserialize, Type, Hash)]
#[sqlx(transparent)]
pub struct Slug(String);

impl Slug {
    /// Create a new Slug from the string in input.
    pub fn new(value: &str) -> Self {
        Slug::from_str(value).expect("the input is not a valid slug")
    }

    /// Combine this Slug with another value, after it is converted to a Slug
    pub fn combine<T: Into<Slug>>(&self, other: T) -> Self {
        let value = format!("{}-{}", self.0, other.into().0);
        Slug::new(&value)
    }
}

impl str::FromStr for Slug {
    type Err = SlugParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(SlugParserError::EmptyInput)
        } else {
            Ok(Slug(slugify(s)))
        }
    }
}

#[derive(Debug, Eq, PartialEq, Error)]
pub enum SlugParserError {
    #[error("a slug cannot be blank")]
    EmptyInput,
}

impl fmt::Display for Slug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ops::Deref for Slug {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl convert::From<String> for Slug {
    fn from(value: String) -> Self {
        Slug::new(&value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod slug_tests {
        use super::*;
        use pretty_assertions::assert_eq;
        use std::ops::Deref;

        #[test]
        fn it_should_create_slugs_from_strings() {
            let result = Slug::new("my first string");
            assert_eq!(result.to_string(), "my-first-string");
        }

        #[test]
        fn it_should_dereference_slugs() {
            let result = Slug::new("my first string");
            assert_eq!(result.deref(), "my-first-string");
        }

        #[test]
        #[should_panic(expected = "the input is not a valid slug")]
        fn it_should_panic_when_input_is_empty() {
            Slug::new("");
        }

        #[test]
        fn it_should_compose_a_slug_with_another_value() {
            let slug = Slug::new("Left value");
            let result = slug.combine(String::from("Right Value"));

            assert_eq!("left-value-right-value", result.to_string());
        }
    }
}
