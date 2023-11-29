//! the module includes everything related to row converters

use std::borrow::Cow;
use std::fmt;
use std::fmt::Formatter;

/// Map the row to the result output.
pub trait ToOutputConverter<T> {
    fn to_output(self) -> Result<T, ConversionErrors>;
}

impl<R: ToOutputConverter<T>, T> ToOutputConverter<Vec<T>> for Vec<R> {
    fn to_output(self) -> Result<Vec<T>, ConversionErrors> {
        let mut output = Vec::with_capacity(self.len());
        for el in self.into_iter() {
            let item = el.to_output()?;
            output.push(item);
        }
        Ok(output)
    }
}

impl<R: ToOutputConverter<T>, T> ToOutputConverter<Option<T>> for Option<R> {
    fn to_output(self) -> Result<Option<T>, ConversionErrors> {
        self.map(|it| it.to_output()).transpose()
    }
}

pub trait Converter<T>: Sized {
    fn try_convert(row: &T) -> Result<Self, ConversionErrors>;
}

pub trait OptionConverter<T>: Sized {
    fn try_convert(row: &T) -> Result<Option<Self>, ConversionErrors>;
}

/// It contains the conversion errors, after an attempt to convert a database row
/// into a domain value
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct ConversionErrors(Vec<ConversionError>);

impl ConversionErrors {
    /// It creates a new `ConversionErrors` value
    pub fn new() -> Self {
        ConversionErrors::default()
    }

    /// Returns a boolean indicating whether the conversion includes any error.
    pub fn has_errors(&self) -> bool {
        !self.0.is_empty()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ConversionError {
    message: Cow<'static, str>,
}

impl ConversionError {
    /// It creates a new conversion error with the given error `message`
    pub fn new(message: &'static str) -> ConversionError {
        ConversionError {
            message: Cow::from(message),
        }
    }
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Conversion error: {}", self.message)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod to_output_tests {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_implement_to_output_for_vectors() {
            let value = vec![1, 2, 3];
            let result = value.to_output();
            let output = result.expect("invalid conversion");
            assert_eq!(vec!["1", "2", "3"], output);
        }

        #[test]
        fn it_should_return_the_error_when_the_conversion_failed_for_the_vec() {
            let value = vec![1, 42, 3];
            let result = value.to_output();
            assert!(result.is_err());
        }

        #[test]
        fn it_should_implement_to_output_for_options() {
            let value = Some(1);
            let result = value.to_output();
            let output = result.expect("invalid conversion");
            assert_eq!(Some(String::from("1")), output);
        }

        #[test]
        fn it_should_return_the_error_when_the_conversion_failed_for_the_option() {
            let value = Some(42);
            let result = value.to_output();
            assert!(result.is_err());
        }

        impl ToOutputConverter<String> for i32 {
            fn to_output(self) -> Result<String, ConversionErrors> {
                if self == 42 {
                    Err(ConversionErrors::new())
                } else {
                    Ok(self.to_string())
                }
            }
        }
    }

    mod conversion_error_tests {
        use super::*;

        #[test]
        fn it_should_create_a_conversion_error() {
            let error = ConversionError::new("invalid decimal value");
            assert_eq!("Conversion error: invalid decimal value", error.to_string());
        }
    }

    mod conversion_errors_tests {
        use super::*;

        #[test]
        fn it_should_create_new_conversion_errors() {
            let conversion_errors = ConversionErrors::new();
            assert!(!conversion_errors.has_errors());
        }
    }
}
