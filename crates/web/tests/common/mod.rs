use dockertest::waitfor::{MessageSource, MessageWait};
use dockertest::{DockerTest, Image, Source, TestBodySpecification};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
use sqlx::PgPool;
use std::net::TcpListener;
use web::server;

const POSTGRES_USER: &str = "postgres";
const POSTGRES_PASSWORD: &str = "postgres";
const POSTGRES_DB: &str = "postgres";

pub const IMAGE_NAME: &str = "postgres";

#[derive(Debug)]
pub struct ServiceUnderTest {
    base_endpoint_url: String,
    pg_connect_options: PgConnectOptions,
}

impl ServiceUnderTest {
    pub fn endpoint(&self, path: &str) -> String {
        format!("{}{}", self.base_endpoint_url, path)
    }

    pub async fn run_database_migrations(&self) {
        // Migrate database
        let connection_pool = get_connection_pool(self.pg_connect_options.clone());
        sqlx::migrate!("../../migrations")
            .run(&connection_pool)
            .await
            .expect("Failed to migrate the database");
    }
}

pub async fn spawn_app(port: u32) -> ServiceUnderTest {
    let pg_connect_options = PgConnectOptions::new()
        .application_name("trenako-test")
        .host("127.0.0.1")
        .database(POSTGRES_DB)
        .username(POSTGRES_USER)
        .password(POSTGRES_PASSWORD)
        .port(port as u16)
        .ssl_mode(PgSslMode::Prefer);

    let pg_pool = get_connection_pool(pg_connect_options.clone());

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = server::run(listener, pg_pool, 2).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    ServiceUnderTest {
        base_endpoint_url: format!("http://127.0.0.1:{}", port),
        pg_connect_options,
    }
}

fn get_connection_pool(options: PgConnectOptions) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(options)
}

pub fn create_docker_test() -> DockerTest {
    let mut test = DockerTest::new().with_default_source(Source::DockerHub);
    test.provide_container(create_composition(IMAGE_NAME));
    test
}

fn create_composition(repo: &str) -> TestBodySpecification {
    let image = Image::with_repository(repo).tag("15.1-alpine");
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
