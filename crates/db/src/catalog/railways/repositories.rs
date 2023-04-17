use crate::catalog::railways::railway_row::RailwayRow;
use anyhow::Context;
use async_trait::async_trait;
use catalog::common::TrackGauge;
use catalog::railways::period_of_activity::RailwayStatus;
use catalog::railways::queries::find_all_railways::FindAllRailwaysRepository;
use catalog::railways::queries::find_railway_by_id::FindRailwayByIdRepository;
use catalog::railways::railway::Railway;
use catalog::railways::railway_id::RailwayId;
use common::contacts::WebsiteUrl;
use common::contacts::{MailAddress, PhoneNumber};
use common::organizations::OrganizationEntityType;
use common::queries::converters::ToOutputConverter;
use common::queries::errors::DatabaseError;
use common::queries::single_result::QueryError;
use common::socials::Handler;
use common::unit_of_work::postgres::PgUnitOfWork;

#[derive(Debug)]
pub struct RailwaysRepository;

#[async_trait]
impl<'db> FindAllRailwaysRepository<'db, PgUnitOfWork<'db>> for RailwaysRepository {
    async fn find_all(&self, unit_of_work: &mut PgUnitOfWork<'db>) -> Result<Vec<Railway>, QueryError> {
        let results = sqlx::query_as!(
            RailwayRow,
            r#"SELECT
                railway_id as "railway_id: RailwayId",
                name,
                abbreviation,
                registered_company_name,
                organization_entity_type as "organization_entity_type?: OrganizationEntityType",
                description_en,
                description_it,
                country,
                operating_since,
                operating_until,
                status as "status?: RailwayStatus",
                gauge_meters,
                track_gauge as "track_gauge?: TrackGauge",
                headquarters as "headquarters!: Vec<String>",
                total_length_mi,
                total_length_km,
                contact_email as "contact_email?: MailAddress",
                contact_website_url as "contact_website_url?: WebsiteUrl",
                contact_phone as "contact_phone?: PhoneNumber",
                socials_facebook as "socials_facebook?: Handler",
                socials_instagram as "socials_instagram?: Handler",
                socials_linkedin as "socials_linkedin?: Handler",
                socials_twitter as "socials_twitter?: Handler",
                socials_youtube as "socials_youtube?: Handler",
                created_at,
                last_modified_at,
                version
            FROM railways
            ORDER BY name"#,
        )
        .fetch_all(&mut unit_of_work.transaction)
        .await
        .context("A database failure was encountered while trying to fetch railways.")?;

        let mut output: Vec<Railway> = Vec::with_capacity(results.len());
        for row in results.into_iter() {
            let railway = row.to_output().map_err(QueryError::ConversionError)?;
            output.push(railway);
        }

        Ok(output)
    }
}

#[async_trait]
impl<'db> FindRailwayByIdRepository<'db, PgUnitOfWork<'db>> for RailwaysRepository {
    async fn find_by_id(
        &self,
        railway_id: &RailwayId,
        unit_of_work: &mut PgUnitOfWork<'db>,
    ) -> Result<Option<Railway>, DatabaseError> {
        let result = sqlx::query_as!(
            RailwayRow,
            r#"SELECT
                railway_id as "railway_id: RailwayId",
                name,
                abbreviation,
                registered_company_name,
                organization_entity_type as "organization_entity_type?: OrganizationEntityType",
                description_en,
                description_it,
                country,
                operating_since,
                operating_until,
                status as "status?: RailwayStatus",
                gauge_meters,
                track_gauge as "track_gauge?: TrackGauge",
                headquarters as "headquarters!: Vec<String>",
                total_length_mi,
                total_length_km,
                contact_email as "contact_email?: MailAddress",
                contact_website_url as "contact_website_url?: WebsiteUrl",
                contact_phone as "contact_phone?: PhoneNumber",
                socials_facebook as "socials_facebook?: Handler",
                socials_instagram as "socials_instagram?: Handler",
                socials_linkedin as "socials_linkedin?: Handler",
                socials_twitter as "socials_twitter?: Handler",
                socials_youtube as "socials_youtube?: Handler",
                created_at,
                last_modified_at,
                version
            FROM railways 
            WHERE railway_id = $1"#,
            railway_id
        )
        .fetch_optional(&mut unit_of_work.transaction)
        .await
        .context("A database failure was encountered while trying to fetch a railway.")?;

        result.map(row_to_railway).transpose()
    }
}

fn row_to_railway(row: RailwayRow) -> Result<Railway, DatabaseError> {
    row.to_output().map_err(DatabaseError::ConversionError)
}
