use crate::catalog::brands::brand_row::BrandRow;
use anyhow::Context;
use async_trait::async_trait;
use catalog::brands::brand::Brand;
use catalog::brands::brand_id::BrandId;
use catalog::brands::brand_kind::BrandKind;
use catalog::brands::brand_status::BrandStatus;
use catalog::brands::commands::new_brand::NewBrandCommand;
use catalog::brands::commands::repositories::NewBrandRepository;
use catalog::brands::queries::find_all_brands::FindAllBrandsRepository;
use catalog::brands::queries::find_brand_by_id::FindBrandByIdRepository;
use common::contacts::WebsiteUrl;
use common::contacts::{MailAddress, PhoneNumber};
use common::organizations::OrganizationEntityType;
use common::queries::converters::ToOutputConverter;
use common::queries::errors::DatabaseError;
use common::socials::Handler;
use common::unit_of_work::postgres::PgUnitOfWork;

#[derive(Debug)]
pub struct BrandsRepository;

#[async_trait]
impl<'db> NewBrandRepository<'db, PgUnitOfWork<'db>> for BrandsRepository {
    async fn exists(&self, brand_id: &BrandId, unit_of_work: &mut PgUnitOfWork) -> Result<bool, anyhow::Error> {
        let result = sqlx::query!("SELECT brand_id FROM brands WHERE brand_id = $1 LIMIT 1", brand_id)
            .fetch_optional(&mut unit_of_work.transaction)
            .await
            .context("A database failure was encountered while trying to check for a brand existence.")?;

        Ok(result.is_some())
    }

    async fn insert(&self, new_brand: &NewBrandCommand, unit_of_work: &mut PgUnitOfWork) -> Result<(), anyhow::Error> {
        let brand_id = &new_brand.brand_id;
        let request = &new_brand.payload;
        let metadata = &new_brand.metadata;

        sqlx::query!(
                r#"INSERT INTO brands (
                    brand_id,
                    name,
                    registered_company_name,
                    organization_entity_type,
                    group_name,
                    description_en,
                    description_it,
                    kind,
                    status,
                    contact_email, contact_website_url, contact_phone,
                    address_street_address, address_extended_address, address_city, address_region, address_postal_code, address_country,
                    socials_facebook, socials_instagram, socials_linkedin, socials_twitter, socials_youtube,
                    created_at,
                    version
                )
                VALUES (
                    $1, $2, $3, $4, $5, $6,
                    $7, $8, $9, $10, $11, $12, 
                    $13, $14, $15, $16, $17, $18,
                    $19, $20, $21, $22, $23, $24, $25
                )"#,
                brand_id as &BrandId,
                request.name,
                request.registered_company_name,
                request.organization_entity_type as Option<OrganizationEntityType>,
                request.group_name,
                request.description.english(),
                request.description.italian(),
                request.kind as BrandKind,
                request.status as Option<BrandStatus>,
                request.contact_email.as_ref() as Option<&MailAddress>,
                request.contact_website_url.as_ref().map(|x| x.to_string()),
                request.contact_phone.as_ref() as Option<&PhoneNumber>,
                request.address_street_address,
                request.address_extended_address,
                request.address_city,
                request.address_region,
                request.address_postal_code,
                request.address_country,
                request.facebook_handler.as_ref() as Option<&Handler>,
                request.instagram_handler.as_ref() as Option<&Handler>,
                request.linkedin_handler.as_ref() as Option<&Handler>,
                request.twitter_handler.as_ref() as Option<&Handler>,
                request.youtube_handler.as_ref() as Option<&Handler>,
                metadata.created(),
                metadata.version() as i32
            )
            .execute(&mut unit_of_work.transaction)
            .await
            .context("A database failure was encountered while trying to store a brand.")?;

        Ok(())
    }
}

#[async_trait]
impl<'db> FindAllBrandsRepository<'db, PgUnitOfWork<'db>> for BrandsRepository {
    async fn find_all(&self, unit_of_work: &mut PgUnitOfWork) -> Result<Vec<Brand>, DatabaseError> {
        let results: Vec<BrandRow> = sqlx::query_as!(BrandRow,
                r#"SELECT
                    brand_id as "brand_id!: BrandId", 
                    name, 
                    registered_company_name, 
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
                FROM brands
                ORDER BY name"#)
            .fetch_all(&mut unit_of_work.transaction)
            .await
            .context("A database failure was encountered while trying to fetch the brands.")?;

        let mut output: Vec<Brand> = Vec::with_capacity(results.len());
        for row in results.into_iter() {
            let brand = row.to_output().map_err(DatabaseError::ConversionError)?;
            output.push(brand);
        }

        Ok(output)
    }
}

#[async_trait]
impl<'db> FindBrandByIdRepository<'db, PgUnitOfWork<'db>> for BrandsRepository {
    async fn find_by_id(
        &self,
        brand_id: &BrandId,
        unit_of_work: &mut PgUnitOfWork,
    ) -> Result<Option<Brand>, DatabaseError> {
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
                FROM brands WHERE brand_id = $1"#, 
                brand_id)
            .fetch_optional(&mut unit_of_work.transaction)
            .await
            .context("A database failure was encountered while trying to fetch a brand.")?;

        result.to_output().map_err(DatabaseError::ConversionError)
    }
}
