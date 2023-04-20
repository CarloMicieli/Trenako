use crate::web::problem_detail::ProblemDetail;
use actix_web::error::JsonPayloadError;
use actix_web::web::JsonConfig;
use actix_web::{Error, HttpRequest};
use uuid::Uuid;

pub fn json_config() -> JsonConfig {
    JsonConfig::default()
        // limit request payload size
        .limit(4096)
        // only accept application/json content type
        .content_type(|mime| mime == mime::APPLICATION_JSON)
        // use custom error handler
        .error_handler(problem_details_json_error)
}

fn problem_details_json_error(err: JsonPayloadError, _req: &HttpRequest) -> Error {
    let request_id = Uuid::new_v4();
    tracing::warn!("{}, {}", request_id, err);
    ProblemDetail::bad_request(request_id, &err.to_string()).into()
}
