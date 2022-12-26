use crate::brands::brand::Brand;
use crate::brands::brand_id::BrandId;
use crate::brands::brand_status::BrandStatus;
use crate::brands::brand_type::BrandType;
use chrono::{DateTime, Utc};
use common::address::Address;
use common::contact::{ContactInfo, MailAddress, WebsiteUrl};
use common::metadata::Metadata;
use common::organizations::OrganizationEntityType;
use common::socials::Socials;
use isocountry::CountryCode;

pub fn acme() -> Brand {
    let now: DateTime<Utc> = Utc::now();
    let address = Address::builder()
        .street_address("Viale Lombardia, 27")
        .postal_code("20131")
        .city("Milano")
        .region("MI")
        .country_code(CountryCode::ITA)
        .build()
        .unwrap();

    let contact_info = ContactInfo::new(
        Some(MailAddress::new("mail@acmetreni.com")),
        Some(WebsiteUrl::new("http://www.acmetreni.com")),
        None,
    );

    let socials = Socials::builder().facebook("Acmetreni").build();

    Brand::new(
        BrandId::new("acme"),
        "ACME",
        Some("Associazione Costruzioni Modellistiche Esatte"),
        Some(OrganizationEntityType::LimitedCompany),
        None,
        None,
        Some(address),
        Some(contact_info),
        BrandType::Industrial,
        BrandStatus::Active,
        Some(socials),
        Metadata::created_at(now),
    )
}

pub fn roco() -> Brand {
    let now: DateTime<Utc> = Utc::now();
    let address = Address::builder()
        .street_address("Plainbachstraße 4")
        .postal_code("A-5101")
        .city("Bergheim")
        .country_code(CountryCode::AUT)
        .build()
        .unwrap();

    let contact_info = ContactInfo::new(
        Some(MailAddress::new("webshop@roco.cc")),
        Some(WebsiteUrl::new("https://www.roco.cc")),
        None,
    );

    let socials = Socials::builder()
        .facebook("roco.cc")
        .instagram("rococc")
        .youtube("UCmPH1NgeyzOCKxfH3uP-wsQ")
        .build();

    Brand::new(
        BrandId::new("roco"),
        "Roco",
        Some("Modelleisenbahn GmbH"),
        Some(OrganizationEntityType::LimitedCompany),
        Some("modelleisenbahn"),
        None,
        Some(address),
        Some(contact_info),
        BrandType::Industrial,
        BrandStatus::Active,
        Some(socials),
        Metadata::created_at(now),
    )
}
