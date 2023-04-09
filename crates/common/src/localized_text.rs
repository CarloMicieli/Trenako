//! the module includes everything related to localized texts

use crate::validation::Validator;
use std::collections::HashMap;
use strum_macros;
use strum_macros::{Display, EnumString};
use validator::{Validate, ValidationErrors};

/// It represents a multi-language text.
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct LocalizedText(HashMap<Language, String>);

impl LocalizedText {
    pub fn add_english(&mut self, label: Option<&String>) {
        if let Some(label) = label {
            self.0.insert(Language::English, label.to_owned());
        }
    }

    pub fn add_italian(&mut self, label: Option<&String>) {
        if let Some(label) = label {
            self.0.insert(Language::Italian, label.to_owned());
        }
    }

    /// Creates a new `LocalizedText` with an English label
    pub fn with_english(label: &str) -> Self {
        let mut labels = HashMap::new();
        labels.insert(Language::English, label.to_string());
        LocalizedText(labels)
    }

    /// Creates a new `LocalizedText` with an Italian label
    pub fn with_italian(label: &str) -> Self {
        let mut labels = HashMap::new();
        labels.insert(Language::Italian, label.to_string());
        LocalizedText(labels)
    }

    /// Returns the label in English, if exists
    pub fn english(&self) -> Option<&String> {
        self.0.get(&Language::English)
    }

    /// Returns the label in Italian, if exists
    pub fn italian(&self) -> Option<&String> {
        self.0.get(&Language::Italian)
    }
}

impl Validate for LocalizedText {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut validator = Validator::new();

        validator.validate_length_optional("en", None, Some(2500), self.english());
        validator.validate_length_optional("it", None, Some(2500), self.italian());

        validator.into()
    }
}

#[derive(Debug, Default)]
pub struct LocalizedTextBuilder {
    english: Option<String>,
    italian: Option<String>,
}

impl LocalizedTextBuilder {
    /// add an English label
    pub fn english_text(mut self, label: &str) -> Self {
        self.english = Some(label.to_string());
        self
    }

    /// add an Italian label
    pub fn italian_text(mut self, label: &str) -> Self {
        self.italian = Some(label.to_string());
        self
    }

    /// Build a new `LocalizedText` value
    pub fn build(self) -> LocalizedText {
        let mut values = HashMap::with_capacity(2);

        if let Some(english) = self.english {
            values.insert(Language::English, english);
        }

        if let Some(italian) = self.italian {
            values.insert(Language::Italian, italian);
        }

        LocalizedText(values)
    }
}

/// The languages supported by the application
#[derive(Debug, Eq, PartialEq, Hash, Display, EnumString, Serialize, Deserialize, Clone, Copy)]
pub enum Language {
    /// the English language
    #[strum(serialize = "en")]
    #[serde(rename = "en")]
    English,

    /// the Italian language
    #[strum(serialize = "it")]
    #[serde(rename = "it")]
    Italian,
}

#[cfg(test)]
mod test {
    use super::*;

    mod languages {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use strum::ParseError;

        #[rstest]
        #[case(Language::English, "en")]
        #[case(Language::Italian, "it")]
        fn it_should_display_languages(#[case] language: Language, #[case] expected: &str) {
            assert_eq!(expected, language.to_string());
        }

        #[rstest]
        #[case("en", Ok(Language::English))]
        #[case("it", Ok(Language::Italian))]
        fn it_should_parse_languages(#[case] input: &str, #[case] expected: Result<Language, ParseError>) {
            let result = input.parse::<Language>();
            assert_eq!(expected, result);
        }
    }

    mod localize_texts {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_english_localized_texts() {
            let localized_text = LocalizedText::with_english("Hello world");
            assert_eq!(Some(&String::from("Hello world")), localized_text.english());
            assert_eq!(None, localized_text.italian());
        }

        #[test]
        fn it_should_create_italian_localized_texts() {
            let localized_text = LocalizedText::with_italian("Buongiorno");
            assert_eq!(Some(&String::from("Buongiorno")), localized_text.italian());
            assert_eq!(None, localized_text.english());
        }

        #[test]
        fn it_should_build_localize_texts() {
            let localized_text = LocalizedTextBuilder::default()
                .english_text("hello world")
                .italian_text("Buongiorno")
                .build();

            assert_eq!(Some(&String::from("Buongiorno")), localized_text.italian());
            assert_eq!(Some(&String::from("hello world")), localized_text.english());
        }
    }

    mod localize_texts_validation {
        use super::*;
        use crate::test_helpers::random_str;

        #[test]
        fn it_should_validate_localized_text() {
            let localized_text = LocalizedTextBuilder::default()
                .english_text("hello world")
                .italian_text("Buongiorno")
                .build();

            let result = localized_text.validate();
            assert!(result.is_ok());
        }

        #[test]
        fn it_should_validate_the_english_text() {
            let value = random_str(2501);
            let localized_text = LocalizedText::with_english(&value);

            let result = localized_text.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert!(errors.contains_key("en"));
            assert_eq!(errors["en"].len(), 1);
            assert_eq!(errors["en"][0].code, "length");
            assert_eq!(errors["en"][0].params["value"], value);
            assert_eq!(errors["en"][0].params["max"], 2500);
        }

        #[test]
        fn it_should_validate_the_italian_text() {
            let value = random_str(2501);
            let localized_text = LocalizedText::with_italian(&value);

            let result = localized_text.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert!(errors.contains_key("it"));
            assert_eq!(errors["it"].len(), 1);
            assert_eq!(errors["it"][0].code, "length");
            assert_eq!(errors["it"][0].params["value"], value);
            assert_eq!(errors["it"][0].params["max"], 2500);
        }
    }
}
