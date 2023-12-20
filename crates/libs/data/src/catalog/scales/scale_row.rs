use catalog::common::TrackGauge;
use catalog::scales::scale_id::ScaleId;
use catalog::scales::standard::Standard;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug)]
pub struct ScaleRow {
    pub scale_id: ScaleId,
    pub name: String,
    pub ratio: Decimal,
    pub gauge_millimeters: Option<Decimal>,
    pub gauge_inches: Option<Decimal>,
    pub track_gauge: TrackGauge,
    pub description_de: Option<String>,
    pub description_en: Option<String>,
    pub description_fr: Option<String>,
    pub description_it: Option<String>,
    pub standards: Vec<Standard>,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub last_modified_at: Option<DateTime<Utc>>,
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[allow(dead_code)]
    pub fn new_scale_row(name: &str, ratio: Decimal, track_gauge: TrackGauge, created_at: DateTime<Utc>) -> ScaleRow {
        ScaleRow {
            scale_id: ScaleId::new(name),
            name: String::from(name),
            ratio,
            gauge_millimeters: None,
            gauge_inches: None,
            track_gauge,
            description_de: None,
            description_en: None,
            description_fr: None,
            description_it: None,
            standards: Vec::new(),
            version: 1,
            created_at,
            last_modified_at: None,
        }
    }
}
