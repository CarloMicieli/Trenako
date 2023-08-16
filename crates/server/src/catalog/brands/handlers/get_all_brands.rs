use crate::app::AppState;
use crate::catalog::brands::routes;
use crate::hateoas::representations::CollectionModel;
use crate::web::queries::{to_response_error, QueryResponseError};
use axum::extract::{Query, State};
use catalog::brands::brand::Brand;
use catalog::brands::queries::find_all_brands::find_all_brands;
use common::queries::pagination::PageRequest;
use data::catalog::brands::repositories::BrandsRepository;
use uuid::Uuid;

pub async fn handle(
    Query(_page_request): Query<PageRequest>,
    State(app_state): State<AppState>,
) -> Result<CollectionModel<Brand>, QueryResponseError> {
    let database = app_state.get_database();
    let repo = BrandsRepository;

    let results = find_all_brands(repo, database).await;
    results
        .map(|brands| CollectionModel::of(brands, Vec::new()))
        .map_err(|why| to_response_error(Uuid::new_v4(), why, routes::BRANDS_ROOT_API))
}
