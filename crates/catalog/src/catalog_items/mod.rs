//! the module includes everything related to catalog items

pub mod availability_status;
pub mod catalog_item;
pub mod catalog_item_id;
pub mod catalog_item_request;
pub mod catalog_item_response;
pub mod category;
pub mod commands;
pub mod control;
pub mod delivery_date;
pub mod epoch;
pub mod item_number;
pub mod length_over_buffers;
pub mod power_method;
pub mod queries;
pub mod rolling_stock;
pub mod rolling_stock_id;
pub mod rolling_stock_request;
pub mod rolling_stock_response;
pub mod service_level;
pub mod technical_specifications;

#[cfg(test)]
pub mod test_data;
