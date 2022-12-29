use crate::scales::scale_id::ScaleId;
use chrono::{DateTime, Utc};

/// A response for new scales creation
#[derive(Debug, PartialEq, Eq)]
pub struct ScaleCreated {
    pub scale_id: ScaleId,
    pub created_at: DateTime<Utc>,
}

/// A response for scale updates
#[derive(Debug, PartialEq, Eq)]
pub struct ScaleUpdated {
    pub scale_id: ScaleId,
    pub last_modified_at: DateTime<Utc>,
}
