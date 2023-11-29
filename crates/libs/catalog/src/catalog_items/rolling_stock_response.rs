//! the rolling stock command responses

use crate::catalog_items::rolling_stock_id::RollingStockId;
use chrono::{DateTime, Utc};

/// A response for new rolling stocks creation
#[derive(Debug, PartialEq, Eq)]
pub struct RollingStockCreated {
    pub rolling_stock_id: RollingStockId,
    pub created_at: DateTime<Utc>,
}

/// A response for rolling stock updates
#[derive(Debug, PartialEq, Eq)]
pub struct RollingStockUpdated {
    pub rolling_stock_id: RollingStockId,
    pub last_modified_at: DateTime<Utc>,
}
