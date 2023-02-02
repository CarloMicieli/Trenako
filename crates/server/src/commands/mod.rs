use crate::web::problem_detail::ProblemDetail;
use actix_web::HttpResponse;
use async_trait::async_trait;
use sqlx::PgPool;
use std::fmt::{Debug, Display};
use tracing_actix_web::RequestId;

/// It represents a command
#[async_trait]
pub trait Command {
    /// The Output type for this command
    type Output: Debug + IntoHttpResponse;
    /// The Error type for this command
    type Error: Debug + Display;

    async fn execute(self, pg_pool: &PgPool) -> Result<Self::Output, Self::Error>;
}

/// It handles a web request
pub async fn handle_web_request<C: Command>(command: C, request_id: RequestId, pg_pool: &PgPool) -> HttpResponse {
    match command.execute(pg_pool).await {
        Ok(output) => output.into_http_response(),
        Err(why) => {
            tracing::error!("{:?}", why);
            ProblemDetail::error(*request_id, &why.to_string()).to_response()
        }
    }
}

/// A trait for converting types to HttpResponses.
pub trait IntoHttpResponse {
    /// Convert self to an HttpResponse
    fn into_http_response(self) -> HttpResponse;
}
