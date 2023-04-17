use crate::railways::railway::Railway;
use crate::railways::railway_id::RailwayId;
use async_trait::async_trait;
use common::queries::errors::DatabaseError;
use common::queries::errors::QueryError;
use common::unit_of_work::{Database, UnitOfWork};

/// The query to find a railway company with the given `railway_id`
pub async fn find_railway_by_id<'db, U, Repo, DB>(
    railway_id: &RailwayId,
    repo: Repo,
    db: DB,
) -> Result<Railway, QueryError>
where
    U: UnitOfWork<'db>,
    Repo: FindRailwayByIdRepository<'db, U>,
    DB: Database<'db, U>,
{
    let mut unit_of_work = db.begin().await?;

    let result = repo
        .find_by_id(railway_id, &mut unit_of_work)
        .await?
        .map(Ok)
        .unwrap_or_else(|| Err(QueryError::EmptyResultSet));

    unit_of_work.commit().await?;

    result
}

#[async_trait]
pub trait FindRailwayByIdRepository<'db, U: UnitOfWork<'db>> {
    async fn find_by_id(&self, railway_id: &RailwayId, unit_of_work: &mut U) -> Result<Option<Railway>, DatabaseError>;
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::anyhow;
    use async_trait::async_trait;
    use common::in_memory::InMemoryRepository;
    use common::queries::errors::DatabaseError;
    use common::unit_of_work::noop::NoOpUnitOfWork;
    use isocountry::CountryCode;
    use tokio;

    mod find_by_id_query {
        use super::*;
        use common::unit_of_work::noop::NoOpDatabase;
        use pretty_assertions::assert_eq;

        #[tokio::test]
        async fn it_should_return_a_result_when_the_railway_is_found() {
            let repo = InMemoryFindRailwayByIdRepository::with(railway_with_name("FS"));

            let result = find_railway_by_id(&RailwayId::new("FS"), repo, NoOpDatabase).await;

            assert!(result.is_ok());
            let result = result.unwrap();
            assert_eq!(RailwayId::new("FS"), result.railway_id);
            assert_eq!("FS", result.name);
        }

        #[tokio::test]
        async fn it_should_return_an_error_when_the_railway_is_not_found() {
            let repo = InMemoryFindRailwayByIdRepository::new();

            let result = find_railway_by_id(&RailwayId::new("ACME"), repo, NoOpDatabase).await;

            assert!(result.is_err());
            let error = result.unwrap_err();
            assert_eq!("No results were found", error.to_string());
        }

        #[tokio::test]
        async fn it_should_return_an_error_when_the_query_fails() {
            let repo = FindRailwayByIdRepositoryWithError;

            let result = find_railway_by_id(&RailwayId::new("ACME"), repo, NoOpDatabase).await;

            assert!(result.is_err());
            let error = result.unwrap_err();
            assert_eq!("something bad happened", error.to_string());
            assert_eq!("something bad happened", error.to_string());
        }
    }

    struct InMemoryFindRailwayByIdRepository(InMemoryRepository<RailwayId, Railway>);

    impl InMemoryFindRailwayByIdRepository {
        pub fn new() -> Self {
            InMemoryFindRailwayByIdRepository(InMemoryRepository::empty())
        }

        pub fn with(railway: Railway) -> Self {
            let repo = InMemoryFindRailwayByIdRepository(InMemoryRepository::empty());
            repo.0.add(railway.railway_id.clone(), railway);
            repo
        }
    }

    #[async_trait]
    impl FindRailwayByIdRepository<'static, NoOpUnitOfWork> for InMemoryFindRailwayByIdRepository {
        async fn find_by_id(
            &self,
            railway_id: &RailwayId,
            _unit_of_work: &mut NoOpUnitOfWork,
        ) -> Result<Option<Railway>, DatabaseError> {
            Ok(self.0.find_by_id(railway_id))
        }
    }

    fn railway_with_name(name: &str) -> Railway {
        Railway {
            railway_id: RailwayId::new(name),
            name: name.to_string(),
            abbreviation: None,
            registered_company_name: None,
            organization_entity_type: None,
            description: Default::default(),
            country: CountryCode::ITA,
            period_of_activity: None,
            gauge: None,
            headquarters: vec![],
            total_length: None,
            contact_info: None,
            socials: None,
            metadata: Default::default(),
        }
    }

    struct FindRailwayByIdRepositoryWithError;

    #[async_trait]
    impl FindRailwayByIdRepository<'static, NoOpUnitOfWork> for FindRailwayByIdRepositoryWithError {
        async fn find_by_id(
            &self,
            _railway_id: &RailwayId,
            _unit_of_work: &mut NoOpUnitOfWork,
        ) -> Result<Option<Railway>, DatabaseError> {
            Err(DatabaseError::UnexpectedError(anyhow!("something bad happened")))
        }
    }
}
