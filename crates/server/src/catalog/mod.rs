use crate::catalog::brands::routes::configure_brand_routes;
use crate::catalog::catalog_items::routes::configure_catalog_items_routes;
use crate::catalog::railways::routes::configure_railway_routes;
use crate::catalog::scales::routes::configure_scale_routes;
use actix_web::web;

pub mod brands;
pub mod catalog_items;
pub mod railways;
pub mod scales;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    cfg.configure(configure_brand_routes);
    cfg.configure(configure_catalog_items_routes);
    cfg.configure(configure_railway_routes);
    cfg.configure(configure_scale_routes);
}
