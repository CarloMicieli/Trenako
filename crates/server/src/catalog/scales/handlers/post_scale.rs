use crate::catalog::scales::repositories::PgNewScaleRepository;
use crate::catalog::scales::routes::SCALE_ROOT_API;
use crate::web::problem_detail::ProblemDetail;
use crate::web::responders::ToCreated;
use actix_web::{web, HttpResponse, Responder};
use catalog::scales::commands::new_scales::{create_new_scale, ScaleCreationError};
use catalog::scales::scale_request::ScaleRequest;
use catalog::scales::scale_response::ScaleCreated;
use common::unit_of_work::postgres::PgDatabase;
use sqlx::PgPool;
use tracing_actix_web::RequestId;

pub async fn handle(
    request_id: RequestId,
    request: web::Json<ScaleRequest>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let repo = PgNewScaleRepository;
    let database = PgDatabase::new(&db_pool);

    let result = create_new_scale(request.0, repo, database).await;
    match result {
        Ok(created) => created.to_created(),
        Err(why) => match why {
            ScaleCreationError::ScaleAlreadyExists(_) => HttpResponse::Conflict().finish(),
            _ => {
                tracing::error!("{:?}", why);
                ProblemDetail::error(*request_id, &why.to_string()).to_response()
            }
        },
    }
}

impl ToCreated for ScaleCreated {
    fn location(&self) -> String {
        format!("{}/{}", SCALE_ROOT_API, self.scale_id)
    }
}
