use rust_decimal::Decimal;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Display, Formatter};

/// It represents the {@code Ratio} between a model railway size
/// and the size of an actual train.
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct Ratio(Decimal);

impl TryFrom<Decimal> for Ratio {
    type Error = ();

    fn try_from(value: Decimal) -> Result<Self, Self::Error> {
        if value.is_sign_positive() && !value.is_zero() {
            Ok(Ratio(value))
        } else {
            Err(())
        }
    }
}

impl Display for Ratio {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "1:{}", self.0)
    }
}

impl PartialOrd for Ratio {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod ratios {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_new_ratios() {
            let value = Decimal::from(87);
            assert_eq!(Ok(Ratio(value)), Ratio::try_from(value));
        }

        #[test]
        fn it_should_only_allow_non_negative_ratios() {
            assert_eq!(Err(()), Ratio::try_from(Decimal::from(0)));
            assert_eq!(Err(()), Ratio::try_from(Decimal::from(-1)));
        }

        #[test]
        fn it_should_display_ratios() {
            let ratio1 = Ratio::try_from(Decimal::from(87));
            assert_eq!("1:87", ratio1.unwrap().to_string());
        }

        #[test]
        fn it_should_compare_two_ratios() {
            let ratio1 = Ratio::try_from(Decimal::from(87)).unwrap();
            let ratio2 = Ratio::try_from(Decimal::from(160)).unwrap();

            assert!(ratio1 > ratio2, "1:87 > 1:160 must hold true");
            assert!(ratio2 < ratio1, "1:160 < 1:87 must hold true");
        }
    }
}
