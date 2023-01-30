use crate::brands::brand_id::BrandId;
use crate::catalog_items::availability_status::AvailabilityStatus;
use crate::catalog_items::catalog_item_id::CatalogItemId;
use crate::catalog_items::catalog_item_request::CatalogItemRequest;
use crate::catalog_items::catalog_item_response::CatalogItemCreated;
use crate::catalog_items::category::{
    Category, ElectricMultipleUnitType, FreightCarType, LocomotiveType, PassengerCarType, RailcarType,
    RollingStockCategory,
};
use crate::catalog_items::control::{Control, DccInterface};
use crate::catalog_items::delivery_date::DeliveryDate;
use crate::catalog_items::epoch::Epoch;
use crate::catalog_items::item_number::ItemNumber;
use crate::catalog_items::power_method::PowerMethod;
use crate::catalog_items::rolling_stock_id::RollingStockId;
use crate::catalog_items::rolling_stock_request::RollingStockRequest;
use crate::catalog_items::service_level::ServiceLevel;
use crate::catalog_items::technical_specifications::{Coupling, CouplingSocket, FeatureFlag, Radius};
use crate::railways::railway_id::RailwayId;
use crate::scales::scale_id::ScaleId;
use async_trait::async_trait;
use chrono::Utc;
use common::length::Length;
use common::localized_text::LocalizedText;
use common::metadata::Metadata;
use common::unit_of_work::{Database, DatabaseError, UnitOfWork};
use std::result;
use thiserror::Error;

pub type Result<R> = result::Result<R, CatalogItemCreationError>;

pub async fn create_new_catalog_item<
    'db,
    U: UnitOfWork<'db>,
    R: NewCatalogItemRepository<'db, U>,
    RR: NewRollingStockRepository<'db, U>,
    DB: Database<'db, U>,
>(
    request: CatalogItemRequest,
    repo: R,
    rs_repo: RR,
    db: DB,
) -> Result<CatalogItemCreated> {
    let brand_id = BrandId::new(&request.brand);
    let scale_id = ScaleId::new(&request.scale);
    let catalog_item_id = CatalogItemId::of(&brand_id, &request.item_number);

    let mut unit_of_work = db.begin().await?;

    if repo.exists_already(&catalog_item_id, &mut unit_of_work).await? {
        return Err(CatalogItemCreationError::CatalogItemAlreadyExists(catalog_item_id));
    }

    if !repo.brand_exists(&brand_id, &mut unit_of_work).await? {
        return Err(CatalogItemCreationError::BrandNotFound(request.brand.to_string()));
    }

    if !repo.scale_exists(&scale_id, &mut unit_of_work).await? {
        return Err(CatalogItemCreationError::ScaleNotFound(request.scale.to_string()));
    }

    let command = NewCatalogItemCommand::try_from(request)?;

    repo.insert(&command, &mut unit_of_work).await?;

    for rs in command.rolling_stocks {
        rs_repo.insert(&rs, &mut unit_of_work).await?;
    }

    unit_of_work.commit().await?;

    Ok(CatalogItemCreated {
        catalog_item_id,
        created_at: *command.metadata.created(),
    })
}

#[derive(Debug, Clone)]
pub struct NewCatalogItemCommand {
    pub catalog_item_id: CatalogItemId,
    pub payload: CatalogItemCommandPayload,
    pub rolling_stocks: Vec<NewRollingStockCommand>,
    pub metadata: Metadata,
}

impl TryFrom<CatalogItemRequest> for NewCatalogItemCommand {
    type Error = CatalogItemCreationError;

    fn try_from(request: CatalogItemRequest) -> result::Result<Self, Self::Error> {
        let brand_id = BrandId::new(&request.brand);
        let catalog_item_id = CatalogItemId::of(&brand_id, &request.item_number);

        let rolling_stocks: Vec<NewRollingStockCommand> = request
            .rolling_stocks
            .clone()
            .into_iter()
            .map(|rs| NewRollingStockCommand::new(&catalog_item_id, rs).unwrap())
            .collect();
        let payload = CatalogItemCommandPayload::try_from(request)?;
        let metadata = Metadata::created_at(Utc::now());

        Ok(NewCatalogItemCommand {
            catalog_item_id,
            payload,
            rolling_stocks,
            metadata,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CatalogItemCommandPayload {
    pub brand_id: BrandId,
    pub item_number: ItemNumber,
    pub scale_id: ScaleId,
    pub category: Category,
    pub description: LocalizedText,
    pub details: LocalizedText,
    pub power_method: PowerMethod,
    pub delivery_date: Option<DeliveryDate>,
    pub availability_status: Option<AvailabilityStatus>,
    pub count: i32,
}

impl TryFrom<CatalogItemRequest> for CatalogItemCommandPayload {
    type Error = CatalogItemCreationError;

    fn try_from(request: CatalogItemRequest) -> result::Result<Self, Self::Error> {
        let brand_id = BrandId::new(&request.brand);
        let scale_id = ScaleId::new(&request.scale);

        let payload = CatalogItemCommandPayload {
            brand_id,
            item_number: request.item_number,
            scale_id,
            category: request.category,
            description: request.description,
            details: request.details,
            power_method: request.power_method,
            delivery_date: request.delivery_date,
            availability_status: request.availability_status,
            count: request.count,
        };
        Ok(payload)
    }
}

#[derive(Debug, Clone)]
pub struct NewRollingStockCommand {
    pub catalog_item_id: CatalogItemId,
    pub rolling_stock_id: RollingStockId,
    pub railway_id: RailwayId,
    pub payload: RollingStockPayload,
}

impl NewRollingStockCommand {
    pub fn new(catalog_item_id: &CatalogItemId, request: RollingStockRequest) -> Result<NewRollingStockCommand> {
        Ok(NewRollingStockCommand {
            catalog_item_id: catalog_item_id.clone(),
            rolling_stock_id: RollingStockId::new(),
            railway_id: RailwayId::new(request.railway()),
            payload: RollingStockPayload::try_from(request)?,
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct RollingStockPayload {
    pub category: Option<RollingStockCategory>,
    pub epoch: Option<Epoch>,
    pub livery: Option<String>,
    pub length_over_buffers_mm: Option<Length>,
    pub length_over_buffers_in: Option<Length>,
    pub type_name: Option<String>,
    pub road_number: Option<String>,
    pub series: Option<String>,
    pub depot: Option<String>,
    pub dcc_interface: Option<DccInterface>,
    pub control: Option<Control>,
    pub electric_multiple_unit_type: Option<ElectricMultipleUnitType>,
    pub freight_car_type: Option<FreightCarType>,
    pub locomotive_type: Option<LocomotiveType>,
    pub passenger_car_type: Option<PassengerCarType>,
    pub railcar_type: Option<RailcarType>,
    pub service_level: Option<ServiceLevel>,
    pub is_dummy: bool,
    pub minimum_radius: Option<Radius>,
    pub coupling_socket: Option<CouplingSocket>,
    pub close_couplers: Option<FeatureFlag>,
    pub digital_shunting_coupling: Option<FeatureFlag>,
    pub flywheel_fitted: Option<FeatureFlag>,
    pub metal_body: Option<FeatureFlag>,
    pub interior_lights: Option<FeatureFlag>,
    pub lights: Option<FeatureFlag>,
    pub spring_buffers: Option<FeatureFlag>,
}

impl TryFrom<RollingStockRequest> for RollingStockPayload {
    type Error = CatalogItemCreationError;

    fn try_from(request: RollingStockRequest) -> result::Result<Self, Self::Error> {
        let category = request.category();
        let (minimum_radius, coupling, flywheel_fitted, metal_body, interior_lights, lights, spring_buffers) =
            if let Some(ts) = request.technical_specifications() {
                (
                    ts.minimum_radius,
                    ts.coupling,
                    ts.flywheel_fitted,
                    ts.metal_body,
                    ts.interior_lights,
                    ts.lights,
                    ts.spring_buffers,
                )
            } else {
                (
                    None,
                    None,
                    FeatureFlag::default(),
                    FeatureFlag::default(),
                    FeatureFlag::default(),
                    FeatureFlag::default(),
                    FeatureFlag::default(),
                )
            };

        let Coupling {
            socket,
            close_couplers,
            digital_shunting,
        } = coupling.unwrap_or_default();

        let (millimeters, inches) = if let Some(length_over_buffers) = request.length_over_buffers() {
            (length_over_buffers.millimeters, length_over_buffers.inches)
        } else {
            (None, None)
        };

        match request {
            RollingStockRequest::ElectricMultipleUnitRequest {
                railway: _,
                epoch,
                livery,
                length_over_buffers: _,
                technical_specifications: _,
                type_name,
                road_number,
                series,
                depot,
                electric_multiple_unit_type,
                dcc_interface,
                control,
                is_dummy,
            } => Ok(RollingStockPayload {
                category: Some(category),
                epoch: Some(epoch),
                livery,
                length_over_buffers_mm: millimeters,
                length_over_buffers_in: inches,
                type_name: Some(type_name),
                road_number,
                series,
                depot,
                dcc_interface,
                control,
                electric_multiple_unit_type: Some(electric_multiple_unit_type),
                is_dummy,
                minimum_radius,
                coupling_socket: Some(socket),
                close_couplers: Some(close_couplers),
                digital_shunting_coupling: Some(digital_shunting),
                flywheel_fitted: Some(flywheel_fitted),
                metal_body: Some(metal_body),
                interior_lights: Some(interior_lights),
                lights: Some(lights),
                spring_buffers: Some(spring_buffers),
                ..RollingStockPayload::default()
            }),
            RollingStockRequest::RailcarRequest {
                railway: _,
                epoch,
                livery,
                length_over_buffers: _,
                technical_specifications: _,
                type_name,
                road_number,
                series,
                depot,
                railcar_type,
                dcc_interface,
                control,
                is_dummy,
            } => Ok(RollingStockPayload {
                category: Some(category),
                epoch: Some(epoch),
                livery,
                length_over_buffers_mm: millimeters,
                length_over_buffers_in: inches,
                type_name: Some(type_name),
                road_number,
                series,
                depot,
                dcc_interface,
                control,
                railcar_type: Some(railcar_type),
                is_dummy,
                minimum_radius,
                coupling_socket: Some(socket),
                close_couplers: Some(close_couplers),
                digital_shunting_coupling: Some(digital_shunting),
                flywheel_fitted: Some(flywheel_fitted),
                metal_body: Some(metal_body),
                interior_lights: Some(interior_lights),
                lights: Some(lights),
                spring_buffers: Some(spring_buffers),
                ..RollingStockPayload::default()
            }),
            RollingStockRequest::LocomotiveRequest {
                railway: _,
                epoch,
                livery,
                length_over_buffers: _,
                technical_specifications: _,
                class_name,
                road_number,
                series,
                depot,
                locomotive_type,
                dcc_interface,
                control,
                is_dummy,
            } => Ok(RollingStockPayload {
                category: Some(category),
                epoch: Some(epoch),
                livery,
                length_over_buffers_mm: millimeters,
                length_over_buffers_in: inches,
                type_name: Some(class_name),
                road_number: Some(road_number),
                series,
                depot,
                dcc_interface,
                control,
                locomotive_type: Some(locomotive_type),
                is_dummy,
                minimum_radius,
                coupling_socket: Some(socket),
                close_couplers: Some(close_couplers),
                digital_shunting_coupling: Some(digital_shunting),
                flywheel_fitted: Some(flywheel_fitted),
                metal_body: Some(metal_body),
                interior_lights: Some(interior_lights),
                lights: Some(lights),
                spring_buffers: Some(spring_buffers),
                ..RollingStockPayload::default()
            }),
            RollingStockRequest::PassengerCarRequest {
                railway: _,
                epoch,
                livery,
                length_over_buffers: _,
                technical_specifications: _,
                type_name,
                road_number,
                series,
                passenger_car_type,
                service_level,
            } => Ok(RollingStockPayload {
                category: Some(category),
                epoch: Some(epoch),
                livery,
                length_over_buffers_mm: millimeters,
                length_over_buffers_in: inches,
                type_name: Some(type_name),
                road_number,
                series,
                passenger_car_type,
                service_level,
                minimum_radius,
                coupling_socket: Some(socket),
                close_couplers: Some(close_couplers),
                digital_shunting_coupling: Some(digital_shunting),
                flywheel_fitted: Some(flywheel_fitted),
                metal_body: Some(metal_body),
                interior_lights: Some(interior_lights),
                lights: Some(lights),
                spring_buffers: Some(spring_buffers),
                ..RollingStockPayload::default()
            }),
            RollingStockRequest::FreightCarRequest {
                railway: _,
                epoch,
                livery,
                length_over_buffers: _,
                technical_specifications: _,
                type_name,
                road_number,
                freight_car_type,
            } => Ok(RollingStockPayload {
                category: Some(category),
                epoch: Some(epoch),
                livery,
                length_over_buffers_mm: millimeters,
                length_over_buffers_in: inches,
                type_name: Some(type_name),
                road_number,
                freight_car_type,
                minimum_radius,
                coupling_socket: Some(socket),
                close_couplers: Some(close_couplers),
                digital_shunting_coupling: Some(digital_shunting),
                flywheel_fitted: Some(flywheel_fitted),
                metal_body: Some(metal_body),
                interior_lights: Some(interior_lights),
                lights: Some(lights),
                spring_buffers: Some(spring_buffers),
                ..RollingStockPayload::default()
            }),
        }
    }
}

#[derive(Debug, Error)]
pub enum CatalogItemCreationError {
    #[error("Error while interacting with the database: {0}")]
    Database(#[from] sqlx::error::Error),

    #[error("the catalog item request is not valid")]
    InvalidRequest,

    #[error("This catalog item already exists (id: {0})")]
    CatalogItemAlreadyExists(CatalogItemId),

    #[error("{0}")]
    DatabaseError(#[from] DatabaseError),

    #[error("Brand not found (name: {0})")]
    BrandNotFound(String),

    #[error("Scale not found (name: {0})")]
    ScaleNotFound(String),
}

/// The persistence related functionality for the new catalog item creation
#[async_trait]
pub trait NewCatalogItemRepository<'db, U: UnitOfWork<'db>> {
    /// Checks if a catalog with the input id already exists
    async fn exists_already(&self, catalog_item_id: &CatalogItemId, unit_of_work: &mut U) -> Result<bool>;

    /// Inserts a new catalog item
    async fn insert(&self, new_item: &NewCatalogItemCommand, unit_of_work: &mut U) -> Result<()>;

    /// Checks if the brand exists
    async fn brand_exists(&self, brand_id: &BrandId, unit_of_work: &mut U) -> Result<bool>;

    /// Checks if the scale exists
    async fn scale_exists(&self, scale_id: &ScaleId, unit_of_work: &mut U) -> Result<bool>;
}

/// The persistence related functionality for the new rolling stock creation
#[async_trait]
pub trait NewRollingStockRepository<'db, U: UnitOfWork<'db>> {
    /// Inserts a new catalog item
    async fn insert(&self, new_item: &NewRollingStockCommand, unit_of_work: &mut U) -> Result<()>;

    /// Checks if the railway exists
    async fn railway_exists(&self, railway_id: &RailwayId, unit_of_work: &mut U) -> Result<bool>;
}

#[cfg(test)]
mod test {
    use super::*;
    use common::in_memory::InMemoryRepository;
    use common::localized_text::LocalizedText;
    use common::unit_of_work::noop::NoOpUnitOfWork;
    use std::str::FromStr;

    mod new_catalog_item_command {
        use crate::brands::brand_id::BrandId;
        use crate::catalog_items::catalog_item_id::CatalogItemId;
        use crate::catalog_items::commands::new_catalog_item::test::{
            catalog_item, new_catalog_item, InMemoryNewCatalogItemRepository, InMemoryNewRollingStockRepository,
        };
        use crate::catalog_items::commands::new_catalog_item::{create_new_catalog_item, CatalogItemCreationError};
        use crate::catalog_items::item_number::ItemNumber;
        use crate::scales::scale_id::ScaleId;
        use common::unit_of_work::noop::NoOpDatabase;

        #[tokio::test]
        async fn it_should_return_an_error_when_the_brand_is_not_found() {
            let repo = InMemoryNewCatalogItemRepository::new();
            let rr_repo = InMemoryNewRollingStockRepository::new();
            let db = NoOpDatabase;

            let request = new_catalog_item();
            let result = create_new_catalog_item(request, repo, rr_repo, db).await;

            assert!(result.is_err());

            match result {
                Err(CatalogItemCreationError::BrandNotFound(brand_name)) => assert_eq!("ACME", brand_name),
                _ => panic!("CatalogItemCreationError::BrandNotFound is expected (found: {result:?})"),
            }
        }

        #[tokio::test]
        async fn it_should_return_an_error_when_the_scale_is_not_found() {
            let repo = InMemoryNewCatalogItemRepository::new().with_brand(BrandId::new("ACME"));
            let rr_repo = InMemoryNewRollingStockRepository::new();
            let db = NoOpDatabase;

            let request = new_catalog_item();
            let result = create_new_catalog_item(request, repo, rr_repo, db).await;

            assert!(result.is_err());

            match result {
                Err(CatalogItemCreationError::ScaleNotFound(scale_name)) => assert_eq!("H0", scale_name),
                _ => panic!("CatalogItemCreationError::ScaleNotFound is expected (found: {result:?})"),
            }
        }

        #[tokio::test]
        async fn it_should_return_an_error_when_the_catalog_item_already_exists() {
            let repo =
                InMemoryNewCatalogItemRepository::of(catalog_item(BrandId::new("ACME"), ItemNumber::new("123456")));
            let rr_repo = InMemoryNewRollingStockRepository::new();
            let db = NoOpDatabase;

            let request = new_catalog_item();
            let result = create_new_catalog_item(request, repo, rr_repo, db).await;

            assert!(result.is_err());

            match result {
                Err(CatalogItemCreationError::CatalogItemAlreadyExists(catalog_item_id)) => {
                    assert_eq!("acme-123456", catalog_item_id.to_string())
                }
                _ => panic!("CatalogItemCreationError::CatalogItemAlreadyExists is expected (found: {result:?})"),
            }
        }

        #[tokio::test]
        async fn it_should_create_a_new_catalog_item() {
            let repo = InMemoryNewCatalogItemRepository::new()
                .with_brand(BrandId::new("ACME"))
                .with_scale(ScaleId::new("H0"));
            let rr_repo = InMemoryNewRollingStockRepository::new();
            let db = NoOpDatabase;

            let request = new_catalog_item();
            let result = create_new_catalog_item(request, repo, rr_repo, db).await;

            assert!(result.is_ok());
            assert_eq!(
                CatalogItemId::of(&BrandId::new("ACME"), &ItemNumber::new("123456")),
                result.unwrap().catalog_item_id
            );
        }
    }

    fn new_catalog_item() -> CatalogItemRequest {
        CatalogItemRequest {
            brand: "ACME".to_string(),
            item_number: ItemNumber::new("123456"),
            scale: "H0".to_string(),
            category: Category::Locomotives,
            power_method: PowerMethod::DC,
            description: LocalizedText::with_italian("Descrizione"),
            details: LocalizedText::with_italian("Dettagli"),
            delivery_date: Some(DeliveryDate::ByYear(2022)),
            availability_status: Some(AvailabilityStatus::Available),
            rolling_stocks: Vec::new(),
            count: 1,
        }
    }

    fn catalog_item(brand_id: BrandId, item_number: ItemNumber) -> NewCatalogItemCommand {
        NewCatalogItemCommand {
            catalog_item_id: CatalogItemId::of(&brand_id, &item_number),
            payload: CatalogItemCommandPayload {
                brand_id,
                item_number,
                scale_id: ScaleId::new("H0"),
                category: Category::Locomotives,
                description: LocalizedText::with_italian("Descrizione"),
                details: LocalizedText::with_italian("Dettagli"),
                power_method: PowerMethod::DC,
                delivery_date: Some(DeliveryDate::ByYear(2022)),
                availability_status: Some(AvailabilityStatus::Available),
                count: 1,
            },
            rolling_stocks: Vec::new(),
            metadata: Metadata::created_at(Utc::now()),
        }
    }

    struct InMemoryNewCatalogItemRepository {
        catalog_items: InMemoryRepository<CatalogItemId, NewCatalogItemCommand>,
        brands: Vec<BrandId>,
        scales: Vec<ScaleId>,
    }

    impl InMemoryNewCatalogItemRepository {
        pub fn new() -> Self {
            InMemoryNewCatalogItemRepository {
                catalog_items: InMemoryRepository::empty(),
                brands: Vec::new(),
                scales: Vec::new(),
            }
        }

        pub fn of(command: NewCatalogItemCommand) -> Self {
            let id = CatalogItemId::from_str(&command.catalog_item_id.to_string()).unwrap();
            InMemoryNewCatalogItemRepository {
                catalog_items: InMemoryRepository::of(id, command),
                brands: Vec::new(),
                scales: Vec::new(),
            }
        }

        pub fn with_brand(mut self, brand_id: BrandId) -> Self {
            self.brands.push(brand_id);
            self
        }

        pub fn with_scale(mut self, scale_id: ScaleId) -> Self {
            self.scales.push(scale_id);
            self
        }
    }

    #[async_trait]
    impl NewCatalogItemRepository<'static, NoOpUnitOfWork> for InMemoryNewCatalogItemRepository {
        async fn exists_already(
            &self,
            catalog_item_id: &CatalogItemId,
            _unit_of_work: &mut NoOpUnitOfWork,
        ) -> Result<bool> {
            Ok(self.catalog_items.contains(catalog_item_id))
        }

        async fn insert(&self, new_item: &NewCatalogItemCommand, _unit_of_work: &mut NoOpUnitOfWork) -> Result<()> {
            let id = CatalogItemId::from_str(&new_item.catalog_item_id.to_string()).unwrap();
            self.catalog_items.add(id, new_item.clone());
            Ok(())
        }

        async fn brand_exists(&self, brand_id: &BrandId, _unit_of_work: &mut NoOpUnitOfWork) -> Result<bool> {
            let result = self.brands.contains(brand_id);
            Ok(result)
        }

        async fn scale_exists(&self, scale_id: &ScaleId, _unit_of_work: &mut NoOpUnitOfWork) -> Result<bool> {
            let result = self.scales.contains(scale_id);
            Ok(result)
        }
    }

    struct InMemoryNewRollingStockRepository {
        rolling_stocks: InMemoryRepository<RollingStockId, NewRollingStockCommand>,
        railways: Vec<RailwayId>,
    }

    impl InMemoryNewRollingStockRepository {
        fn new() -> Self {
            InMemoryNewRollingStockRepository {
                rolling_stocks: InMemoryRepository::empty(),
                railways: Vec::new(),
            }
        }
    }

    #[async_trait]
    impl NewRollingStockRepository<'static, NoOpUnitOfWork> for InMemoryNewRollingStockRepository {
        async fn insert(&self, new_item: &NewRollingStockCommand, _unit_of_work: &mut NoOpUnitOfWork) -> Result<()> {
            let rolling_stock_id = RollingStockId::from_str(new_item.catalog_item_id.value()).unwrap();
            self.rolling_stocks.add(rolling_stock_id, new_item.clone());
            Ok(())
        }

        async fn railway_exists(&self, railway_id: &RailwayId, _unit_of_work: &mut NoOpUnitOfWork) -> Result<bool> {
            let result = self.railways.contains(railway_id);
            Ok(result)
        }
    }
}
