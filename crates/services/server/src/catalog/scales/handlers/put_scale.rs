use crate::state::AppState;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use catalog::scales::scale_id::ScaleId;
use catalog::scales::scale_request::ScaleRequest;

pub async fn handle(
    Path(_scale_id): Path<ScaleId>,
    State(_app_state): State<AppState>,
    Json(_request): Json<ScaleRequest>,
) -> impl IntoResponse {
    ().into_response()
}
