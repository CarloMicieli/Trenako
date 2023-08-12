use crate::app::AppState;
use crate::catalog::scales::routes;
use crate::web::queries::to_response_error;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use catalog::scales::queries::find_scale_by_id::find_scale_by_id;
use catalog::scales::scale_id::ScaleId;
use db::catalog::scales::repositories::ScalesRepository;
use uuid::Uuid;

pub async fn handle(Path(scale_id): Path<ScaleId>, State(app_state): State<AppState>) -> impl IntoResponse {
    let database = app_state.get_database();
    let repo = ScalesRepository;

    let result = find_scale_by_id(&scale_id, repo, database).await;
    match result {
        Ok(scale) => Json(scale).into_response(),
        Err(why) => {
            let path = format!("{}/{}", routes::SCALE_ROOT_API, scale_id);
            to_response_error(Uuid::new_v4(), why, &path).into_response()
        }
    }
}
