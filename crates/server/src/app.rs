use crate::catalog::catalog_router;
use crate::health_check;
use axum;
use axum::routing::get;
use axum::Router;
use common::unit_of_work::postgres::PgDatabase;
use configuration::Settings;
use sqlx::PgPool;
use std::net::TcpListener;
use std::sync::Arc;

/// Run the web server
pub async fn run(tcp_listener: TcpListener, settings: &Settings) {
    axum::Server::from_tcp(tcp_listener)
        .unwrap()
        .serve(build_app(settings).into_make_service())
        .await
        .unwrap();
}

pub fn build_app(settings: &Settings) -> Router {
    let app_state = AppState::from_settings(settings);
    let management_router = Router::new().route("/health-check", get(health_check::handler));

    catalog_router().merge(management_router).with_state(app_state.clone())
}

#[derive(Clone)]
pub struct AppState {
    pub pg_pool: Arc<PgPool>,
}

impl AppState {
    pub fn from_settings(settings: &Settings) -> Self {
        let pg_pool = Arc::new(settings.database.get_connection_pool());
        AppState { pg_pool }
    }

    pub fn get_database(&self) -> PgDatabase {
        PgDatabase::new(&self.pg_pool)
    }
}
