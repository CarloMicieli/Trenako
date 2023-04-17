use crate::brands::brand::Brand;
use crate::brands::brand_id::BrandId;
use async_trait::async_trait;
use common::queries::errors::{DatabaseError, QueryError};
use common::unit_of_work::{Database, UnitOfWork};

/// The query to find a modelling brand by its `brand_id`
pub async fn find_brand_by_id<'db, U, Repo, DB>(brand_id: &BrandId, repo: Repo, db: DB) -> Result<Brand, QueryError>
where
    U: UnitOfWork<'db>,
    Repo: FindBrandByIdRepository<'db, U>,
    DB: Database<'db, U>,
{
    let mut unit_of_work = db.begin().await?;

    let result = repo
        .find_by_id(brand_id, &mut unit_of_work)
        .await?
        .map(Ok)
        .unwrap_or_else(|| Err(QueryError::EmptyResultSet));

    unit_of_work.commit().await?;

    result
}

/// The find brand by id repository
#[async_trait]
pub trait FindBrandByIdRepository<'db, U: UnitOfWork<'db>> {
    async fn find_by_id(&self, brand_id: &BrandId, unit_of_work: &mut U) -> Result<Option<Brand>, DatabaseError>;
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::anyhow;
    use async_trait::async_trait;
    use common::in_memory::InMemoryRepository;
    use common::unit_of_work::noop::NoOpUnitOfWork;
    use tokio;

    mod find_by_id_query {
        use super::*;
        use common::unit_of_work::noop::NoOpDatabase;
        use pretty_assertions::assert_eq;

        #[tokio::test]
        async fn it_should_return_a_result_when_the_brand_is_found() {
            let repo = InMemoryFindBrandByIdRepository::with(brand_with_name("ACME"));

            let result = find_brand_by_id(&BrandId::new("ACME"), repo, NoOpDatabase).await;

            assert!(result.is_ok());
            let result = result.unwrap();
            assert_eq!(BrandId::new("ACME"), result.brand_id);
            assert_eq!("ACME", result.name);
        }

        #[tokio::test]
        async fn it_should_return_an_error_when_the_brand_is_not_found() {
            let repo = InMemoryFindBrandByIdRepository::new();

            let result = find_brand_by_id(&BrandId::new("ACME"), repo, NoOpDatabase).await;

            assert!(result.is_err());
            let error = result.unwrap_err();
            assert_eq!("No results were found", error.to_string());
        }

        #[tokio::test]
        async fn it_should_return_an_error_when_the_query_fails() {
            let repo = FindBrandByIdRepositoryWithError;

            let result = find_brand_by_id(&BrandId::new("ACME"), repo, NoOpDatabase).await;

            assert!(result.is_err());
            let error = result.unwrap_err();
            assert_eq!("something bad happened", error.to_string());
            assert_eq!("something bad happened", error.to_string());
        }
    }

    struct InMemoryFindBrandByIdRepository(InMemoryRepository<BrandId, Brand>);

    impl InMemoryFindBrandByIdRepository {
        pub fn new() -> Self {
            InMemoryFindBrandByIdRepository(InMemoryRepository::empty())
        }

        pub fn with(brand: Brand) -> Self {
            let repo = InMemoryFindBrandByIdRepository(InMemoryRepository::empty());
            repo.0.add(brand.brand_id.clone(), brand);
            repo
        }
    }

    #[async_trait]
    impl FindBrandByIdRepository<'static, NoOpUnitOfWork> for InMemoryFindBrandByIdRepository {
        async fn find_by_id(
            &self,
            brand_id: &BrandId,
            _unit_of_work: &mut NoOpUnitOfWork,
        ) -> Result<Option<Brand>, DatabaseError> {
            Ok(self.0.find_by_id(brand_id))
        }
    }

    fn brand_with_name(name: &str) -> Brand {
        Brand {
            brand_id: BrandId::new(name),
            name: name.to_string(),
            registered_company_name: None,
            organization_entity_type: None,
            group_name: None,
            description: Default::default(),
            address: None,
            contact_info: None,
            kind: Default::default(),
            status: None,
            socials: None,
            metadata: Default::default(),
        }
    }

    struct FindBrandByIdRepositoryWithError;

    #[async_trait]
    impl FindBrandByIdRepository<'static, NoOpUnitOfWork> for FindBrandByIdRepositoryWithError {
        async fn find_by_id(
            &self,
            _brand_id: &BrandId,
            _unit_of_work: &mut NoOpUnitOfWork,
        ) -> Result<Option<Brand>, DatabaseError> {
            Err(DatabaseError::UnexpectedError(anyhow!("something bad happened")))
        }
    }
}
