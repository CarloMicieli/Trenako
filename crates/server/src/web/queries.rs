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
            QueryError::EmptyResultSet => ProblemDetail::not_found(*request_id, path),
            QueryError::UnexpectedError(why) => ProblemDetail::error(*request_id, &why.to_string()),
        };

        problem_details.to_response()
    }
}
