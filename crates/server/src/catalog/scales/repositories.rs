use anyhow::{Context, Error};
use async_trait::async_trait;
use catalog::common::TrackGauge;
use catalog::scales::commands::new_scales::NewScaleCommand;
use catalog::scales::commands::repositories::ScaleRepository;
use catalog::scales::queries::scale_row::ScaleRow;
use catalog::scales::scale_id::ScaleId;
use catalog::scales::standard::Standard;
use common::queries::single_result::QueryRepository;
use common::unit_of_work::postgres::PgUnitOfWork;

pub struct PgScaleRepository;

#[async_trait]
impl<'db> ScaleRepository<'db, PgUnitOfWork<'db>> for PgScaleRepository {
    async fn exists(&self, scale_id: &ScaleId, unit_of_work: &mut PgUnitOfWork) -> Result<bool, anyhow::Error> {
        let result = sqlx::query!("SELECT scale_id FROM scales WHERE scale_id = $1 LIMIT 1", scale_id)
            .fetch_optional(&mut unit_of_work.transaction)
            .await
            .context("A database failure was encountered while trying to check for scale existence.")?;

        Ok(result.is_some())
    }

    async fn insert(&self, new_scale: &NewScaleCommand, unit_of_work: &mut PgUnitOfWork) -> Result<(), anyhow::Error> {
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
                    description_en,
                    description_it,
                    standards,
                    created_at,
                    version
                )
                VALUES (
                    $1, $2, $3, $4, $5, $6,
                    $7, $8, $9, $10, $11
                )"#,
            scale_id as &ScaleId,
            request.name,
            request.ratio,
            request.gauge_millimeters,
            request.gauge_inches,
            request.track_gauge as TrackGauge,
            request.description.english(),
            request.description.italian(),
            &request.standards as &Vec<Standard>,
            metadata.created(),
            metadata.version() as i32
        )
        .execute(&mut unit_of_work.transaction)
        .await
        .context("A database failure was encountered while trying to store a scale.")?;

        Ok(())
    }
}

#[async_trait]
impl<'db> QueryRepository<'db, PgUnitOfWork<'db>, ScaleId, ScaleRow> for PgScaleRepository {
    async fn find_by_id(id: &ScaleId, unit_of_work: &mut PgUnitOfWork<'db>) -> Result<Option<ScaleRow>, Error> {
        let result = sqlx::query_as!(
            ScaleRow,
            r#"SELECT
                scale_id as "scale_id: ScaleId",
                name,
                ratio,
                gauge_millimeters,
                gauge_inches,
                track_gauge as "track_gauge: TrackGauge",
                description_en,
                description_it,
                standards as "standards!: Vec<Standard>",
                created_at,
                last_modified_at,
                version
            FROM scales
            WHERE scale_id = $1"#,
            id
        )
        .fetch_optional(&mut unit_of_work.transaction)
        .await
        .context("A database failure was encountered while trying to fetch a scale.")?;

        Ok(result)
    }
}
