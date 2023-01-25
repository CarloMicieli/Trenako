use crate::common::seeding::{seed_brands, seed_scales};
use crate::common::{create_docker_test, spawn_app, IMAGE_NAME};
use serde_json::json;

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

        let expected_location = format!("{}/{}", API_CATALOG_ITEMS, "acme-123456");

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
    })
    .await
}
