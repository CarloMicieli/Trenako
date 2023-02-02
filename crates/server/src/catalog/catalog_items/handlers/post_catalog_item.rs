use crate::catalog::catalog_items::repositories::{PgNewCatalogItemRepository, PgNewRollingStockRepository};
use crate::catalog::catalog_items::routes::CATALOG_ITEM_ROOT_API;
use crate::web::problem_detail::ProblemDetail;
use crate::web::responders::ToCreated;
use actix_web::http::StatusCode;
use actix_web::web::Data;
use actix_web::{web, HttpResponse, ResponseError};
use catalog::catalog_items::catalog_item_request::CatalogItemRequest;
use catalog::catalog_items::catalog_item_response::CatalogItemCreated;
use catalog::catalog_items::commands::new_catalog_item::{create_new_catalog_item, CatalogItemCreationError};
use common::unit_of_work::postgres::PgDatabase;
use sqlx::PgPool;
use std::fmt;
use tracing_actix_web::RequestId;
use uuid::Uuid;

pub async fn handle(
    request_id: RequestId,
    request: web::Json<CatalogItemRequest>,
    db_pool: Data<PgPool>,
) -> Result<HttpResponse, CatalogItemCreationResponseError> {
    let repo = PgNewCatalogItemRepository;
    let rr_repo = PgNewRollingStockRepository;
    let database = PgDatabase::new(&db_pool);

    let result = create_new_catalog_item(request.0, repo, rr_repo, database).await;
    result.map(|created| created.to_created()).map_err(|why| {
        tracing::error!("{:?}", why);
        CatalogItemCreationResponseError {
            request_id: *request_id,
            error: why,
        }
    })
}

impl ToCreated for CatalogItemCreated {
    fn location(&self) -> String {
        format!("{}/{}", CATALOG_ITEM_ROOT_API, self.catalog_item_id)
    }
}

#[derive(Debug)]
pub struct CatalogItemCreationResponseError {
    request_id: Uuid,
    error: CatalogItemCreationError,
}

impl fmt::Display for CatalogItemCreationResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.error)
    }
}

impl ResponseError for CatalogItemCreationResponseError {
    fn status_code(&self) -> StatusCode {
        match self.error {
            CatalogItemCreationError::BrandNotFound(_) => StatusCode::UNPROCESSABLE_ENTITY,
            CatalogItemCreationError::CatalogItemAlreadyExists(_) => StatusCode::CONFLICT,
            CatalogItemCreationError::RailwayNotFound(_) => StatusCode::UNPROCESSABLE_ENTITY,
            CatalogItemCreationError::ScaleNotFound(_) => StatusCode::UNPROCESSABLE_ENTITY,
            CatalogItemCreationError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            CatalogItemCreationError::InvalidRequest => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let CatalogItemCreationResponseError { request_id, error } = self;

        let problem_details = match error {
            CatalogItemCreationError::BrandNotFound(_) => {
                ProblemDetail::unprocessable_entity(*request_id, &error.to_string())
            }
            CatalogItemCreationError::CatalogItemAlreadyExists(_) => {
                ProblemDetail::resource_already_exists(*request_id, &error.to_string())
            }
            CatalogItemCreationError::RailwayNotFound(_) => {
                ProblemDetail::unprocessable_entity(*request_id, &error.to_string())
            }
            CatalogItemCreationError::ScaleNotFound(_) => {
                ProblemDetail::unprocessable_entity(*request_id, &error.to_string())
            }
            CatalogItemCreationError::UnexpectedError(why) => ProblemDetail::error(*request_id, &why.to_string()),
            CatalogItemCreationError::InvalidRequest => ProblemDetail::bad_request(*request_id, ""),
        };

        problem_details.to_response()
    }
}
