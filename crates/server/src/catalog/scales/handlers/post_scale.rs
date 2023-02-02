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
