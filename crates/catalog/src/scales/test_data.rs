use crate::scales::ratio::Ratio;
use crate::scales::scale::Scale;
use crate::scales::scale_gauge::Gauge;
use crate::scales::scale_id::ScaleId;
use chrono::Utc;
use common::metadata::Metadata;
use rust_decimal::Decimal;
use std::collections::HashSet;

pub fn h0() -> Scale {
    Scale::new(
        ScaleId::new("H0"),
        "H0",
        Some("Scale H0"),
        Ratio::try_from(Decimal::from(87)).unwrap(),
        Gauge::H0,
        HashSet::new(),
        Metadata::created_at(Utc::now()),
    )
}

pub fn n() -> Scale {
    Scale::new(
        ScaleId::new("N"),
        "N",
        Some("Scale N"),
        Ratio::try_from(Decimal::from(160)).unwrap(),
        Gauge::N,
        HashSet::new(),
        Metadata::created_at(Utc::now()),
    )
}
