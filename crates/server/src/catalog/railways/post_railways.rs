use async_trait::async_trait;
use catalog::common::TrackGauge;
use catalog::railways::commands::new_railways::{NewRailwayCommand, NewRailwayRepository};
use catalog::railways::period_of_activity::RailwayStatus;
use catalog::railways::railway_id::RailwayId;
use common::contacts::{MailAddress, PhoneNumber};
use common::organizations::OrganizationEntityType;
use common::socials::Handler;
use sqlx::PgPool;

pub struct PgNewRailwayRepository<'repo> {
    pg_pool: &'repo PgPool,
}

impl<'repo> PgNewRailwayRepository<'repo> {
    pub fn new(pg_pool: &PgPool) -> PgNewRailwayRepository {
        PgNewRailwayRepository { pg_pool }
    }
}

#[async_trait]
impl<'repo> NewRailwayRepository for PgNewRailwayRepository<'repo> {
    async fn exists_already(&self, _railway_id: &RailwayId) -> catalog::railways::commands::new_railways::Result<bool> {
        Ok(false)
    }

    async fn insert(&self, new_railway: &NewRailwayCommand) -> catalog::railways::commands::new_railways::Result<()> {
        let mut transaction = self.pg_pool.begin().await?;

        let railway_id = &new_railway.railway_id;
        let request = &new_railway.payload;
        let metadata = &new_railway.metadata;

        sqlx::query!(
            r#"INSERT INTO railways (
                railway_id,
                name,
                abbreviation,
                registered_company_name,
                organization_entity_type,
                description_it,
                country,
                operating_since,
                operating_until,
                status,
                gauge_m,
                track_gauge,
                headquarters,
                total_length_mi,
                total_length_km,
                contact_email,
                contact_website_url,
                contact_phone,
                socials_facebook,
                socials_instagram,
                socials_linkedin,
                socials_twitter,
                socials_youtube,
                created_at,
                version
            )
            VALUES (
                $1, $2, $3, $4, $5, $6,
                $7, $8, $9, $10, $11, $12, 
                $13, $14, $15, $16, $17, $18,
                $19, $20, $21, $22, $23, $24, $25
            )"#,
            railway_id as &RailwayId,
            request.name,
            request.abbreviation,
            request.registered_company_name,
            request.organization_entity_type.as_ref() as Option<&OrganizationEntityType>,
            request.description,
            request.country,
            request.operating_since,
            request.operating_until,
            request.status.as_ref() as Option<&RailwayStatus>,
            request.gauge_m,
            request.track_gauge.as_ref() as Option<&TrackGauge>,
            request.headquarters,
            request.total_length_mi,
            request.total_length_km,
            request.contact_email.as_ref() as Option<&MailAddress>,
            request.contact_website_url.as_ref().map(|x| x.to_string()),
            request.contact_phone.as_ref() as Option<&PhoneNumber>,
            request.socials_facebook.as_ref() as Option<&Handler>,
            request.socials_instagram.as_ref() as Option<&Handler>,
            request.socials_linkedin.as_ref() as Option<&Handler>,
            request.socials_twitter.as_ref() as Option<&Handler>,
            request.socials_youtube.as_ref() as Option<&Handler>,
            metadata.created(),
            metadata.version() as i32
        )
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }
}
