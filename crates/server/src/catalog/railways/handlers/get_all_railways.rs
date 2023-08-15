use crate::app::AppState;
use crate::catalog::railways::routes;
use crate::web::queries::to_response_error;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::Json;
use catalog::railways::queries::find_all_railways::find_all_railways;
use common::queries::pagination::PageRequest;
use data::catalog::railways::repositories::RailwaysRepository;
use hateoas::representations::CollectionModel;
use uuid::Uuid;

pub async fn handle(Query(_page_request): Query<PageRequest>, State(app_state): State<AppState>) -> impl IntoResponse {
    let database = app_state.get_database();
    let repo = RailwaysRepository;

    let results = find_all_railways(repo, database).await;
    match results {
        Ok(railways) => {
            let model = CollectionModel {
                items: railways,
                links: Vec::new(),
            };
            Json(model).into_response()
        }
        Err(why) => to_response_error(Uuid::new_v4(), why, routes::RAILWAY_ROOT_API).into_response(),
    }
}
