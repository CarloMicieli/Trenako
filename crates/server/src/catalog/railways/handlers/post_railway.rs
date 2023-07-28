use crate::catalog::railways::routes::RAILWAY_ROOT_API;
use crate::web::responders::{ToCreated, ToError, ToProblemDetail};
use actix_web::{web, Error, HttpResponse};
use catalog::railways::commands::new_railways::{create_new_railway, RailwayCreationError};
use catalog::railways::railway_request::RailwayRequest;
use catalog::railways::railway_response::RailwayCreated;
use common::unit_of_work::postgres::PgDatabase;
use db::catalog::railways::repositories::RailwaysRepository;
use problem::ProblemDetail;
use sqlx::PgPool;
use tracing_actix_web::RequestId;
use uuid::Uuid;

pub async fn handle(
    request_id: RequestId,
    request: web::Json<RailwayRequest>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    let repo = RailwaysRepository;
    let database = PgDatabase::new(&db_pool);

    let result = create_new_railway(request.0, repo, database).await;
    result
        .map(|created| created.to_created())
        .map_err(|why| why.to_error(*request_id))
}

impl ToCreated for RailwayCreated {
    fn location(&self) -> String {
        format!("{}/{}", RAILWAY_ROOT_API, self.railway_id)
    }
}

impl ToProblemDetail for RailwayCreationError {
    fn to_problem_detail(self, request_id: Uuid) -> ProblemDetail {
        match self {
            RailwayCreationError::RailwayAlreadyExists(_) => {
                ProblemDetail::resource_already_exists(request_id, &self.to_string())
            }
            RailwayCreationError::DatabaseError(why) => ProblemDetail::error(request_id, &why.to_string()),
            RailwayCreationError::UnexpectedError(why) => ProblemDetail::error(request_id, &why.to_string()),
            RailwayCreationError::InvalidRequest(_) => ProblemDetail::bad_request(request_id, ""),
        }
    }
}

impl ToError for RailwayCreationError {}

#[cfg(test)]
mod test {
    use super::*;

    mod railway_created {
        use super::*;
        use actix_web::http::header::HeaderValue;
        use actix_web::http::header::LOCATION;
        use actix_web::http::StatusCode;
        use catalog::railways::railway_id::RailwayId;
        use chrono::Utc;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_a_created_response() {
            let created = RailwayCreated {
                railway_id: RailwayId::new("FS"),
                created_at: Utc::now(),
            };

            let http_response = created.to_created();

            assert_eq!(StatusCode::CREATED, http_response.status());

            let expected_location: &HeaderValue = &HeaderValue::from_static("/api/railways/fs");
            assert_eq!(Some(expected_location), http_response.headers().get(LOCATION));
        }
    }

    mod railway_creation_response_error {
        use super::*;
        use actix_web::http::header::CONTENT_TYPE;
        use actix_web::http::StatusCode;
        use anyhow::anyhow;
        use catalog::railways::railway_id::RailwayId;
        use common::queries::errors::DatabaseError;
        use pretty_assertions::assert_eq;
        use problem::helpers::from_http_response;
        use reqwest::header::HeaderValue;
        use validator::ValidationErrors;

        #[tokio::test]
        async fn it_should_return_conflict_when_the_railway_already_exists() {
            let err = RailwayCreationError::RailwayAlreadyExists(RailwayId::new("FS")).to_error(Uuid::new_v4());

            let response = err.error_response();
            assert_eq!(StatusCode::CONFLICT, response.status());

            let expected_content_type: &HeaderValue = &HeaderValue::from_static("application/problem+json");
            assert_eq!(Some(expected_content_type), response.headers().get(CONTENT_TYPE));

            let http_response_values = from_http_response(response).await.expect("invalid http response");
            http_response_values
                .assert_status_is(StatusCode::CONFLICT)
                .assert_type_is("https://httpstatuses.com/409")
                .assert_detail_is("The railway already exists (id: fs)")
                .assert_title_is("The resource already exists");
        }

        #[tokio::test]
        async fn it_should_return_bad_request_for_invalid_request() {
            let err = RailwayCreationError::InvalidRequest(ValidationErrors::new()).to_error(Uuid::new_v4());

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
                RailwayCreationError::UnexpectedError(anyhow!("Something bad just happened")).to_error(Uuid::new_v4());

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
            let err = RailwayCreationError::DatabaseError(DatabaseError::UnexpectedError(anyhow!(
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
