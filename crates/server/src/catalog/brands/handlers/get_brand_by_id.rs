use crate::app::AppState;
use crate::catalog::brands::routes;
use crate::web::queries::to_response_error;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use catalog::brands::brand_id::BrandId;
use catalog::brands::queries::find_brand_by_id::find_brand_by_id;
use db::catalog::brands::repositories::BrandsRepository;
use uuid::Uuid;

pub async fn handle(Path(brand_id): Path<BrandId>, State(app_state): State<AppState>) -> impl IntoResponse {
    let database = app_state.get_database();
    let repo = BrandsRepository;

    let result = find_brand_by_id(&brand_id, repo, database).await;
    match result {
        Ok(brand) => Json(brand).into_response(),
        Err(why) => to_response_error(Uuid::new_v4(), why, routes::BRAND_ROOT_API).into_response(),
    }
}
