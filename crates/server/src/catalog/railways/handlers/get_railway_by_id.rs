use crate::app::AppState;
use crate::catalog::railways::routes;
use crate::hateoas::representations::EntityModel;
use crate::web::queries::{to_response_error, QueryResponseError};
use axum::extract::{Path, State};
use catalog::railways::queries::find_railway_by_id::find_railway_by_id;
use catalog::railways::railway::Railway;
use catalog::railways::railway_id::RailwayId;
use data::catalog::railways::repositories::RailwaysRepository;
use uuid::Uuid;

pub async fn handle(
    Path(railway_id): Path<RailwayId>,
    State(app_state): State<AppState>,
) -> Result<EntityModel<Railway>, QueryResponseError> {
    let database = app_state.get_database();
    let repo = RailwaysRepository;

    let result = find_railway_by_id(&railway_id, repo, database).await;
    result
        .map(|railway| EntityModel::of(railway, Vec::new()))
        .map_err(|why| {
            let path = format!("{}/{}", routes::RAILWAY_ROOT_API, railway_id);
            to_response_error(Uuid::new_v4(), why, &path)
        })
}
