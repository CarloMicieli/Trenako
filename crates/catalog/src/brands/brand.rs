//! the brand view models

use crate::brands::brand_id::BrandId;
use crate::brands::brand_kind::BrandKind;
use crate::brands::brand_status::BrandStatus;
use common::address::Address;
use common::contacts::ContactInformation;
use common::localized_text::LocalizedText;
use common::metadata::Metadata;
use common::organizations::OrganizationEntityType;
use common::socials::Socials;
use std::{cmp, fmt};

/// It represents a model railways manufacturer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Brand {
    /// the brand unique identifier (an url encoded string)
    pub brand_id: BrandId,
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
    /// the brand metadata
    pub metadata: Metadata,
}

impl Brand {
    /// Creates a new modelling rail brand
    pub fn new(
        brand_id: BrandId,
        name: &str,
        registered_company_name: Option<&str>,
        organization_entity_type: Option<OrganizationEntityType>,
        group_name: Option<&str>,
        description: Option<&str>,
        address: Option<Address>,
        contact_info: Option<ContactInformation>,
        kind: BrandKind,
        status: Option<BrandStatus>,
        socials: Option<Socials>,
        metadata: Metadata,
    ) -> Self {
        Brand {
            brand_id,
            name: String::from(name),
            registered_company_name: registered_company_name.map(String::from),
            organization_entity_type,
            group_name: group_name.map(String::from),
            description: description.map(LocalizedText::with_italian).unwrap_or_default(),
            address,
            contact_info,
            kind,
            status,
            socials,
            metadata,
        }
    }

    /// this brand unique identifier (an url encoded string)
    pub fn brand_id(&self) -> &BrandId {
        &self.brand_id
    }

    /// this brand name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// this brand description
    pub fn description(&self) -> Option<&String> {
        self.description.italian()
    }

    /// this brand registered company name
    pub fn registered_company_name(&self) -> Option<&String> {
        self.registered_company_name.as_ref()
    }

    /// the organization entity type
    pub fn organization_entity_type(&self) -> Option<OrganizationEntityType> {
        self.organization_entity_type
    }

    /// this brand group name (if any)
    pub fn group_name(&self) -> Option<&String> {
        self.group_name.as_ref()
    }

    /// the contact information (email, phone, website url)
    pub fn contact_info(&self) -> Option<&ContactInformation> {
        self.contact_info.as_ref()
    }

    /// the postal address
    pub fn address(&self) -> Option<&Address> {
        self.address.as_ref()
    }

    /// this brand status
    pub fn status(&self) -> Option<&BrandStatus> {
        self.status.as_ref()
    }

    /// this brand kind
    pub fn kind(&self) -> BrandKind {
        self.kind
    }

    /// the social profiles
    pub fn socials(&self) -> Option<&Socials> {
        self.socials.as_ref()
    }

    /// the metadata
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

impl fmt::Display for Brand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl cmp::PartialEq for Brand {
    fn eq(&self, other: &Self) -> bool {
        self.brand_id.eq(&other.brand_id)
    }
}

impl cmp::Eq for Brand {}

#[cfg(test)]
mod tests {
    use super::*;

    mod brands {
        use super::*;
        use crate::brands::test_data::{acme, roco};
        use chrono::{DateTime, Utc};
        use common::contacts::{MailAddress, WebsiteUrl};
        use isocountry::CountryCode;
        use pretty_assertions::{assert_eq, assert_ne};

        #[test]
        fn it_should_create_brands() {
            let now: DateTime<Utc> = Utc::now();
            let address = Address::builder()
                .street_address("Viale Lombardia, 27")
                .postal_code("20131")
                .city("Milano")
                .region("MI")
                .country(CountryCode::ITA)
                .build()
                .unwrap();

            let contact_info = ContactInformation::new(
                Some(MailAddress::new("mail@acmetreni.com")),
                Some(WebsiteUrl::new("http://www.acmetreni.com")),
                None,
            );

            let socials = Socials::builder().facebook("Acmetreni").build().unwrap();

            let brand = Brand::new(
                BrandId::new("ACME"),
                "ACME",
                Some("Associazione Costruzioni Modellistiche Esatte"),
                Some(OrganizationEntityType::LimitedCompany),
                None,
                None,
                Some(address.clone()),
                Some(contact_info.clone()),
                BrandKind::Industrial,
                Some(BrandStatus::Active),
                Some(socials.clone()),
                Metadata::created_at(now),
            );

            assert_eq!("ACME", brand.to_string());

            assert_eq!(&BrandId::new("ACME"), brand.brand_id());
            assert_eq!("ACME", brand.name());
            assert_eq!(
                Some(&"Associazione Costruzioni Modellistiche Esatte".to_string()),
                brand.registered_company_name()
            );
            assert_eq!(
                Some(OrganizationEntityType::LimitedCompany),
                brand.organization_entity_type()
            );
            assert_eq!(None, brand.group_name());
            assert_eq!(None, brand.description());
            assert_eq!(BrandKind::Industrial, brand.kind());
            assert_eq!(Some(&BrandStatus::Active), brand.status());
            assert_eq!(Some(&address), brand.address());
            assert_eq!(Some(&contact_info), brand.contact_info());
            assert_eq!(Some(&socials), brand.socials());
            assert_eq!(Metadata::created_at(now), brand.metadata);
        }

        #[test]
        fn it_should_compare_two_brands() {
            let acme = acme();
            let roco = roco();

            assert_eq!(acme, acme);
            assert_ne!(acme, roco);
        }

        #[test]
        fn it_should_display_brands() {
            assert_eq!("ACME", acme().to_string());
        }
    }
}
