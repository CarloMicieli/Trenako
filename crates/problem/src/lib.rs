use axum::http::{header, HeaderName, StatusCode};
use axum::response::IntoResponse;
use axum::response::Response;
use common::trn::Trn;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use url::Url;
use uuid::Uuid;

/// A problem detail
///
/// # Details
///
/// From RFC-7807
/// "problem detail" is a way to carry machine-readable details of errors in a HTTP response to avoid
/// the need to define new error response formats for HTTP APIs.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProblemDetail {
    /// A URI reference (RFC-3986) that identifies the problem type. This specification
    /// encourages that, when dereferenced, it provide human-readable documentation for the
    /// problem type (e.g., using HTML). When this member is not present,
    /// its value is assumed to be "about:blank".
    #[serde(rename = "type")]
    pub problem_type: Url,
    /// A short, human-readable summary of the problem type. It SHOULD NOT change from occurrence
    /// to occurrence of the problem, except for purposes of localization(e.g., using proactive
    /// content negotiation; see RFC-7231, Section 3.4).
    pub title: String,
    /// A human-readable explanation specific to this occurrence of the problem.
    pub detail: String,
    /// The HTTP status code(RFC-7231, Section 6) generated by the origin server for this
    /// occurrence of the problem.
    pub status: u16,
    /// A TRN reference that identifies the specific occurrence of the problem.
    /// It may or may not yield further information if dereferenced.
    pub instance: Trn,
}

static PROBLEM_DETAIL_CONTENT_TYPE: &str = "application/problem+json";

impl ProblemDetail {
    /// Creates a new problem detail from an error
    pub fn error(id: Uuid, error: &str) -> ProblemDetail {
        let status_code = StatusCode::INTERNAL_SERVER_ERROR.as_u16();
        let type_url = Url::parse("https://httpstatuses.com/500").unwrap();
        ProblemDetail {
            problem_type: type_url,
            title: String::from("Error: Internal Server Error"),
            detail: error.to_owned(),
            status: status_code,
            instance: Trn::instance(&id),
        }
    }

    /// Creates a new problem detail for unprocessable entity
    pub fn unprocessable_entity(id: Uuid, detail: &str) -> ProblemDetail {
        let type_url = Url::parse("https://httpstatuses.com/422").unwrap();
        ProblemDetail {
            problem_type: type_url,
            title: String::from("Unprocessable entity"),
            detail: detail.to_owned(),
            status: StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
            instance: Trn::instance(&id),
        }
    }

    /// Creates a new problem detail for bad requests
    pub fn bad_request(id: Uuid, detail: &str) -> ProblemDetail {
        let type_url = Url::parse("https://httpstatuses.com/400").unwrap();
        ProblemDetail {
            problem_type: type_url,
            title: String::from("Bad request"),
            detail: detail.to_owned(),
            status: StatusCode::BAD_REQUEST.as_u16(),
            instance: Trn::instance(&id),
        }
    }

    /// Creates a new problem detail for a resource which exists already
    pub fn resource_already_exists(id: Uuid, detail: &str) -> ProblemDetail {
        let type_url = Url::parse("https://httpstatuses.com/409").unwrap();
        ProblemDetail {
            problem_type: type_url,
            title: String::from("The resource already exists"),
            detail: detail.to_owned(),
            status: StatusCode::CONFLICT.as_u16(),
            instance: Trn::instance(&id),
        }
    }

    /// Creates a new problem detail for a resource not found
    pub fn not_found(id: Uuid, detail: &str) -> ProblemDetail {
        let type_url = Url::parse("https://httpstatuses.com/404").unwrap();
        ProblemDetail {
            problem_type: type_url,
            title: String::from("The resource was not found"),
            detail: detail.to_owned(),
            status: StatusCode::NOT_FOUND.as_u16(),
            instance: Trn::instance(&id),
        }
    }
}

impl Display for ProblemDetail {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.title)
    }
}

impl IntoResponse for ProblemDetail {
    fn into_response(self) -> Response {
        let status_code = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let headers: [(HeaderName, &'static str); 1] = [(header::CONTENT_TYPE, PROBLEM_DETAIL_CONTENT_TYPE)];
        (status_code, headers, axum::response::Json(self)).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod problem_details {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_a_new_problem_detail_from_errors() {
            let id = Uuid::new_v4();
            let instance_id = Trn::instance(&id);
            let error = "my first error";
            let type_url = Url::parse("https://httpstatuses.com/500").unwrap();

            let problem_detail = ProblemDetail::error(id, error);

            assert_eq!(type_url, problem_detail.problem_type);
            assert_eq!("Error: Internal Server Error", problem_detail.title);
            assert_eq!(error, problem_detail.detail);
            assert_eq!(500, problem_detail.status);
            assert_eq!(instance_id, problem_detail.instance);
        }

        #[test]
        fn it_should_create_a_new_problem_detail_resource_already_exists() {
            let id = Uuid::new_v4();
            let instance_id = Trn::instance(&id);
            let detail = "my first conflict";
            let type_url = Url::parse("https://httpstatuses.com/409").unwrap();

            let problem_detail = ProblemDetail::resource_already_exists(id, detail);

            assert_eq!(type_url, problem_detail.problem_type);
            assert_eq!("The resource already exists", problem_detail.title);
            assert_eq!(detail, problem_detail.detail);
            assert_eq!(409, problem_detail.status);
            assert_eq!(instance_id, problem_detail.instance);
        }

        #[test]
        fn it_should_create_a_new_problem_detail_resource_not_found() {
            let id = Uuid::new_v4();
            let instance_id = Trn::instance(&id);
            let detail = "my first conflict";
            let type_url = Url::parse("https://httpstatuses.com/404").unwrap();

            let problem_detail = ProblemDetail::not_found(id, detail);

            assert_eq!(type_url, problem_detail.problem_type);
            assert_eq!("The resource was not found", problem_detail.title);
            assert_eq!(detail, problem_detail.detail);
            assert_eq!(404, problem_detail.status);
            assert_eq!(instance_id, problem_detail.instance);
        }

        #[test]
        fn it_should_create_a_new_problem_detail_for_unprocessable_entities() {
            let id = Uuid::new_v4();
            let instance_id = Trn::instance(&id);
            let detail = "my first conflict";
            let type_url = Url::parse("https://httpstatuses.com/422").unwrap();

            let problem_detail = ProblemDetail::unprocessable_entity(id, detail);

            assert_eq!(type_url, problem_detail.problem_type);
            assert_eq!("Unprocessable entity", problem_detail.title);
            assert_eq!(detail, problem_detail.detail);
            assert_eq!(422, problem_detail.status);
            assert_eq!(instance_id, problem_detail.instance);
        }

        #[test]
        fn it_should_create_a_new_problem_detail_for_bad_requests() {
            let id = Uuid::new_v4();
            let instance_id = Trn::instance(&id);
            let detail = "my first conflict";
            let type_url = Url::parse("https://httpstatuses.com/400").unwrap();

            let problem_detail = ProblemDetail::bad_request(id, detail);

            assert_eq!(type_url, problem_detail.problem_type);
            assert_eq!("Bad request", problem_detail.title);
            assert_eq!(detail, problem_detail.detail);
            assert_eq!(400, problem_detail.status);
            assert_eq!(instance_id, problem_detail.instance);
        }
    }
}
