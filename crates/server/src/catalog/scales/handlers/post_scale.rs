use crate::catalog::scales::routes::SCALE_ROOT_API;
use crate::web::problem_detail::ProblemDetail;
use crate::web::responders::{ToCreated, ToError, ToProblemDetail};
use actix_web::{web, Error, HttpResponse};
use catalog::scales::commands::new_scales::{create_new_scale, ScaleCreationError};
use catalog::scales::scale_request::ScaleRequest;
use catalog::scales::scale_response::ScaleCreated;
use common::unit_of_work::postgres::PgDatabase;
use db::catalog::scales::repositories::ScalesRepository;
use sqlx::PgPool;
use tracing_actix_web::RequestId;
use uuid::Uuid;

pub async fn handle(
    request_id: RequestId,
    request: web::Json<ScaleRequest>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    let repo = ScalesRepository;
    let database = PgDatabase::new(&db_pool);

    let result = create_new_scale(request.0, repo, database).await;
    result
        .map(|created| created.to_created())
        .map_err(|why| why.to_error(*request_id))
}

impl ToCreated for ScaleCreated {
    fn location(&self) -> String {
        format!("{}/{}", SCALE_ROOT_API, self.scale_id)
    }
}

impl ToProblemDetail for ScaleCreationError {
    fn to_problem_detail(self, request_id: Uuid) -> ProblemDetail {
        match self {
            ScaleCreationError::ScaleAlreadyExists(_) => {
                ProblemDetail::resource_already_exists(request_id, &self.to_string())
            }
            ScaleCreationError::DatabaseError(why) => ProblemDetail::error(request_id, &why.to_string()),
            ScaleCreationError::UnexpectedError(why) => ProblemDetail::error(request_id, &why.to_string()),
            ScaleCreationError::InvalidRequest(_) => ProblemDetail::bad_request(request_id, ""),
        }
    }
}

impl ToError for ScaleCreationError {}

#[cfg(test)]
mod test {
    use super::*;

    mod scale_created {
        use super::*;
        use actix_web::http::header::HeaderValue;
        use actix_web::http::header::LOCATION;
        use actix_web::http::StatusCode;
        use catalog::scales::scale_id::ScaleId;
        use chrono::Utc;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_a_created_response() {
            let created = ScaleCreated {
                scale_id: ScaleId::new("H0"),
                created_at: Utc::now(),
            };

            let http_response = created.to_created();

            assert_eq!(StatusCode::CREATED, http_response.status());

            let expected_location: &HeaderValue = &HeaderValue::from_static("/api/scales/h0");
            assert_eq!(Some(expected_location), http_response.headers().get(LOCATION));
        }
    }

    mod scale_creation_response_error {
        use super::*;
        use crate::web::problem_detail::helpers::from_http_response;
        use actix_web::http::header::CONTENT_TYPE;
        use actix_web::http::StatusCode;
        use anyhow::anyhow;
        use catalog::scales::scale_id::ScaleId;
        use common::queries::errors::DatabaseError;
        use pretty_assertions::assert_eq;
        use reqwest::header::HeaderValue;
        use validator::ValidationErrors;

        #[tokio::test]
        async fn it_should_return_conflict_when_the_scale_already_exists() {
            let err = ScaleCreationError::ScaleAlreadyExists(ScaleId::new("H0")).to_error(Uuid::new_v4());

            let response = err.error_response();
            assert_eq!(StatusCode::CONFLICT, response.status());

            let expected_content_type: &HeaderValue = &HeaderValue::from_static("application/problem+json");
            assert_eq!(Some(expected_content_type), response.headers().get(CONTENT_TYPE));

            let http_response_values = from_http_response(response).await.expect("invalid http response");
            http_response_values
                .assert_status_is(StatusCode::CONFLICT)
                .assert_type_is("https://httpstatuses.com/409")
                .assert_detail_is("The scale already exists (id: h0)")
                .assert_title_is("The resource already exists");
        }

        #[tokio::test]
        async fn it_should_return_bad_request_for_invalid_request() {
            let err = ScaleCreationError::InvalidRequest(ValidationErrors::new()).to_error(Uuid::new_v4());

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
            let err =
                ScaleCreationError::UnexpectedError(anyhow!("Something bad just happened")).to_error(Uuid::new_v4());

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
            let err = ScaleCreationError::DatabaseError(DatabaseError::UnexpectedError(anyhow!(
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
