use dockertest::waitfor::{MessageSource, MessageWait};
use dockertest::{DockerTest, Image, Source, TestBodySpecification};
use server::app;
use server::configuration::{DatabaseSettings, ServerSettings, Settings};
use sqlx::PgPool;
use std::net::TcpListener;

const POSTGRES_USER: &str = "postgres";
const POSTGRES_PASSWORD: &str = "postgres";
const POSTGRES_DB: &str = "postgres";

pub const IMAGE_NAME: &str = "postgres";

#[derive(Debug)]
pub struct ServiceUnderTest {
    base_endpoint_url: String,
    database_setting: DatabaseSettings,
}

impl ServiceUnderTest {
    pub fn endpoint(&self, path: &str) -> String {
        format!("{}{}", self.base_endpoint_url, path)
    }

    pub async fn run_database_migrations(&self) {
        // Migrate database
        let pg_pool = self.pg_pool();
        sqlx::migrate!("../../migrations")
            .run(&pg_pool)
            .await
            .expect("Failed to migrate the database");
    }

    pub fn pg_pool(&self) -> PgPool {
        self.database_setting.get_connection_pool()
    }
}

pub async fn spawn_app(port: u32) -> ServiceUnderTest {
    let settings = Settings {
        server: ServerSettings {
            host: String::from("127.0.0.1"),
            port: 0,
            workers: 2,
        },
        database: DatabaseSettings::new(POSTGRES_USER, POSTGRES_PASSWORD, "127.0.0.1", port as u16, POSTGRES_DB),
    };

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = app::run(listener, &settings).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    ServiceUnderTest {
        base_endpoint_url: format!("http://127.0.0.1:{}", port),
        database_setting: settings.database,
    }
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
