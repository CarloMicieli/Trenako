use crate::data::common::{Contacts, Socials};
use rust_decimal::Decimal;

#[derive(Debug, Deserialize, Clone)]
pub struct Scale {
    pub name: String,
    #[serde(rename = "registeredCompanyName")]
    pub registered_company_name: String,
    pub country: String,
    #[serde(rename = "periodOfActivity")]
    pub period_of_activity: PeriodOfActivity,
    pub length: Length,
    pub gauge: Gauge,
    #[serde(rename = "contactInfo")]
    pub contact_info: Contacts,
    pub socials: Socials,
    pub headquarters: Option<String>,
    pub version: u8,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PeriodOfActivity {
    #[serde(rename = "operatingSince")]
    pub operating_since: String,
    #[serde(rename = "operatingUntil")]
    pub operating_until: Option<String>,
    pub status: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Gauge {
    #[serde(rename = "trackGauge")]
    pub track_gauge: String,
    pub meters: Decimal,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Length {
    pub kilometers: Option<Decimal>,
    pub miles: Option<Decimal>,
}
