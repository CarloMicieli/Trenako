//! the scale command requests

use crate::scales::ratio::Ratio;
use crate::scales::scale_gauge::Gauge;
use crate::scales::standard::Standard;
use common::localized_text::LocalizedText;
use validator::Validate;

/// A request to create/update rail transport modelling scales
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, Validate)]
pub struct ScaleRequest {
    /// the scale name
    #[validate(length(min = 1, max = 50))]
    pub name: String,
    /// the ratio between the real world and the model (e.g. 1/87 or 1:87)
    #[validate(required, custom = "crate::scales::ratio::validate_ratio")]
    pub ratio: Option<Ratio>,
    /// the track gauge
    #[validate(required)]
    pub gauge: Option<Gauge>,
    /// the modelling scale description
    #[validate]
    pub description: LocalizedText,
    /// the list of standards
    #[serde(default)]
    pub standards: Vec<Standard>,
}

#[cfg(test)]
mod test {
    mod scale_request_validation {
        use crate::scales::scale_request::ScaleRequest;
        use crate::test_helpers::random_str;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use validator::Validate;

        #[rstest]
        #[case(random_str(0))]
        #[case(random_str(51))]
        fn it_should_validate_the_scale_name(#[case] input: String) {
            let request = ScaleRequest {
                name: input.clone(),
                ..ScaleRequest::default()
            };

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert!(errors.contains_key("name"));
            assert_eq!(errors["name"].len(), 1);
            assert_eq!(errors["name"][0].code, "length");
            assert_eq!(errors["name"][0].params["value"], input);
            assert_eq!(errors["name"][0].params["min"], 1);
            assert_eq!(errors["name"][0].params["max"], 50);
        }

        #[test]
        fn it_should_validate_the_scale_gauge() {
            let request = ScaleRequest::default();

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert!(errors.contains_key("gauge"));
            assert_eq!(errors["gauge"].len(), 1);
            assert_eq!(errors["gauge"][0].code, "required");
        }

        #[test]
        fn it_should_validate_the_scale_ratio() {
            let request = ScaleRequest::default();

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert!(errors.contains_key("ratio"));
            assert_eq!(errors["ratio"].len(), 1);
            assert_eq!(errors["ratio"][0].code, "required");
        }
    }
}
