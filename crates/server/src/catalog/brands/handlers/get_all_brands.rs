use crate::catalog::brands::routes;
use crate::web::queries::{to_response_error, QueryResponseError};
use actix_web::{web, HttpResponse};
use catalog::brands::brand::Brand;
use catalog::brands::queries::find_all_brands::find_all_brands;
use common::queries::pagination::PageRequest;
use common::unit_of_work::postgres::PgDatabase;
use db::catalog::brands::repositories::BrandsRepository;
use hateoas::representations::CollectionModel;
use sqlx::PgPool;
use tracing_actix_web::RequestId;

pub async fn handle(
    request_id: RequestId,
    _page_request: web::Query<PageRequest>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, QueryResponseError> {
    let database = PgDatabase::new(&db_pool);
    let repo = BrandsRepository;

    let results = find_all_brands(repo, database).await;

    results
        .map(to_http_response)
        .map_err(|why| to_response_error(*request_id, why, routes::BRAND_ROOT_API))
}

fn to_http_response(items: Vec<Brand>) -> HttpResponse {
    let model = CollectionModel {
        items,
        links: Vec::new(),
    };
    HttpResponse::Ok().json(model)
}
