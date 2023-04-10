//! the module includes everything related to measure units

use rust_decimal::Decimal;
use rust_decimal_macros::*;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct MeasureUnitConverter {
    from: MeasureUnit,
    to: MeasureUnit,
    ratio: Decimal,
}

impl MeasureUnitConverter {
    /// Create a new measure unit converter
    fn new(from: MeasureUnit, to: MeasureUnit, ratio: Decimal) -> Self {
        if from == to {
            Self::same_unit(from)
        } else {
            MeasureUnitConverter { from, to, ratio }
        }
    }

    fn same_unit(mu: MeasureUnit) -> Self {
        MeasureUnitConverter {
            from: mu,
            to: mu,
            ratio: 1.into(),
        }
    }

    /// Convert the input using the current measure unit converter
    pub fn convert(&self, value: Decimal) -> Decimal {
        value * self.ratio
    }
}

impl fmt::Display for MeasureUnitConverter {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Converter from {:?} to {:?}", self.from, self.to)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MeasureUnit {
    Millimeters,
    Inches,
    Meters,
    Miles,
    Kilometers,
}

impl MeasureUnit {
    /// the measure unit symbol
    pub fn symbol(&self) -> &str {
        match self {
            MeasureUnit::Miles => "mi",
            MeasureUnit::Inches => "in",
            MeasureUnit::Meters => "m",
            MeasureUnit::Millimeters => "mm",
            MeasureUnit::Kilometers => "km",
        }
    }

    pub fn same_as(&self, value: Decimal, other_mu: MeasureUnit, other_value: Decimal) -> bool {
        let value_converted = self.to(other_mu).convert(value);
        let diff = other_value - value_converted;
        Decimal::abs(&diff) < dec!(0.01)
    }

    pub fn to(&self, other: MeasureUnit) -> MeasureUnitConverter {
        match (self, other) {
            (MeasureUnit::Inches, MeasureUnit::Millimeters) => MeasureUnitConverter::new(
                MeasureUnit::Inches,
                MeasureUnit::Millimeters,
                MeasureUnit::INCHES_TO_MILLIMETERS,
            ),
            (MeasureUnit::Millimeters, MeasureUnit::Inches) => MeasureUnitConverter::new(
                MeasureUnit::Millimeters,
                MeasureUnit::Inches,
                MeasureUnit::MILLIMETERS_TO_INCHES,
            ),
            (MeasureUnit::Meters, MeasureUnit::Millimeters) => MeasureUnitConverter::new(
                MeasureUnit::Meters,
                MeasureUnit::Millimeters,
                MeasureUnit::METERS_TO_MILLIMETERS,
            ),
            (MeasureUnit::Millimeters, MeasureUnit::Meters) => MeasureUnitConverter::new(
                MeasureUnit::Millimeters,
                MeasureUnit::Meters,
                MeasureUnit::MILLIMETERS_TO_METERS,
            ),
            (MeasureUnit::Kilometers, MeasureUnit::Miles) => MeasureUnitConverter::new(
                MeasureUnit::Kilometers,
                MeasureUnit::Miles,
                MeasureUnit::KILOMETERS_TO_MILES,
            ),
            (MeasureUnit::Miles, MeasureUnit::Kilometers) => MeasureUnitConverter::new(
                MeasureUnit::Miles,
                MeasureUnit::Kilometers,
                MeasureUnit::MILES_TO_KILOMETERS,
            ),
            (MeasureUnit::Inches, MeasureUnit::Inches) => MeasureUnitConverter::same_unit(MeasureUnit::Inches),
            (MeasureUnit::Meters, MeasureUnit::Meters) => MeasureUnitConverter::same_unit(MeasureUnit::Millimeters),
            (MeasureUnit::Millimeters, MeasureUnit::Millimeters) => {
                MeasureUnitConverter::same_unit(MeasureUnit::Millimeters)
            }
            (MeasureUnit::Kilometers, MeasureUnit::Kilometers) => {
                MeasureUnitConverter::same_unit(MeasureUnit::Kilometers)
            }
            (MeasureUnit::Miles, MeasureUnit::Miles) => MeasureUnitConverter::same_unit(MeasureUnit::Inches),
            _ => panic!("invalid converter"),
        }
    }

    const INCHES_TO_MILLIMETERS: Decimal = dec!(25.4);
    const MILLIMETERS_TO_INCHES: Decimal = dec!(0.0393701);
    const MILES_TO_KILOMETERS: Decimal = dec!(1.60934);
    const KILOMETERS_TO_MILES: Decimal = dec!(0.621371);
    const METERS_TO_MILLIMETERS: Decimal = dec!(1000.0);
    const MILLIMETERS_TO_METERS: Decimal = dec!(0.001);
}

impl fmt::Display for MeasureUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod measure_units_tests {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use rust_decimal_macros::dec;

        #[rstest]
        #[case(dec!(16.5), MeasureUnit::Inches, dec!(0.65), true)]
        #[case(dec!(16.5), MeasureUnit::Inches, dec!(0.63), false)]
        #[case(dec!(16.5), MeasureUnit::Inches, dec!(0.66), false)]
        fn it_should_check_if_the_value_in_another_measure_unit_is_the_same(
            #[case] input: Decimal,
            #[case] other_mu: MeasureUnit,
            #[case] other_value: Decimal,
            #[case] expected: bool,
        ) {
            let result = MeasureUnit::Millimeters.same_as(input, other_mu, other_value);
            assert_eq!(expected, result);
        }

        #[test]
        fn measure_unit_symbol_should_return_the_symbol() {
            assert_eq!(MeasureUnit::Miles.symbol(), "mi");
            assert_eq!(MeasureUnit::Millimeters.symbol(), "mm");
            assert_eq!(MeasureUnit::Inches.symbol(), "in");
            assert_eq!(MeasureUnit::Kilometers.symbol(), "km");
            assert_eq!(MeasureUnit::Meters.symbol(), "m");
        }

        #[rstest]
        #[case(dec!(1.0), MeasureUnit::Inches, MeasureUnit::Inches, dec!(1.0))]
        #[case(dec!(1.0), MeasureUnit::Kilometers, MeasureUnit::Kilometers, dec!(1.0))]
        #[case(dec!(1.0), MeasureUnit::Meters, MeasureUnit::Meters, dec!(1.0))]
        #[case(dec!(1.0), MeasureUnit::Miles, MeasureUnit::Miles, dec!(1.0))]
        #[case(dec!(1.0), MeasureUnit::Millimeters, MeasureUnit::Millimeters, dec!(1.0))]
        #[case(dec!(1.0), MeasureUnit::Millimeters, MeasureUnit::Meters, dec!(0.0010))]
        #[case(dec!(1.0), MeasureUnit::Meters, MeasureUnit::Millimeters, dec!(1000.0))]
        #[case(dec!(1.0), MeasureUnit::Inches, MeasureUnit::Millimeters, dec!(25.40))]
        #[case(dec!(1.0), MeasureUnit::Millimeters, MeasureUnit::Inches, dec!(0.03937010))]
        #[case(dec!(1.0), MeasureUnit::Kilometers, MeasureUnit::Miles, dec!(0.6213710))]
        #[case(dec!(1.0), MeasureUnit::Miles, MeasureUnit::Kilometers, dec!(1.609340))]
        fn it_should_convert_between_measure_units(
            #[case] value: Decimal,
            #[case] from_mu: MeasureUnit,
            #[case] to_mu: MeasureUnit,
            #[case] expected: Decimal,
        ) {
            let converted = from_mu.to(to_mu).convert(value);
            assert_eq!(expected, converted);
        }
    }
}
