use crate::app::AppState;
use crate::catalog::catalog_items::routes;
use crate::web::queries::to_response_error;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use catalog::catalog_items::catalog_item_id::CatalogItemId;
use catalog::catalog_items::queries::find_catalog_item_by_id::find_catalog_item_by_id;
use data::catalog::catalog_item::repositories::CatalogItemsRepository;
use uuid::Uuid;

pub async fn handle(
    Path(catalog_item_id): Path<CatalogItemId>,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    let database = app_state.get_database();
    let repo = CatalogItemsRepository;

    let result = find_catalog_item_by_id(&catalog_item_id, repo, database).await;
    match result {
        Ok(catalog_item) => Json(catalog_item).into_response(),
        Err(why) => {
            let path = format!("{}/{}", routes::CATALOG_ITEMS_ROOT_API, catalog_item_id);
            to_response_error(Uuid::new_v4(), why, &path).into_response()
        }
    }
}
