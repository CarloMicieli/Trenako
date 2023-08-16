use crate::app::AppState;
use crate::catalog::railways::routes;
use crate::hateoas::representations::CollectionModel;
use crate::web::queries::{to_response_error, QueryResponseError};
use axum::extract::{Query, State};
use catalog::railways::queries::find_all_railways::find_all_railways;
use catalog::railways::railway::Railway;
use common::queries::pagination::PageRequest;
use data::catalog::railways::repositories::RailwaysRepository;
use uuid::Uuid;

pub async fn handle(
    Query(_page_request): Query<PageRequest>,
    State(app_state): State<AppState>,
) -> Result<CollectionModel<Railway>, QueryResponseError> {
    let database = app_state.get_database();
    let repo = RailwaysRepository;

    let results = find_all_railways(repo, database).await;
    results
        .map(|railways| CollectionModel::of(railways, Vec::new()))
        .map_err(|why| to_response_error(Uuid::new_v4(), why, routes::RAILWAY_ROOT_API))
}
