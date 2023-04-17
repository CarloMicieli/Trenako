use crate::queries::converters::ConversionErrors;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),

    #[error("Error during the row conversion")]
    ConversionError(ConversionErrors),
}

#[derive(Debug, Error)]
pub enum QueryError {
    #[error("No results were found")]
    EmptyResultSet,

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),

    #[error("Error during the row conversion")]
    ConversionError(ConversionErrors),

    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),
}
