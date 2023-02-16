use crate::railways::period_of_activity::PeriodOfActivity;
use crate::railways::railway_gauge::RailwayGauge;
use crate::railways::railway_length::RailwayLength;
use common::contacts::ContactInformation;
use common::localized_text::LocalizedText;
use common::organizations::OrganizationEntityType;
use common::socials::Socials;
use isocountry::CountryCode;
use validator::Validate;

/// A request to create/update railways
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Validate)]
pub struct RailwayRequest {
    /// the railway name
    #[validate(length(min = 2, max = 25))]
    pub name: String,
    /// the railway abbreviated name
    #[validate(length(min = 2, max = 10))]
    pub abbreviation: Option<String>,
    /// the registered company name
    #[validate(length(max = 100))]
    pub registered_company_name: Option<String>,
    /// the organization entity type
    pub organization_entity_type: Option<OrganizationEntityType>,
    /// the railway description
    #[validate]
    pub description: LocalizedText,
    /// the registration country
    #[validate(required)]
    pub country: Option<CountryCode>,
    /// the period of activity
    #[validate(custom = "crate::railways::period_of_activity::validate_period_of_activity")]
    pub period_of_activity: Option<PeriodOfActivity>,
    /// the track gauge
    #[validate]
    pub gauge: Option<RailwayGauge>,
    /// the railway headquarter
    #[validate(length(max = 100))]
    pub headquarters: Option<String>,
    /// the railway total length
    #[validate]
    pub total_length: Option<RailwayLength>,
    /// the contacts information
    #[validate]
    pub contact_info: Option<ContactInformation>,
    /// the social profiles
    #[validate]
    pub socials: Option<Socials>,
}

#[cfg(test)]
mod test {
    mod railway_request_validation {
        use crate::railways::railway_request::RailwayRequest;
        use crate::test_helpers::random_str;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use validator::Validate;

        #[rstest]
        #[case(random_str(0))]
        #[case(random_str(1))]
        #[case(random_str(26))]
        fn it_should_validate_the_railway_name(#[case] input: String) {
            let request = RailwayRequest {
                name: input.clone(),
                ..RailwayRequest::default()
            };

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert!(errors.contains_key("name"));
            assert_eq!(errors["name"].len(), 1);
            assert_eq!(errors["name"][0].code, "length");
            assert_eq!(errors["name"][0].params["value"], input);
            assert_eq!(errors["name"][0].params["min"], 2);
            assert_eq!(errors["name"][0].params["max"], 25);
        }

        #[test]
        fn it_should_validate_the_railway_country() {
            let request = RailwayRequest::default();

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert!(errors.contains_key("country"));
            assert_eq!(errors["country"].len(), 1);
            assert_eq!(errors["country"][0].code, "required");
        }

        #[rstest]
        #[case(random_str(101))]
        fn it_should_validate_the_registered_company_name(#[case] input: String) {
            let request = RailwayRequest {
                name: "ABC".to_string(),
                registered_company_name: Some(input.clone()),
                ..RailwayRequest::default()
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
        fn it_should_validate_the_abbreviation(#[case] input: String) {
            let request = RailwayRequest {
                name: "ABC".to_string(),
                abbreviation: Some(input.clone()),
                ..RailwayRequest::default()
            };

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert!(errors.contains_key("abbreviation"));
            assert_eq!(errors["abbreviation"].len(), 1);
            assert_eq!(errors["abbreviation"][0].code, "length");
            assert_eq!(errors["abbreviation"][0].params["value"], input);
            assert_eq!(errors["abbreviation"][0].params["min"], 2);
            assert_eq!(errors["abbreviation"][0].params["max"], 10);
        }
    }
}
