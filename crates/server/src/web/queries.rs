use axum::response::{IntoResponse, Response};
use common::queries::errors::QueryError;
use problem::ProblemDetail;
use std::fmt;
use uuid::Uuid;

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

impl IntoResponse for QueryResponseError {
    fn into_response(self) -> Response {
        let QueryResponseError {
            request_id,
            error,
            path,
        } = self;

        let problem_details = match error {
            QueryError::ConversionError(_) => ProblemDetail::error(request_id, "Something went wrong"),
            QueryError::DatabaseError(_) => ProblemDetail::error(request_id, "Something went wrong"),
            QueryError::EmptyResultSet => ProblemDetail::not_found(request_id, &path),
            QueryError::UnexpectedError(why) => ProblemDetail::error(request_id, &why.to_string()),
        };
        problem_details.into_response()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod query_response_errors {
        use super::*;
        use crate::testing::extract_body;
        use anyhow::anyhow;
        use axum::http::StatusCode;
        use common::queries::converters::ConversionErrors;
        use common::queries::errors::DatabaseError;
        use common::trn::Trn;
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
            let status_code = query_response_errors.into_response().status();
            assert_eq!(expected_status_code, status_code);
        }

        #[tokio::test]
        async fn it_should_produce_a_problem_detail_for_unexpected_error() {
            let error = query_error(QueryError::UnexpectedError(anyhow!("something bad happened")));
            let response = error.into_response();

            assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

            let body_bytes = extract_body(response).await.expect("unable to extract the body");
            let problem_detail = serde_json::from_slice::<ProblemDetail>(&body_bytes).unwrap();

            assert_eq!(
                problem_detail.problem_type,
                Url::parse("https://httpstatuses.com/500").unwrap()
            );
            assert_eq!(problem_detail.title, "Error: Internal Server Error");
            assert_eq!(problem_detail.detail, "something bad happened");
            assert_eq!(problem_detail.status, 500);
            assert_eq!(
                problem_detail.instance,
                Trn::from_str("trn:instance:1a29fa04-8704-48d4-ab8b-31594eeaf504").unwrap()
            );
        }

        #[tokio::test]
        async fn it_should_produce_a_problem_detail_for_empty_result_sets() {
            let error = query_error(QueryError::EmptyResultSet);
            let response = error.into_response();

            assert_eq!(response.status(), StatusCode::NOT_FOUND);

            let body_bytes = extract_body(response).await.expect("unable to extract the body");
            let problem_detail = serde_json::from_slice::<ProblemDetail>(&body_bytes).unwrap();

            assert_eq!(
                problem_detail.problem_type,
                Url::parse("https://httpstatuses.com/404").unwrap()
            );
            assert_eq!(problem_detail.title, "The resource was not found");
            assert_eq!(problem_detail.detail, "/my-path");
            assert_eq!(problem_detail.status, 404);
            assert_eq!(
                problem_detail.instance,
                Trn::from_str("trn:instance:1a29fa04-8704-48d4-ab8b-31594eeaf504").unwrap()
            );
        }

        #[tokio::test]
        async fn it_should_produce_a_problem_detail_for_conversion_errors() {
            let error = query_error(QueryError::ConversionError(ConversionErrors::new()));
            let response = error.into_response();

            assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

            let body_bytes = extract_body(response).await.expect("unable to extract the body");
            let problem_detail = serde_json::from_slice::<ProblemDetail>(&body_bytes).unwrap();

            assert_eq!(
                problem_detail.problem_type,
                Url::parse("https://httpstatuses.com/500").unwrap()
            );
            assert_eq!(problem_detail.title, "Error: Internal Server Error");
            assert_eq!(problem_detail.detail, "Something went wrong");
            assert_eq!(problem_detail.status, 500);
            assert_eq!(
                problem_detail.instance,
                Trn::from_str("trn:instance:1a29fa04-8704-48d4-ab8b-31594eeaf504").unwrap()
            );
        }

        #[tokio::test]
        async fn it_should_produce_a_problem_detail_for_database_errors() {
            let error = query_error(QueryError::DatabaseError(DatabaseError::UnexpectedError(anyhow!(
                "something bad happened"
            ))));
            let response = error.into_response();

            assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

            let body_bytes = extract_body(response).await.expect("unable to extract the body");
            let problem_detail = serde_json::from_slice::<ProblemDetail>(&body_bytes).unwrap();

            assert_eq!(
                problem_detail.problem_type,
                Url::parse("https://httpstatuses.com/500").unwrap()
            );
            assert_eq!(problem_detail.title, "Error: Internal Server Error");
            assert_eq!(problem_detail.detail, "Something went wrong");
            assert_eq!(problem_detail.status, 500);
            assert_eq!(
                problem_detail.instance,
                Trn::from_str("trn:instance:1a29fa04-8704-48d4-ab8b-31594eeaf504").unwrap()
            );
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
