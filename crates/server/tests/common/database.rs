use crate::common::IMAGE_NAME;
use dockertest::waitfor::{MessageSource, MessageWait};
use dockertest::{Image, TestBodySpecification};
use server::configuration::DatabaseSettings;
use sqlx::PgPool;

const POSTGRES_USER: &str = "postgres";
const POSTGRES_PASSWORD: &str = "postgres";
const POSTGRES_DB: &str = "postgres";

pub fn create_postgres_container() -> TestBodySpecification {
    let image = Image::with_repository(IMAGE_NAME).tag("15.1-alpine");
    let message = r#"listening on IPv4 address "0.0.0.0", port 5432"#;
    let mut composition = TestBodySpecification::with_image(image).set_wait_for(Box::new(MessageWait {
        message: String::from(message),
        source: MessageSource::Stderr,
        timeout: 5,
    }));

    composition.modify_port_map(5432, 0);
    composition.modify_env("POSTGRES_DB", POSTGRES_DB);
    composition.modify_env("POSTGRES_USER", POSTGRES_USER);
    composition.modify_env("POSTGRES_PASSWORD", POSTGRES_PASSWORD);
    composition
}

#[derive(Debug)]
pub struct Database(DatabaseSettings);

impl Database {
    pub fn new(port: u16) -> Self {
        let database_settings = DatabaseSettings::new(POSTGRES_USER, POSTGRES_PASSWORD, "127.0.0.1", port, POSTGRES_DB);
        Database(database_settings)
    }

    pub fn test_settings(&self) -> DatabaseSettings {
        self.0.clone()
    }

    pub fn pg_pool(&self) -> PgPool {
        self.0.get_connection_pool()
    }

    pub async fn run_database_migrations(&self) {
        sqlx::migrate!("../../migrations")
            .run(&self.pg_pool())
            .await
            .expect("Failed to migrate the database");
    }
}
