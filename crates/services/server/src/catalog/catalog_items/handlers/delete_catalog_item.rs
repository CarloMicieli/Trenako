use crate::state::AppState;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use catalog::catalog_items::catalog_item_id::CatalogItemId;

pub async fn handle(
    Path(_catalog_item_id): Path<CatalogItemId>,
    State(_app_state): State<AppState>,
) -> impl IntoResponse {
    ().into_response()
}
