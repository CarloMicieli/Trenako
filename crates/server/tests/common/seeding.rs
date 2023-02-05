use catalog::brands::brand_request::BrandRequest;
use catalog::brands::commands::new_brand::NewBrandCommand;
use catalog::brands::commands::repositories::BrandRepository;
use catalog::catalog_items::catalog_item_request::CatalogItemRequest;
use catalog::catalog_items::commands::new_catalog_item::NewCatalogItemCommand;
use catalog::catalog_items::commands::repositories::CatalogItemRepository;
use catalog::railways::commands::new_railways::NewRailwayCommand;
use catalog::railways::commands::repositories::RailwayRepository;
use catalog::railways::railway_request::RailwayRequest;
use catalog::scales::commands::new_scales::NewScaleCommand;
use catalog::scales::commands::repositories::ScaleRepository;
use catalog::scales::scale_request::ScaleRequest;
use common::unit_of_work::postgres::PgDatabase;
use common::unit_of_work::{Database, UnitOfWork};
use serde_derive::Deserialize;
use server::catalog::brands::repositories::PgBrandRepository;
use server::catalog::catalog_items::repositories::PgCatalogItemRepository;
use server::catalog::railways::repositories::PgRailwayRepository;
use server::catalog::scales::repositories::PgScaleRepository;
use sqlx::PgPool;

pub async fn seed_brands(pg_pool: &PgPool) {
    let db = PgDatabase::new(pg_pool);
    let mut unit_of_work = db.begin().await.unwrap();

    let repo = PgBrandRepository;

    let brands = brands();
    let brands: Vec<NewBrandCommand> = brands
        .items
        .into_iter()
        .map(|it| NewBrandCommand::try_from(it).expect("invalid brand request"))
        .collect();

    for b in brands {
        repo.insert(&b, &mut unit_of_work).await.unwrap();
    }

    unit_of_work.commit().await.unwrap();
}

pub async fn seed_catalog_items(pg_pool: &PgPool) {
    let db = PgDatabase::new(pg_pool);
    let mut unit_of_work = db.begin().await.unwrap();

    let repo = PgCatalogItemRepository;

    let scales: Vec<NewCatalogItemCommand> = catalog_items()
        .items
        .into_iter()
        .map(|it| NewCatalogItemCommand::try_from(it).expect("invalid catalog item request"))
        .collect();

    for s in scales {
        repo.insert(&s, &mut unit_of_work).await.unwrap();
    }

    unit_of_work.commit().await.unwrap();
}

pub async fn seed_railways(pg_pool: &PgPool) {
    let db = PgDatabase::new(pg_pool);
    let mut unit_of_work = db.begin().await.unwrap();

    let repo = PgRailwayRepository;

    let railways: Vec<NewRailwayCommand> = railways()
        .items
        .into_iter()
        .map(|it| NewRailwayCommand::try_from(it).expect("invalid railway request"))
        .collect();

    for s in railways {
        repo.insert(&s, &mut unit_of_work).await.unwrap();
    }

    unit_of_work.commit().await.unwrap();
}

pub async fn seed_scales(pg_pool: &PgPool) {
    let db = PgDatabase::new(pg_pool);
    let mut unit_of_work = db.begin().await.unwrap();

    let repo = PgScaleRepository;

    let scales: Vec<NewScaleCommand> = scales()
        .items
        .into_iter()
        .map(|it| NewScaleCommand::try_from(it).expect("invalid scale request"))
        .collect();

    for s in scales {
        repo.insert(&s, &mut unit_of_work).await.unwrap();
    }

    unit_of_work.commit().await.unwrap();
}

fn brands() -> Brands {
    serde_json::from_str::<Brands>(data::BRANDS).expect("Invalid BRANDS data for seeding")
}

fn catalog_items() -> CatalogItems {
    serde_json::from_str::<CatalogItems>(data::CATALOG_ITEMS).expect("Invalid CATALOG_ITEMS data for seeding")
}

fn railways() -> Railways {
    serde_json::from_str::<Railways>(data::RAILWAYS).expect("Invalid RAILWAYS data for seeding")
}

fn scales() -> Scales {
    serde_json::from_str::<Scales>(data::SCALES).expect("Invalid SCALES data for seeding")
}

#[cfg(not(windows))]
mod data {
    pub const BRANDS: &str = include_str!("../resources/brands.json");
    pub const CATALOG_ITEMS: &str = include_str!("../resources/catalog_items.json");
    pub const RAILWAYS: &str = include_str!("../resources/railways.json");
    pub const SCALES: &str = include_str!("../resources/scales.json");
}

#[cfg(windows)]
mod data {
    pub const BRANDS: &str = include_str!("..\\resources\\brands.json");
    pub const CATALOG_ITEMS: &str = include_str!("..\\resources\\catalog_items.json");
    pub const RAILWAYS: &str = include_str!("..\\resources\\railways.json");
    pub const SCALES: &str = include_str!("..\\resources\\scales.json");
}

#[derive(Deserialize)]
struct Brands {
    items: Vec<BrandRequest>,
}

#[derive(Deserialize)]
struct CatalogItems {
    items: Vec<CatalogItemRequest>,
}

#[derive(Deserialize)]
struct Railways {
    items: Vec<RailwayRequest>,
}

#[derive(Deserialize)]
struct Scales {
    items: Vec<ScaleRequest>,
}
