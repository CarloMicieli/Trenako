use common::slug::Slug;
use std::fmt;
use std::fmt::Formatter;
use std::str;
use std::str::FromStr;

/// It represents the unique identifier for a brand.
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone)]
pub struct BrandId(Slug);

impl BrandId {
    pub fn new(id: &str) -> Self {
        BrandId::from_str(id).expect("invalid brand id")
    }
}

impl fmt::Display for BrandId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl str::FromStr for BrandId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Slug::from_str(s).map(BrandId)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod brand_ids {
        use super::*;
        use pretty_assertions::{assert_eq, assert_ne};

        #[test]
        fn it_should_create_new_brand_ids() {
            let brand_id = BrandId::new("brand name");
            assert_eq!("brand-name", brand_id.to_string());
        }

        #[test]
        fn it_should_check_whether_two_brand_ids_are_equal() {
            let id1 = BrandId::new("brand name");
            let id2 = BrandId::new("brand name");
            let id3 = BrandId::new("another brand name");

            assert_eq!(id1, id1);
            assert_eq!(id1, id2);
            assert_ne!(id1, id3);
        }

        #[test]
        fn it_should_compare_two_brand_ids() {
            let id1 = BrandId::new("brand 1");
            let id2 = BrandId::new("brand 2");

            assert!(id1 < id2);
            assert!(id2 > id1);
        }
    }
}
