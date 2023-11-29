//! the brand command responses

use crate::brands::brand_id::BrandId;
use chrono::{DateTime, Utc};

/// It represents a response for brands creation
#[derive(Debug, PartialEq, Eq)]
pub struct BrandCreated {
    /// the brand id for the new brand
    pub brand_id: BrandId,
    /// the brand creation timestamp
    pub created_at: DateTime<Utc>,
}

/// It represents a response for brand updates
#[derive(Debug, PartialEq, Eq)]
pub struct BrandUpdated {
    /// the brand id for the updated brand
    pub brand_id: BrandId,
    /// the brand update timestamp
    pub last_modified_at: DateTime<Utc>,
}
