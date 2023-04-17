use crate::scales::scale::Scale;
use async_trait::async_trait;
use common::queries::errors::{DatabaseError, QueryError};
use common::unit_of_work::{Database, UnitOfWork};

pub async fn find_all_scales<'db, U, Repo, DB>(repo: Repo, db: DB) -> Result<Vec<Scale>, QueryError>
where
    U: UnitOfWork<'db>,
    Repo: FindAllScalesRepository<'db, U>,
    DB: Database<'db, U>,
{
    let mut unit_of_work = db.begin().await?;

    let result = repo.find_all(&mut unit_of_work).await?;

    unit_of_work.commit().await?;

    Ok(result)
}

#[async_trait]
pub trait FindAllScalesRepository<'db, U: UnitOfWork<'db>> {
    async fn find_all(&self, unit_of_work: &mut U) -> Result<Vec<Scale>, DatabaseError>;
}
