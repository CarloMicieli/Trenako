use crate::common::seeding::{seed_brands, seed_scales};
use crate::common::{create_docker_test, spawn_app, IMAGE_NAME};
use catalog::brands::brand_id::BrandId;
use catalog::catalog_items::availability_status::AvailabilityStatus;
use catalog::catalog_items::catalog_item_id::CatalogItemId;
use catalog::catalog_items::category::Category;
use catalog::catalog_items::delivery_date::DeliveryDate;
use catalog::catalog_items::power_method::PowerMethod;
use catalog::scales::scale_id::ScaleId;
use serde_json::json;
use std::str::FromStr;

pub mod common;

const API_CATALOG_ITEMS: &str = "/api/catalog-items";

#[tokio::test]
async fn post_new_catalog_items() {
    let test = create_docker_test();

    test.run_async(|ops| async move {
        let (_, port) = ops.handle(IMAGE_NAME).host_port(5432).unwrap();

        let sut = spawn_app(*port).await;
        sut.run_database_migrations().await;

        let pg_pool = sut.pg_pool();
        seed_brands(&pg_pool).await;
        seed_scales(&pg_pool).await;

        let catalog_item_id = CatalogItemId::from_str("acme-123456").unwrap();
        let expected_location = format!("{API_CATALOG_ITEMS}/{catalog_item_id}");

        let request = json!({
            "brand" : "ACME",
            "item_number" : "123456",
            "category" : "LOCOMOTIVES",
            "scale" : "H0",
            "power_method" : "DC",
            "description" : {
                "it" : "Locomotiva elettrica E 402A 015 nella livrea di origine rosso/bianco, pantografi 52 Sommerfeldt"
            },
            "details" : {
                "it" : "Motore a 5 poli"
            },
            "delivery_date" : "2005",
            "availability_status" : "AVAILABLE",
            "count" : 1,
            "rolling_stocks": []
        });

        let client = reqwest::Client::new();
        let endpoint = sut.endpoint(API_CATALOG_ITEMS);
        let response = client
            .post(endpoint)
            .json(&request)
            .send()
            .await
            .expect("Failed to execute request.");

        assert!(response.status().is_success());
        assert_eq!(Some(0), response.content_length());
        assert_eq!(expected_location, response.headers()["Location"]);

        let saved = sqlx::query_as!(
            Saved,
            r#"SELECT
                    item_number,
                    brand_id as "brand_id: BrandId",  scale_id as "scale_id: ScaleId", 
                    category as "category: Category",
                    power_method as "power_method: PowerMethod",
                    description_it, details_it,
                    delivery_date, 
                    availability_status as "availability_status: AvailabilityStatus", 
                    count
                FROM catalog_items WHERE catalog_item_id = $1"#,
            catalog_item_id as CatalogItemId
        )
        .fetch_one(&pg_pool)
        .await
        .expect("Failed to fetch saved catalog item.");

        assert_eq!(BrandId::new("ACME"), saved.brand_id);
        assert_eq!("123456", saved.item_number);
        assert_eq!(Category::Locomotives, saved.category);
        assert_eq!(ScaleId::new("H0"), saved.scale_id);
        assert_eq!(PowerMethod::DC, saved.power_method);
        assert_eq!(
            Some(String::from(
                "Locomotiva elettrica E 402A 015 nella livrea di origine rosso/bianco, pantografi 52 Sommerfeldt"
            )),
            saved.description_it
        );
        assert_eq!(Some(String::from("Motore a 5 poli")), saved.details_it);
        assert_eq!(Some(DeliveryDate::by_year(2005).to_string()), saved.delivery_date);
        assert_eq!(Some(AvailabilityStatus::Available), saved.availability_status);
        assert_eq!(1, saved.count);
    })
    .await
}

#[derive(Debug)]
struct Saved {
    brand_id: BrandId,
    item_number: String,
    category: Category,
    scale_id: ScaleId,
    power_method: PowerMethod,
    description_it: Option<String>,
    details_it: Option<String>,
    delivery_date: Option<String>,
    availability_status: Option<AvailabilityStatus>,
    count: i32,
}
