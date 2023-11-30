use crate::state::AppState;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use catalog::brands::brand_id::BrandId;
use catalog::brands::brand_request::BrandRequest;

pub async fn handle(
    Path(_brand_id): Path<BrandId>,
    State(_app_state): State<AppState>,
    Json(_request): Json<BrandRequest>,
) -> impl IntoResponse {
    ().into_response()
}
