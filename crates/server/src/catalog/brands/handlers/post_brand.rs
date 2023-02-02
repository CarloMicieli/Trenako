use crate::catalog::brands::repositories::PgNewBrandRepository;
use crate::catalog::brands::routes::BRAND_ROOT_API;
use crate::web::problem_detail::ProblemDetail;
use crate::web::responders::ToCreated;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use catalog::brands::brand_request::BrandRequest;
use catalog::brands::brand_response::BrandCreated;
use catalog::brands::commands::new_brand::{create_new_brand, BrandCreationError};
use common::unit_of_work::postgres::PgDatabase;
use sqlx::PgPool;
use std::fmt;
use tracing_actix_web::RequestId;
use uuid::Uuid;

pub async fn handle(
    request_id: RequestId,
    request: web::Json<BrandRequest>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, BrandCreationResponseError> {
    let repo = PgNewBrandRepository;
    let database = PgDatabase::new(&db_pool);

    let result = create_new_brand(request.0, repo, database).await;
    result.map(|created| created.to_created()).map_err(|why| {
        tracing::error!("{:?}", why);
        BrandCreationResponseError {
            request_id: *request_id,
            error: why,
        }
    })
}

impl ToCreated for BrandCreated {
    fn location(&self) -> String {
        format!("{}/{}", BRAND_ROOT_API, self.brand_id)
    }
}

#[derive(Debug)]
pub struct BrandCreationResponseError {
    request_id: Uuid,
    error: BrandCreationError,
}

impl fmt::Display for BrandCreationResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.error)
    }
}

impl ResponseError for BrandCreationResponseError {
    fn status_code(&self) -> StatusCode {
        match self.error {
            BrandCreationError::BrandAlreadyExists(_) => StatusCode::CONFLICT,
            BrandCreationError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            BrandCreationError::InvalidRequest => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let BrandCreationResponseError { request_id, error } = self;

        let problem_details = match error {
            BrandCreationError::BrandAlreadyExists(_) => {
                ProblemDetail::resource_already_exists(*request_id, &error.to_string())
            }
            BrandCreationError::UnexpectedError(why) => ProblemDetail::error(*request_id, &why.to_string()),
            BrandCreationError::InvalidRequest => ProblemDetail::bad_request(*request_id, ""),
        };

        problem_details.to_response()
    }
}
