use sqlx::PgPool;
use std::net::TcpListener;
use testcontainers::{clients, images::postgres, RunnableImage};
use web::server;

#[derive(Debug)]
pub struct ServiceUnderTest {
    base_endpoint_url: String,
}

impl ServiceUnderTest {
    pub fn endpoint(&self, path: &str) -> String {
        format!("{}{}", self.base_endpoint_url, path)
    }
}

pub async fn spawn_app() -> ServiceUnderTest {
    let docker = clients::Cli::default();
    let image = RunnableImage::from(postgres::Postgres::default()).with_tag("14.5-alpine");
    let node = docker.run(image);

    let connection_string = &format!(
        "postgres://postgres:postgres@127.0.0.1:{}/postgres",
        node.get_host_port_ipv4(5432)
    );

    let pg_pool = configure_database(connection_string).await;

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = server::run(listener, pg_pool, 2).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    ServiceUnderTest {
        base_endpoint_url: format!("http://127.0.0.1:{}", port),
    }
}

async fn configure_database(config: &str) -> PgPool {
    // Migrate database
    let connection_pool = PgPool::connect(config).await.expect("Failed to connect to Postgres.");
    sqlx::migrate!("../../migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}
