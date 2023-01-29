pub mod common;

use crate::common::{create_docker_test, spawn_app, IMAGE_NAME};
use catalog::common::TrackGauge;
use catalog::scales::scale_id::ScaleId;
use rust_decimal::Decimal;
use serde_json::json;
use uuid::Uuid;

const API_SCALES: &str = "/api/scales";

#[tokio::test]
async fn post_new_scales() {
    let test = create_docker_test();

    test.run_async(|ops| async move {
        let (_, port) = ops.handle(IMAGE_NAME).host_port(5432).unwrap();

        let sut = spawn_app(*port).await;
        let client = reqwest::Client::new();
        sut.run_database_migrations().await;

        let scale_name = Uuid::new_v4().to_string();
        let scale_id = ScaleId::new(&scale_name);
        let expected_location = format!("{API_SCALES}/{scale_id}");

        let ratio_value = Decimal::from_str_exact("87").unwrap();

        let gauge_mm = Decimal::from_str_exact("16.5").unwrap();
        let gauge_in = Decimal::from_str_exact("0.65").unwrap();

        let request = json!({
            "name" : scale_name,
            "ratio" : 87.0,
            "gauge" : {
                "millimeters" : 16.5,
                "inches" : 0.65,
                "track_gauge" : "STANDARD"
            },
            "description" : {
                "it": "descrizione"
            },
            "standards" : ["NEM", "NMRA"]
        });

        let endpoint = sut.endpoint(API_SCALES);
        let response = client
            .post(endpoint)
            .json(&request)
            .send()
            .await
            .expect("Failed to execute request.");

        assert!(response.status().is_success());
        assert_eq!(Some(0), response.content_length());
        assert_eq!(expected_location, response.headers()["Location"]);

        let pg_pool = sut.pg_pool();
        let saved = sqlx::query_as!(
            Saved,
            r#"SELECT
                scale_id as "scale_id: ScaleId",
                name,
                ratio,
                gauge_millimeters,
                gauge_inches,
                track_gauge as "track_gauge: TrackGauge",
                description_it,
                standards
            FROM scales
            WHERE name = $1"#,
            &scale_name
        )
        .fetch_one(&pg_pool)
        .await
        .expect("Failed to fetch saved scale.");

        assert_eq!(scale_id, saved.scale_id);
        assert_eq!(scale_name, saved.name);
        assert_eq!(Some(String::from("descrizione")), saved.description_it);
        assert_eq!(ratio_value, saved.ratio);
        assert_eq!(Some(gauge_mm), saved.gauge_millimeters);
        assert_eq!(Some(gauge_in), saved.gauge_inches);
        assert_eq!(TrackGauge::Standard, saved.track_gauge);
        assert_eq!(Some(String::from("NEM,NMRA")), saved.standards);
    })
    .await;
}

struct Saved {
    scale_id: ScaleId,
    name: String,
    ratio: Decimal,
    gauge_millimeters: Option<Decimal>,
    gauge_inches: Option<Decimal>,
    track_gauge: TrackGauge,
    description_it: Option<String>,
    standards: Option<String>,
}
