use crate::state::AppState;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use catalog::brands::brand_request::BrandRequest;
use catalog::railways::railway_id::RailwayId;

pub async fn handle(
    Path(_railway_id): Path<RailwayId>,
    State(_app_state): State<AppState>,
    Json(_request): Json<BrandRequest>,
) -> impl IntoResponse {
    ().into_response()
}
