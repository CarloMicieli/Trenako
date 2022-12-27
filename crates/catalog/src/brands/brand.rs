use crate::brands::brand_id::BrandId;
use crate::brands::brand_kind::BrandKind;
use crate::brands::brand_status::BrandStatus;
use common::address::Address;
use common::contact::ContactInfo;
use common::metadata::Metadata;
use common::organizations::OrganizationEntityType;
use common::socials::Socials;
use std::{cmp, fmt};

/// A model railways manufacturer.
#[derive(Debug)]
pub struct Brand {
    brand_id: BrandId,
    name: String,
    registered_company_name: Option<String>,
    organization_entity_type: Option<OrganizationEntityType>,
    group_name: Option<String>,
    description: Option<String>,
    address: Option<Address>,
    contact_info: Option<ContactInfo>,
    kind: BrandKind,
    status: BrandStatus,
    socials: Option<Socials>,
    metadata: Metadata,
}

impl Brand {
    pub fn new(
        brand_id: BrandId,
        name: &str,
        registered_company_name: Option<&str>,
        organization_entity_type: Option<OrganizationEntityType>,
        group_name: Option<&str>,
        description: Option<&str>,
        address: Option<Address>,
        contact_info: Option<ContactInfo>,
        kind: BrandKind,
        status: BrandStatus,
        socials: Option<Socials>,
        metadata: Metadata,
    ) -> Self {
        Brand {
            brand_id,
            name: String::from(name),
            registered_company_name: registered_company_name.map(String::from),
            organization_entity_type,
            group_name: group_name.map(String::from),
            description: description.map(String::from),
            address,
            contact_info,
            kind,
            status,
            socials,
            metadata,
        }
    }

    /// Returns this brand unique identifier
    pub fn brand_id(&self) -> &BrandId {
        &self.brand_id
    }

    /// Returns this brand name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns this brand description
    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Returns this brand registered company name
    pub fn registered_company_name(&self) -> Option<&String> {
        self.registered_company_name.as_ref()
    }

    /// Returns the organization entity type for this brand
    pub fn organization_entity_type(&self) -> Option<OrganizationEntityType> {
        self.organization_entity_type
    }

    /// Returns this brand group name (if any)
    pub fn group_name(&self) -> Option<&String> {
        self.group_name.as_ref()
    }

    /// Returns this brand kind
    pub fn kind(&self) -> BrandKind {
        self.kind
    }

    /// Returns the contact info for this brand
    pub fn contact_info(&self) -> Option<&ContactInfo> {
        self.contact_info.as_ref()
    }

    /// Returns the postal address for this brand
    pub fn address(&self) -> Option<&Address> {
        self.address.as_ref()
    }

    /// Returns this brand status
    pub fn status(&self) -> BrandStatus {
        self.status
    }

    /// Returns the social handlers for this brand
    pub fn socials(&self) -> Option<&Socials> {
        self.socials.as_ref()
    }

    /// Returns the metadata for this brand
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
        use common::contact::{MailAddress, WebsiteUrl};
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
                .country_code(CountryCode::ITA)
                .build()
                .unwrap();

            let contact_info = ContactInfo::new(
                Some(MailAddress::new("mail@acmetreni.com")),
                Some(WebsiteUrl::new("http://www.acmetreni.com")),
                None,
            );

            let socials = Socials::builder().facebook("Acmetreni").build();

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
                BrandStatus::Active,
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
            assert_eq!(BrandStatus::Active, brand.status());
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
