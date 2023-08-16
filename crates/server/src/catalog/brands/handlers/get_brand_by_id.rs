use crate::app::AppState;
use crate::catalog::brands::routes;
use crate::hateoas::representations::EntityModel;
use crate::web::queries::{to_response_error, QueryResponseError};
use axum::extract::{Path, State};
use catalog::brands::brand::Brand;
use catalog::brands::brand_id::BrandId;
use catalog::brands::queries::find_brand_by_id::find_brand_by_id;
use data::catalog::brands::repositories::BrandsRepository;
use uuid::Uuid;

pub async fn handle(
    Path(brand_id): Path<BrandId>,
    State(app_state): State<AppState>,
) -> Result<EntityModel<Brand>, QueryResponseError> {
    let database = app_state.get_database();
    let repo = BrandsRepository;

    let result = find_brand_by_id(&brand_id, repo, database).await;
    result
        .map(|brand| EntityModel::of(brand, vec![]))
        .map_err(|why| to_response_error(Uuid::new_v4(), why, routes::BRANDS_ROOT_API))
}
