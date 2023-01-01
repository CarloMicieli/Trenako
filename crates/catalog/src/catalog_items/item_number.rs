use common::slug::Slug;
use sqlx::Type;
use std::fmt::Formatter;
use std::str::FromStr;
use std::{convert, fmt};
use thiserror::Error;

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

impl FromStr for ItemNumber {
    type Err = ItemNumberParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(ItemNumberParserError::EmptyItemNumber)
        } else {
            Ok(ItemNumber(s.to_owned()))
        }
    }
}

#[derive(Debug, Eq, PartialEq, Error)]
pub enum ItemNumberParserError {
    #[error("Item number cannot be blank")]
    EmptyItemNumber,
}

impl fmt::Display for ItemNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl convert::From<ItemNumber> for Slug {
    fn from(item_number: ItemNumber) -> Self {
        Slug::new(item_number.value())
    }
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
        #[case("", Err(ItemNumberParserError::EmptyItemNumber))]
        #[case("1234", Ok(ItemNumber::new("1234")))]
        fn it_should_parse_string_as_item_numbers(
            #[case] input: &str,
            #[case] expected: Result<ItemNumber, ItemNumberParserError>,
        ) {
            let result = ItemNumber::from_str(input);
            assert_eq!(expected, result);
        }
    }
}
