use actix_web::http::header::LOCATION;
use actix_web::{Error, HttpResponse};
use problem::ProblemDetail;
use std::fmt;
use uuid::Uuid;

/// The trait to convert from a command output to a "CREATED" http response
pub trait ToCreated {
    /// Creates a http response with the CREATED http status
    fn to_created(&self) -> HttpResponse {
        HttpResponse::Created()
            .insert_header((LOCATION, self.location()))
            .finish()
    }

    /// The location for the resource created
    fn location(&self) -> String;
}

pub trait ToProblemDetail {
    /// Convert this value to a problem detail with the given `request_id`
    fn to_problem_detail(self, request_id: Uuid) -> ProblemDetail;
}

pub trait ToError: ToProblemDetail + fmt::Debug + Sized {
    /// Convert this value to an HTTP error
    fn to_error(self, request_id: Uuid) -> Error {
        tracing::error!("{:?}", &self);
        self.to_problem_detail(request_id).into()
    }
}
