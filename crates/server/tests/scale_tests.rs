pub mod common;

use crate::common::{create_docker_test, spawn_app, IMAGE_NAME};
use ::common::localized_text::LocalizedText;
use catalog::common::TrackGauge;
use catalog::scales::ratio::Ratio;
use catalog::scales::scale_gauge::Gauge;
use catalog::scales::scale_request::ScaleRequest;
use catalog::scales::standard::Standard;
use rust_decimal::Decimal;
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
        let expected_location = format!("{API_SCALES}/{scale_name}");

        let ratio_value = Decimal::from_str_exact("87").unwrap();
        let ratio = Ratio::try_from(ratio_value).unwrap();

        let gauge_mm = Decimal::from_str_exact("16.5").unwrap();
        let gauge_in = Decimal::from_str_exact("0.65").unwrap();
        let gauge = Gauge::new(TrackGauge::Standard, gauge_mm, gauge_in).unwrap();

        let request = ScaleRequest {
            name: scale_name.clone(),
            ratio,
            gauge,
            description: LocalizedText::with_italian("Descrizione"),
            standards: vec![Standard::NEM, Standard::NMRA],
        };

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
                scale_id,
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

        assert_eq!(request.name, saved.scale_id);
        assert_eq!(request.name, saved.name);
        assert_eq!(request.description.italian(), saved.description_it.as_ref());
        assert_eq!(ratio_value, saved.ratio);
        assert_eq!(Some(gauge_mm), saved.gauge_millimeters);
        assert_eq!(Some(gauge_in), saved.gauge_inches);
        assert_eq!(TrackGauge::Standard, saved.track_gauge);
        assert_eq!(Some(String::from("NEM,NMRA")), saved.standards);
    })
    .await;
}

struct Saved {
    scale_id: String,
    name: String,
    ratio: Decimal,
    gauge_millimeters: Option<Decimal>,
    gauge_inches: Option<Decimal>,
    track_gauge: TrackGauge,
    description_it: Option<String>,
    standards: Option<String>,
}
