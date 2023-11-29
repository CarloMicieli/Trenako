use crate::scales::commands::new_scales::NewScaleCommand;
use crate::scales::scale_id::ScaleId;
use async_trait::async_trait;
use common::unit_of_work::UnitOfWork;

/// The persistence related functionality for the scale commands
#[async_trait]
pub trait NewScaleRepository<'db, U: UnitOfWork<'db>> {
    /// Checks if a scale with the input id already exists
    async fn exists(&self, scale_id: &ScaleId, unit_of_work: &mut U) -> Result<bool, anyhow::Error>;

    /// Inserts a new scale
    async fn insert(&self, new_scale: &NewScaleCommand, unit_of_work: &mut U) -> Result<(), anyhow::Error>;
}

#[cfg(test)]
pub mod in_memory {
    use crate::scales::commands::new_scales::NewScaleCommand;
    use crate::scales::commands::repositories::NewScaleRepository;
    use crate::scales::scale_id::ScaleId;
    use async_trait::async_trait;
    use common::in_memory::InMemoryRepository;
    use common::unit_of_work::noop::NoOpUnitOfWork;

    /// An in-memory scale repository
    pub struct InMemoryScaleRepository(InMemoryRepository<ScaleId, NewScaleCommand>);

    impl InMemoryScaleRepository {
        /// Creates an empty in memory scales repository
        pub fn empty() -> Self {
            InMemoryScaleRepository(InMemoryRepository::empty())
        }

        /// Creates a new in-memory scales repository with an initial element
        pub fn with(command: NewScaleCommand) -> Self {
            let id = ScaleId::new(&command.scale_id);
            InMemoryScaleRepository(InMemoryRepository::of(id, command))
        }
    }

    #[async_trait]
    impl NewScaleRepository<'static, NoOpUnitOfWork> for InMemoryScaleRepository {
        async fn exists(&self, scale_id: &ScaleId, _unit_of_work: &mut NoOpUnitOfWork) -> Result<bool, anyhow::Error> {
            Ok(self.0.contains(scale_id))
        }

        async fn insert(
            &self,
            new_scale: &NewScaleCommand,
            _unit_of_work: &mut NoOpUnitOfWork,
        ) -> Result<(), anyhow::Error> {
            let id = ScaleId::new(&new_scale.scale_id);
            self.0.add(id, new_scale.clone());
            Ok(())
        }
    }
}
