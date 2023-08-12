use crate::app::AppState;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use catalog::catalog_items::catalog_item_id::CatalogItemId;
use catalog::catalog_items::catalog_item_request::CatalogItemRequest;

pub async fn handle(
    Path(_catalog_item_id): Path<CatalogItemId>,
    State(_app_state): State<AppState>,
    Json(_request): Json<CatalogItemRequest>,
) -> impl IntoResponse {
    ().into_response()
}
