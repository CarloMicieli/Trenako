use crate::catalog::brands::routes;
use crate::hateoas::representations::EntityModel;
use crate::state::AppState;
use crate::web::problem::ProblemDetail;
use crate::web::responders::ToProblemDetail;
use axum::extract::{Path, State};
use catalog::brands::brand::Brand;
use catalog::brands::brand_id::BrandId;
use catalog::brands::queries::find_brand_by_id::find_brand_by_id;
use data::catalog::brands::repositories::BrandsRepository;
use uuid::Uuid;

#[tracing::instrument(name = "get_brand_by_id", skip(app_state))]
pub async fn handle(
    Path(brand_id): Path<BrandId>,
    State(app_state): State<AppState>,
) -> Result<EntityModel<Brand>, ProblemDetail> {
    let database = app_state.get_database();
    let repo = BrandsRepository;

    let result = find_brand_by_id(&brand_id, repo, database).await;
    result
        .map(|brand| EntityModel::of(brand, vec![]))
        .map_err(|why| why.to_problem_detail(Uuid::new_v4(), Some(routes::BRANDS_ROOT_API)))
}
