use crate::brands::brand_kind::BrandKind;
use crate::brands::brand_status::BrandStatus;
use common::address::Address;
use common::contacts::ContactInformation;
use common::organizations::OrganizationEntityType;
use common::socials::Socials;

/// A request to create/update model railways brands
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct BrandRequest {
    pub name: String,
    pub registered_company_name: Option<String>,
    pub organization_entity_type: Option<OrganizationEntityType>,
    pub group_name: Option<String>,
    pub description: Option<String>,
    pub contact_info: Option<ContactInformation>,
    pub address: Option<Address>,
    pub socials: Option<Socials>,
    pub kind: BrandKind,
    pub status: Option<BrandStatus>,
}
