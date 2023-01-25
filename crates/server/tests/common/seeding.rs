use catalog::brands::brand_request::BrandRequest;
use catalog::brands::commands::new_brand::{NewBrandCommand, NewBrandRepository};
use catalog::scales::commands::new_scales::{NewScaleCommand, NewScaleRepository};
use catalog::scales::scale_request::ScaleRequest;
use common::unit_of_work::postgres::PgDatabase;
use common::unit_of_work::{Database, UnitOfWork};
use serde_derive::Deserialize;
use server::catalog::brands::post_brands::PgNewBrandRepository;
use server::catalog::scales::post_scales::PgNewScaleRepository;
use sqlx::PgPool;

pub async fn seed_brands(pg_pool: &PgPool) {
    let db = PgDatabase::new(pg_pool);
    let mut unit_of_work = db.begin().await.unwrap();

    let repo = PgNewBrandRepository;

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

pub async fn seed_scales(pg_pool: &PgPool) {
    let db = PgDatabase::new(pg_pool);
    let mut unit_of_work = db.begin().await.unwrap();

    let repo = PgNewScaleRepository;

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

fn scales() -> Scales {
    serde_json::from_str::<Scales>(data::SCALES).expect("Invalid SCALES data for seeding")
}

#[cfg(not(windows))]
mod data {
    pub const BRANDS: &str = include_str!("../resources/brands.json");
    pub const SCALES: &str = include_str!("../resources/scales.json");
}

#[cfg(windows)]
mod data {
    pub const BRANDS: &str = include_str!("..\\resources\\brands.json");
    pub const SCALES: &str = include_str!("..\\resources\\scales.json");
}

#[derive(Deserialize)]
struct Brands {
    items: Vec<BrandRequest>,
}

#[derive(Deserialize)]
struct Scales {
    items: Vec<ScaleRequest>,
}
