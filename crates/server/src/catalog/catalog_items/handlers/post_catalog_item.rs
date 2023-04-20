use crate::catalog::catalog_items::routes::CATALOG_ITEM_ROOT_API;
use crate::web::problem_detail::ProblemDetail;
use crate::web::responders::{ToCreated, ToError, ToProblemDetail};
use actix_web::web::Data;
use actix_web::{web, Error, HttpResponse};
use catalog::catalog_items::catalog_item_request::CatalogItemRequest;
use catalog::catalog_items::catalog_item_response::CatalogItemCreated;
use catalog::catalog_items::commands::new_catalog_item::{create_new_catalog_item, CatalogItemCreationError};
use common::unit_of_work::postgres::PgDatabase;
use db::catalog::catalog_item::repositories::{CatalogItemsRepository, RollingStocksRepository};
use sqlx::PgPool;
use tracing_actix_web::RequestId;
use uuid::Uuid;

pub async fn handle(
    request_id: RequestId,
    request: web::Json<CatalogItemRequest>,
    db_pool: Data<PgPool>,
) -> Result<HttpResponse, Error> {
    let repo = CatalogItemsRepository;
    let rr_repo = RollingStocksRepository;
    let database = PgDatabase::new(&db_pool);

    let result = create_new_catalog_item(request.0, repo, rr_repo, database).await;
    result
        .map(|created| created.to_created())
        .map_err(|why| why.to_error(*request_id))
}

impl ToCreated for CatalogItemCreated {
    fn location(&self) -> String {
        format!("{}/{}", CATALOG_ITEM_ROOT_API, self.catalog_item_id)
    }
}

impl ToProblemDetail for CatalogItemCreationError {
    fn to_problem_detail(self, request_id: Uuid) -> ProblemDetail {
        match self {
            CatalogItemCreationError::BrandNotFound(_) => {
                ProblemDetail::unprocessable_entity(request_id, &self.to_string())
            }
            CatalogItemCreationError::CatalogItemAlreadyExists(_) => {
                ProblemDetail::resource_already_exists(request_id, &self.to_string())
            }
            CatalogItemCreationError::RailwayNotFound(_) => {
                ProblemDetail::unprocessable_entity(request_id, &self.to_string())
            }
            CatalogItemCreationError::ScaleNotFound(_) => {
                ProblemDetail::unprocessable_entity(request_id, &self.to_string())
            }
            CatalogItemCreationError::UnexpectedError(why) => ProblemDetail::error(request_id, &why.to_string()),
            CatalogItemCreationError::DatabaseError(why) => ProblemDetail::error(request_id, &why.to_string()),
            CatalogItemCreationError::InvalidRequest(_) => ProblemDetail::bad_request(request_id, ""),
        }
    }
}

impl ToError for CatalogItemCreationError {}

#[cfg(test)]
mod test {
    use super::*;

    mod catalog_item_created {
        use super::*;
        use actix_web::http::header::HeaderValue;
        use actix_web::http::header::LOCATION;
        use actix_web::http::StatusCode;
        use catalog::brands::brand_id::BrandId;
        use catalog::catalog_items::catalog_item_id::CatalogItemId;
        use catalog::catalog_items::item_number::ItemNumber;
        use chrono::Utc;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_a_created_response() {
            let id = CatalogItemId::of(&BrandId::new("ACME"), &ItemNumber::new("123456"));
            let created = CatalogItemCreated {
                catalog_item_id: id,
                created_at: Utc::now(),
            };

            let http_response = created.to_created();

            assert_eq!(StatusCode::CREATED, http_response.status());

            let expected_location: &HeaderValue = &HeaderValue::from_static("/api/catalog-items/acme-123456");
            assert_eq!(Some(expected_location), http_response.headers().get(LOCATION));
        }
    }

    mod railway_creation_response_error {
        use super::*;
        use crate::web::problem_detail::helpers::from_http_response;
        use actix_web::http::header::CONTENT_TYPE;
        use actix_web::http::StatusCode;
        use anyhow::anyhow;
        use catalog::brands::brand_id::BrandId;
        use catalog::catalog_items::catalog_item_id::CatalogItemId;
        use catalog::catalog_items::item_number::ItemNumber;
        use catalog::railways::railway_id::RailwayId;
        use catalog::scales::scale_id::ScaleId;
        use common::queries::errors::DatabaseError;
        use pretty_assertions::assert_eq;
        use reqwest::header::HeaderValue;
        use validator::ValidationErrors;

        #[tokio::test]
        async fn it_should_return_conflict_when_the_railway_already_exists() {
            let id = CatalogItemId::of(&BrandId::new("ACME"), &ItemNumber::new("123456"));
            let err = CatalogItemCreationError::CatalogItemAlreadyExists(id).to_error(Uuid::new_v4());

            let response = err.error_response();
            assert_eq!(StatusCode::CONFLICT, response.status());

            let expected_content_type: &HeaderValue = &HeaderValue::from_static("application/problem+json");
            assert_eq!(Some(expected_content_type), response.headers().get(CONTENT_TYPE));

            let http_response_values = from_http_response(response).await.expect("invalid http response");
            http_response_values
                .assert_status_is(StatusCode::CONFLICT)
                .assert_type_is("https://httpstatuses.com/409")
                .assert_detail_is("The catalog item already exists (id: acme-123456)")
                .assert_title_is("The resource already exists");
        }

        #[tokio::test]
        async fn it_should_return_unprocessable_entity_when_the_brand_was_not_found() {
            let err = CatalogItemCreationError::BrandNotFound(BrandId::new("ACME")).to_error(Uuid::new_v4());

            let response = err.error_response();
            assert_eq!(StatusCode::UNPROCESSABLE_ENTITY, response.status());

            let expected_content_type: &HeaderValue = &HeaderValue::from_static("application/problem+json");
            assert_eq!(Some(expected_content_type), response.headers().get(CONTENT_TYPE));

            let http_response_values = from_http_response(response).await.expect("invalid http response");
            http_response_values
                .assert_status_is(StatusCode::UNPROCESSABLE_ENTITY)
                .assert_type_is("https://httpstatuses.com/422")
                .assert_detail_is("Unable to create the catalog item due to brand not found (id: acme)")
                .assert_title_is("Unprocessable entity");
        }

        #[tokio::test]
        async fn it_should_return_unprocessable_entity_when_the_scale_was_not_found() {
            let err = CatalogItemCreationError::ScaleNotFound(ScaleId::new("H0")).to_error(Uuid::new_v4());

            let response = err.error_response();
            assert_eq!(StatusCode::UNPROCESSABLE_ENTITY, response.status());

            let expected_content_type: &HeaderValue = &HeaderValue::from_static("application/problem+json");
            assert_eq!(Some(expected_content_type), response.headers().get(CONTENT_TYPE));

            let http_response_values = from_http_response(response).await.expect("invalid http response");
            http_response_values
                .assert_status_is(StatusCode::UNPROCESSABLE_ENTITY)
                .assert_type_is("https://httpstatuses.com/422")
                .assert_detail_is("Unable to create the catalog item due to scale not found (id: h0)")
                .assert_title_is("Unprocessable entity");
        }

        #[tokio::test]
        async fn it_should_return_unprocessable_entity_when_the_railway_was_not_found() {
            let err = CatalogItemCreationError::RailwayNotFound(RailwayId::new("FS")).to_error(Uuid::new_v4());

            let response = err.error_response();
            assert_eq!(StatusCode::UNPROCESSABLE_ENTITY, response.status());

            let expected_content_type: &HeaderValue = &HeaderValue::from_static("application/problem+json");
            assert_eq!(Some(expected_content_type), response.headers().get(CONTENT_TYPE));

            let http_response_values = from_http_response(response).await.expect("invalid http response");
            http_response_values
                .assert_status_is(StatusCode::UNPROCESSABLE_ENTITY)
                .assert_type_is("https://httpstatuses.com/422")
                .assert_detail_is("Unable to create the catalog item due to railway not found (id: fs)")
                .assert_title_is("Unprocessable entity");
        }

        #[tokio::test]
        async fn it_should_return_bad_request_for_invalid_request() {
            let err = CatalogItemCreationError::InvalidRequest(ValidationErrors::new()).to_error(Uuid::new_v4());

            let response = err.error_response();
            assert_eq!(StatusCode::BAD_REQUEST, response.status());

            let expected_content_type: &HeaderValue = &HeaderValue::from_static("application/problem+json");
            assert_eq!(Some(expected_content_type), response.headers().get(CONTENT_TYPE));

            let http_response_values = from_http_response(response).await.expect("invalid http response");
            http_response_values
                .assert_status_is(StatusCode::BAD_REQUEST)
                .assert_type_is("https://httpstatuses.com/400")
                .assert_detail_is("")
                .assert_title_is("Bad request");
        }

        #[tokio::test]
        async fn it_should_return_an_internal_server_error_for_generic_errors() {
            let err = CatalogItemCreationError::UnexpectedError(anyhow!("Something bad just happened"))
                .to_error(Uuid::new_v4());

            let response = err.error_response();
            assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, response.status());

            let expected_content_type: &HeaderValue = &HeaderValue::from_static("application/problem+json");
            assert_eq!(Some(expected_content_type), response.headers().get(CONTENT_TYPE));

            let http_response_values = from_http_response(response).await.expect("invalid http response");
            http_response_values
                .assert_status_is(StatusCode::INTERNAL_SERVER_ERROR)
                .assert_type_is("https://httpstatuses.com/500")
                .assert_detail_is("Something bad just happened")
                .assert_title_is("Error: Internal Server Error");
        }

        #[tokio::test]
        async fn it_should_return_an_internal_server_error_for_database_errors() {
            let err = CatalogItemCreationError::DatabaseError(DatabaseError::UnexpectedError(anyhow!(
                "Something bad just happened"
            )))
            .to_error(Uuid::new_v4());

            let response = err.error_response();
            assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, response.status());

            let expected_content_type: &HeaderValue = &HeaderValue::from_static("application/problem+json");
            assert_eq!(Some(expected_content_type), response.headers().get(CONTENT_TYPE));

            let http_response_values = from_http_response(response).await.expect("invalid http response");
            http_response_values
                .assert_status_is(StatusCode::INTERNAL_SERVER_ERROR)
                .assert_type_is("https://httpstatuses.com/500")
                .assert_detail_is("Something bad just happened")
                .assert_title_is("Error: Internal Server Error");
        }
    }
}
