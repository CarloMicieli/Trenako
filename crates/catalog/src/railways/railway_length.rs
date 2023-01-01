use common::measure_units::MeasureUnit;
use rust_decimal::Decimal;
use std::fmt::{Display, Formatter};

/// The overall length of tracks (in km and miles) operated by a railway company
#[derive(Debug, Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct RailwayLength {
    /// the total railway network in kilometers
    pub kilometers: Decimal,
    /// the total railway network in miles
    pub miles: Decimal,
}

impl RailwayLength {
    pub fn new(kilometers: Decimal, miles: Decimal) -> Self {
        RailwayLength { kilometers, miles }
    }

    /// Creates a new railway length from the kilometers value
    pub fn of_kilometers(kilometers: Decimal) -> Self {
        let miles = MeasureUnit::Kilometers.to(MeasureUnit::Miles).convert(kilometers);
        RailwayLength { kilometers, miles }
    }

    /// Creates a new railway length from the miles value
    pub fn of_miles(miles: Decimal) -> Self {
        let kilometers = MeasureUnit::Miles.to(MeasureUnit::Kilometers).convert(miles);
        RailwayLength { kilometers, miles }
    }

    /// Returns the length of track in Kilometers
    pub fn kilometers(&self) -> Decimal {
        self.kilometers
    }

    /// Returns the length of track in Miles
    pub fn miles(&self) -> Decimal {
        self.miles
    }
}

impl Display for RailwayLength {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "kilometers: {}, miles: {}", self.kilometers, self.miles)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod railway_lengths {
        use super::*;
        use pretty_assertions::assert_eq;
        use rust_decimal_macros::dec;

        #[test]
        fn it_should_create_new_railway_lengths() {
            let miles = dec!(100);
            let kilometers = dec!(100);
            let len = RailwayLength::new(kilometers, miles);
            assert_eq!(miles, len.miles());
            assert_eq!(kilometers, len.kilometers());
        }

        #[test]
        fn it_should_display_a_railway_length_value() {
            let miles = dec!(100);
            let kilometers = dec!(100);
            let len = RailwayLength::new(kilometers, miles);
            assert_eq!("kilometers: 100, miles: 100", len.to_string());
        }

        #[test]
        fn it_should_create_a_railway_length_from_kilometers() {
            let kilometers = dec!(100);
            let length = RailwayLength::of_kilometers(kilometers);

            assert_eq!(kilometers, length.kilometers());
            assert_eq!(dec!(62.137100), length.miles());
        }

        #[test]
        fn it_should_create_a_railway_length_from_miles() {
            let miles = dec!(100);
            let length = RailwayLength::of_miles(miles);

            assert_eq!(dec!(160.93400), length.kilometers());
            assert_eq!(miles, length.miles());
        }
    }
}
