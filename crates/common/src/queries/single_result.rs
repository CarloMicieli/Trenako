//! the module includes everything related to single result queries

use crate::queries::converters::{ConversionErrors, ToOutputConverter};
use crate::unit_of_work::{Database, UnitOfWork};
use async_trait::async_trait;
use std::fmt;
use thiserror::Error;

/// A query criteria by the given identifier
#[derive(Debug, PartialEq, Eq)]
pub struct ByIdCriteria<'db, Id>
where
    Id: fmt::Display,
{
    /// the identifier to query for
    pub id: &'db Id,
}

impl<'db, Id> ByIdCriteria<'db, Id>
where
    Id: fmt::Display,
{
    pub fn new(id: &'db Id) -> Self {
        ByIdCriteria { id }
    }
}

#[async_trait]
pub trait QueryRepository<'db, U: UnitOfWork<'db>, Id, RowType> {
    /// Find the row with the given `id` (if exists).
    async fn find_by_id(id: &Id, unit_of_work: &mut U) -> Result<Option<RowType>, anyhow::Error>;
}

/// A trait for queries which return just a single result.
#[async_trait]
pub trait SingleResultQuery<
    'db,
    U: Send + Sync + UnitOfWork<'db>,
    DB: Send + Sync + Database<'db, U> + 'db,
    Repo: Send + Sync + QueryRepository<'db, U, Self::Id, Self::RowType>,
>
{
    /// the row identifier type
    type Id: fmt::Display + Send + Sync;
    /// the row type
    type RowType: Send + Sync + ToOutputConverter<Self::Output>;
    /// the output type
    type Output: Send + Sync;

    /// Execute the query with the given `criteria` against the `db` database.
    async fn execute(criteria: &ByIdCriteria<'db, Self::Id>, db: DB) -> Result<Self::Output, QueryError> {
        let mut unit_of_work = db.begin().await?;

        let result = Repo::find_by_id(criteria.id, &mut unit_of_work).await?;

        let result = result
            .map(|it| it.to_output())
            .map(|it| it.map_err(QueryError::ConversionError))
            .unwrap_or_else(|| Err(QueryError::EmptyResultSet));

        unit_of_work.commit().await?;

        result
    }
}

#[derive(Debug, Error)]
pub enum QueryError {
    #[error("No results were found")]
    EmptyResultSet,

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),

    #[error("Error during the row conversion")]
    ConversionError(ConversionErrors),
}

#[cfg(test)]
mod test {
    use super::*;

    mod by_id_criteria {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_new_criteria() {
            let id = String::from("My ID");
            let criteria = ByIdCriteria::new(&id);

            assert_eq!(id, criteria.id.to_string());
        }
    }

    pub mod single_result_query {
        use super::*;
        use crate::queries::single_result::test::in_memory::{
            InMemorySingleResultQuery, CONVERSION_ERROR_ID, GENERIC_ERROR_ID, NOT_FOUND_ID, ROW_EXISTS_ID,
        };
        use crate::unit_of_work::noop::NoOpDatabase;
        use tokio;

        #[tokio::test]
        async fn it_should_return_an_output_when_the_row_is_found() {
            let criteria = ByIdCriteria::new(&ROW_EXISTS_ID);
            let result = InMemorySingleResultQuery::execute(&criteria, NoOpDatabase).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn it_should_return_an_empty_result_set_error_when_the_row_is_not_found() {
            let criteria = ByIdCriteria::new(&NOT_FOUND_ID);
            let result = InMemorySingleResultQuery::execute(&criteria, NoOpDatabase).await;
            assert!(result.is_err());
            assert_eq!(result.unwrap_err().to_string(), "No results were found");
        }

        #[tokio::test]
        async fn it_should_return_a_conversion_error_when_the_row_is_invalid() {
            let criteria = ByIdCriteria::new(&CONVERSION_ERROR_ID);
            let result = InMemorySingleResultQuery::execute(&criteria, NoOpDatabase).await;
            assert!(result.is_err());
            assert_eq!(result.unwrap_err().to_string(), "Error during the row conversion");
        }

        #[tokio::test]
        async fn it_should_return_a_generic_error_when_the_row_is_invalid() {
            let criteria = ByIdCriteria::new(&GENERIC_ERROR_ID);
            let result = InMemorySingleResultQuery::execute(&criteria, NoOpDatabase).await;
            assert!(result.is_err());
            assert_eq!(result.unwrap_err().to_string(), "something bad happened");
        }
    }

    pub mod in_memory {
        use super::*;
        use crate::unit_of_work::noop::{NoOpDatabase, NoOpUnitOfWork};
        use anyhow;
        use async_trait::async_trait;

        pub trait MockResponse<Id, RowType> {
            fn find_by_id(id: &Id) -> Result<Option<RowType>, anyhow::Error>;
        }

        #[async_trait]
        impl<Id: Send + Sync, RowType, Repo: MockResponse<Id, RowType>>
            QueryRepository<'static, NoOpUnitOfWork, Id, RowType> for Repo
        {
            async fn find_by_id(id: &Id, _unit_of_work: &mut NoOpUnitOfWork) -> Result<Option<RowType>, anyhow::Error> {
                Repo::find_by_id(id)
            }
        }

        pub const GENERIC_ERROR_ID: u32 = 0;
        pub const CONVERSION_ERROR_ID: u32 = 17;
        pub const NOT_FOUND_ID: u32 = 42;
        pub const ROW_EXISTS_ID: u32 = 10;

        pub struct InMemoryRepo;

        impl MockResponse<u32, RootRow> for InMemoryRepo {
            fn find_by_id(id: &u32) -> Result<Option<RootRow>, anyhow::Error> {
                match *id {
                    GENERIC_ERROR_ID => Err(anyhow::anyhow!("something bad happened")),
                    NOT_FOUND_ID => Ok(None),
                    _ => Ok(Some(RootRow {
                        id: *id,
                        name: format!("row {}", id),
                    })),
                }
            }
        }

        pub struct InMemorySingleResultQuery;

        #[async_trait]
        impl SingleResultQuery<'static, NoOpUnitOfWork, NoOpDatabase, InMemoryRepo> for InMemorySingleResultQuery {
            type Id = u32;
            type RowType = RootRow;
            type Output = Root;
        }

        impl ToOutputConverter<Root> for RootRow {
            fn to_output(self) -> Result<Root, ConversionErrors> {
                if self.id == CONVERSION_ERROR_ID {
                    Err(ConversionErrors::new())
                } else {
                    Ok(Root {
                        id: self.id,
                        name: self.name,
                    })
                }
            }
        }

        #[derive(Debug, PartialEq)]
        pub struct RootRow {
            pub id: u32,
            pub name: String,
        }

        #[derive(Debug, PartialEq)]
        pub struct Root {
            pub id: u32,
            pub name: String,
        }
    }
}
