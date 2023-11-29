//! the module includes everything related to unit of works

use async_trait::async_trait;

/// It represents a database unit of work, typically to wrap a transaction
#[async_trait]
pub trait UnitOfWork<'transaction> {
    /// Commit the unit of work
    async fn commit(self) -> Result<(), anyhow::Error>;
}

/// It represents a database context, usually its main purpose is to
/// start a new database transaction as part of an unit of work
#[async_trait]
pub trait Database<'db, U: UnitOfWork<'db>> {
    /// Creates a new unit of work, starting a new database transaction
    async fn begin(self) -> Result<U, anyhow::Error>;
}

/// A no-op unit of work, useful for testing
pub mod noop {
    use crate::unit_of_work::{Database, UnitOfWork};
    use async_trait::async_trait;

    pub struct NoOpDatabase;
    pub struct NoOpUnitOfWork;

    #[async_trait]
    impl UnitOfWork<'static> for NoOpUnitOfWork {
        async fn commit(self) -> Result<(), anyhow::Error> {
            Ok(())
        }
    }

    #[async_trait]
    impl Database<'static, NoOpUnitOfWork> for NoOpDatabase {
        async fn begin(self) -> Result<NoOpUnitOfWork, anyhow::Error> {
            Ok(NoOpUnitOfWork)
        }
    }
}

/// A unit of work implementation for Postgres
pub mod postgres {
    use crate::unit_of_work::{Database, UnitOfWork};
    use anyhow::Context;
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
        async fn begin(self) -> Result<PgUnitOfWork<'db>, anyhow::Error> {
            let transaction = self.pg_pool.begin().await.context("Could not begin the transaction")?;
            Ok(PgUnitOfWork { transaction })
        }
    }

    /// A postgres concrete implementation for a database unit of work
    pub struct PgUnitOfWork<'transaction> {
        pub transaction: Transaction<'transaction, Postgres>,
    }

    #[async_trait]
    impl<'transaction> UnitOfWork<'transaction> for PgUnitOfWork<'transaction> {
        async fn commit(self) -> Result<(), anyhow::Error> {
            self.transaction
                .commit()
                .await
                .context("Could not commit the transaction")?;
            Ok(())
        }
    }
}
