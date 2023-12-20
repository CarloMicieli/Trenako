use catalog::common::TrackGauge;
use catalog::railways::period_of_activity::RailwayStatus;
use catalog::railways::railway_id::RailwayId;
use chrono::{DateTime, NaiveDate, Utc};
use common::contacts::{MailAddress, PhoneNumber, WebsiteUrl};
use common::organizations::OrganizationEntityType;
use common::socials::Handler;
use rust_decimal::Decimal;

#[derive(Debug)]
pub struct RailwayRow {
    pub railway_id: RailwayId,
    pub name: String,
    pub abbreviation: Option<String>,
    pub registered_company_name: Option<String>,
    pub organization_entity_type: Option<OrganizationEntityType>,
    pub description_de: Option<String>,
    pub description_en: Option<String>,
    pub description_fr: Option<String>,
    pub description_it: Option<String>,
    pub country: String,
    pub operating_since: Option<NaiveDate>,
    pub operating_until: Option<NaiveDate>,
    pub status: Option<RailwayStatus>,
    pub gauge_meters: Option<Decimal>,
    pub track_gauge: Option<TrackGauge>,
    pub headquarters: Vec<String>,
    pub total_length_mi: Option<Decimal>,
    pub total_length_km: Option<Decimal>,
    pub contact_email: Option<MailAddress>,
    pub contact_website_url: Option<WebsiteUrl>,
    pub contact_phone: Option<PhoneNumber>,
    pub socials_facebook: Option<Handler>,
    pub socials_instagram: Option<Handler>,
    pub socials_linkedin: Option<Handler>,
    pub socials_twitter: Option<Handler>,
    pub socials_youtube: Option<Handler>,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub last_modified_at: Option<DateTime<Utc>>,
}

#[cfg(test)]
pub mod test {
    use super::*;
    use isocountry::CountryCode;

    #[allow(dead_code)]
    pub fn new_railway_row(name: &str, country: CountryCode, created_at: DateTime<Utc>) -> RailwayRow {
        RailwayRow {
            railway_id: RailwayId::new(name),
            name: name.to_owned(),
            abbreviation: None,
            registered_company_name: None,
            organization_entity_type: None,
            description_de: None,
            description_en: None,
            description_fr: None,
            description_it: None,
            country: country.alpha2().to_owned(),
            operating_since: None,
            operating_until: None,
            status: None,
            gauge_meters: None,
            track_gauge: None,
            headquarters: Vec::default(),
            total_length_mi: None,
            total_length_km: None,
            contact_email: None,
            contact_website_url: None,
            contact_phone: None,
            socials_facebook: None,
            socials_instagram: None,
            socials_linkedin: None,
            socials_twitter: None,
            socials_youtube: None,
            version: 1,
            created_at,
            last_modified_at: None,
        }
    }
}
