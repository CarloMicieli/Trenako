use crate::state::AppState;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use catalog::brands::brand_id::BrandId;

pub async fn handle(Path(_brand_id): Path<BrandId>, State(_app_state): State<AppState>) -> impl IntoResponse {
    ().into_response()
}
