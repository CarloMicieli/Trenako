use rust_decimal::Decimal;
use std::collections::HashSet;

#[derive(Debug, Deserialize, Clone)]
pub struct Scale {
    pub name: String,
    pub description: Option<String>,
    pub ratio: Decimal,
    pub gauge: Gauge,
    pub standards: HashSet<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Gauge {
    pub millimeters: Decimal,
    pub inches: Decimal,
    #[serde(rename = "trackGauge")]
    pub track_gauge: String,
}
