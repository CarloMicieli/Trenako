use crate::catalog::scales::repositories::PgNewScaleRepository;
use crate::catalog::scales::routes::SCALE_ROOT_API;
use crate::web::problem_detail::ProblemDetail;
use crate::web::responders::ToCreated;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use catalog::scales::commands::new_scales::{create_new_scale, ScaleCreationError};
use catalog::scales::scale_request::ScaleRequest;
use catalog::scales::scale_response::ScaleCreated;
use common::unit_of_work::postgres::PgDatabase;
use sqlx::PgPool;
use std::fmt;
use tracing_actix_web::RequestId;
use uuid::Uuid;

pub async fn handle(
    request_id: RequestId,
    request: web::Json<ScaleRequest>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, ScaleCreationResponseError> {
    let repo = PgNewScaleRepository;
    let database = PgDatabase::new(&db_pool);

    let result = create_new_scale(request.0, repo, database).await;
    result.map(|created| created.to_created()).map_err(|why| {
        tracing::error!("{:?}", why);
        ScaleCreationResponseError {
            request_id: *request_id,
            error: why,
        }
    })
}

impl ToCreated for ScaleCreated {
    fn location(&self) -> String {
        format!("{}/{}", SCALE_ROOT_API, self.scale_id)
    }
}

#[derive(Debug)]
pub struct ScaleCreationResponseError {
    request_id: Uuid,
    error: ScaleCreationError,
}

impl fmt::Display for ScaleCreationResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.error)
    }
}

impl ResponseError for ScaleCreationResponseError {
    fn status_code(&self) -> StatusCode {
        match self.error {
            ScaleCreationError::ScaleAlreadyExists(_) => StatusCode::CONFLICT,
            ScaleCreationError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ScaleCreationError::InvalidRequest => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let ScaleCreationResponseError { request_id, error } = self;

        let problem_details = match error {
            ScaleCreationError::ScaleAlreadyExists(_) => {
                ProblemDetail::resource_already_exists(*request_id, &error.to_string())
            }
            ScaleCreationError::UnexpectedError(why) => ProblemDetail::error(*request_id, &why.to_string()),
            ScaleCreationError::InvalidRequest => ProblemDetail::bad_request(*request_id, ""),
        };

        problem_details.to_response()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod scale_created {
        use super::*;
        use actix_web::http::header::HeaderValue;
        use actix_web::http::header::LOCATION;
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
        use anyhow::anyhow;
        use catalog::scales::scale_id::ScaleId;
        use pretty_assertions::assert_eq;
        use reqwest::header::HeaderValue;

        #[tokio::test]
        async fn it_should_return_conflict_when_the_scale_already_exists() {
            let err = ScaleCreationResponseError {
                error: ScaleCreationError::ScaleAlreadyExists(ScaleId::new("H0")),
                request_id: Uuid::new_v4(),
            };

            let status_code = err.status_code();
            let response = err.error_response();

            assert_eq!(StatusCode::CONFLICT, status_code);
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
            let err = ScaleCreationResponseError {
                error: ScaleCreationError::InvalidRequest,
                request_id: Uuid::new_v4(),
            };

            let status_code = err.status_code();
            let response = err.error_response();

            assert_eq!(StatusCode::BAD_REQUEST, status_code);
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
            let err = ScaleCreationResponseError {
                error: ScaleCreationError::UnexpectedError(anyhow!("Something bad just happened")),
                request_id: Uuid::new_v4(),
            };

            let status_code = err.status_code();
            let response = err.error_response();

            assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, status_code);
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
