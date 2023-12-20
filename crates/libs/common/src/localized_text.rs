//! the module includes everything related to localized texts

use crate::validation::Validator;
use strum_macros;
use strum_macros::{Display, EnumString};
use validator::{Validate, ValidationErrors};

/// It represents a multi-language text.
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct LocalizedText {
    /// the text in German
    pub de: Option<String>,
    /// the text in English
    pub en: Option<String>,
    /// the text in French
    pub fr: Option<String>,
    /// the text in Italian
    pub it: Option<String>,
}

impl LocalizedText {
    /// Add a new label for the English language
    pub fn add_english(&mut self, label: Option<&String>) {
        if let Some(label) = label {
            self.en = Some(label.to_owned());
        }
    }

    /// Add a new label for the Italian language
    pub fn add_italian(&mut self, label: Option<&String>) {
        if let Some(label) = label {
            self.it = Some(label.to_owned());
        }
    }

    pub fn insert(&mut self, language: Language, label: Option<&String>) {
        if let Some(label) = label {
            match language {
                Language::English => self.en = Some(label.to_owned()),
                Language::French => self.fr = Some(label.to_owned()),
                Language::German => self.de = Some(label.to_owned()),
                Language::Italian => self.it = Some(label.to_owned()),
            }
        }
    }

    /// Creates a new `LocalizedText` with an English label
    pub fn with_english(label: &str) -> Self {
        LocalizedText {
            en: Some(label.to_string()),
            ..Default::default()
        }
    }

    /// Creates a new `LocalizedText` with an Italian label
    pub fn with_italian(label: &str) -> Self {
        LocalizedText {
            it: Some(label.to_string()),
            ..Default::default()
        }
    }

    /// Creates a new `LocalizedText` with a French label
    pub fn with_french(label: &str) -> Self {
        LocalizedText {
            fr: Some(label.to_string()),
            ..Default::default()
        }
    }

    /// Creates a new `LocalizedText` with a German label
    pub fn with_german(label: &str) -> Self {
        LocalizedText {
            de: Some(label.to_string()),
            ..Default::default()
        }
    }

    /// Returns the label in English, if exists
    pub fn english(&self) -> Option<&String> {
        self.en.as_ref()
    }

    /// Returns the label in Italian, if exists
    pub fn italian(&self) -> Option<&String> {
        self.it.as_ref()
    }

    /// Returns the label in French, if exists
    pub fn french(&self) -> Option<&String> {
        self.fr.as_ref()
    }

    /// Returns the label in German, if exists
    pub fn german(&self) -> Option<&String> {
        self.de.as_ref()
    }
}

impl Validate for LocalizedText {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut validator = Validator::new();

        validator.validate_length_optional("en", None, Some(2500), self.english());
        validator.validate_length_optional("it", None, Some(2500), self.italian());
        validator.validate_length_optional("fr", None, Some(2500), self.french());
        validator.validate_length_optional("de", None, Some(2500), self.german());

        validator.into()
    }
}

#[derive(Debug, Default)]
pub struct LocalizedTextBuilder {
    english: Option<String>,
    italian: Option<String>,
    french: Option<String>,
    german: Option<String>,
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

    /// add a French label
    pub fn french_text(mut self, label: &str) -> Self {
        self.french = Some(label.to_string());
        self
    }

    /// add a German label
    pub fn german_text(mut self, label: &str) -> Self {
        self.german = Some(label.to_string());
        self
    }

    /// Build a new `LocalizedText` value
    pub fn build(self) -> LocalizedText {
        let mut localized_text = LocalizedText::default();
        localized_text.insert(Language::English, self.english.as_ref());
        localized_text.insert(Language::French, self.french.as_ref());
        localized_text.insert(Language::German, self.german.as_ref());
        localized_text.insert(Language::Italian, self.italian.as_ref());
        localized_text
    }
}

/// The languages supported by the application
#[derive(Debug, Eq, PartialEq, Hash, Display, EnumString, Serialize, Deserialize, Clone, Copy)]
pub enum Language {
    /// the French language
    #[strum(serialize = "fr")]
    #[serde(rename = "fr")]
    French,

    /// the English language
    #[strum(serialize = "en")]
    #[serde(rename = "en")]
    English,

    /// the German language
    #[strum(serialize = "de")]
    #[serde(rename = "de")]
    German,

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
        #[case(Language::French, "fr")]
        #[case(Language::German, "de")]
        #[case(Language::Italian, "it")]
        fn it_should_display_languages(#[case] language: Language, #[case] expected: &str) {
            assert_eq!(expected, language.to_string());
        }

        #[rstest]
        #[case("de", Ok(Language::German))]
        #[case("en", Ok(Language::English))]
        #[case("fr", Ok(Language::French))]
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
            assert_eq!(None, localized_text.french());
            assert_eq!(None, localized_text.german());
            assert_eq!(None, localized_text.italian());
        }

        #[test]
        fn it_should_create_italian_localized_texts() {
            let localized_text = LocalizedText::with_italian("Buongiorno");
            assert_eq!(Some(&String::from("Buongiorno")), localized_text.italian());
            assert_eq!(None, localized_text.english());
            assert_eq!(None, localized_text.french());
            assert_eq!(None, localized_text.german());
        }

        #[test]
        fn it_should_create_french_localized_texts() {
            let localized_text = LocalizedText::with_french("Bonjour");
            assert_eq!(Some(&String::from("Bonjour")), localized_text.french());
            assert_eq!(None, localized_text.english());
            assert_eq!(None, localized_text.german());
            assert_eq!(None, localized_text.italian());
        }

        #[test]
        fn it_should_create_german_localized_texts() {
            let localized_text = LocalizedText::with_german("Guten Morgen");
            assert_eq!(Some(&String::from("Guten Morgen")), localized_text.german());
            assert_eq!(None, localized_text.english());
            assert_eq!(None, localized_text.french());
            assert_eq!(None, localized_text.italian());
        }

        #[test]
        fn it_should_build_localize_texts() {
            let localized_text = LocalizedTextBuilder::default()
                .english_text("Good Morning")
                .french_text("Bonjour")
                .german_text("Guten Morgen")
                .italian_text("Buongiorno")
                .build();

            assert_eq!(Some(&String::from("Buongiorno")), localized_text.italian());
            assert_eq!(Some(&String::from("Good Morning")), localized_text.english());
            assert_eq!(Some(&String::from("Guten Morgen")), localized_text.german());
            assert_eq!(Some(&String::from("Bonjour")), localized_text.french());
        }
    }

    mod localize_texts_validation {
        use super::*;
        use crate::test_helpers::random_str;

        #[test]
        fn it_should_validate_localized_text() {
            let localized_text = LocalizedTextBuilder::default()
                .english_text("Good Morning")
                .french_text("Bonjour")
                .german_text("Guten Morgen")
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

        #[test]
        fn it_should_validate_the_french_text() {
            let value = random_str(2501);
            let localized_text = LocalizedText::with_french(&value);

            let result = localized_text.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert!(errors.contains_key("fr"));
            assert_eq!(errors["fr"].len(), 1);
            assert_eq!(errors["fr"][0].code, "length");
            assert_eq!(errors["fr"][0].params["value"], value);
            assert_eq!(errors["fr"][0].params["max"], 2500);
        }

        #[test]
        fn it_should_validate_the_german_text() {
            let value = random_str(2501);
            let localized_text = LocalizedText::with_german(&value);

            let result = localized_text.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert!(errors.contains_key("de"));
            assert_eq!(errors["de"].len(), 1);
            assert_eq!(errors["de"][0].code, "length");
            assert_eq!(errors["de"][0].params["value"], value);
            assert_eq!(errors["de"][0].params["max"], 2500);
        }
    }
}
