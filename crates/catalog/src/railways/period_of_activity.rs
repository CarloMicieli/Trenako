use chrono::NaiveDate;

/// It represents the period of activity for a railway company
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeriodOfActivity {
    operating_since: Date,
    operating_until: Option<Date>,
    status: RailwayStatus,
}

impl PeriodOfActivity {
    pub fn new(operating_since: Date, operating_until: Option<Date>, status: RailwayStatus) -> Self {
        PeriodOfActivity {
            operating_since,
            operating_until,
            status,
        }
    }

    pub fn active_railway(operating_since: Date) -> Self {
        PeriodOfActivity {
            operating_since,
            operating_until: None,
            status: RailwayStatus::Active,
        }
    }

    pub fn inactive_railway(operating_since: Date, operating_until: Date) -> Self {
        PeriodOfActivity {
            operating_since,
            operating_until: Some(operating_until),
            status: RailwayStatus::Inactive,
        }
    }

    pub fn operating_since(&self) -> &Date {
        &self.operating_since
    }

    pub fn operating_until(&self) -> Option<&Date> {
        self.operating_until.as_ref()
    }

    pub fn status(&self) -> RailwayStatus {
        self.status
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
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

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
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
}
