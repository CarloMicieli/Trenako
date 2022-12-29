use crate::catalog_items::availability_status::AvailabilityStatus;
use crate::catalog_items::category::Category;
use crate::catalog_items::power_method::PowerMethod;
use crate::catalog_items::rolling_stock_request::RollingStockRequest;

/// A request to create/update catalog items
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct CatalogItemRequest {
    pub brand: String,
    pub item_number: String,
    pub scale: String,
    pub category: Category,
    pub description: Option<String>,
    pub details: Option<String>,
    pub power_method: Option<PowerMethod>,
    pub delivery_date: Option<String>,
    pub availability_status: Option<AvailabilityStatus>,
    pub rolling_stocks: Vec<RollingStockRequest>,
    pub count: i32,
}
