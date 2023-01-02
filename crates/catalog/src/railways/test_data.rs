use crate::railways::railway::Railway;
use crate::railways::railway_gauge::RailwayGauge;
use crate::railways::railway_id::RailwayId;
use crate::railways::railway_length::RailwayLength;
use chrono::Utc;
use common::contacts::ContactInformation;
use common::metadata::Metadata;
use common::organizations::OrganizationEntityType;
use common::socials::SocialsBuilder;
use isocountry::CountryCode;
use rust_decimal_macros::dec;

pub fn die_bahn() -> Railway {
    let metadata = Metadata::created_at(Utc::now());
    let socials = SocialsBuilder::default()
        .instagram("deutschebahn")
        .linkedin("deutschebahn")
        .twitter("db_presse")
        .youtube("deutschebahnkonzern")
        .build()
        .unwrap();
    let length = RailwayLength::of_kilometers(dec!(24564.0));
    let gauge = RailwayGauge::standard();
    let contact_info = ContactInformation::builder()
        .website_url("https://www.deutschebahn.com")
        .build()
        .unwrap();

    Railway::new(
        RailwayId::new("db"),
        "DB",
        Some("DB"),
        "Deutsche Bahn AG",
        Some(OrganizationEntityType::StateOwnedEnterprise),
        None,
        None,
        Some(length),
        Some(gauge),
        CountryCode::DEU,
        Some("Berlin"),
        Some(contact_info),
        Some(socials),
        metadata,
    )
}

pub fn fs() -> Railway {
    let metadata = Metadata::created_at(Utc::now());
    let socials = SocialsBuilder::default()
        .instagram("fsitaliane")
        .linkedin("ferrovie-dello-stato-s-p-a-")
        .twitter("FSitaliane")
        .youtube("fsitaliane")
        .build()
        .unwrap();
    let length = RailwayLength::of_kilometers(dec!(24564.0));
    let gauge = RailwayGauge::standard();
    let contact_info = ContactInformation::builder()
        .website_url("https://www.fsitaliane.it")
        .build()
        .unwrap();

    Railway::new(
        RailwayId::new("fs"),
        "FS",
        Some("FS"),
        "Ferrovie dello stato italiane",
        Some(OrganizationEntityType::StateOwnedEnterprise),
        None,
        None,
        Some(length),
        Some(gauge),
        CountryCode::ITA,
        Some("Rome"),
        Some(contact_info),
        Some(socials),
        metadata,
    )
}
