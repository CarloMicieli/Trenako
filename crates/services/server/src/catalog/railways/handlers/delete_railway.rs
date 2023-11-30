use crate::state::AppState;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use catalog::scales::scale_id::ScaleId;

pub async fn handle(Path(_scale_id): Path<ScaleId>, State(_app_state): State<AppState>) -> impl IntoResponse {
    ().into_response()
}
