use crate::railways::period_of_activity::PeriodOfActivity;
use crate::railways::railway_gauge::RailwayGauge;
use crate::railways::railway_length::RailwayLength;
use common::contact::ContactInfo;
use common::organizations::OrganizationEntityType;
use common::socials::Socials;

/// A request to create/update railways
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct RailwayRequest {
    pub name: String,
    pub registered_company_name: Option<String>,
    pub organization_entity_type: Option<OrganizationEntityType>,
    pub description: Option<String>,
    pub country: String,
    pub period_of_activity: Option<PeriodOfActivity>,
    pub gauge: Option<RailwayGauge>,
    pub headquarters: Option<String>,
    pub total_length: Option<RailwayLength>,
    pub contact_info: Option<ContactInfo>,
    pub socials: Option<Socials>,
}
