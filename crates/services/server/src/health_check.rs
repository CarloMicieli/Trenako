//! the health check web handler

use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn handler(State(app_state): State<AppState>) -> impl IntoResponse {
    let db_pool = app_state.pg_pool;
    let is_database_connected = sqlx::query("SELECT 1").fetch_one(&*db_pool).await.is_ok();

    if is_database_connected {
        (StatusCode::OK, ())
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, ())
    }
}
