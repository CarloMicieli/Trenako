use crate::data::common::{Contacts, Socials};
use jsonschema::{Draft, JSONSchema};
use rust_decimal::Decimal;
use schemars::schema::RootSchema;
use schemars::{schema_for, JsonSchema};
use serde_json::Value;
use std::str;

#[derive(Debug, Deserialize, Clone, JsonSchema)]
pub struct Railway {
    pub name: String,
    #[serde(rename = "registeredCompanyName")]
    pub registered_company_name: String,
    pub ownership: Option<String>,
    pub country: String,
    #[serde(rename = "periodOfActivity")]
    pub period_of_activity: PeriodOfActivity,
    pub length: Length,
    pub gauge: Gauge,
    #[serde(rename = "contactInfo")]
    pub contact_info: Contacts,
    pub socials: Socials,
    pub headquarters: Option<String>,
    pub version: u8,
}

#[derive(Debug, Deserialize, Clone, JsonSchema)]
pub struct PeriodOfActivity {
    #[serde(rename = "operatingSince")]
    pub operating_since: String,
    #[serde(rename = "operatingUntil")]
    pub operating_until: Option<String>,
    pub status: Status,
}

#[derive(Debug, Deserialize, Clone, JsonSchema)]
pub struct Gauge {
    #[serde(rename = "trackGauge")]
    pub track_gauge: String,
    pub meters: Decimal,
}

#[derive(Debug, Deserialize, Clone, JsonSchema)]
pub struct Length {
    pub kilometers: Option<Decimal>,
    pub miles: Option<Decimal>,
}

#[derive(Debug, Deserialize, Clone, JsonSchema)]
pub enum Status {
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "INACTIVE")]
    Inactive,
}

impl str::FromStr for Railway {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let schema: RootSchema = schema_for!(Railway);
        let schema: Value = serde_json::from_str(&serde_json::to_string_pretty(&schema).unwrap()).unwrap();

        let compiled = JSONSchema::options()
            .with_draft(Draft::Draft7)
            .compile(&schema)
            .expect("A valid schema");
        let railway: Value = serde_json::from_str(&s).unwrap();

        let result = compiled.validate(&railway);
        match result {
            Ok(()) => {
                let railway: Railway = serde_json::from_str(&s).unwrap();
                Ok(railway)
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
