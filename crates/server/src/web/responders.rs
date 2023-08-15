use crate::web::problem::ProblemDetail;
use uuid::Uuid;

pub trait ToProblemDetail {
    /// Convert this value to a problem detail with the given `request_id`
    fn to_problem_detail(self, request_id: Uuid) -> ProblemDetail;
}
