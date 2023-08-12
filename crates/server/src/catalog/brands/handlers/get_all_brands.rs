use crate::app::AppState;
use crate::catalog::brands::routes;
use crate::web::queries::to_response_error;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::Json;
use catalog::brands::queries::find_all_brands::find_all_brands;
use common::queries::pagination::PageRequest;
use db::catalog::brands::repositories::BrandsRepository;
use hateoas::representations::CollectionModel;
use uuid::Uuid;

pub async fn handle(Query(_page_request): Query<PageRequest>, State(app_state): State<AppState>) -> impl IntoResponse {
    let database = app_state.get_database();
    let repo = BrandsRepository;

    let results = find_all_brands(repo, database).await;
    match results {
        Ok(items) => {
            let model = CollectionModel {
                items,
                links: Vec::new(),
            };
            Json(model).into_response()
        }
        Err(why) => to_response_error(Uuid::new_v4(), why, routes::BRAND_ROOT_API).into_response(),
    }
}
