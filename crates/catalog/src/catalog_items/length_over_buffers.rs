use common::length::Length;
use common::measure_units::MeasureUnit;
use rust_decimal::Decimal;
use thiserror::Error;

/// The rail vehicle measurement method expressed as the length over buffers
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct LengthOverBuffers {
    /// the overall length in inches
    pub inches: Option<Length>,
    /// the overall length in millimeters
    pub millimeters: Option<Length>,
}

impl LengthOverBuffers {
    /// Creates a new length over buffers value
    pub fn new(inches: Option<Decimal>, millimeters: Option<Decimal>) -> Result<Self, LengthOverBuffersError> {
        match (inches, millimeters) {
            (Some(inches), _) if inches.is_sign_negative() || inches.is_zero() => {
                Err(LengthOverBuffersError::NonPositiveValue)
            }
            (_, Some(mm)) if mm.is_sign_negative() || mm.is_zero() => Err(LengthOverBuffersError::NonPositiveValue),
            (Some(inches), Some(mm)) if !MeasureUnit::Millimeters.same_as(mm, MeasureUnit::Inches, inches) => {
                Err(LengthOverBuffersError::DifferentValues)
            }
            _ => {
                let inches = inches.map(Length::Inches);
                let millimeters = millimeters.map(Length::Millimeters);
                Ok(LengthOverBuffers { inches, millimeters })
            }
        }
    }

    /// Creates a new length over buffers value in millimeters
    pub fn from_millimeters(millimeters: Length) -> Self {
        let inches = MeasureUnit::Millimeters
            .to(MeasureUnit::Inches)
            .convert(millimeters.quantity());
        LengthOverBuffers {
            inches: Some(Length::Inches(inches)),
            millimeters: Some(millimeters),
        }
    }

    /// Creates a new length over buffers value in inches
    pub fn from_inches(inches: Length) -> Self {
        let millimeters = MeasureUnit::Inches
            .to(MeasureUnit::Millimeters)
            .convert(inches.quantity());
        LengthOverBuffers {
            inches: Some(inches),
            millimeters: Some(Length::Millimeters(millimeters)),
        }
    }

    /// the length over buffers value in inches
    pub fn inches(&self) -> Option<&Length> {
        self.inches.as_ref()
    }

    /// the length over buffers value in millimeters
    pub fn millimeters(&self) -> Option<&Length> {
        self.millimeters.as_ref()
    }
}

#[derive(Debug, PartialEq, Error)]
pub enum LengthOverBuffersError {
    #[error("the value in millimeters is not matching the one in inches")]
    DifferentValues,
    #[error("The length over buffers must be positive")]
    NonPositiveValue,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod length_over_buffer_tests {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use rust_decimal_macros::dec;

        #[rstest]
        #[case(None, None, Ok(LengthOverBuffers { inches: None, millimeters: None}))]
        #[case(Some(dec!(0.0)), Some(dec!(0.0)), Err(LengthOverBuffersError::NonPositiveValue))]
        #[case(Some(dec!(-0.65)), Some(dec!(-16.5)), Err(LengthOverBuffersError::NonPositiveValue))]
        #[case(Some(dec!(0.65)), Some(dec!(16.3)), Err(LengthOverBuffersError::DifferentValues))]
        fn it_should_create_new_length_over_buffers_values(
            #[case] inches: Option<Decimal>,
            #[case] millimeters: Option<Decimal>,
            #[case] expected: Result<LengthOverBuffers, LengthOverBuffersError>,
        ) {
            let result = LengthOverBuffers::new(inches, millimeters);
            assert_eq!(expected, result);
        }

        #[test]
        fn it_should_create_new_length_over_buffer_from_inches() {
            let inches = Length::Inches(dec!(42));
            let lob = LengthOverBuffers::from_inches(inches);
            assert_eq!(Some(&inches), lob.inches());
            assert_eq!(Some(&Length::Millimeters(dec!(1066.8))), lob.millimeters());
        }

        #[test]
        fn it_should_create_new_length_over_buffer_from_millimeters() {
            let millimeters = Length::Millimeters(dec!(42));
            let lob = LengthOverBuffers::from_millimeters(millimeters);
            assert_eq!(Some(&millimeters), lob.millimeters());
            assert_eq!(Some(&Length::Inches(dec!(1.6535442))), lob.inches());
        }
    }
}
