use common::slug::Slug;
use sqlx::Type;
use std::fmt;
use std::ops;
use std::str;
use std::str::FromStr;

/// It represents a unique scale id
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Serialize, Deserialize, Type)]
#[sqlx(transparent)]
pub struct ScaleId(Slug);

impl ScaleId {
    /// Creates a new unique identifier for a modelling scale
    pub fn new(id: &str) -> Self {
        ScaleId::from_str(id).expect("invalid scale id")
    }
}

impl str::FromStr for ScaleId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Slug::from_str(s).map(ScaleId)
    }
}

impl fmt::Display for ScaleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ops::Deref for ScaleId {
    type Target = Slug;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod scale_ids {
        use super::*;
        use pretty_assertions::assert_eq;
        use std::str::FromStr;

        #[test]
        fn it_should_return_an_error_when_the_scale_id_is_empty() {
            let result = ScaleId::from_str("");
            assert!(result.is_err());
        }

        #[test]
        fn it_should_create_new_scale_ids() {
            let scale_id = ScaleId::new("scale name");
            assert_eq!("scale-name", scale_id.to_string());
        }

        #[test]
        fn it_should_create_new_scale_ids_from_str() {
            let scale_id = ScaleId::from_str("scale name").unwrap();
            assert_eq!("scale-name", scale_id.to_string());
        }

        #[test]
        fn it_should_check_whether_two_scale_ids_are_equal() {
            let id1 = ScaleId::new("scale name");
            let id2 = ScaleId::new("scale name");
            let id3 = ScaleId::new("another scale name");

            assert_eq!(id1, id1);
            assert_eq!(id1, id2);
            assert_ne!(id1, id3);
        }

        #[test]
        fn it_should_compare_two_scale_ids() {
            let id1 = ScaleId::new("scale 1");
            let id2 = ScaleId::new("scale 2");

            assert!(id1 < id2);
            assert!(id2 > id1);
        }
    }
}
