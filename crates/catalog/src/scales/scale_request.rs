use crate::scales::ratio::Ratio;
use crate::scales::scale_gauge::Gauge;
use crate::scales::standard::Standard;
use common::localized_text::LocalizedText;

/// A request to create/update rail transport modelling scales
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ScaleRequest {
    /// the scale name
    pub name: String,
    /// the ratio between the real world and the model (e.g. 1/87 or 1:87)
    pub ratio: Ratio,
    /// the track gauge
    pub gauge: Option<Gauge>,
    /// the modelling scale description
    pub description: LocalizedText,
    /// the list of standards
    pub standards: Vec<Standard>,
}
