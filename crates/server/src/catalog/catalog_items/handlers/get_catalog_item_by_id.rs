use crate::app::AppState;
use crate::catalog::catalog_items::routes;
use crate::hateoas::representations::EntityModel;
use crate::web::queries::{to_response_error, QueryResponseError};
use axum::extract::{Path, State};
use catalog::catalog_items::catalog_item::CatalogItem;
use catalog::catalog_items::catalog_item_id::CatalogItemId;
use catalog::catalog_items::queries::find_catalog_item_by_id::find_catalog_item_by_id;
use data::catalog::catalog_item::repositories::CatalogItemsRepository;
use uuid::Uuid;

pub async fn handle(
    Path(catalog_item_id): Path<CatalogItemId>,
    State(app_state): State<AppState>,
) -> Result<EntityModel<CatalogItem>, QueryResponseError> {
    let database = app_state.get_database();
    let repo = CatalogItemsRepository;

    let result = find_catalog_item_by_id(&catalog_item_id, repo, database).await;
    result
        .map(|catalog_item| EntityModel::of(catalog_item, Vec::new()))
        .map_err(|why| {
            let path = format!("{}/{}", routes::CATALOG_ITEMS_ROOT_API, catalog_item_id);
            to_response_error(Uuid::new_v4(), why, &path)
        })
}
