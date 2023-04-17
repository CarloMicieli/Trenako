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
