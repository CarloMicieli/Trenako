//! the brand command repositories

use crate::brands::brand_id::BrandId;
use crate::brands::commands::new_brand::NewBrandCommand;
use async_trait::async_trait;
use common::unit_of_work::UnitOfWork;

/// The persistence related functionality for the brand commands
#[async_trait]
pub trait BrandRepository<'db, U: UnitOfWork<'db>> {
    /// Checks if a brand with the input id already exists
    async fn exists(&self, brand_id: &BrandId, unit_of_work: &mut U) -> Result<bool, anyhow::Error>;

    /// Inserts a new brand
    async fn insert(&self, new_brand: &NewBrandCommand, unit_of_work: &mut U) -> Result<(), anyhow::Error>;
}

#[cfg(test)]
pub mod in_memory {
    use crate::brands::brand_id::BrandId;
    use crate::brands::commands::new_brand::NewBrandCommand;
    use crate::brands::commands::repositories::BrandRepository;
    use async_trait::async_trait;
    use common::in_memory::InMemoryRepository;
    use common::unit_of_work::noop::NoOpUnitOfWork;

    /// An in-memory brand repository
    pub struct InMemoryBrandRepository(InMemoryRepository<BrandId, NewBrandCommand>);

    impl InMemoryBrandRepository {
        /// Creates an empty in memory brands repository
        pub fn empty() -> Self {
            InMemoryBrandRepository(InMemoryRepository::empty())
        }

        /// Creates a new in-memory brands repository with an initial element
        pub fn with(command: NewBrandCommand) -> Self {
            let id = BrandId::new(&command.brand_id);
            InMemoryBrandRepository(InMemoryRepository::of(id, command))
        }
    }

    #[async_trait]
    impl BrandRepository<'static, NoOpUnitOfWork> for InMemoryBrandRepository {
        async fn exists(&self, brand_id: &BrandId, _unit_of_work: &mut NoOpUnitOfWork) -> Result<bool, anyhow::Error> {
            Ok(self.0.contains(brand_id))
        }

        async fn insert(
            &self,
            new_brand: &NewBrandCommand,
            _unit_of_work: &mut NoOpUnitOfWork,
        ) -> Result<(), anyhow::Error> {
            let id = BrandId::new(&new_brand.brand_id);
            self.0.add(id, new_brand.clone());
            Ok(())
        }
    }
}
