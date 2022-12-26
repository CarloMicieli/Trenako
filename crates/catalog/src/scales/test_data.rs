use crate::common::TrackGauge;
use crate::scales::ratio::Ratio;
use crate::scales::scale::Scale;
use crate::scales::scale_gauge::Gauge;
use crate::scales::scale_id::ScaleId;
use chrono::Utc;
use common::metadata::Metadata;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::collections::HashSet;

pub fn h0() -> Scale {
    let gauge = Gauge::from_millimeters(TrackGauge::Standard, dec!(16.5));
    Scale::new(
        ScaleId::new("H0"),
        "H0",
        Some("Scale H0"),
        Ratio::try_from(Decimal::from(87)).unwrap(),
        gauge,
        HashSet::new(),
        Metadata::created_at(Utc::now()),
    )
}

pub fn n() -> Scale {
    let gauge = Gauge::from_millimeters(TrackGauge::Standard, dec!(9.0));
    Scale::new(
        ScaleId::new("N"),
        "N",
        Some("Scale N"),
        Ratio::try_from(Decimal::from(160)).unwrap(),
        gauge,
        HashSet::new(),
        Metadata::created_at(Utc::now()),
    )
}
