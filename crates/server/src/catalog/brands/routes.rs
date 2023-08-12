use crate::app::AppState;
use crate::catalog::brands::handlers;
use axum::routing::{delete, get};
use axum::Router;

pub const BRAND_ROOT_API: &str = "/api/brands";
pub const BRAND_API: &str = "/api/brands/:brand_id";

pub fn brands_router() -> Router<AppState> {
    Router::new()
        .route(BRAND_ROOT_API, get(handlers::get_all_brands).post(handlers::post_brand))
        .route(
            BRAND_API,
            delete(handlers::delete_brand)
                .get(handlers::get_brand_by_id)
                .put(handlers::put_brand),
        )
}
