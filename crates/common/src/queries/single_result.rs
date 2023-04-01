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
    async fn find_by_id(id: &Id, unit_of_work: &mut U) -> Result<Option<RowType>, anyhow::Error>;
}

#[async_trait]
pub trait SingleResultQuery<
    'db,
    U: Send + Sync + UnitOfWork<'db>,
    DB: Send + Sync + Database<'db, U> + 'db,
    Repo: Send + Sync + QueryRepository<'db, U, Self::Id, Self::RowType>,
>
{
    type Id: fmt::Display + Send + Sync;
    type RowType: Send + Sync + ToOutputConverter<Self::Output>;
    type Output: Send + Sync;

    /// Execute the query using the given criteria and unit_of_work.
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
}
