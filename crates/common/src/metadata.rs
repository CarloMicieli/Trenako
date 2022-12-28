use chrono::{DateTime, Utc};

/// The metadata information for the current resource
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    version: u8,
    created: DateTime<Utc>,
    last_modified: Option<DateTime<Utc>>,
}

impl Metadata {
    /// Creates a new metadata value
    pub fn new(version: u8, created: DateTime<Utc>, last_modified: Option<DateTime<Utc>>) -> Self {
        Metadata {
            version,
            created,
            last_modified,
        }
    }

    /// Creates metadata for a newly created resource
    pub fn created_at(created: DateTime<Utc>) -> Self {
        Metadata {
            version: 1u8,
            created,
            last_modified: None,
        }
    }

    /// Updates this metadata after an update
    pub fn updated_at(self, last_modified: DateTime<Utc>) -> Self {
        Metadata {
            version: self.version + 1,
            created: self.created,
            last_modified: Some(last_modified),
        }
    }

    /// The resource version
    pub fn version(&self) -> u8 {
        self.version
    }

    /// The resource creation timestamp
    pub fn created(&self) -> &DateTime<Utc> {
        &self.created
    }

    /// The resource last update timestamp
    pub fn last_modified(&self) -> Option<&DateTime<Utc>> {
        self.last_modified.as_ref()
    }
}

impl Default for Metadata {
    fn default() -> Self {
        let now: DateTime<Utc> = Utc::now();
        Metadata::created_at(now)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod metadata {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_metadata() {
            let now: DateTime<Utc> = Utc::now();
            let metadata = Metadata::created_at(now);
            assert_eq!(1, metadata.version());
            assert_eq!(&now, metadata.created());
            assert_eq!(None, metadata.last_modified());
        }

        #[test]
        fn it_should_update_metadata() {
            let now: DateTime<Utc> = Utc::now();
            let metadata = Metadata::created_at(now).updated_at(now);
            assert_eq!(2, metadata.version());
            assert_eq!(&now, metadata.created());
            assert_eq!(Some(&now), metadata.last_modified());
        }
    }
}
