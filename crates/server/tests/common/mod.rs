use crate::common::database::Database;
use dockertest::{DockerTest, Source};
use once_cell::sync::Lazy;
use server::app;
use server::configuration::{ServerSettings, Settings};
use server::telemetry::{get_subscriber, init_subscriber};
use sqlx::PgPool;
use std::net::TcpListener;

pub const IMAGE_NAME: &str = "postgres";

pub mod database;
pub mod seeding;

#[derive(Debug)]
pub struct ServiceUnderTest {
    pub base_endpoint_url: String,
    pub database: Database,
}

impl ServiceUnderTest {
    pub fn endpoint(&self, path: &str) -> String {
        format!("{}{}", self.base_endpoint_url, path)
    }

    pub async fn run_database_migrations(&self) {
        self.database.run_database_migrations().await
    }

    pub fn pg_pool(&self) -> PgPool {
        self.database.pg_pool()
    }
}

// Ensure that the `tracing` stack is only initialised once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});

pub async fn spawn_app(postgres_port: u32) -> ServiceUnderTest {
    Lazy::force(&TRACING);

    let database = Database::new(postgres_port as u16);
    let database_settings = database.test_settings();
    let settings = Settings {
        server: ServerSettings {
            host: String::from("127.0.0.1"),
            port: 0,
            workers: 2,
        },
        database: database_settings,
    };

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = app::run(listener, &settings).expect("Failed to bind address");
    let _handle = tokio::spawn(server);

    ServiceUnderTest {
        base_endpoint_url: format!("http://127.0.0.1:{port}"),
        database,
    }
}

pub fn create_docker_test() -> DockerTest {
    let mut test = DockerTest::new().with_default_source(Source::DockerHub);
    test.provide_container(database::create_postgres_container());
    test
}
