use crate::catalog::scales::scale_row::ScaleRow;
use anyhow::Context;
use async_trait::async_trait;
use catalog::common::TrackGauge;
use catalog::scales::queries::find_all_scales::FindAllScalesRepository;
use catalog::scales::queries::find_scale_by_id::FindScaleByIdRepository;
use catalog::scales::scale::Scale;
use catalog::scales::scale_id::ScaleId;
use catalog::scales::standard::Standard;
use common::queries::converters::ToOutputConverter;
use common::queries::errors::DatabaseError;
use common::queries::single_result::QueryError;
use common::unit_of_work::postgres::PgUnitOfWork;

#[derive(Debug)]
pub struct ScalesRepository;

#[async_trait]
impl<'db> FindAllScalesRepository<'db, PgUnitOfWork<'db>> for ScalesRepository {
    async fn find_all(&self, unit_of_work: &mut PgUnitOfWork) -> Result<Vec<Scale>, QueryError> {
        let results = sqlx::query_as!(
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
            ORDER BY name"#
        )
        .fetch_all(&mut unit_of_work.transaction)
        .await
        .context("A database failure was encountered while trying to fetch scales.")?;

        let mut output: Vec<Scale> = Vec::with_capacity(results.len());
        for row in results.into_iter() {
            let scale = row.to_output().map_err(QueryError::ConversionError)?;
            output.push(scale);
        }

        Ok(output)
    }
}

#[async_trait]
impl<'db> FindScaleByIdRepository<'db, PgUnitOfWork<'db>> for ScalesRepository {
    async fn find_by_id(
        &self,
        scale_id: &ScaleId,
        unit_of_work: &mut PgUnitOfWork<'db>,
    ) -> Result<Option<Scale>, DatabaseError> {
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
            scale_id
        )
        .fetch_optional(&mut unit_of_work.transaction)
        .await
        .context("A database failure was encountered while trying to fetch a scale.")?;

        result.map(row_to_scale).transpose()
    }
}

fn row_to_scale(row: ScaleRow) -> Result<Scale, DatabaseError> {
    row.to_output().map_err(DatabaseError::ConversionError)
}
