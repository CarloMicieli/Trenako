use itertools::Itertools;
use std::fmt;
use std::fmt::Formatter;
use std::str;

/// It represents the service level for a passenger cars, like first or second class.
/// Values of service level can also include multiple service levels, like mixed first
/// and second class.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum ServiceLevel {
    FirstClass,
    SecondClass,
    ThirdClass,
    FirstAndSecondClass,
    FirstSecondAndThirdClass,
    SecondAndThirdClass,
}

impl ServiceLevel {
    const FIRST_CLASS: &'static str = "1cl";
    const SECOND_CLASS: &'static str = "2cl";
    const THIRD_CLASS: &'static str = "3cl";
}

impl fmt::Display for ServiceLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ServiceLevel::FirstClass => {
                write!(f, "{}", ServiceLevel::FIRST_CLASS)
            }
            ServiceLevel::SecondClass => {
                write!(f, "{}", ServiceLevel::SECOND_CLASS)
            }
            ServiceLevel::ThirdClass => {
                write!(f, "{}", ServiceLevel::THIRD_CLASS)
            }
            ServiceLevel::FirstAndSecondClass => {
                write!(f, "{}/{}", ServiceLevel::FIRST_CLASS, ServiceLevel::SECOND_CLASS)
            }
            ServiceLevel::FirstSecondAndThirdClass => write!(
                f,
                "{}/{}/{}",
                ServiceLevel::FIRST_CLASS,
                ServiceLevel::SECOND_CLASS,
                ServiceLevel::THIRD_CLASS
            ),
            ServiceLevel::SecondAndThirdClass => {
                write!(f, "{}/{}", ServiceLevel::SECOND_CLASS, ServiceLevel::THIRD_CLASS)
            }
        }
    }
}

impl str::FromStr for ServiceLevel {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("item number cannot be blank");
        }

        let service_level;
        if s.contains('/') {
            let tokens: Vec<&str> = s.split_terminator('/').sorted().dedup().collect();

            if tokens.len() == 2 {
                let first = tokens[0];
                let second = tokens[1];
                if first == ServiceLevel::FIRST_CLASS && second == ServiceLevel::SECOND_CLASS {
                    service_level = ServiceLevel::FirstAndSecondClass;
                } else if first == ServiceLevel::SECOND_CLASS && second == ServiceLevel::THIRD_CLASS {
                    service_level = ServiceLevel::SecondAndThirdClass;
                } else {
                    return Err("Invalid mixed service level");
                }
            } else if tokens.len() == 3 {
                let first = tokens[0];
                let second = tokens[1];
                let third = tokens[2];

                if first == ServiceLevel::FIRST_CLASS
                    && second == ServiceLevel::SECOND_CLASS
                    && third == ServiceLevel::THIRD_CLASS
                {
                    service_level = ServiceLevel::FirstSecondAndThirdClass;
                } else {
                    return Err("Invalid mixed service level");
                }
            } else {
                return Err("Invalid mixed service level: max number of values is 3");
            }
        } else {
            service_level = match s {
                ServiceLevel::FIRST_CLASS => ServiceLevel::FirstClass,
                ServiceLevel::SECOND_CLASS => ServiceLevel::SecondClass,
                ServiceLevel::THIRD_CLASS => ServiceLevel::ThirdClass,
                _ => return Err("Wrong value for service level"),
            };
        }
        Ok(service_level)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod service_levels {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_convert_string_slices_to_service_levels() {
            let service_level = "1cl".parse::<ServiceLevel>();
            assert!(service_level.is_ok());
            assert_eq!(service_level.unwrap(), ServiceLevel::FirstClass);
        }

        #[test]
        fn it_should_convert_string_slices_to_mixed_service_levels() {
            let service_level = "1cl/2cl/3cl/2cl".parse::<ServiceLevel>();
            assert!(service_level.is_ok());
            assert_eq!(service_level.unwrap(), ServiceLevel::FirstSecondAndThirdClass);
        }

        #[test]
        fn it_should_fail_to_convert_invalid_values_to_service_levels() {
            let empty_string = "".parse::<ServiceLevel>();
            assert!(empty_string.is_err());

            let invalid_value = "aaaa".parse::<ServiceLevel>();
            assert!(invalid_value.is_err());
        }

        #[test]
        fn it_should_fail_to_convert_string_slices_to_mixed_service_levels() {
            let wrong = "1cl/2cl/4cl/2cl".parse::<ServiceLevel>();
            assert!(wrong.is_err());
        }

        #[test]
        fn it_should_display_service_level_values() {
            assert_eq!("1cl", format!("{}", ServiceLevel::FirstClass));
            assert_eq!("1cl/2cl", format!("{}", ServiceLevel::FirstAndSecondClass));
        }
    }
}
