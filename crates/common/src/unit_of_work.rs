use async_trait::async_trait;
use thiserror::Error;

/// It represents a database unit of work, typically to wrap a transaction
#[async_trait]
pub trait UnitOfWork<'transaction> {
    /// Commit the unit of work
    async fn commit(self) -> Result<(), DatabaseError>;
}

/// It represents a database context, usually its main purpose is to
/// start a new database transaction as part of an unit of work
#[async_trait]
pub trait Database<'db, U: UnitOfWork<'db>> {
    /// Creates a new unit of work, starting a new database transaction
    async fn begin(self) -> Result<U, DatabaseError>;
}

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("{0}")]
    GenericError(#[from] sqlx::Error),

    #[error("Could not start a transaction (inner: {0})")]
    BeginTransactionError(String),

    #[error("Could not commit the transaction (inner: {0})")]
    CommitTransactionError(String),
}

pub mod noop {
    use crate::unit_of_work::{Database, DatabaseError, UnitOfWork};
    use async_trait::async_trait;

    pub struct NoOpDatabase;
    pub struct NoOpUnitOfWork;

    #[async_trait]
    impl UnitOfWork<'static> for NoOpUnitOfWork {
        async fn commit(self) -> Result<(), DatabaseError> {
            Ok(())
        }
    }

    #[async_trait]
    impl Database<'static, NoOpUnitOfWork> for NoOpDatabase {
        async fn begin(self) -> Result<NoOpUnitOfWork, DatabaseError> {
            Ok(NoOpUnitOfWork)
        }
    }
}

pub mod postgres {
    use crate::unit_of_work::{Database, DatabaseError, UnitOfWork};
    use async_trait::async_trait;
    use sqlx::{PgPool, Postgres, Transaction};

    /// A postgres concrete implementation for a database connection wrapper
    pub struct PgDatabase<'db> {
        pg_pool: &'db PgPool,
    }

    impl<'db> PgDatabase<'db> {
        /// Creates a new postgres database pooled connection
        pub fn new(pg_pool: &'db PgPool) -> Self {
            PgDatabase { pg_pool }
        }
    }

    #[async_trait]
    impl<'db> Database<'db, PgUnitOfWork<'db>> for PgDatabase<'db> {
        async fn begin(self) -> Result<PgUnitOfWork<'db>, DatabaseError> {
            let transaction = self
                .pg_pool
                .begin()
                .await
                .map_err(|err| DatabaseError::BeginTransactionError(format!("{}", err)))?;
            Ok(PgUnitOfWork { transaction })
        }
    }

    /// A postgres concrete implementation for a database unit of work
    pub struct PgUnitOfWork<'transaction> {
        pub transaction: Transaction<'transaction, Postgres>,
    }

    #[async_trait]
    impl<'transaction> UnitOfWork<'transaction> for PgUnitOfWork<'transaction> {
        async fn commit(self) -> Result<(), DatabaseError> {
            self.transaction
                .commit()
                .await
                .map_err(|err| DatabaseError::CommitTransactionError(format!("{}", err)))?;
            Ok(())
        }
    }
}
