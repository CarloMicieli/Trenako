use crate::web::problem::ProblemDetail;
use crate::web::responders::ToProblemDetail;
use common::queries::errors::QueryError;
use uuid::Uuid;

impl ToProblemDetail for QueryError {
    fn to_problem_detail(self, request_id: Uuid, path: Option<&str>) -> ProblemDetail {
        match self {
            QueryError::ConversionError(_) => ProblemDetail::error(request_id, "Something went wrong"),
            QueryError::DatabaseError(_) => ProblemDetail::error(request_id, "Something went wrong"),
            QueryError::EmptyResultSet => ProblemDetail::not_found(request_id, path.unwrap_or("")),
            QueryError::UnexpectedError(why) => ProblemDetail::error(request_id, &why.to_string()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod query_response_errors {
        use super::*;
        use anyhow::anyhow;
        use common::queries::converters::ConversionErrors;
        use common::queries::errors::DatabaseError;
        use common::trn::Trn;
        use pretty_assertions::assert_eq;
        use std::str::FromStr;
        use tokio;
        use url::Url;

        #[tokio::test]
        async fn it_should_produce_a_problem_detail_for_unexpected_error() {
            let error = QueryError::UnexpectedError(anyhow!("something bad happened"));
            let problem_detail = error.to_problem_detail(request_id(), Some("/my-path"));

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
            let error = QueryError::EmptyResultSet;
            let problem_detail = error.to_problem_detail(request_id(), Some("/my-path"));

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
            let error = QueryError::ConversionError(ConversionErrors::new());
            let problem_detail = error.to_problem_detail(request_id(), Some("/my-path"));

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
            let error = QueryError::DatabaseError(DatabaseError::UnexpectedError(anyhow!("something bad happened")));
            let problem_detail = error.to_problem_detail(request_id(), Some("/my-path"));

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

        fn request_id() -> Uuid {
            Uuid::parse_str("1a29fa04-8704-48d4-ab8b-31594eeaf504").unwrap()
        }
    }
}
