use crate::catalog::scales::scale_row::ScaleRow;
use catalog::scales::ratio::Ratio;
use catalog::scales::scale::Scale;
use catalog::scales::scale_gauge::Gauge;
use common::localized_text::LocalizedText;
use common::metadata::Metadata;
use common::queries::converters::{ConversionErrors, Converter, ToOutputConverter};
use std::collections::HashSet;

impl ToOutputConverter<Scale> for ScaleRow {
    fn to_output(self) -> Result<Scale, ConversionErrors> {
        let row = self;
        let description = LocalizedText::try_convert(&row)?;
        let metadata = Metadata::try_convert(&row)?;
        let ratio = Ratio::try_convert(&row)?;
        let gauge = Gauge::try_convert(&row)?;

        Ok(Scale {
            scale_id: row.scale_id,
            name: row.name,
            ratio,
            gauge,
            description,
            standards: HashSet::from_iter(row.standards.iter().cloned()),
            metadata,
        })
    }
}

impl Converter<ScaleRow> for Gauge {
    fn try_convert(row: &ScaleRow) -> Result<Self, ConversionErrors> {
        match (row.gauge_inches, row.gauge_millimeters) {
            (Some(inches), Some(millimeters)) if inches.is_sign_positive() && millimeters.is_sign_positive() => {
                Gauge::new(row.track_gauge, millimeters, inches).map_err(|_| ConversionErrors::new())
            }
            (Some(inches), None) if inches.is_sign_positive() => {
                Gauge::from_inches(row.track_gauge, inches).map_err(|_| ConversionErrors::new())
            }
            (None, Some(millimeters)) if millimeters.is_sign_positive() => {
                Gauge::from_millimeters(row.track_gauge, millimeters).map_err(|_| ConversionErrors::new())
            }
            _ => Err(ConversionErrors::new()),
        }
    }
}

impl Converter<ScaleRow> for Ratio {
    fn try_convert(row: &ScaleRow) -> Result<Self, ConversionErrors> {
        if row.ratio.is_sign_positive() {
            Ratio::try_from(row.ratio).map_err(|_| ConversionErrors::new())
        } else {
            Err(ConversionErrors::new())
        }
    }
}

impl Converter<ScaleRow> for LocalizedText {
    fn try_convert(value: &ScaleRow) -> Result<Self, ConversionErrors> {
        let mut localized_text = LocalizedText::default();

        localized_text.add_english(value.description_en.as_ref());
        localized_text.add_italian(value.description_it.as_ref());

        Ok(localized_text)
    }
}

impl Converter<ScaleRow> for Metadata {
    fn try_convert(value: &ScaleRow) -> Result<Self, ConversionErrors> {
        Ok(Metadata::new(
            value.version as u8,
            value.created_at,
            value.last_modified_at,
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::catalog::scales::scale_row::test::new_scale_row;
    use catalog::common::TrackGauge;
    use chrono::Utc;
    use rust_decimal_macros::dec;

    fn default_row() -> ScaleRow {
        new_scale_row("H0", dec!(87.0), TrackGauge::Standard, Utc::now())
    }

    mod scale_row_converter {
        use super::*;
        use catalog::scales::scale_id::ScaleId;
        use catalog::scales::standard::Standard;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_convert_scale_rows() {
            let row = ScaleRow {
                scale_id: ScaleId::new("H0"),
                name: String::from("H0"),
                standards: vec![Standard::NEM],
                track_gauge: TrackGauge::Standard,
                gauge_inches: Some(dec!(0.65)),
                gauge_millimeters: Some(dec!(16.5)),
                ..default_row()
            };

            let result = row.to_output();
            assert!(result.is_ok());

            let scale = result.unwrap();
            assert_eq!(scale.scale_id, ScaleId::new("H0"));
            assert_eq!(scale.name, "H0");
            assert_eq!(scale.standards, HashSet::from([Standard::NEM]));
        }
    }

    mod gauge_converter {
        use super::*;
        use common::length::Length;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_convert_gauges() {
            let row = ScaleRow {
                track_gauge: TrackGauge::Standard,
                gauge_inches: Some(dec!(0.65)),
                gauge_millimeters: Some(dec!(16.5)),
                ..default_row()
            };

            let result = Gauge::try_convert(&row);
            assert!(result.is_ok());

            let gauge = result.unwrap();
            assert_eq!(gauge.track_gauge, TrackGauge::Standard);
            assert_eq!(gauge.inches, Length::Inches(dec!(0.65)));
            assert_eq!(gauge.millimeters, Length::Millimeters(dec!(16.5)));
        }

        #[test]
        fn it_should_fail_to_convert_negative_gauges() {
            let row = ScaleRow {
                track_gauge: TrackGauge::Standard,
                gauge_inches: Some(dec!(-0.65)),
                gauge_millimeters: Some(dec!(-16.5)),
                ..default_row()
            };

            let result = Gauge::try_convert(&row);
            assert!(result.is_err());
        }
    }

    mod ratio_converter {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_convert_positive_ratios() {
            let row = ScaleRow {
                ratio: dec!(87.0),
                ..default_row()
            };

            let result = Ratio::try_convert(&row);

            assert!(result.is_ok());

            let ratio = result.unwrap();
            assert_eq!(ratio, Ratio::try_from(dec!(87.0)).unwrap());
        }

        #[test]
        fn it_should_fail_to_convert_negative_ratios() {
            let row = ScaleRow {
                ratio: dec!(-87.0),
                ..default_row()
            };

            let result = Ratio::try_convert(&row);

            assert!(result.is_err());
        }
    }

    mod description_converter {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_convert_description() {
            let row = ScaleRow {
                description_en: Some(String::from("description")),
                description_it: Some(String::from("descrizione")),
                ..default_row()
            };

            let result = LocalizedText::try_convert(&row);

            assert!(result.is_ok());

            let description = result.unwrap();
            assert_eq!(Some(&String::from("description")), description.english());
            assert_eq!(Some(&String::from("descrizione")), description.italian());
        }
    }

    mod metadata_converter {
        use super::*;
        use chrono::Utc;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_convert_metadata() {
            let now = Utc::now();
            let row = ScaleRow {
                created_at: now,
                version: 42,
                ..default_row()
            };

            let result = Metadata::try_convert(&row);

            assert!(result.is_ok());

            let metadata = result.unwrap();
            assert_eq!(&now, metadata.created());
            assert_eq!(None, metadata.last_modified());
            assert_eq!(42, metadata.version());
        }
    }
}
