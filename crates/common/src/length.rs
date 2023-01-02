use crate::measure_units::MeasureUnit;
use rust_decimal::prelude::Zero;
use rust_decimal::Decimal;
use std::cmp;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;
use std::ops;
use thiserror::Error;

/// It represents a length.
///
/// Lengths are defined by a non-negative value and a measure unit.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Length {
    Inches(Decimal),
    Kilometers(Decimal),
    Meters(Decimal),
    Miles(Decimal),
    Millimeters(Decimal),
}

#[derive(Debug, PartialEq, Error)]
pub enum LengthError {
    #[error("invalid length value")]
    InvalidValue(#[from] rust_decimal::Error),
    #[error("length values cannot be negative")]
    NegativeValue,
}

impl Length {
    /// Returns a `Length` value with a given measure unit  
    ///
    /// # Panics
    ///
    /// This function panics if `value` is < 0.
    pub fn new(value: Decimal, measure_unit: MeasureUnit) -> Self {
        Self::try_new(value, measure_unit).expect("invalid length value")
    }

    /// Checked version of `Length::new`. Will return `Err` instead of panicking at run-time.
    pub fn try_new(value: Decimal, measure_unit: MeasureUnit) -> Result<Self, LengthError> {
        if value.is_sign_negative() {
            Err(LengthError::NegativeValue)
        } else {
            let length = match measure_unit {
                MeasureUnit::Millimeters => Length::Millimeters(value),
                MeasureUnit::Inches => Length::Inches(value),
                MeasureUnit::Meters => Length::Meters(value),
                MeasureUnit::Miles => Length::Miles(value),
                MeasureUnit::Kilometers => Length::Kilometers(value),
            };
            Ok(length)
        }
    }

    /// this `Length` quantity
    pub fn quantity(&self) -> Decimal {
        match self {
            Length::Millimeters(mm) => *mm,
            Length::Inches(ins) => *ins,
            Length::Meters(m) => *m,
            Length::Miles(mi) => *mi,
            Length::Kilometers(km) => *km,
        }
    }

    /// this `Length` measure unit
    pub fn measure_unit(&self) -> MeasureUnit {
        match self {
            Length::Millimeters(_) => MeasureUnit::Millimeters,
            Length::Inches(_) => MeasureUnit::Inches,
            Length::Meters(_) => MeasureUnit::Meters,
            Length::Miles(_) => MeasureUnit::Miles,
            Length::Kilometers(_) => MeasureUnit::Kilometers,
        }
    }

    pub fn get_value_as(&self, measure_unit: MeasureUnit) -> Decimal {
        if self.measure_unit() == measure_unit {
            self.quantity()
        } else {
            self.measure_unit().to(measure_unit).convert(self.quantity())
        }
    }
}

impl Default for Length {
    fn default() -> Self {
        Length::Millimeters(Decimal::zero())
    }
}

impl fmt::Display for Length {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.quantity(), self.measure_unit().symbol())
    }
}

impl ops::Add for Length {
    type Output = Length;

    fn add(self, rhs: Self) -> Self::Output {
        let (val1, mu1) = (self.quantity(), self.measure_unit());
        let (val2, mu2) = (rhs.quantity(), rhs.measure_unit());

        let new_value = val1 + mu2.to(mu1).convert(val2);

        Length::new(new_value, self.measure_unit())
    }
}

impl cmp::PartialEq for Length {
    fn eq(&self, other: &Self) -> bool {
        let value1 = self.quantity();
        let value2 = other.get_value_as(self.measure_unit());
        value1 == value2
    }
}

impl cmp::Eq for Length {}

impl cmp::PartialOrd for Length {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let value1 = self.quantity();
        let value2 = other.get_value_as(self.measure_unit());
        value1.partial_cmp(&value2)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod lengths {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use rust_decimal::prelude::FromPrimitive;
        use rust_decimal_macros::dec;

        #[test]
        fn it_should_create_new_lengths() {
            let l = Length::new(dec!(42.), MeasureUnit::Millimeters);
            assert_eq!(dec!(42.0), l.quantity());
            assert_eq!(MeasureUnit::Millimeters, l.measure_unit());
        }

        #[test]
        fn it_should_ensure_lengths_are_non_negative() {
            assert_eq!(
                Err(LengthError::NegativeValue),
                Length::try_new(dec!(-1.), MeasureUnit::Inches)
            );
            assert_eq!(
                Ok(Length::default()),
                Length::try_new(Decimal::ZERO, MeasureUnit::Millimeters)
            );
        }

        #[rstest]
        #[case(42.0f32, MeasureUnit::Inches, "42 in")]
        #[case(42.0f32, MeasureUnit::Meters, "42 m")]
        #[case(42.0f32, MeasureUnit::Millimeters, "42 mm")]
        #[case(42.0f32, MeasureUnit::Miles, "42 mi")]
        #[case(42.0f32, MeasureUnit::Kilometers, "42 km")]
        fn it_should_display_lengths(#[case] input: f32, #[case] measure_unit: MeasureUnit, #[case] expected: &str) {
            let value = Decimal::from_f32(input).unwrap();
            let length = Length::new(value, measure_unit);
            assert_eq!(expected, length.to_string());
        }

        #[test]
        fn it_should_sum_two_lengths() {
            let l1 = Length::new(dec!(20.6), MeasureUnit::Millimeters);
            let l2 = Length::new(dec!(21.4), MeasureUnit::Millimeters);

            let l = l1 + l2;
            assert_eq!(dec!(42.0), l.quantity());
            assert_eq!(MeasureUnit::Millimeters, l.measure_unit());
        }

        #[test]
        fn it_should_sum_two_lengths_converting_measure_units() {
            let l1 = Length::new(dec!(16.6), MeasureUnit::Millimeters);
            let l2 = Length::new(dec!(1.0), MeasureUnit::Inches);

            let l = l1 + l2;
            assert_eq!(dec!(42.0), l.quantity());
            assert_eq!(MeasureUnit::Millimeters, l.measure_unit());
        }

        #[test]
        fn it_should_compare_two_lengths() {
            let l1 = Length::new(dec!(20.6), MeasureUnit::Millimeters);
            let l2 = Length::new(dec!(21.4), MeasureUnit::Millimeters);

            assert_eq!(l1, l1);
            assert_ne!(l1, l2);
        }

        #[test]
        fn it_should_sort_length_values() {
            let l1 = Length::new(dec!(20.6), MeasureUnit::Millimeters);
            let l2 = Length::new(dec!(21.4), MeasureUnit::Millimeters);
            let l3 = Length::new(dec!(1.0), MeasureUnit::Meters);

            assert!(l1 < l2);
            assert!(l2 > l1);
            assert!(l3 > l1);
        }
    }
}
