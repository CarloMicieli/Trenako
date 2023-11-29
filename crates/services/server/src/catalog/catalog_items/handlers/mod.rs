mod delete_catalog_item;
mod delete_rolling_stock;
mod get_catalog_item_by_id;
mod get_rolling_stock_by_id;
mod post_catalog_item;
mod post_rolling_stock;
mod put_catalog_item;
mod put_rolling_stock;

pub use delete_catalog_item::handle as delete_catalog_item;
pub use delete_rolling_stock::handle as delete_rolling_stock;
pub use get_catalog_item_by_id::handle as get_catalog_item_by_id;
pub use get_rolling_stock_by_id::handle as get_rolling_stock_by_id;
pub use post_catalog_item::handle as post_catalog_item;
pub use post_rolling_stock::handle as post_rolling_stock;
pub use put_catalog_item::handle as put_catalog_item;
pub use put_rolling_stock::handle as put_rolling_stock;
