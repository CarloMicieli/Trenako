mod common;

use crate::common::spawn_app;
use pretty_assertions::assert_eq;

#[tokio::test]
async fn health_check_works() {
    let sut = spawn_app().await;

    let client = reqwest::Client::new();

    let endpoint = sut.endpoint("/health-check");
    let response = client.get(endpoint).send().await.expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
