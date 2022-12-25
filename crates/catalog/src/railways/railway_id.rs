use common::slug::Slug;
use std::fmt;
use std::str::FromStr;

/// It represents a unique railway id
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Serialize, Deserialize)]
pub struct RailwayId(Slug);

impl RailwayId {
    pub fn new(id: &str) -> Self {
        let slug = Slug::new(id);
        RailwayId(slug)
    }
}

impl FromStr for RailwayId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(())
        } else {
            Slug::from_str(s).map(RailwayId)
        }
    }
}

impl fmt::Display for RailwayId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod railway_ids {
        use super::*;
        use pretty_assertions::{assert_eq, assert_ne};

        #[test]
        fn it_should_return_an_error_when_brand_id_is_empty() {
            let result = RailwayId::from_str("");
            assert!(result.is_err());
        }

        #[test]
        fn it_should_create_new_railway_ids() {
            let railway_id = RailwayId::new("railway name");
            assert_eq!("railway-name", railway_id.to_string());
        }

        #[test]
        fn it_should_create_new_railway_ids_from_str() {
            let railway_id = RailwayId::from_str("railway name").unwrap();
            assert_eq!("railway-name", railway_id.to_string());
        }

        #[test]
        fn it_should_check_whether_two_railway_ids_are_equals() {
            let id1 = RailwayId::new("railway name");
            let id2 = RailwayId::new("railway name");
            let id3 = RailwayId::new("another railway name");
            assert_eq!(id1, id2);
            assert_ne!(id1, id3);
        }

        #[test]
        fn it_should_check_compare_two_railway_ids() {
            let id1 = RailwayId::new("name 1");
            let id2 = RailwayId::new("name 2");
            assert!(id1 < id2);
            assert!(id2 > id1);
        }
    }
}
