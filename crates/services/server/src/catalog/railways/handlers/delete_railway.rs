use crate::state::AppState;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use catalog::scales::scale_id::ScaleId;

#[tracing::instrument(name = "delete_railway", skip(_app_state))]
pub async fn handle(Path(_scale_id): Path<ScaleId>, State(_app_state): State<AppState>) -> impl IntoResponse {
    ().into_response()
}
