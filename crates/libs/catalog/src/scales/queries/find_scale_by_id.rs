use crate::scales::scale::Scale;
use crate::scales::scale_id::ScaleId;
use async_trait::async_trait;
use common::queries::errors::{DatabaseError, QueryError};
use common::unit_of_work::{Database, UnitOfWork};

/// The query to find a modelling scale with the given `scale_id`
pub async fn find_scale_by_id<'db, U, Repo, DB>(scale_id: &ScaleId, repo: Repo, db: DB) -> Result<Scale, QueryError>
where
    U: UnitOfWork<'db>,
    Repo: FindScaleByIdRepository<'db, U>,
    DB: Database<'db, U>,
{
    let mut unit_of_work = db.begin().await?;

    let result = repo
        .find_by_id(scale_id, &mut unit_of_work)
        .await?
        .map(Ok)
        .unwrap_or_else(|| Err(QueryError::EmptyResultSet));

    unit_of_work.commit().await?;

    result
}

#[async_trait]
pub trait FindScaleByIdRepository<'db, U: UnitOfWork<'db>> {
    async fn find_by_id(&self, scale_id: &ScaleId, unit_of_work: &mut U) -> Result<Option<Scale>, DatabaseError>;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::scales::ratio::Ratio;
    use crate::scales::scale_gauge::Gauge;
    use anyhow::anyhow;
    use async_trait::async_trait;
    use common::in_memory::InMemoryRepository;
    use common::queries::errors::DatabaseError;
    use common::unit_of_work::noop::NoOpUnitOfWork;
    use rust_decimal_macros::dec;
    use tokio;

    mod find_by_id_query {
        use super::*;
        use common::unit_of_work::noop::NoOpDatabase;
        use pretty_assertions::assert_eq;

        #[tokio::test]
        async fn it_should_return_a_result_when_the_scale_is_found() {
            let repo = InMemoryFindScaleByIdRepository::with(scale_with_name("H0"));

            let result = find_scale_by_id(&ScaleId::new("H0"), repo, NoOpDatabase).await;

            assert!(result.is_ok());
            let result = result.unwrap();
            assert_eq!(ScaleId::new("H0"), result.scale_id);
            assert_eq!("H0", result.name);
        }

        #[tokio::test]
        async fn it_should_return_an_error_when_the_scale_is_not_found() {
            let repo = InMemoryFindScaleByIdRepository::new();

            let result = find_scale_by_id(&ScaleId::new("H0"), repo, NoOpDatabase).await;

            assert!(result.is_err());
            let error = result.unwrap_err();
            assert_eq!("No results were found", error.to_string());
        }

        #[tokio::test]
        async fn it_should_return_an_error_when_the_query_fails() {
            let repo = FindScaleByIdRepositoryWithError;

            let result = find_scale_by_id(&ScaleId::new("H0"), repo, NoOpDatabase).await;

            assert!(result.is_err());
            let error = result.unwrap_err();
            assert_eq!("something bad happened", error.to_string());
            assert_eq!("something bad happened", error.to_string());
        }
    }

    struct InMemoryFindScaleByIdRepository(InMemoryRepository<ScaleId, Scale>);

    impl InMemoryFindScaleByIdRepository {
        pub fn new() -> Self {
            InMemoryFindScaleByIdRepository(InMemoryRepository::empty())
        }

        pub fn with(scale: Scale) -> Self {
            let repo = InMemoryFindScaleByIdRepository(InMemoryRepository::empty());
            repo.0.add(scale.scale_id.clone(), scale);
            repo
        }
    }

    #[async_trait]
    impl FindScaleByIdRepository<'static, NoOpUnitOfWork> for InMemoryFindScaleByIdRepository {
        async fn find_by_id(
            &self,
            scale_id: &ScaleId,
            _unit_of_work: &mut NoOpUnitOfWork,
        ) -> Result<Option<Scale>, DatabaseError> {
            Ok(self.0.find_by_id(scale_id))
        }
    }

    fn scale_with_name(name: &str) -> Scale {
        Scale {
            scale_id: ScaleId::new(name),
            name: name.to_string(),
            ratio: Ratio::try_from(dec!(87.0)).unwrap(),
            gauge: Gauge::H0,
            description: Default::default(),
            standards: Default::default(),
            metadata: Default::default(),
        }
    }

    struct FindScaleByIdRepositoryWithError;

    #[async_trait]
    impl FindScaleByIdRepository<'static, NoOpUnitOfWork> for FindScaleByIdRepositoryWithError {
        async fn find_by_id(
            &self,
            _scale_id: &ScaleId,
            _unit_of_work: &mut NoOpUnitOfWork,
        ) -> Result<Option<Scale>, DatabaseError> {
            Err(DatabaseError::UnexpectedError(anyhow!("something bad happened")))
        }
    }
}
