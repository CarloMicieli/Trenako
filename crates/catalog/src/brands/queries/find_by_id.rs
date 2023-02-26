use crate::brands::brand::Brand;
use crate::brands::brand_id::BrandId;
use async_trait::async_trait;
use common::unit_of_work::UnitOfWork;
use thiserror::Error;

#[async_trait]
pub trait FindBrandByIdQuery<'db, U: UnitOfWork<'db>> {
    /// Find the brand with the input id (if exists)
    async fn execute(&self, brand_id: &BrandId, unit_of_work: &mut U) -> Result<Brand, QueryError>;
}

#[derive(Debug, Error)]
pub enum QueryError {
    #[error("The result set is empty")]
    EmptyResultSet,

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
