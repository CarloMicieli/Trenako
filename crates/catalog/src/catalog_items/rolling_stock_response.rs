use crate::catalog_items::rolling_stock_id::RollingStockId;

/// A response for new rolling stocks creation
#[derive(Debug, PartialEq, Eq)]
pub struct RollingStockCreated {
    pub rolling_stock_id: RollingStockId,
}

/// A response for rolling stock updates
#[derive(Debug, PartialEq, Eq)]
pub struct RollingStockUpdated {
    pub rolling_stock_id: RollingStockId,
}
