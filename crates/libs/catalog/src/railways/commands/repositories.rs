use crate::railways::commands::new_railways::NewRailwayCommand;
use crate::railways::railway_id::RailwayId;
use async_trait::async_trait;
use common::unit_of_work::UnitOfWork;

/// The persistence related functionality for the railway commands
#[async_trait]
pub trait NewRailwayRepository<'db, U: UnitOfWork<'db>> {
    /// Checks if a railway with the input id already exists
    async fn exists(&self, railway_id: &RailwayId, unit_of_work: &mut U) -> Result<bool, anyhow::Error>;

    /// Inserts a new railway
    async fn insert(&self, new_railway: &NewRailwayCommand, unit_of_work: &mut U) -> Result<(), anyhow::Error>;
}

#[cfg(test)]
pub mod in_memory {
    use crate::railways::commands::new_railways::NewRailwayCommand;
    use crate::railways::commands::repositories::NewRailwayRepository;
    use crate::railways::railway_id::RailwayId;
    use async_trait::async_trait;
    use common::in_memory::InMemoryRepository;
    use common::unit_of_work::noop::NoOpUnitOfWork;

    /// An in-memory railway repository
    pub struct InMemoryRailwayRepository(InMemoryRepository<RailwayId, NewRailwayCommand>);

    impl InMemoryRailwayRepository {
        /// Creates an empty in memory railways repository
        pub fn empty() -> Self {
            InMemoryRailwayRepository(InMemoryRepository::empty())
        }

        /// Creates a new in-memory railways repository with an initial element
        pub fn with(new_railway_command: NewRailwayCommand) -> Self {
            let id = RailwayId::new(&new_railway_command.railway_id);
            InMemoryRailwayRepository(InMemoryRepository::of(id, new_railway_command))
        }
    }

    #[async_trait]
    impl NewRailwayRepository<'static, NoOpUnitOfWork> for InMemoryRailwayRepository {
        async fn exists(
            &self,
            railway_id: &RailwayId,
            _unit_of_work: &mut NoOpUnitOfWork,
        ) -> Result<bool, anyhow::Error> {
            Ok(self.0.contains(railway_id))
        }

        async fn insert(
            &self,
            new_railway: &NewRailwayCommand,
            _unit_of_work: &mut NoOpUnitOfWork,
        ) -> Result<(), anyhow::Error> {
            let id = RailwayId::new(&new_railway.railway_id);
            self.0.add(id, new_railway.clone());
            Ok(())
        }
    }
}
