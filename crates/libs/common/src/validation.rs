//! This module provides some validation helpers

use std::borrow::Cow;
use validator::{validate_length, ValidationError, ValidationErrors};

/// A validator helper
#[derive(Debug)]
pub struct Validator(ValidationErrors);

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Validator> for Result<(), ValidationErrors> {
    fn from(value: Validator) -> Self {
        if value.has_errors() {
            Err(value.errors())
        } else {
            Ok(())
        }
    }
}

impl Validator {
    /// Creates a new validator, without any error
    pub fn new() -> Validator {
        Validator(ValidationErrors::new())
    }

    /// Returns true when the validation produced errors, false otherwise
    pub fn has_errors(&self) -> bool {
        !self.0.is_empty()
    }

    /// Returns the validation errors (if any)
    pub fn errors(self) -> ValidationErrors {
        self.0
    }

    pub fn add_nested(self, field: &'static str, result: Result<(), ValidationErrors>) -> Self {
        let parent: Result<(), ValidationErrors> = self.into();
        let result = ValidationErrors::merge(parent, field, result);

        match result {
            Err(errors) => Validator(errors),
            _ => Validator::default(),
        }
    }

    pub fn validate_length(&mut self, field: &'static str, min: Option<u64>, max: Option<u64>, input: &String) {
        if !validate_length(input, min, max, None) {
            let mut error = ValidationError::new("length");

            if let Some(min) = min {
                error.add_param(Cow::from("min"), &min);
            }

            if let Some(max) = max {
                error.add_param(Cow::from("max"), &max);
            }

            error.add_param(Cow::from("value"), input);
            self.0.add(field, error);
        }
    }

    pub fn validate_length_optional(
        &mut self,
        field: &'static str,
        min: Option<u64>,
        max: Option<u64>,
        input: Option<&String>,
    ) {
        if let Some(input) = input {
            self.validate_length(field, min, max, input);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod validator_tests {
        use super::*;

        #[test]
        fn it_should_create_a_new_validator_without_errors() {
            let validator = Validator::new();
            assert!(!validator.has_errors());
        }

        #[test]
        fn it_should_validate_the_string_length() {
            let mut validator = Validator::new();

            validator.validate_length("field_name", Some(10), Some(25), &String::from("bad"));

            assert!(validator.has_errors());
            let errors = validator.errors();
            let errors = errors.field_errors();
            assert!(errors.contains_key("field_name"));
            assert_eq!(errors["field_name"].len(), 1);
            assert_eq!(errors["field_name"][0].code, "length");
            assert_eq!(errors["field_name"][0].params["value"], "bad");
            assert_eq!(errors["field_name"][0].params["min"], 10);
            assert_eq!(errors["field_name"][0].params["max"], 25);
        }

        #[test]
        fn it_should_validate_the_string_length_without_min() {
            let mut validator = Validator::new();

            validator.validate_length("field_name", None, Some(2), &String::from("bad"));

            assert!(validator.has_errors());
            let errors = validator.errors();
            let errors = errors.field_errors();
            assert!(errors.contains_key("field_name"));
            assert_eq!(errors["field_name"].len(), 1);
            assert_eq!(errors["field_name"][0].code, "length");
            assert_eq!(errors["field_name"][0].params["value"], "bad");
            assert_eq!(errors["field_name"][0].params["max"], 2);
        }

        #[test]
        fn it_should_validate_the_optional_string_length() {
            let mut validator = Validator::new();

            validator.validate_length_optional("field_name", Some(10), Some(25), Some(&String::from("bad")));

            assert!(validator.has_errors());
            let errors = validator.errors();
            let errors = errors.field_errors();
            assert!(errors.contains_key("field_name"));
            assert_eq!(errors["field_name"].len(), 1);
            assert_eq!(errors["field_name"][0].code, "length");
            assert_eq!(errors["field_name"][0].params["value"], "bad");
            assert_eq!(errors["field_name"][0].params["min"], 10);
            assert_eq!(errors["field_name"][0].params["max"], 25);
        }

        #[test]
        fn it_should_validate_the_optional_string_length_without_min() {
            let mut validator = Validator::new();

            validator.validate_length_optional("field_name", None, Some(2), Some(&String::from("bad")));

            assert!(validator.has_errors());
            let errors = validator.errors();
            let errors = errors.field_errors();
            assert!(errors.contains_key("field_name"));
            assert_eq!(errors["field_name"].len(), 1);
            assert_eq!(errors["field_name"][0].code, "length");
            assert_eq!(errors["field_name"][0].params["value"], "bad");
            assert_eq!(errors["field_name"][0].params["max"], 2);
        }
    }
}
