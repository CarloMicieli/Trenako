use crate::common::seeding::{seed_brands, seed_scales};
use crate::common::{create_docker_test, spawn_app, IMAGE_NAME};

pub mod common;

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
    })
    .await
}
