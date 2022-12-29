use crate::catalog_items::catalog_item_id::CatalogItemId;
use chrono::{DateTime, Utc};

/// A response for new catalog items creation
#[derive(Debug, PartialEq, Eq)]
pub struct CatalogItemCreated {
    pub catalog_item_id: CatalogItemId,
    pub created_at: DateTime<Utc>,
}

/// A response for catalog items update
#[derive(Debug, PartialEq, Eq)]
pub struct CatalogItemUpdated {
    pub catalog_item_id: CatalogItemId,
    pub last_modified_at: DateTime<Utc>,
}
