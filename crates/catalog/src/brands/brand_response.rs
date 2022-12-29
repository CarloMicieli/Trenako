use crate::brands::brand_id::BrandId;
use chrono::{DateTime, Utc};

/// A response for new brands creation
#[derive(Debug, PartialEq, Eq)]
pub struct BrandCreated {
    pub brand_id: BrandId,
    pub created_at: DateTime<Utc>,
}

/// A response for modified brands
#[derive(Debug, PartialEq, Eq)]
pub struct BrandUpdated {
    pub brand_id: BrandId,
    pub last_modified_at: DateTime<Utc>,
}
