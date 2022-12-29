use crate::scales::ratio::Ratio;
use crate::scales::scale_gauge::Gauge;
use crate::scales::standard::Standard;

/// A request to create/update rail transport modelling scales
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ScaleRequest {
    pub name: String,
    pub ratio: Ratio,
    pub gauge: Option<Gauge>,
    pub description: Option<String>,
    pub standards: Vec<Standard>,
}
