use common::length::Length;
use common::measure_units::MeasureUnit;

/// The rail vehicle measurement method
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct LengthOverBuffer {
    /// the overall length in inches
    pub inches: Option<Length>,
    /// the overall length in millimeters
    pub millimeters: Option<Length>,
}

impl LengthOverBuffer {
    /// Creates a new length over buffer value in millimeters
    pub fn from_millimeters(millimeters: Length) -> Self {
        let inches = MeasureUnit::Millimeters
            .to(MeasureUnit::Inches)
            .convert(millimeters.quantity());
        LengthOverBuffer {
            inches: Some(Length::Inches(inches)),
            millimeters: Some(millimeters),
        }
    }

    /// Creates a new length over buffer value in inches
    pub fn from_inches(inches: Length) -> Self {
        let millimeters = MeasureUnit::Inches
            .to(MeasureUnit::Millimeters)
            .convert(inches.quantity());
        LengthOverBuffer {
            inches: Some(inches),
            millimeters: Some(Length::Millimeters(millimeters)),
        }
    }

    /// The length over buffer value in inches
    pub fn inches(&self) -> Option<&Length> {
        self.inches.as_ref()
    }

    /// The length over buffer value in millimeters
    pub fn millimeters(&self) -> Option<&Length> {
        self.millimeters.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod length_over_buffer_tests {
        use super::*;
        use pretty_assertions::assert_eq;
        use rust_decimal_macros::dec;

        #[test]
        fn it_should_create_new_length_over_buffer_from_inches() {
            let inches = Length::Inches(dec!(42));
            let lob = LengthOverBuffer::from_inches(inches);
            assert_eq!(Some(&inches), lob.inches());
            assert_eq!(Some(&Length::Millimeters(dec!(1066.8))), lob.millimeters());
        }

        #[test]
        fn it_should_create_new_length_over_buffer_from_millimeters() {
            let millimeters = Length::Millimeters(dec!(42));
            let lob = LengthOverBuffer::from_millimeters(millimeters);
            assert_eq!(Some(&millimeters), lob.millimeters());
            assert_eq!(Some(&Length::Inches(dec!(1.6535442))), lob.inches());
        }
    }
}
