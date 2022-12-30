use chrono::NaiveDate;
use sqlx::Type;
use strum_macros;
use strum_macros::{Display, EnumString};

/// It represents the period of activity for a railway company
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct PeriodOfActivity {
    operating_since: Date,
    operating_until: Option<Date>,
    status: RailwayStatus,
}

impl PeriodOfActivity {
    /// Creates a new railway period of activity
    pub fn new(operating_since: Date, operating_until: Option<Date>, status: RailwayStatus) -> Self {
        PeriodOfActivity {
            operating_since,
            operating_until,
            status,
        }
    }

    /// Creates a new active railway
    pub fn active_railway(operating_since: Date) -> Self {
        PeriodOfActivity {
            operating_since,
            operating_until: None,
            status: RailwayStatus::Active,
        }
    }

    /// Creates a new inactive railway
    pub fn inactive_railway(operating_since: Date, operating_until: Date) -> Self {
        PeriodOfActivity {
            operating_since,
            operating_until: Some(operating_until),
            status: RailwayStatus::Inactive,
        }
    }

    /// The moment since this railway has been active
    pub fn operating_since(&self) -> &Date {
        &self.operating_since
    }

    /// The moment when the railway stopped to be active (if any)
    pub fn operating_until(&self) -> Option<&Date> {
        self.operating_until.as_ref()
    }

    /// The railway status
    pub fn status(&self) -> RailwayStatus {
        self.status
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum Date {
    Year(u32),
    ExactDay(NaiveDate),
}

impl Date {
    pub fn with_year(year: u32) -> Self {
        Date::Year(year)
    }

    pub fn with_exact_day(date: NaiveDate) -> Self {
        Date::ExactDay(date)
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize, EnumString, Display, Type)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[sqlx(type_name = "railway_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RailwayStatus {
    Active,
    Inactive,
}

#[cfg(test)]
mod test {
    use super::*;

    mod periods_of_activity {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_new_active_periods_of_activity() {
            let active = PeriodOfActivity::active_railway(Date::with_year(1900));
            assert_eq!(RailwayStatus::Active, active.status());
            assert_eq!(&Date::with_year(1900), active.operating_since());
            assert_eq!(None, active.operating_until());
        }

        #[test]
        fn it_should_create_new_inactive_periods_of_activity() {
            let end_date = NaiveDate::from_ymd_opt(2000, 12, 24).unwrap();
            let active = PeriodOfActivity::inactive_railway(Date::with_year(1900), Date::ExactDay(end_date));
            assert_eq!(RailwayStatus::Inactive, active.status());
            assert_eq!(&Date::with_year(1900), active.operating_since());
            assert_eq!(Some(&Date::ExactDay(end_date)), active.operating_until());
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
