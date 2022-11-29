use crate::data::common::{Contacts, Socials};
use jsonschema::{Draft, JSONSchema};
use schemars::schema::RootSchema;
use schemars::{schema_for, JsonSchema};
use serde_json::Value;
use std::str;

#[derive(Debug, Deserialize, Clone, JsonSchema)]
pub struct Brand {
    pub name: String,
    #[serde(rename = "registeredCompanyName")]
    pub registered_company_name: String,
    #[serde(rename = "groupName")]
    pub group_name: Option<String>,
    pub description: Option<String>,
    pub address: Option<Address>,
    #[serde(rename = "contactInfo")]
    pub contact_info: Option<Contacts>,
    pub social: Option<Socials>,
    pub kind: Kind,
    pub status: Status,
    pub version: u8,
}

#[derive(Debug, Deserialize, Clone, JsonSchema)]
pub enum Kind {
    #[serde(rename = "INDUSTRIAL")]
    Industrial,
    #[serde(rename = "BRASS_MODELS")]
    BrassModels,
}

#[derive(Debug, Deserialize, Clone, JsonSchema)]
pub enum Status {
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "OUT_OF_BUSINESS")]
    OutOfBusiness,
}

#[derive(Debug, Deserialize, Clone, JsonSchema)]
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

impl str::FromStr for Brand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let schema: RootSchema = schema_for!(Brand);
        let schema: Value = serde_json::from_str(&serde_json::to_string_pretty(&schema).unwrap()).unwrap();

        let compiled = JSONSchema::options()
            .with_draft(Draft::Draft7)
            .compile(&schema)
            .expect("A valid schema");
        let brand: Value = serde_json::from_str(&s).unwrap();

        let result = compiled.validate(&brand);
        match result {
            Ok(()) => {
                let brand: Brand = serde_json::from_str(&s).unwrap();
                Ok(brand)
            }
            Err(errors) => {
                for error in errors {
                    println!("{:?}", error);
                }
                Err("".to_owned())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod brands {
        use super::*;
        use std::str::FromStr;

        #[test]
        fn it_should_parse_yaml_files() {
            let contents = r#"
{
  "name": "ACME",
  "registeredCompanyName": "Associazione Costruzioni Modellistiche Esatte",
  "groupName": null,
  "description": null,
  "address": {
    "streetAddress": "Viale Lombardia, 27",
    "extendedAddress": null,
    "postalCode": "20131",
    "city": "Milano",
    "region": "MI",
    "country": "it"
  },
  "contactInfo": {
    "email": "mail@acmetreni.com",
    "websiteUrl": "http://www.acmetreni.com",
    "phone": null
  },
  "social": {
    "instragram": null,
    "facebook": null,
    "youtube": null,
    "twitter": null
  },
  "kind": "INDUSTRIAL",
  "status": "ACTIVE",
  "version": 1
}
            "#;

            let result = Brand::from_str(contents);
            assert!(result.is_ok());
        }
    }
}
