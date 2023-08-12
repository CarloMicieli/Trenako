use crate::app::AppState;
use crate::catalog::scales::routes;
use crate::web::queries::to_response_error;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::Json;
use catalog::scales::queries::find_all_scales::find_all_scales;
use common::queries::pagination::PageRequest;
use db::catalog::scales::repositories::ScalesRepository;
use hateoas::representations::CollectionModel;
use uuid::Uuid;

pub async fn handle(Query(_page_request): Query<PageRequest>, State(app_state): State<AppState>) -> impl IntoResponse {
    let database = app_state.get_database();
    let repo = ScalesRepository;

    let results = find_all_scales(repo, database).await;
    match results {
        Ok(scales) => {
            let model = CollectionModel {
                items: scales,
                links: Vec::new(),
            };
            Json(model).into_response()
        }
        Err(why) => to_response_error(Uuid::new_v4(), why, routes::SCALE_ROOT_API).into_response(),
    }
}
