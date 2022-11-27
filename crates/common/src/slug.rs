use slug::slugify;
use std::fmt;
use std::ops;
use std::str;

/// A SEO friendly string
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct Slug(String);

impl Slug {
    /// Create a new Slug from the string slice in input.
    pub fn new(value: &str) -> Self {
        if value.is_empty() {
            panic!("A slug cannot be empty")
        }
        Slug(slugify(value))
    }
}

impl str::FromStr for Slug {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(())
        } else {
            Ok(Slug(slugify(s)))
        }
    }
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
        #[should_panic(expected = "A slug cannot be empty")]
        fn it_should_panic_when_input_is_empty() {
            Slug::new("");
        }
    }
}
