//! the brand command requests

use crate::brands::brand_kind::BrandKind;
use crate::brands::brand_status::BrandStatus;
use common::address::Address;
use common::contacts::ContactInformation;
use common::localized_text::LocalizedText;
use common::organizations::OrganizationEntityType;
use common::socials::Socials;
use validator::Validate;

/// A request to create/update model railways brands
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, Validate)]
pub struct BrandRequest {
    /// the name
    #[validate(length(min = 3, max = 50))]
    pub name: String,
    /// the registered company name
    #[validate(length(max = 100))]
    pub registered_company_name: Option<String>,
    /// the organization entity type
    pub organization_entity_type: Option<OrganizationEntityType>,
    /// the group name in case the brand is part of a group
    #[validate(length(max = 100))]
    pub group_name: Option<String>,
    /// the description
    #[validate]
    pub description: LocalizedText,
    /// the brand main address
    #[validate]
    pub address: Option<Address>,
    /// the contact information
    #[validate]
    pub contact_info: Option<ContactInformation>,
    /// the brand kind
    pub kind: BrandKind,
    /// the brand status
    pub status: Option<BrandStatus>,
    /// the brand social profiles
    #[validate]
    pub socials: Option<Socials>,
}

#[cfg(test)]
mod test {
    use super::*;

    mod brand_request_validation {
        use super::*;
        use crate::test_helpers::random_str;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use validator::Validate;

        #[rstest]
        #[case(random_str(0))]
        #[case(random_str(1))]
        #[case(random_str(2))]
        #[case(random_str(51))]
        fn it_should_validate_the_brand_name(#[case] input: String) {
            let request = BrandRequest {
                name: input.clone(),
                ..BrandRequest::default()
            };

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert!(errors.contains_key("name"));
            assert_eq!(errors["name"].len(), 1);
            assert_eq!(errors["name"][0].code, "length");
            assert_eq!(errors["name"][0].params["value"], input);
            assert_eq!(errors["name"][0].params["min"], 3);
            assert_eq!(errors["name"][0].params["max"], 50);
        }

        #[rstest]
        #[case(random_str(101))]
        fn it_should_validate_the_registered_company_name(#[case] input: String) {
            let request = BrandRequest {
                name: "ABC".to_string(),
                registered_company_name: Some(input.clone()),
                ..BrandRequest::default()
            };

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert!(errors.contains_key("registered_company_name"));
            assert_eq!(errors["registered_company_name"].len(), 1);
            assert_eq!(errors["registered_company_name"][0].code, "length");
            assert_eq!(errors["registered_company_name"][0].params["value"], input);
            assert_eq!(errors["registered_company_name"][0].params["max"], 100);
        }

        #[rstest]
        #[case(random_str(101))]
        fn it_should_validate_the_group_name(#[case] input: String) {
            let request = BrandRequest {
                name: "ABC".to_string(),
                group_name: Some(input.clone()),
                ..BrandRequest::default()
            };

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert!(errors.contains_key("group_name"));
            assert_eq!(errors["group_name"].len(), 1);
            assert_eq!(errors["group_name"][0].code, "length");
            assert_eq!(errors["group_name"][0].params["value"], input);
            assert_eq!(errors["group_name"][0].params["max"], 100);
        }
    }
}
