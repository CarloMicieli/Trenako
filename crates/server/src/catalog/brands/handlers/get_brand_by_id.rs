use crate::catalog::brands::queries::PgFindBrandByIdQuery;
use crate::hateoas::representations::EntityModel;
use crate::web::problem_detail::ProblemDetail;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use catalog::brands::brand::Brand;
use catalog::brands::brand_id::BrandId;
use catalog::brands::queries::find_by_id::{FindBrandByIdQuery, QueryError};
use common::unit_of_work::postgres::PgDatabase;
use common::unit_of_work::{Database, UnitOfWork};
use serde::Serialize;
use sqlx::PgPool;
use std::fmt;
use tracing_actix_web::RequestId;
use uuid::Uuid;

pub async fn handle(
    request_id: RequestId,
    brand_id: web::Path<BrandId>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, QueryResponseError> {
    let result = exec(&brand_id, &db_pool).await;

    result.map(to_http_response).map_err(|why| {
        tracing::error!("{:?}", why);
        QueryResponseError {
            request_id: *request_id,
            error: why,
            path: brand_id.to_string(),
        }
    })
}

fn to_http_response<R>(result: R) -> HttpResponse
where
    R: Serialize + PartialEq + Clone,
{
    let model = EntityModel::of(result, Vec::new());
    HttpResponse::Ok().json(model)
}

async fn exec(brand_id: &BrandId, db_pool: &PgPool) -> Result<Brand, QueryError> {
    let query = PgFindBrandByIdQuery;
    let database = PgDatabase::new(db_pool);

    let mut unit_of_work = database.begin().await?;

    let brand = query.execute(brand_id, &mut unit_of_work).await?;

    unit_of_work.commit().await?;

    Ok(brand)
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
            QueryError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            QueryError::EmptyResultSet => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let QueryResponseError {
            request_id,
            error,
            path,
        } = self;

        let problem_details = match error {
            QueryError::UnexpectedError(why) => ProblemDetail::error(*request_id, &why.to_string()),
            QueryError::EmptyResultSet => ProblemDetail::not_found(*request_id, path),
        };

        problem_details.to_response()
    }
}
