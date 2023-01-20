use async_trait::async_trait;
use catalog::common::TrackGauge;
use catalog::scales::commands::new_scales::Result;
use catalog::scales::commands::new_scales::{NewScaleCommand, NewScaleRepository};
use catalog::scales::scale_id::ScaleId;
use sqlx::PgPool;

pub struct PgNewScaleRepository<'repo> {
    pg_pool: &'repo PgPool,
}

impl<'repo> PgNewScaleRepository<'repo> {
    pub fn new(pg_pool: &PgPool) -> PgNewScaleRepository {
        PgNewScaleRepository { pg_pool }
    }
}

#[async_trait]
impl<'repo> NewScaleRepository for PgNewScaleRepository<'repo> {
    async fn exists_already(&self, _scale_id: &ScaleId) -> Result<bool> {
        Ok(false)
    }

    async fn insert(&self, new_scale: &NewScaleCommand) -> Result<()> {
        let mut transaction = self.pg_pool.begin().await?;

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
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }
}
