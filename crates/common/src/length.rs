use crate::measure_units::MeasureUnit;
use rust_decimal::prelude::{FromPrimitive, Zero};
use rust_decimal::Decimal;
use std::cmp;
use std::fmt;
use std::fmt::Formatter;
use std::ops;

/// It represents a length.
///
/// Lengths are defined by a non negative quantity and a measure unit.
#[derive(Debug, Copy, Clone)]
pub enum Length {
    Millimeters(Decimal),
    Inches(Decimal),
    Miles(Decimal),
    Kilometers(Decimal),
}

impl Length {
    pub fn new(value: f32, measure_unit: MeasureUnit) -> Option<Self> {
        let val = Decimal::from_f32(value)?;
        Length::create(val, measure_unit)
    }

    fn create(value: Decimal, measure_unit: MeasureUnit) -> Option<Self> {
        if value.is_sign_negative() {
            None
        } else {
            let length = match measure_unit {
                MeasureUnit::Millimeters => Length::Millimeters(value),
                MeasureUnit::Inches => Length::Inches(value),
                MeasureUnit::Miles => Length::Miles(value),
                MeasureUnit::Kilometers => Length::Kilometers(value),
            };
            Some(length)
        }
    }

    /// Returns this [Length] quantity
    pub fn quantity(&self) -> Decimal {
        match self {
            Length::Millimeters(mm) => *mm,
            Length::Inches(ins) => *ins,
            Length::Miles(mi) => *mi,
            Length::Kilometers(km) => *km,
        }
    }

    /// Returns this [Length] measure unit
    pub fn measure_unit(&self) -> MeasureUnit {
        match self {
            Length::Millimeters(_) => MeasureUnit::Millimeters,
            Length::Inches(_) => MeasureUnit::Inches,
            Length::Miles(_) => MeasureUnit::Miles,
            Length::Kilometers(_) => MeasureUnit::Kilometers,
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

        Length::create(new_value, self.measure_unit()).unwrap()
    }
}

impl cmp::PartialEq for Length {
    fn eq(&self, other: &Self) -> bool {
        let (val1, mu1) = (self.quantity(), self.measure_unit());
        let (val2, mu2) = (other.quantity(), other.measure_unit());

        if mu1 == mu2 {
            val1 == val2
        } else {
            let val2_converted = mu2.to(mu1).convert(val2);
            val1 == val2_converted
        }
    }
}

impl cmp::Eq for Length {}

#[cfg(test)]
mod test {
    use super::*;

    mod lengths {
        use super::*;
        use pretty_assertions::assert_eq;
        use rust_decimal_macros::dec;

        #[test]
        fn it_should_create_new_lengths() {
            let l = Length::new(42_f32, MeasureUnit::Millimeters).unwrap();
            assert_eq!(dec!(42.0), l.quantity());
            assert_eq!(MeasureUnit::Millimeters, l.measure_unit());
        }

        #[test]
        fn it_should_ensure_lengths_are_non_negative() {
            assert_eq!(None, Length::new(-1_f32, MeasureUnit::Inches));
            assert_eq!(Some(Length::default()), Length::new(0_f32, MeasureUnit::Millimeters));
        }

        #[test]
        fn it_should_display_lengths() {
            let l = Length::new(42_f32, MeasureUnit::Millimeters).unwrap();
            assert_eq!("42 mm", l.to_string());
        }

        #[test]
        fn it_should_sum_two_lengths() {
            let l1 = Length::new(20.6_f32, MeasureUnit::Millimeters).unwrap();
            let l2 = Length::new(21.4_f32, MeasureUnit::Millimeters).unwrap();

            let l = l1 + l2;
            assert_eq!(dec!(42.0), l.quantity());
            assert_eq!(MeasureUnit::Millimeters, l.measure_unit());
        }

        #[test]
        fn it_should_sum_two_lengths_converting_measure_units() {
            let l1 = Length::new(16.6_f32, MeasureUnit::Millimeters).unwrap();
            let l2 = Length::new(1_f32, MeasureUnit::Inches).unwrap();

            let l = l1 + l2;
            assert_eq!(dec!(42.0), l.quantity());
            assert_eq!(MeasureUnit::Millimeters, l.measure_unit());
        }

        #[test]
        fn it_should_compare_two_lengths() {
            let l1 = Length::new(20.6_f32, MeasureUnit::Millimeters).unwrap();
            let l2 = Length::new(21.4_f32, MeasureUnit::Millimeters).unwrap();

            assert_eq!(l1, l1);
            assert_ne!(l1, l2);
        }
    }
}
