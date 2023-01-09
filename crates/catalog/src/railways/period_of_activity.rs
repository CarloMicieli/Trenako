use chrono::NaiveDate;
use sqlx::Type;
use strum_macros;
use strum_macros::{Display, EnumString};
use thiserror::Error;

/// It represents the period of activity for a railway company
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct PeriodOfActivity {
    /// the date when the railway started its operation
    pub operating_since: Option<NaiveDate>,
    /// the date when the railway ended its operation, if not active anymore
    pub operating_until: Option<NaiveDate>,
    /// the railway status
    pub status: RailwayStatus,
}

impl PeriodOfActivity {
    /// Creates a new railway period of activity
    pub fn new(
        operating_since: Option<NaiveDate>,
        operating_until: Option<NaiveDate>,
        status: RailwayStatus,
    ) -> Result<Self, PeriodOfActivityError> {
        validate_dates(operating_since, operating_until)?;

        Ok(PeriodOfActivity {
            operating_since,
            operating_until,
            status,
        })
    }

    /// Creates a new active railway
    pub fn active_railway(operating_since: NaiveDate) -> Self {
        PeriodOfActivity::new(Some(operating_since), None, RailwayStatus::Active)
            .expect("the period of activity is not valid")
    }

    /// Creates a new inactive railway
    pub fn inactive_railway(operating_since: NaiveDate, operating_until: NaiveDate) -> Self {
        PeriodOfActivity::new(Some(operating_since), Some(operating_until), RailwayStatus::Inactive)
            .expect("the period of activity is not valid")
    }

    /// the moment since this railway has been active
    pub fn operating_since(&self) -> Option<&NaiveDate> {
        self.operating_since.as_ref()
    }

    /// the moment when the railway stopped to be active (if any)
    pub fn operating_until(&self) -> Option<&NaiveDate> {
        self.operating_until.as_ref()
    }

    /// the railway status
    pub fn status(&self) -> RailwayStatus {
        self.status
    }
}

fn validate_dates(
    operating_since: Option<NaiveDate>,
    operating_until: Option<NaiveDate>,
) -> Result<(), PeriodOfActivityError> {
    match (operating_until, operating_since) {
        (Some(since), Some(until)) => {
            if since > until {
                Ok(())
            } else {
                Err(PeriodOfActivityError::SinceDateAfterUntil)
            }
        }
        _ => Ok(()),
    }
}

#[derive(Debug, Eq, PartialEq, Error)]
pub enum PeriodOfActivityError {
    #[error("the operating since date must happen before of any until date")]
    SinceDateAfterUntil,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize, EnumString, Display, Type)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[sqlx(type_name = "railway_status", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RailwayStatus {
    Active,
    Inactive,
}

impl Default for RailwayStatus {
    fn default() -> Self {
        RailwayStatus::Active
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod periods_of_activity {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;

        #[test]
        fn it_should_create_new_active_periods_of_activity() {
            let start_date = NaiveDate::from_ymd_opt(1900, 12, 24).unwrap();
            let active = PeriodOfActivity::active_railway(start_date);
            assert_eq!(RailwayStatus::Active, active.status());
            assert_eq!(Some(&start_date), active.operating_since());
            assert_eq!(None, active.operating_until());
        }

        #[test]
        fn it_should_create_new_inactive_periods_of_activity() {
            let start_date = NaiveDate::from_ymd_opt(1900, 12, 24).unwrap();
            let end_date = NaiveDate::from_ymd_opt(2000, 12, 24).unwrap();
            let active = PeriodOfActivity::inactive_railway(start_date, end_date);
            assert_eq!(RailwayStatus::Inactive, active.status());
            assert_eq!(Some(&start_date), active.operating_since());
            assert_eq!(Some(&end_date), active.operating_until());
        }

        #[rstest]
        #[case(None, None, RailwayStatus::Active, Ok(PeriodOfActivity::default()))]
        #[case(
            d1900_12_25(),
            d1900_12_24(),
            RailwayStatus::Inactive,
            Err(PeriodOfActivityError::SinceDateAfterUntil)
        )]
        fn it_should_validate_the_inputs(
            #[case] since: Option<NaiveDate>,
            #[case] until: Option<NaiveDate>,
            #[case] railway_status: RailwayStatus,
            #[case] expected: Result<PeriodOfActivity, PeriodOfActivityError>,
        ) {
            let result = PeriodOfActivity::new(since, until, railway_status);
            assert_eq!(expected, result);
        }

        fn d1900_12_24() -> Option<NaiveDate> {
            Some(NaiveDate::from_ymd_opt(1900, 12, 24).unwrap())
        }

        fn d1900_12_25() -> Option<NaiveDate> {
            Some(NaiveDate::from_ymd_opt(1900, 12, 25).unwrap())
        }
    }

    mod railway_status {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use strum::ParseError;

        #[rstest]
        #[case("ACTIVE", Ok(RailwayStatus::Active))]
        #[case("INACTIVE", Ok(RailwayStatus::Inactive))]
        #[case("invalid", Err(ParseError::VariantNotFound))]
        fn it_should_parse_string_as_railway_status(
            #[case] input: &str,
            #[case] expected: Result<RailwayStatus, ParseError>,
        ) {
            let status = input.parse::<RailwayStatus>();
            assert_eq!(expected, status);
        }

        #[rstest]
        #[case(RailwayStatus::Active, "ACTIVE")]
        #[case(RailwayStatus::Inactive, "INACTIVE")]
        fn it_should_display_railway_status(#[case] input: RailwayStatus, #[case] expected: &str) {
            assert_eq!(expected, input.to_string());
        }
    }
}
