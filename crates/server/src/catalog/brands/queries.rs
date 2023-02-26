use anyhow::Context;
use async_trait::async_trait;
use catalog::brands::brand::Brand;
use catalog::brands::brand_id::BrandId;
use catalog::brands::brand_kind::BrandKind;
use catalog::brands::brand_status::BrandStatus;
use catalog::brands::queries::find_by_id::{FindBrandByIdQuery, QueryError};
use chrono::{DateTime, Utc};
use common::address::{Address, AddressBuilder};
use common::contacts::{ContactInformation, MailAddress, PhoneNumber, WebsiteUrl};
use common::localized_text::LocalizedText;
use common::metadata::Metadata;
use common::organizations::OrganizationEntityType;
use common::socials::{Handler, Socials};
use common::unit_of_work::postgres::PgUnitOfWork;

pub struct PgFindBrandByIdQuery;

#[async_trait]
impl<'db> FindBrandByIdQuery<'db, PgUnitOfWork<'db>> for PgFindBrandByIdQuery {
    async fn execute(&self, brand_id: &BrandId, unit_of_work: &mut PgUnitOfWork<'db>) -> Result<Brand, QueryError> {
        let result: Option<BrandRow> = sqlx::query_as!(BrandRow,
                r#"SELECT
                    brand_id as "brand_id!: BrandId", 
                    name, registered_company_name, 
                    organization_entity_type as "organization_entity_type: OrganizationEntityType", 
                    group_name, 
                    description_en, description_it, 
                    kind as "kind: BrandKind", 
                    status as "status?: BrandStatus",
                    contact_email as "contact_email?: MailAddress", 
                    contact_website_url as "contact_website_url?: WebsiteUrl", 
                    contact_phone as "contact_phone?: PhoneNumber",
                    address_street_address, address_extended_address, address_city, address_region, address_postal_code, address_country,
                    socials_facebook as "socials_facebook?: Handler", 
                    socials_instagram as "socials_instagram?: Handler",     
                    socials_linkedin as "socials_linkedin?: Handler",    
                    socials_twitter as "socials_twitter?: Handler",    
                    socials_youtube as "socials_youtube?: Handler",
                    created_at,
                    last_modified_at,
                    version
                FROM brands WHERE brand_id = $1"#, &brand_id)
            .fetch_optional(&mut unit_of_work.transaction)
            .await
            .context("A database failure was encountered while trying to fetch a brand.")?;

        result
            .map(from_row_to_brand)
            .unwrap_or_else(|| Err(QueryError::EmptyResultSet))
    }
}

fn from_row_to_brand(row: BrandRow) -> Result<Brand, QueryError> {
    let localized_text = LocalizedText::try_from(&row)?;
    let address = Address::try_from(&row)?;
    let socials = Socials::try_from(&row)?;
    let contacts = ContactInformation::try_from(&row)?;
    let metadata = Metadata::try_from(&row)?;

    Ok(Brand {
        brand_id: row.brand_id,
        name: row.name,
        registered_company_name: row.registered_company_name,
        organization_entity_type: row.organization_entity_type,
        group_name: row.group_name,
        description: localized_text,
        address: Some(address),
        contact_info: Some(contacts),
        kind: row.kind,
        status: row.status,
        socials: Some(socials),
        metadata,
    })
}

impl TryFrom<&BrandRow> for Metadata {
    type Error = QueryError;

    fn try_from(value: &BrandRow) -> Result<Self, Self::Error> {
        Ok(Metadata::new(
            value.version as u8,
            value.created_at,
            value.last_modified_at,
        ))
    }
}

impl TryFrom<&BrandRow> for LocalizedText {
    type Error = QueryError;

    fn try_from(value: &BrandRow) -> Result<Self, Self::Error> {
        let mut localized_text = LocalizedText::default();

        localized_text.add_english(value.description_en.as_ref());
        localized_text.add_italian(value.description_it.as_ref());

        Ok(localized_text)
    }
}

impl TryFrom<&BrandRow> for ContactInformation {
    type Error = QueryError;

    fn try_from(value: &BrandRow) -> Result<Self, Self::Error> {
        let contacts = ContactInformation::new(
            value.contact_email.clone(),
            value.contact_website_url.clone(),
            value.contact_phone.clone(),
        );

        Ok(contacts)
    }
}

impl TryFrom<&BrandRow> for Socials {
    type Error = QueryError;

    fn try_from(value: &BrandRow) -> Result<Self, Self::Error> {
        Ok(Socials {
            facebook: value.socials_facebook.clone(),
            instagram: value.socials_instagram.clone(),
            twitter: value.socials_twitter.clone(),
            youtube: value.socials_youtube.clone(),
            linkedin: value.socials_linkedin.clone(),
        })
    }
}

impl TryFrom<&BrandRow> for Address {
    type Error = QueryError;

    fn try_from(value: &BrandRow) -> Result<Self, Self::Error> {
        match (
            &value.address_street_address,
            &value.address_city,
            &value.address_postal_code,
            &value.address_country,
        ) {
            (Some(street_address), Some(city), Some(postal_code), Some(country)) => {
                let mut builder = AddressBuilder::default()
                    .street_address(street_address)
                    .country_code(country)
                    .postal_code(postal_code)
                    .city(city);

                if let Some(extended_address) = &value.address_extended_address {
                    builder = builder.extended_address(extended_address);
                }

                if let Some(region) = &value.address_region {
                    builder = builder.region(region);
                }

                Ok(builder.build().unwrap())
            }
            _ => Err(QueryError::EmptyResultSet), //TODO: fixme
        }
    }
}

#[derive(Debug)]
pub struct BrandRow {
    pub brand_id: BrandId,
    pub name: String,
    pub registered_company_name: Option<String>,
    pub organization_entity_type: Option<OrganizationEntityType>,
    pub group_name: Option<String>,
    pub description_en: Option<String>,
    pub description_it: Option<String>,
    pub kind: BrandKind,
    pub status: Option<BrandStatus>,
    pub contact_email: Option<MailAddress>,
    pub contact_website_url: Option<WebsiteUrl>,
    pub contact_phone: Option<PhoneNumber>,
    pub address_street_address: Option<String>,
    pub address_extended_address: Option<String>,
    pub address_city: Option<String>,
    pub address_region: Option<String>,
    pub address_postal_code: Option<String>,
    pub address_country: Option<String>,
    pub socials_facebook: Option<Handler>,
    pub socials_instagram: Option<Handler>,
    pub socials_linkedin: Option<Handler>,
    pub socials_twitter: Option<Handler>,
    pub socials_youtube: Option<Handler>,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub last_modified_at: Option<DateTime<Utc>>,
}
