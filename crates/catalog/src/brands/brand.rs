use crate::brands::brand_id::BrandId;
use crate::brands::brand_status::BrandStatus;
use crate::brands::brand_type::BrandType;
use common::address::Address;
use common::metadata::Metadata;
use common::socials::Socials;
use std::fmt;

/// A model railways manufacturer.
#[derive(Debug)]
pub struct Brand {
    brand_id: BrandId,
    name: String,
    registered_company_name: Option<String>,
    group_name: Option<String>,
    description: Option<String>,
    address: Option<Address>,
    brand_type: BrandType,
    status: BrandStatus,
    socials: Option<Socials>,
    metadata: Metadata,
}

impl Brand {
    pub fn new(
        brand_id: BrandId,
        name: &str,
        registered_company_name: Option<&str>,
        group_name: Option<&str>,
        description: Option<&str>,
        address: Option<Address>,
        brand_type: BrandType,
        status: BrandStatus,
        socials: Option<Socials>,
        metadata: Metadata,
    ) -> Self {
        Brand {
            brand_id,
            name: String::from(name),
            registered_company_name: registered_company_name.map(|s| String::from(s)),
            group_name: group_name.map(|s| String::from(s)),
            description: description.map(|s| String::from(s)),
            address,
            brand_type,
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

    /// Returns this brand group name (if any)
    pub fn group_name(&self) -> Option<&String> {
        self.group_name.as_ref()
    }

    /// Returns this brand type
    pub fn brand_type(&self) -> BrandType {
        self.brand_type
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

impl PartialEq for Brand {
    fn eq(&self, other: &Self) -> bool {
        self.brand_id.eq(&other.brand_id)
    }
}

impl Eq for Brand {}

#[cfg(test)]
mod tests {
    use super::*;

    mod brands {
        use super::*;
        use chrono::{DateTime, Utc};
        use isocountry::CountryCode;
        use pretty_assertions::assert_eq;

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

            let socials = Socials::builder().facebook("Acmetreni").build();

            let brand = Brand::new(
                BrandId::new("ACME"),
                "ACME",
                Some("Associazione Costruzioni Modellistiche Esatte"),
                None,
                None,
                Some(address.clone()),
                BrandType::Industrial,
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
            assert_eq!(None, brand.group_name());
            assert_eq!(None, brand.description());
            assert_eq!(BrandType::Industrial, brand.brand_type());
            assert_eq!(BrandStatus::Active, brand.status());
            assert_eq!(Some(&address), brand.address());
            assert_eq!(Some(&socials), brand.socials());
            assert_eq!(Metadata::created_at(now), brand.metadata);
        }
    }
}
