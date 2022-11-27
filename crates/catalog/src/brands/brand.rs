use crate::brands::brand_id::BrandId;
use crate::brands::brand_status::BrandStatus;
use crate::brands::brand_type::BrandType;
use std::fmt;

/// A model railways manufacturer.
#[derive(Debug)]
pub struct Brand {
    brand_id: BrandId,
    name: String,
    registered_company_name: Option<String>,
    group_name: Option<String>,
    description: Option<String>,
    brand_type: BrandType,
    status: BrandStatus,
}

impl Brand {
    pub fn new(
        brand_id: BrandId,
        name: &str,
        registered_company_name: Option<&str>,
        group_name: Option<&str>,
        description: Option<&str>,
        brand_type: BrandType,
        status: BrandStatus,
    ) -> Self {
        Brand {
            brand_id,
            name: String::from(name),
            registered_company_name: registered_company_name.map(|s| String::from(s)),
            group_name: group_name.map(|s| String::from(s)),
            description: description.map(|s| String::from(s)),
            brand_type,
            status,
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

    /// Returns this brand status
    pub fn status(&self) -> BrandStatus {
        self.status
    }
}

impl fmt::Display for Brand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod brands {
        use super::*;

        #[test]
        fn it_should_create_brands() {
            let brand = Brand::new(
                BrandId::new("ACME"),
                "ACME",
                Some("Associazione Costruzioni Modellistiche Esatte"),
                None,
                None,
                BrandType::Industrial,
                BrandStatus::Active,
            );

            assert_eq!("ACME", brand.to_string());

            assert_eq!(BrandId::new("ACME"), brand.brand_id);
            assert_eq!("ACME", brand.name);
            assert_eq!(
                Some("Associazione Costruzioni Modellistiche Esatte".to_string()),
                brand.registered_company_name
            );
            assert_eq!(None, brand.group_name);
            assert_eq!(None, brand.description);
            assert_eq!(BrandType::Industrial, brand.brand_type);
            assert_eq!(BrandStatus::Active, brand.status);
        }
    }
}
