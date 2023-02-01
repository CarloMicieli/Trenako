use async_trait::async_trait;
use catalog::common::TrackGauge;
use catalog::scales::commands::new_scales::Result;
use catalog::scales::commands::new_scales::{NewScaleCommand, NewScaleRepository};
use catalog::scales::scale_id::ScaleId;
use common::unit_of_work::postgres::PgUnitOfWork;

pub struct PgNewScaleRepository;

#[async_trait]
impl<'db> NewScaleRepository<'db, PgUnitOfWork<'db>> for PgNewScaleRepository {
    async fn exists_already(&self, scale_id: &ScaleId, unit_of_work: &mut PgUnitOfWork) -> Result<bool> {
        let result = sqlx::query!("SELECT scale_id FROM scales WHERE scale_id = $1 LIMIT 1", scale_id)
            .fetch_optional(&mut unit_of_work.transaction)
            .await?;

        Ok(result.is_some())
    }

    async fn insert(&self, new_scale: &NewScaleCommand, unit_of_work: &mut PgUnitOfWork) -> Result<()> {
        let scale_id = &new_scale.scale_id;
        let request = &new_scale.payload;
        let metadata = &new_scale.metadata;

        sqlx::query!(
            r#"INSERT INTO scales (
                    scale_id,
                    name,
                    ratio,
                    gauge_millimeters,
                    gauge_inches,
                    track_gauge,
                    description_it,
                    standards,
                    created_at,
                    version
                )
                VALUES (
                    $1, $2, $3, $4, $5, $6,
                    $7, $8, $9, $10
                )"#,
            scale_id as &ScaleId,
            request.name,
            request.ratio,
            request.gauge_millimeters,
            request.gauge_inches,
            request.track_gauge as TrackGauge,
            request.description,
            request.standards,
            metadata.created(),
            metadata.version() as i32
        )
        .execute(&mut unit_of_work.transaction)
        .await?;

        Ok(())
    }
}
