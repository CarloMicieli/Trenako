use crate::railways::period_of_activity::PeriodOfActivity;
use crate::railways::railway_gauge::RailwayGauge;
use crate::railways::railway_length::RailwayLength;
use common::contacts::ContactInformation;
use common::localized_text::LocalizedText;
use common::organizations::OrganizationEntityType;
use common::socials::Socials;
use isocountry::CountryCode;

/// A request to create/update railways
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RailwayRequest {
    /// the railway name
    pub name: String,
    /// the railway abbreviated name
    pub abbreviation: Option<String>,
    /// the registered company name
    pub registered_company_name: Option<String>,
    /// the organization entity type
    pub organization_entity_type: Option<OrganizationEntityType>,
    /// the railway description
    pub description: LocalizedText,
    /// the registration country
    pub country: CountryCode,
    /// the period of activity
    pub period_of_activity: Option<PeriodOfActivity>,
    /// the track gauge
    pub gauge: Option<RailwayGauge>,
    /// the railway headquarter
    pub headquarters: Option<String>,
    /// the railway total length
    pub total_length: Option<RailwayLength>,
    /// the contacts information
    pub contact_info: Option<ContactInformation>,
    /// the social profiles
    pub socials: Option<Socials>,
}
