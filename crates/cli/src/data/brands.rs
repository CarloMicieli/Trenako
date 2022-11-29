use crate::data::common::{Contacts, Socials};

#[derive(Debug, Deserialize, Clone)]
pub struct Brand {
    pub name: String,
    #[serde(rename = "registeredCompanyName")]
    pub registered_company_name: String,
    #[serde(rename = "groupName")]
    pub group_name: String,
    pub description: String,
    pub address: Address,
    #[serde(rename = "contactInfo")]
    pub contact_info: Contacts,
    pub social: Socials,
    pub kind: String,
    pub status: String,
    pub version: u8,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Address {
    #[serde(rename = "streetAddress")]
    pub street_address: String,
    #[serde(rename = "extendedAddress")]
    pub extended_address: Option<String>,
    #[serde(rename = "postalCode")]
    pub postal_code: String,
    pub city: String,
    pub region: Option<String>,
    pub country: String,
}
