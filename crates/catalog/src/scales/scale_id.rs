use common::slug::Slug;
use std::fmt;

/// It represents a unique scale id
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone)]
pub struct ScaleId(Slug);

impl ScaleId {
    pub fn new(id: &str) -> Self {
        let slug = Slug::new(id);
        ScaleId(slug)
    }
}

impl fmt::Display for ScaleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod scale_ids {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_new_brand_ids() {
            let brand_id = ScaleId::new("scale name");
            assert_eq!("scale-name", brand_id.to_string());
        }
    }
}
