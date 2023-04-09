//! the catalog item number

use common::slug::Slug;
use sqlx::Type;
use std::borrow::Cow;
use std::fmt::Formatter;
use std::str::FromStr;
use std::{convert, fmt, ops, str};
use thiserror::Error;
use validator::{validate_length, ValidationError};

/// It represent a catalog item number.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize, Type)]
#[sqlx(transparent)]
pub struct ItemNumber(String);

impl ItemNumber {
    /// Creates a new ItemNumber from the string slice, it needs to panic when the
    /// provided string slice is empty.
    pub fn new(value: &str) -> Self {
        ItemNumber::from_str(value).expect("input is not a valid item number")
    }

    /// Returns the item number value, this cannot be blank.
    pub fn value(&self) -> &str {
        &self.0
    }
}

pub fn validate_item_number(input: &ItemNumber) -> Result<(), ValidationError> {
    if !validate_length(&input.0, Some(1), Some(25), None) {
        let mut error = ValidationError::new("length");
        error.add_param(Cow::from("min"), &1);
        error.add_param(Cow::from("max"), &25);
        error.add_param(Cow::from("value"), &input.0);
        Err(error)
    } else {
        Ok(())
    }
}

impl ops::Deref for ItemNumber {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl convert::AsRef<str> for ItemNumber {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl str::FromStr for ItemNumber {
    type Err = ItemNumberError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(ItemNumberError::EmptyItemNumber)
        } else {
            Ok(ItemNumber(s.to_owned()))
        }
    }
}

#[derive(Debug, Eq, PartialEq, Error)]
pub enum ItemNumberError {
    #[error("Item number cannot be blank")]
    EmptyItemNumber,
}

impl fmt::Display for ItemNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl convert::From<ItemNumber> for Slug {
    fn from(item_number: ItemNumber) -> Self {
        Slug::new(item_number.value())
    }
}

#[cfg(test)]
pub fn invalid_item_number() -> ItemNumber {
    ItemNumber("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    mod item_numbers {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;

        #[test]
        fn it_should_create_new_item_numbers() {
            let n = ItemNumber::new("123456");
            assert_eq!(n.value(), "123456");
            assert_eq!(n.to_string(), "123456");
        }

        #[test]
        fn it_should_deref_item_numbers() {
            let item_number = ItemNumber::new("123456");
            assert_eq!("123456", &*item_number);
            assert_eq!("123456", item_number.as_ref());
        }

        #[test]
        fn it_should_convert_item_numbers_to_slugs() {
            let item_number = ItemNumber::new("1234");
            let slug: Slug = item_number.into();
            assert_eq!(Slug::new("1234"), slug);
        }

        #[test]
        #[should_panic(expected = "input is not a valid item number")]
        fn it_should_fail_to_convert_empty_string_slices_as_item_numbers() {
            ItemNumber::new("");
        }

        #[rstest]
        #[case("", Err(ItemNumberError::EmptyItemNumber))]
        #[case("1234", Ok(ItemNumber::new("1234")))]
        fn it_should_parse_string_as_item_numbers(
            #[case] input: &str,
            #[case] expected: Result<ItemNumber, ItemNumberError>,
        ) {
            let result = ItemNumber::from_str(input);
            assert_eq!(expected, result);
        }
    }

    mod item_number_validation {
        use super::*;

        #[test]
        fn it_should_validate_item_numbers() {
            let item_number = ItemNumber("123456".to_string());

            let result = validate_item_number(&item_number);
            assert!(result.is_ok());
        }

        #[test]
        fn it_should_reject_invalid_item_numbers() {
            let item_number = ItemNumber("".to_string());

            let result = validate_item_number(&item_number);
            let error = result.unwrap_err();
            assert_eq!(error.code, "length");
            assert_eq!(error.params["value"], "");
            assert_eq!(error.params["min"], 1);
            assert_eq!(error.params["max"], 25);
        }
    }
}
