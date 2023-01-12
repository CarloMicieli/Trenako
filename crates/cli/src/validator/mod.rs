use crate::dataset::{Dataset, Resource};
use crate::schemas;
use jsonschema::{Draft, JSONSchema};
use serde_json::Value;
use std::str::FromStr;
use thiserror::Error;

pub fn validate_dataset(dataset: Dataset) -> Result<bool, ValidatorError> {
    let validators = Validators::new()?;

    for brand in dataset.brands {
        validators.validate_brand(&brand);
    }

    for catalog_item in dataset.catalog_items {
        validators.validate_catalog_item(&catalog_item);
    }

    for railway in dataset.railways {
        validators.validate_railway(&railway);
    }

    for scale in dataset.scales {
        validators.validate_scale(&scale);
    }

    Ok(true)
}

/// It represents a resource validator, against a JSON schema.
pub struct Validator(JSONSchema);

impl Validator {
    /// Creates a new schema validator for the dataset resources
    pub fn new(schema: &str) -> Result<Self, ValidatorError> {
        let json_schema = json_schema_from_str(schema)?;
        Ok(Validator(json_schema))
    }

    /// Validate the resource with the current validator schema
    pub fn validate(&self, input: &Resource) -> Result<bool, ValidatorError> {
        let input_json = Value::from_str(&input.content)?;
        let result = self.0.validate(&input_json);
        let result = if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
            false
        } else {
            true
        };
        Ok(result)
    }
}

#[derive(Debug, Error)]
pub enum ValidatorError {
    #[error("invalid json")]
    InvalidJson(#[from] serde_json::Error),

    #[error("invalid json schema")]
    InvalidSchema,
}

pub struct Validators {
    brands: Validator,
    catalog_items: Validator,
    railways: Validator,
    scales: Validator,
}

impl Validators {
    /// Creates a new schema validator for the dataset resources
    pub fn new() -> Result<Self, ValidatorError> {
        let brands = Validator::new(schemas::BRANDS_SCHEMA)?;
        let catalog_items = Validator::new(schemas::CATALOG_ITEMS_SCHEMA)?;
        let railways = Validator::new(schemas::RAILWAYS_SCHEMA)?;
        let scales = Validator::new(schemas::SCALES_SCHEMA)?;

        Ok(Self {
            brands,
            catalog_items,
            railways,
            scales,
        })
    }

    pub fn validate_brand(&self, input: &Resource) -> bool {
        self.brands.validate(input).unwrap()
    }

    pub fn validate_catalog_item(&self, input: &Resource) -> bool {
        self.catalog_items.validate(input).unwrap()
    }

    pub fn validate_railway(&self, input: &Resource) -> bool {
        self.railways.validate(input).unwrap()
    }

    pub fn validate_scale(&self, input: &Resource) -> bool {
        self.scales.validate(input).unwrap()
    }
}

fn json_schema_from_str(input: &str) -> Result<JSONSchema, ValidatorError> {
    let schema = serde_json::from_str(input)?;
    let compiled = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema)
        .map_err(|_| ValidatorError::InvalidSchema);
    compiled
}

#[cfg(test)]
mod test {
    use super::*;

    mod validator {
        use super::*;
        use crate::dataset::ResourceType;

        #[test]
        fn it_should_parse_the_json_schemas() {
            assert!(json_schema_from_str(schemas::BRANDS_SCHEMA).is_ok());
            assert!(json_schema_from_str(schemas::CATALOG_ITEMS_SCHEMA).is_ok());
            assert!(json_schema_from_str(schemas::RAILWAYS_SCHEMA).is_ok());
            assert!(json_schema_from_str(schemas::SCALES_SCHEMA).is_ok());
        }

        fn resource_from_json(value: &str) -> Resource {
            Resource {
                file_name: "test_resource".to_string(),
                resource_type: ResourceType::Brands,
                content: value.to_string(),
            }
        }

        #[test]
        fn it_should_validate_a_valid_brand() {
            let brand_value = resource_from_json(
                r#"
                {
                  "name" : "ACME",
                  "registered_company_name" : "Associazione Costruzioni Modellistiche Esatte",
                  "organization_entity_type" : "OTHER",
                  "group_name" : null,
                  "description" : {
                    "it" : null,
                    "en" : null
                  },
                  "address" : {
                    "street_address" : "Viale Lombardia, 27",
                    "extended_address" : null,
                    "postal_code" : "20131",
                    "city" : "Milano",
                    "region" : "MI",
                    "country" : "ITA"
                  },
                  "contact_info" : {
                    "email" : "mail@acmetreni.com",
                    "phone" : null,
                    "website_url" : "http://www.acmetreni.com"
                  },
                  "socials" : {
                    "facebook" : null,
                    "instagram" : null,
                    "linkedin" : null,
                    "twitter" : null,
                    "youtube" : null
                  },
                  "kind" : "INDUSTRIAL",
                  "status" : "ACTIVE"
                }"#,
            );

            let validator = Validators::new().unwrap();
            let result = validator.validate_brand(&brand_value);
            assert!(result, "a valid brand is failing the validation");
        }

        #[test]
        fn it_should_validate_a_valid_catalog_item() {
            let catalog_item_value = resource_from_json(
                r#"
                {
                  "brand" : "ACME",
                  "item_number" : "60023",
                  "scale" : "H0",
                  "category" : "LOCOMOTIVES",
                  "description" : {
                    "it" : "XMPR FS Trenitalia logo verde/rosso, corrimani e prese frontali, pantografi 52/92",
                    "en" : null
                  },
                  "details" : {
                    "it" : null,
                    "en" : null
                  },
                  "power_method" : "DC",
                  "delivery_date" : "2022",
                  "availability_status" : "AVAILABLE",
                  "rolling_stocks" : [ {
                    "category" : "LOCOMOTIVE",
                    "class_name" : "E402 A",
                    "road_number" : "E402 031",
                    "series" : "",
                    "locomotive_type" : "ELECTRIC_LOCOMOTIVE",
                    "railway" : "FS",
                    "epoch" : "VI",
                    "depot" : "",
                    "dcc_interface" : "MTC_21",
                    "control" : "DCC_READY",
                    "livery" : "",
                    "length_over_buffer" : {
                      "inches" : null,
                      "millimeters" : 210.0
                    },
                    "technical_specifications" : {
                      "minimum_radius" : 360.0,
                      "coupling" : {
                        "socket" : "NEM_362",
                        "close_couplers" : "NO",
                        "digital_shunting" : "NO"
                      },
                      "flywheel_fitted" : "NO",
                      "metal_body" : "NO",
                      "interior_lights" : "NO",
                      "lights" : "YES",
                      "spring_buffers" : "NO"
                    },
                    "is_dummy" : false
                  } ],
                  "count" : 1
                }"#,
            );

            let validator = Validators::new().unwrap();
            let result = validator.validate_catalog_item(&catalog_item_value);
            assert!(result, "a valid catalog item is failing the validation");
        }

        #[test]
        fn it_should_validate_a_valid_railway() {
            let railway_value = resource_from_json(
                r#"
                {
                  "name" : "FS",
                  "abbreviation" : "FS",
                  "registered_company_name" : "Ferrovie dello Stato Italiane S.p.A.",
                  "organization_entity_type" : "STATE_OWNED_ENTERPRISE",
                  "country" : "ITA",
                  "description" : {
                    "it" : null,
                    "en" : null
                  },
                  "period_of_activity" : {
                    "operating_since" : "1905-07-01",
                    "operating_until" : null,
                    "status" : "ACTIVE"
                  },
                  "gauge" : {
                    "track_gauge" : "STANDARD",
                    "meters" : 1.435
                  },
                  "total_length" : {
                    "kilometers" : 24564.0,
                    "miles" : null
                  },
                  "contact_info" : {
                    "email" : null,
                    "website_url" : "https://www.fsitaliane.it",
                    "phone" : null
                  },
                  "social" : {
                    "facebook" : null,
                    "instagram" : "fsitaliane",
                    "linkedin" : "ferrovie-dello-stato-s-p-a-",
                    "twitter" : "FSitaliane",
                    "youtube" : "fsitaliane"
                  },
                  "headquarters" : [ "Roma" ]
                }"#,
            );

            let validator = Validators::new().unwrap();
            let result = validator.validate_railway(&railway_value);
            assert!(result, "a valid railway is failing the validation");
        }

        #[test]
        fn it_should_validate_a_valid_scale() {
            let scale_value = resource_from_json(
                r#"
                {
                  "name" : "H0",
                  "description" : {
                    "it" : null,
                    "en" : null
                  },
                  "ratio" : 87.0,
                  "gauge" : {
                    "millimeters" : 16.5,
                    "inches" : 0.65,
                    "track_gauge" : "STANDARD"
                  },
                  "standards" : [ "NEM" ]
                }
            "#,
            );

            let validator = Validators::new().unwrap();
            let result = validator.validate_scale(&scale_value);
            assert!(result, "a valid scale is failing the validation");
        }
    }
}
