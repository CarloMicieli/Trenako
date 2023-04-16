use anyhow::Context;
use async_trait::async_trait;
use catalog::common::TrackGauge;
use catalog::scales::queries::find_all_scales::FindAllScalesRepository;
use catalog::scales::queries::scale_row::ScaleRow;
use catalog::scales::scale::Scale;
use catalog::scales::scale_id::ScaleId;
use catalog::scales::standard::Standard;
use common::queries::converters::ToOutputConverter;
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
