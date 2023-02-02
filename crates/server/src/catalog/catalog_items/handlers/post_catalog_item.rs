use crate::catalog::catalog_items::repositories::{PgNewCatalogItemRepository, PgNewRollingStockRepository};
use crate::catalog::catalog_items::routes::CATALOG_ITEM_ROOT_API;
use crate::web::problem_detail::ProblemDetail;
use actix_web::http::header::LOCATION;
use actix_web::web::Data;
use actix_web::{web, HttpResponse, Responder};
use catalog::catalog_items::catalog_item_request::CatalogItemRequest;
use catalog::catalog_items::commands::new_catalog_item::{create_new_catalog_item, CatalogItemCreationError};
use common::unit_of_work::postgres::PgDatabase;
use sqlx::PgPool;
use tracing_actix_web::RequestId;

pub async fn handle(
    request_id: RequestId,
    request: web::Json<CatalogItemRequest>,
    db_pool: Data<PgPool>,
) -> impl Responder {
    let repo = PgNewCatalogItemRepository;
    let rr_repo = PgNewRollingStockRepository;
    let database = PgDatabase::new(&db_pool);

    let result = create_new_catalog_item(request.0, repo, rr_repo, database).await;
    match result {
        Ok(created) => {
            let location = format!("{}/{}", CATALOG_ITEM_ROOT_API, created.catalog_item_id);
            HttpResponse::Created().insert_header((LOCATION, location)).finish()
        }
        Err(why) => match why {
            CatalogItemCreationError::CatalogItemAlreadyExists(_) => HttpResponse::Conflict().finish(),
            CatalogItemCreationError::BrandNotFound(_) => HttpResponse::UnprocessableEntity().finish(),
            CatalogItemCreationError::RailwayNotFound(_) => HttpResponse::UnprocessableEntity().finish(),
            CatalogItemCreationError::ScaleNotFound(_) => HttpResponse::UnprocessableEntity().finish(),
            _ => {
                tracing::error!("{:?}", why);
                ProblemDetail::error(*request_id, &why.to_string()).to_response()
            }
        },
    }
}
