use crate::hateoas::representations::EntityModel;
use crate::web::problem_detail::ProblemDetail;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use common::queries::single_result::QueryError;
use serde::Serialize;
use std::fmt;
use uuid::Uuid;

pub fn to_http_response<R>(result: R) -> HttpResponse
where
    R: Serialize + PartialEq + Clone,
{
    let model = EntityModel::of(result, Vec::new());
    HttpResponse::Ok().json(model)
}

pub fn to_response_error(request_id: Uuid, error: QueryError, path: &str) -> QueryResponseError {
    tracing::error!("request_id: {}, error: {:?}", request_id, error);
    QueryResponseError {
        request_id,
        error,
        path: String::from(path),
    }
}

#[derive(Debug)]
pub struct QueryResponseError {
    request_id: Uuid,
    error: QueryError,
    path: String,
}

impl fmt::Display for QueryResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.error)
    }
}

impl ResponseError for QueryResponseError {
    fn status_code(&self) -> StatusCode {
        match self.error {
            QueryError::ConversionError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            QueryError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            QueryError::EmptyResultSet => StatusCode::NOT_FOUND,
            QueryError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let QueryResponseError {
            request_id,
            error,
            path,
        } = self;

        let problem_details = match error {
            QueryError::ConversionError(_) => ProblemDetail::error(*request_id, "Something went wrong"),
            QueryError::DatabaseError(_) => ProblemDetail::error(*request_id, "Something went wrong"),
            QueryError::EmptyResultSet => ProblemDetail::not_found(*request_id, path),
            QueryError::UnexpectedError(why) => ProblemDetail::error(*request_id, &why.to_string()),
        };

        problem_details.to_response()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod query_response_errors {
        use super::*;
        use crate::web::trn::Trn;
        use actix_web::body::to_bytes;
        use anyhow::anyhow;
        use common::queries::converters::ConversionErrors;
        use common::queries::errors::DatabaseError;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use std::str::FromStr;
        use tokio;
        use url::Url;

        #[rstest]
        #[case(
            query_error(QueryError::UnexpectedError(anyhow!("error"))),
            StatusCode::INTERNAL_SERVER_ERROR
        )]
        #[case(
            query_error(QueryError::ConversionError(ConversionErrors::new())),
            StatusCode::INTERNAL_SERVER_ERROR
        )]
        #[case(query_error(QueryError::EmptyResultSet), StatusCode::NOT_FOUND)]
        fn it_should_produce_the_correct_status_code(
            #[case] query_response_errors: QueryResponseError,
            #[case] expected_status_code: StatusCode,
        ) {
            let status_code = query_response_errors.status_code();
            assert_eq!(expected_status_code, status_code);
        }

        #[tokio::test]
        async fn it_should_produce_a_problem_detail_for_unexpected_error() {
            let error = query_error(QueryError::UnexpectedError(anyhow!("something bad happened")));
            let response = error.error_response();

            assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

            let body = extract_body(response).await.expect("unable to extract the body");

            assert_eq!(body.problem_type, Url::parse("https://httpstatuses.com/500").unwrap());
            assert_eq!(body.title, "Error: Internal Server Error");
            assert_eq!(body.detail, "something bad happened");
            assert_eq!(body.status, 500);
            assert_eq!(
                body.instance,
                Trn::from_str("trn:instance:1a29fa04-8704-48d4-ab8b-31594eeaf504").unwrap()
            );
        }

        #[tokio::test]
        async fn it_should_produce_a_problem_detail_for_empty_result_sets() {
            let error = query_error(QueryError::EmptyResultSet);
            let response = error.error_response();

            assert_eq!(response.status(), StatusCode::NOT_FOUND);

            let body = extract_body(response).await.expect("unable to extract the body");

            assert_eq!(body.problem_type, Url::parse("https://httpstatuses.com/404").unwrap());
            assert_eq!(body.title, "The resource was not found");
            assert_eq!(body.detail, "/my-path");
            assert_eq!(body.status, 404);
            assert_eq!(
                body.instance,
                Trn::from_str("trn:instance:1a29fa04-8704-48d4-ab8b-31594eeaf504").unwrap()
            );
        }

        #[tokio::test]
        async fn it_should_produce_a_problem_detail_for_conversion_errors() {
            let error = query_error(QueryError::ConversionError(ConversionErrors::new()));
            let response = error.error_response();

            assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

            let body = extract_body(response).await.expect("unable to extract the body");

            assert_eq!(body.problem_type, Url::parse("https://httpstatuses.com/500").unwrap());
            assert_eq!(body.title, "Error: Internal Server Error");
            assert_eq!(body.detail, "Something went wrong");
            assert_eq!(body.status, 500);
            assert_eq!(
                body.instance,
                Trn::from_str("trn:instance:1a29fa04-8704-48d4-ab8b-31594eeaf504").unwrap()
            );
        }

        #[tokio::test]
        async fn it_should_produce_a_problem_detail_for_database_errors() {
            let error = query_error(QueryError::DatabaseError(DatabaseError::UnexpectedError(anyhow!(
                "something bad happened"
            ))));
            let response = error.error_response();

            assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

            let body = extract_body(response).await.expect("unable to extract the body");

            assert_eq!(body.problem_type, Url::parse("https://httpstatuses.com/500").unwrap());
            assert_eq!(body.title, "Error: Internal Server Error");
            assert_eq!(body.detail, "Something went wrong");
            assert_eq!(body.status, 500);
            assert_eq!(
                body.instance,
                Trn::from_str("trn:instance:1a29fa04-8704-48d4-ab8b-31594eeaf504").unwrap()
            );
        }

        async fn extract_body(response: HttpResponse) -> Result<ProblemDetail, anyhow::Error> {
            let body = to_bytes(response.into_body())
                .await
                .map_err(|why| anyhow!("unable to extract the body {:?}", why))?;

            let problem_detail: ProblemDetail =
                serde_json::from_slice(&body).map_err(|why| anyhow!("unable to extract the body {:?}", why))?;
            Ok(problem_detail)
        }

        fn query_error(error: QueryError) -> QueryResponseError {
            QueryResponseError {
                request_id: Uuid::parse_str("1a29fa04-8704-48d4-ab8b-31594eeaf504").unwrap(),
                error,
                path: String::from("/my-path"),
            }
        }
    }
}
