use crate::brands::brand_kind::BrandKind;
use crate::brands::brand_status::BrandStatus;
use common::address::Address;
use common::contacts::ContactInformation;
use common::localized_text::LocalizedText;
use common::organizations::OrganizationEntityType;
use common::socials::Socials;

/// A request to create/update model railways brands
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BrandRequest {
    /// the name
    pub name: String,
    /// the registered company name
    pub registered_company_name: Option<String>,
    /// the organization entity type
    pub organization_entity_type: Option<OrganizationEntityType>,
    /// the group name in case the brand is part of a group
    pub group_name: Option<String>,
    /// the description
    pub description: LocalizedText,
    /// the brand main address
    pub address: Option<Address>,
    /// the contact information
    pub contact_info: Option<ContactInformation>,
    /// the brand kind
    pub kind: BrandKind,
    /// the brand status
    pub status: Option<BrandStatus>,
    /// the brand social profiles
    pub socials: Option<Socials>,
}
