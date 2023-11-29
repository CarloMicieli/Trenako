//! the railway command responses

use crate::railways::railway_id::RailwayId;
use chrono::{DateTime, Utc};

/// A response for new railways creation
#[derive(Debug, PartialEq, Eq)]
pub struct RailwayCreated {
    pub railway_id: RailwayId,
    pub created_at: DateTime<Utc>,
}

/// A response for railways update
#[derive(Debug, PartialEq, Eq)]
pub struct RailwayUpdated {
    pub railway_id: RailwayId,
    pub last_modified_at: DateTime<Utc>,
}
