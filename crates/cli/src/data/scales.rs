use jsonschema::{Draft, JSONSchema};
use rust_decimal::Decimal;
use schemars::schema::RootSchema;
use schemars::{schema_for, JsonSchema};
use serde_json::Value;
use std::collections::HashSet;
use std::str;

#[derive(Debug, Deserialize, Clone, JsonSchema)]
pub struct Scale {
    pub name: String,
    pub description: Option<String>,
    pub ratio: Decimal,
    pub gauge: Gauge,
    pub standards: HashSet<Standard>,
}

#[derive(Debug, Deserialize, Clone, JsonSchema)]
pub struct Gauge {
    pub millimeters: Decimal,
    pub inches: Decimal,
    #[serde(rename = "trackGauge")]
    pub track_gauge: String,
}

#[derive(Debug, Deserialize, Clone, JsonSchema, Eq, PartialEq, Hash)]
pub enum Standard {
    #[serde(rename = "BRITISH")]
    British,
    #[serde(rename = "JAPANESE")]
    Japanese,
    NEM,
    NMRA,
}

impl str::FromStr for Scale {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let schema: RootSchema = schema_for!(Scale);
        let schema: Value = serde_json::from_str(&serde_json::to_string_pretty(&schema).unwrap()).unwrap();

        let compiled = JSONSchema::options()
            .with_draft(Draft::Draft7)
            .compile(&schema)
            .expect("A valid schema");
        let scale: Value = serde_json::from_str(&s).unwrap();

        let result = compiled.validate(&scale);
        match result {
            Ok(()) => {
                let scale: Scale = serde_json::from_str(&s).unwrap();
                Ok(scale)
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
