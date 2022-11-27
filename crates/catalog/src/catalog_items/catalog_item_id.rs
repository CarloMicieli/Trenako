use common::slug::Slug;
use std::fmt;
use std::fmt::Formatter;
use std::ops::Deref;
use std::str::FromStr;

/// A unique identifier for a catalog item
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CatalogItemId(Slug);

impl CatalogItemId {
    /// Returns the value for this catalog item id
    pub fn value(&self) -> &str {
        self.0.deref()
    }
}

impl fmt::Display for CatalogItemId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl FromStr for CatalogItemId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(())
        } else {
            let value = Slug::new(s);
            Ok(CatalogItemId(value))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod catalog_item_ids {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_display_catalog_item_ids() {
            let id: CatalogItemId = CatalogItemId::from_str("acme-60000").unwrap();
            assert_eq!("acme-60000", id.value());
            assert_eq!("acme-60000", id.to_string());
        }
    }
}
