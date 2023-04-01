use crate::railways::period_of_activity::{PeriodOfActivity, RailwayStatus};
use crate::railways::queries::railway_row::RailwayRow;
use crate::railways::railway::Railway;
use crate::railways::railway_gauge::RailwayGauge;
use crate::railways::railway_length::RailwayLength;
use common::contacts::ContactInformation;
use common::length::Length;
use common::localized_text::LocalizedText;
use common::measure_units::MeasureUnit;
use common::metadata::Metadata;
use common::queries::converters::{ConversionErrors, Converter, OptionConverter, ToOutputConverter};
use common::socials::Socials;
use isocountry::CountryCode;

impl ToOutputConverter<Railway> for RailwayRow {
    fn to_output(self) -> Result<Railway, ConversionErrors> {
        let row = self;

        let description = LocalizedText::try_convert(&row)?;
        let socials = Socials::try_convert(&row)?;
        let contact_info = ContactInformation::try_convert(&row)?;
        let metadata = Metadata::try_convert(&row)?;
        let period_of_activity = PeriodOfActivity::try_convert(&row)?;

        let gauge = RailwayGauge::try_convert(&row)?;

        let total_length = RailwayLength::try_convert(&row)?;

        let country = CountryCode::for_alpha2(&row.country).unwrap();

        Ok(Railway {
            railway_id: row.railway_id,
            name: row.name,
            abbreviation: row.abbreviation,
            registered_company_name: row.registered_company_name,
            organization_entity_type: row.organization_entity_type,
            description,
            country,
            period_of_activity,
            gauge,
            headquarters: row.headquarters,
            total_length,
            contact_info,
            socials,
            metadata,
        })
    }
}

impl OptionConverter<RailwayRow> for RailwayLength {
    fn try_convert(row: &RailwayRow) -> Result<Option<Self>, ConversionErrors> {
        match (row.total_length_km, row.total_length_mi) {
            (Some(km), Some(miles)) if km.is_sign_positive() && miles.is_sign_positive() => {
                Ok(Some(RailwayLength::new(km, miles)))
            }
            (Some(km), None) if km.is_sign_positive() => Ok(Some(RailwayLength::of_kilometers(km))),
            (None, Some(miles)) if miles.is_sign_positive() => Ok(Some(RailwayLength::of_miles(miles))),
            (None, None) => Ok(None),
            _ => Err(ConversionErrors::new()),
        }
    }
}

impl OptionConverter<RailwayRow> for RailwayGauge {
    fn try_convert(row: &RailwayRow) -> Result<Option<Self>, ConversionErrors> {
        match (row.track_gauge, row.gauge_meters) {
            (Some(track_gauge), Some(meters)) if meters.is_sign_positive() => Ok(Some(RailwayGauge {
                track_gauge,
                meters: Length::new(meters, MeasureUnit::Meters),
            })),
            (_gauge, Some(meters)) if meters.is_sign_negative() => Err(ConversionErrors::new()),
            _ => Ok(None),
        }
    }
}

impl Converter<RailwayRow> for LocalizedText {
    fn try_convert(value: &RailwayRow) -> Result<Self, ConversionErrors> {
        let mut localized_text = LocalizedText::default();

        localized_text.add_english(value.description_en.as_ref());
        localized_text.add_italian(value.description_it.as_ref());

        Ok(localized_text)
    }
}

impl OptionConverter<RailwayRow> for ContactInformation {
    fn try_convert(row: &RailwayRow) -> Result<Option<Self>, ConversionErrors> {
        match (&row.contact_email, &row.contact_phone, &row.contact_website_url) {
            (None, None, None) => Ok(None),
            (email, phone, website_url) => Ok(Some(ContactInformation {
                email: email.clone(),
                phone: phone.clone(),
                website_url: website_url.clone(),
            })),
        }
    }
}

impl OptionConverter<RailwayRow> for PeriodOfActivity {
    fn try_convert(row: &RailwayRow) -> Result<Option<Self>, ConversionErrors> {
        match (row.status, row.operating_since, row.operating_until) {
            (None, None, None) => Ok(None),
            (Some(status), None, None) => Ok(Some(PeriodOfActivity::new(None, None, status).unwrap())),
            (Some(RailwayStatus::Active), Some(since), None) => Ok(Some(PeriodOfActivity::active_railway(since))),
            (Some(RailwayStatus::Inactive), Some(since), Some(until)) if since < until => {
                Ok(Some(PeriodOfActivity::inactive_railway(since, until)))
            }
            _ => Err(ConversionErrors::new()),
        }
    }
}

impl OptionConverter<RailwayRow> for Socials {
    fn try_convert(value: &RailwayRow) -> Result<Option<Self>, ConversionErrors> {
        match (
            &value.socials_facebook,
            &value.socials_instagram,
            &value.socials_linkedin,
            &value.socials_youtube,
            &value.socials_twitter,
        ) {
            (None, None, None, None, None) => Ok(None),
            (facebook, instagram, linkedin, youtube, twitter) => Ok(Some(Socials {
                facebook: facebook.clone(),
                instagram: instagram.clone(),
                linkedin: linkedin.clone(),
                twitter: twitter.clone(),
                youtube: youtube.clone(),
            })),
        }
    }
}

impl Converter<RailwayRow> for Metadata {
    fn try_convert(value: &RailwayRow) -> Result<Self, ConversionErrors> {
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
    use crate::railways::queries::railway_row::test::new_railway_row;
    use chrono::Utc;

    fn default_row() -> RailwayRow {
        new_railway_row("FS", CountryCode::ITA, Utc::now())
    }

    mod contact_information_converter {
        use super::*;
        use crate::railways::queries::railway_row::RailwayRow;
        use crate::railways::queries::row_converters::test::default_row;
        use common::contacts::{ContactInformation, MailAddress, PhoneNumber, WebsiteUrl};

        #[test]
        fn it_should_return_a_none_when_the_contact_information_are_missing() {
            let row = RailwayRow { ..default_row() };

            let result = ContactInformation::try_convert(&row).expect("the contact information are invalid");
            assert!(result.is_none());
        }

        #[test]
        fn it_should_convert_contact_information_email() {
            let contact_email = Some(MailAddress::new("mail@mail.com"));
            let row = RailwayRow {
                contact_email: contact_email.clone(),
                ..default_row()
            };

            let result = ContactInformation::try_convert(&row).expect("the contact information are invalid");
            let contact_information = result.expect("the contact information are missing");

            assert_eq!(contact_email, contact_information.email);
            assert_eq!(None, contact_information.phone);
            assert_eq!(None, contact_information.website_url);
        }

        #[test]
        fn it_should_convert_contact_information_phone() {
            let contact_phone = Some(PhoneNumber::new("+39029566789"));
            let row = RailwayRow {
                contact_phone: contact_phone.clone(),
                ..default_row()
            };

            let result = ContactInformation::try_convert(&row).expect("the contact information are invalid");
            let contact_information = result.expect("the contact information are missing");

            assert_eq!(None, contact_information.email);
            assert_eq!(contact_phone, contact_information.phone);
            assert_eq!(None, contact_information.website_url);
        }

        #[test]
        fn it_should_convert_contact_information_website_url() {
            let contact_website_url = Some(WebsiteUrl::new("http://localhost"));
            let row = RailwayRow {
                contact_website_url: contact_website_url.clone(),
                ..default_row()
            };

            let result = ContactInformation::try_convert(&row).expect("the contact information are invalid");
            let contact_information = result.expect("the contact information are missing");

            assert_eq!(None, contact_information.email);
            assert_eq!(None, contact_information.phone);
            assert_eq!(contact_website_url, contact_information.website_url);
        }
    }

    mod metadata_converter {
        use super::*;
        use chrono::Utc;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_convert_metadata() {
            let now = Utc::now();
            let row = RailwayRow {
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

    mod period_of_activity_converter {
        use super::*;
        use chrono::NaiveDate;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_convert_inactive_railways() {
            let operating_since = NaiveDate::from_ymd_opt(1900, 1, 1);
            let operating_until = NaiveDate::from_ymd_opt(1990, 12, 31);

            let row = RailwayRow {
                status: Some(RailwayStatus::Inactive),
                operating_since,
                operating_until,
                ..default_row()
            };

            let result = PeriodOfActivity::try_convert(&row).expect("the period of activity is not valid");
            let period_of_activity = result.expect("the period of activity is not present");

            assert_eq!(RailwayStatus::Inactive, period_of_activity.status);
            assert_eq!(operating_since, period_of_activity.operating_since);
            assert_eq!(operating_until, period_of_activity.operating_until);
        }

        #[test]
        fn it_should_convert_active_railways() {
            let operating_since = NaiveDate::from_ymd_opt(1900, 1, 1);

            let row = RailwayRow {
                status: Some(RailwayStatus::Active),
                operating_since,
                operating_until: None,
                ..default_row()
            };

            let result = PeriodOfActivity::try_convert(&row).expect("the period of activity is not valid");
            let period_of_activity = result.expect("the period of activity is not present");

            assert_eq!(RailwayStatus::Active, period_of_activity.status);
            assert_eq!(operating_since, period_of_activity.operating_since);
            assert_eq!(None, period_of_activity.operating_until);
        }

        #[test]
        fn it_should_return_none_when_the_period_of_activity_is_missing() {
            let row = RailwayRow { ..default_row() };

            let result = PeriodOfActivity::try_convert(&row).expect("the period of activity is not valid");
            assert!(result.is_none());
        }

        #[test]
        fn it_should_return_an_error_when_until_date_is_before_the_since_date() {
            let operating_since = NaiveDate::from_ymd_opt(1900, 12, 31);
            let operating_until = NaiveDate::from_ymd_opt(1900, 12, 30);

            let row = RailwayRow {
                status: Some(RailwayStatus::Inactive),
                operating_since,
                operating_until,
                ..default_row()
            };

            let result = PeriodOfActivity::try_convert(&row);
            assert!(result.is_err());
        }
    }

    mod railway_length_converter {
        use super::*;
        use rust_decimal_macros::dec;

        #[test]
        fn it_should_convert_railway_length() {
            let miles = Length::Miles(dec!(621.371));
            let kilometers = Length::Kilometers(dec!(1000));
            let row = RailwayRow {
                total_length_mi: Some(miles.quantity()),
                total_length_km: Some(kilometers.quantity()),
                ..default_row()
            };

            let result = RailwayLength::try_convert(&row).expect("the railway length is invalid");
            let railway_length = result.expect("the railway length is missing");

            assert_eq!(miles, railway_length.miles);
            assert_eq!(kilometers, railway_length.kilometers);
        }

        #[test]
        fn it_should_convert_railway_length_in_miles_only() {
            let miles = Length::Miles(dec!(621.371));
            let kilometers = Length::Kilometers(dec!(999.99720514));
            let row = RailwayRow {
                total_length_mi: Some(miles.quantity()),
                ..default_row()
            };

            let result = RailwayLength::try_convert(&row).expect("the railway length is invalid");
            let railway_length = result.expect("the railway length is missing");

            assert_eq!(miles, railway_length.miles);
            assert_eq!(kilometers, railway_length.kilometers);
        }

        #[test]
        fn it_should_convert_railway_length_in_kilometers_only() {
            let miles = Length::Miles(dec!(621.371));
            let kilometers = Length::Kilometers(dec!(1000));
            let row = RailwayRow {
                total_length_km: Some(kilometers.quantity()),
                ..default_row()
            };

            let result = RailwayLength::try_convert(&row).expect("the railway length is invalid");
            let railway_length = result.expect("the railway length is missing");

            assert_eq!(miles, railway_length.miles);
            assert_eq!(kilometers, railway_length.kilometers);
        }

        #[test]
        fn it_should_fail_to_convert_negative_lengths() {
            let row = RailwayRow {
                total_length_km: Some(dec!(-1.0)),
                total_length_mi: Some(dec!(-1.0)),
                ..default_row()
            };

            let result = RailwayLength::try_convert(&row);
            assert!(result.is_err());
        }

        #[test]
        fn it_should_return_a_none_when_the_railway_length_is_missing() {
            let row = RailwayRow { ..default_row() };

            let result = RailwayLength::try_convert(&row).expect("the railway length is invalid");
            assert!(result.is_none());
        }
    }

    mod railway_gauge_converter {
        use super::*;
        use crate::common::TrackGauge;
        use pretty_assertions::assert_eq;
        use rust_decimal_macros::dec;

        #[test]
        fn it_should_convert_railway_gauges() {
            let row = RailwayRow {
                gauge_meters: Some(dec!(1.435)),
                track_gauge: Some(TrackGauge::Standard),
                ..default_row()
            };

            let result = RailwayGauge::try_convert(&row);
            assert!(result.is_ok());

            let result = result.unwrap();
            assert!(result.is_some());

            let railway_gauge = result.unwrap();
            assert_eq!(TrackGauge::Standard, railway_gauge.track_gauge);
            assert_eq!(Length::Meters(dec!(1.435)), railway_gauge.meters);
        }

        #[test]
        fn it_should_fail_to_convert_invalid_meters_values() {
            let row = RailwayRow {
                gauge_meters: Some(dec!(-1.0)),
                track_gauge: Some(TrackGauge::Standard),
                ..default_row()
            };

            let result = RailwayGauge::try_convert(&row);

            assert!(result.is_err());
        }
    }

    mod socials_converter {
        use super::*;
        use common::socials::Handler;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_return_a_none_when_there_are_no_socials_handler_in_the_row() {
            let row = RailwayRow { ..default_row() };

            let result = Socials::try_convert(&row);

            assert!(result.is_ok());

            let socials = result.unwrap();
            assert!(socials.is_none());
        }

        #[test]
        fn it_should_convert_socials() {
            let row = RailwayRow {
                socials_facebook: Some(Handler::new("facebook")),
                socials_instagram: Some(Handler::new("instagram")),
                socials_linkedin: Some(Handler::new("linkedin")),
                socials_youtube: Some(Handler::new("youtube")),
                socials_twitter: Some(Handler::new("twitter")),
                ..default_row()
            };

            let result = Socials::try_convert(&row);

            assert!(result.is_ok());

            let socials = result.unwrap();
            assert!(socials.is_some());

            let socials = socials.unwrap();
            assert_eq!(Some(Handler::new("facebook")), socials.facebook);
            assert_eq!(Some(Handler::new("instagram")), socials.instagram);
            assert_eq!(Some(Handler::new("linkedin")), socials.linkedin);
            assert_eq!(Some(Handler::new("youtube")), socials.youtube);
            assert_eq!(Some(Handler::new("twitter")), socials.twitter);
        }
    }
}
