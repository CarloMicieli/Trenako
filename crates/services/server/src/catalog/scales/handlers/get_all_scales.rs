use crate::app::AppState;
use crate::catalog::scales::routes;
use crate::hateoas::representations::CollectionModel;
use crate::web::problem::ProblemDetail;
use crate::web::responders::ToProblemDetail;
use axum::extract::{Query, State};
use catalog::scales::queries::find_all_scales::find_all_scales;
use catalog::scales::scale::Scale;
use common::queries::pagination::PageRequest;
use data::catalog::scales::repositories::ScalesRepository;
use uuid::Uuid;

pub async fn handle(
    Query(_page_request): Query<PageRequest>,
    State(app_state): State<AppState>,
) -> Result<CollectionModel<Scale>, ProblemDetail> {
    let database = app_state.get_database();
    let repo = ScalesRepository;

    let results = find_all_scales(repo, database).await;
    results
        .map(|scales| CollectionModel::of(scales, Vec::new()))
        .map_err(|why| why.to_problem_detail(Uuid::new_v4(), Some(routes::SCALE_ROOT_API)))
}
