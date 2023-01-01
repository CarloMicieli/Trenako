use crate::catalog_items::availability_status::AvailabilityStatus;
use crate::catalog_items::category::Category;
use crate::catalog_items::power_method::PowerMethod;
use crate::catalog_items::rolling_stock_request::RollingStockRequest;

/// A request to create/update catalog items
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct CatalogItemRequest {
    /// the brand
    pub brand: String,
    /// the item number
    pub item_number: String,
    /// the scale
    pub scale: String,
    /// the category
    pub category: Category,
    /// the power method
    pub power_method: PowerMethod,
    /// the catalog item description
    pub description: Option<String>,
    /// the catalog item details
    pub details: Option<String>,
    /// the delivery date
    pub delivery_date: Option<String>,
    /// the availability status
    pub availability_status: Option<AvailabilityStatus>,
    /// the rolling stocks included in this catalog item
    pub rolling_stocks: Vec<RollingStockRequest>,
    /// the number of rolling stocks for this catalog item
    pub count: i32,
}
