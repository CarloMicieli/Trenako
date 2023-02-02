use crate::catalog::railways::repositories::PgNewRailwayRepository;
use crate::catalog::railways::routes::RAILWAY_ROOT_API;
use crate::web::problem_detail::ProblemDetail;
use crate::web::responders::ToCreated;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use catalog::railways::commands::new_railways::{create_new_railway, RailwayCreationError};
use catalog::railways::railway_request::RailwayRequest;
use catalog::railways::railway_response::RailwayCreated;
use common::unit_of_work::postgres::PgDatabase;
use sqlx::PgPool;
use std::fmt;
use tracing_actix_web::RequestId;
use uuid::Uuid;

pub async fn handle(
    request_id: RequestId,
    request: web::Json<RailwayRequest>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, RailwayCreationResponseError> {
    let repo = PgNewRailwayRepository;
    let database = PgDatabase::new(&db_pool);

    let result = create_new_railway(request.0, repo, database).await;
    result.map(|created| created.to_created()).map_err(|why| {
        tracing::error!("{:?}", why);
        RailwayCreationResponseError {
            request_id: *request_id,
            error: why,
        }
    })
}

impl ToCreated for RailwayCreated {
    fn location(&self) -> String {
        format!("{}/{}", RAILWAY_ROOT_API, self.railway_id)
    }
}

#[derive(Debug)]
pub struct RailwayCreationResponseError {
    request_id: Uuid,
    error: RailwayCreationError,
}

impl fmt::Display for RailwayCreationResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.error)
    }
}

impl ResponseError for RailwayCreationResponseError {
    fn status_code(&self) -> StatusCode {
        match self.error {
            RailwayCreationError::RailwayAlreadyExists(_) => StatusCode::CONFLICT,
            RailwayCreationError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RailwayCreationError::InvalidRequest => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let RailwayCreationResponseError { request_id, error } = self;

        let problem_details = match error {
            RailwayCreationError::RailwayAlreadyExists(_) => {
                ProblemDetail::resource_already_exists(*request_id, &error.to_string())
            }
            RailwayCreationError::UnexpectedError(why) => ProblemDetail::error(*request_id, &why.to_string()),
            RailwayCreationError::InvalidRequest => ProblemDetail::bad_request(*request_id, ""),
        };

        problem_details.to_response()
    }
}
