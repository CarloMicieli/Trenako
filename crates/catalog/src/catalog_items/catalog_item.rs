use crate::brands::brand_id::BrandId;
use crate::catalog_items::catalog_item_id::CatalogItemId;
use crate::catalog_items::category::Category;
use crate::catalog_items::delivery_date::DeliveryDate;
use crate::catalog_items::item_number::ItemNumber;
use crate::catalog_items::power_method::PowerMethod;
use crate::catalog_items::rolling_stock::RollingStock;
use crate::scales::scale_id::ScaleId;
use common::metadata::Metadata;
use std::cmp;
use std::fmt;
use std::fmt::Formatter;

/// A catalog item, it can contain one or more rolling stock.
///
/// A catalog item is identified by its catalog item number.
#[derive(Debug, Clone)]
pub struct CatalogItem {
    catalog_item_id: CatalogItemId,
    brand: Brand,
    item_number: ItemNumber,
    category: Category,
    description: Option<String>,
    details: Option<String>,
    scale: Scale,
    power_method: PowerMethod,
    rolling_stocks: Vec<RollingStock>,
    delivery_date: Option<DeliveryDate>,
    count: u8,
    metadata: Metadata,
}

impl PartialEq for CatalogItem {
    fn eq(&self, other: &Self) -> bool {
        self.brand == other.brand && self.item_number == other.item_number
    }
}

impl Eq for CatalogItem {}

impl Ord for CatalogItem {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let cmp1 = self.brand().cmp(other.brand());
        if cmp1 == cmp::Ordering::Equal {
            return self.item_number.cmp(&other.item_number);
        }

        cmp1
    }
}

impl PartialOrd for CatalogItem {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl CatalogItem {
    /// Creates a new catalog item
    pub fn new(
        catalog_item_id: CatalogItemId,
        brand: Brand,
        item_number: ItemNumber,
        category: Category,
        scale: Scale,
        description: Option<&str>,
        details: Option<&str>,
        rolling_stocks: Vec<RollingStock>,
        power_method: PowerMethod,
        delivery_date: Option<DeliveryDate>,
        count: u8,
        metadata: Metadata,
    ) -> Self {
        CatalogItem {
            catalog_item_id,
            brand,
            item_number,
            category,
            description: description.map(str::to_string),
            details: details.map(str::to_string),
            scale,
            power_method,
            rolling_stocks,
            delivery_date,
            count,
            metadata,
        }
    }

    pub fn id(&self) -> &CatalogItemId {
        &self.catalog_item_id
    }

    /// Return the Brand for this catalog item.
    pub fn brand(&self) -> &Brand {
        &self.brand
    }

    /// Return the item number as in the corresponding brand catalog.
    pub fn item_number(&self) -> &ItemNumber {
        &self.item_number
    }

    pub fn rolling_stocks(&self) -> &Vec<RollingStock> {
        &self.rolling_stocks
    }

    /// Returns the category for this catalog item
    pub fn category(&self) -> Category {
        self.category
    }

    /// Returns the rolling stocks count for this catalog item
    pub fn count(&self) -> u8 {
        self.count
    }

    /// Returns the description for this catalog item
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns the details for this catalog item
    pub fn details(&self) -> Option<&str> {
        self.details.as_deref()
    }

    /// Returns the scale for this catalog item
    pub fn scale(&self) -> &Scale {
        &self.scale
    }

    /// Returns the power method for this catalog item
    pub fn power_method(&self) -> PowerMethod {
        self.power_method
    }

    /// Returns the delivery date for this catalog item
    pub fn delivery_date(&self) -> Option<&DeliveryDate> {
        self.delivery_date.as_ref()
    }

    /// Returns the metadata for this catalog item
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

/// A model railways manufacturer.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Brand {
    brand_id: BrandId,
    name: String,
}

impl Brand {
    /// Creates a new brand with the given name.
    pub fn new(brand_id: BrandId, name: &str) -> Self {
        Brand {
            brand_id,
            name: name.to_owned(),
        }
    }

    /// Returns this brand unique identifier
    pub fn id(&self) -> &BrandId {
        &self.brand_id
    }

    /// Returns this brand name
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for Brand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.name)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Scale {
    scale_id: ScaleId,
    name: String,
}

impl Scale {
    /// Creates a new Scale with the given name.
    pub fn new(scale_id: ScaleId, name: &str) -> Self {
        Scale {
            scale_id,
            name: name.to_owned(),
        }
    }

    /// Returns this brand unique identifier
    pub fn id(&self) -> &ScaleId {
        &self.scale_id
    }

    /// Returns this brand name
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod brands {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_new_brands() {
            let b = Brand::new(BrandId::new("ACME"), "ACME");
            assert_eq!(&BrandId::new("ACME"), b.id());
            assert_eq!("ACME", b.name());
        }

        #[test]
        fn it_should_display_brand_as_string() {
            let b = Brand::new(BrandId::new("ACME"), "ACME");
            assert_eq!("ACME", b.to_string());
        }
    }

    mod scales {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_new_scales() {
            let s = Scale::new(ScaleId::new("H0"), "H0");
            assert_eq!(&ScaleId::new("H0"), s.id());
            assert_eq!("H0", s.name());
        }

        #[test]
        fn it_should_display_scale_as_string() {
            let s = Scale::new(ScaleId::new("H0"), "H0");
            assert_eq!("H0", s.to_string());
        }
    }

    mod catalog_items {
        use super::*;
        use chrono::{DateTime, Utc};
        use pretty_assertions::assert_eq;
        use std::str::FromStr;

        #[test]
        fn it_should_create_new_catalog_items() {
            let id = CatalogItemId::from_str("acme_123456").unwrap();
            let acme = Brand::new(BrandId::new("ACME"), "ACME");
            let half_zero = Scale::new(ScaleId::new("H0"), "H0");
            let item_number = ItemNumber::new("123456").unwrap();
            let now: DateTime<Utc> = Utc::now();

            let catalog_item = CatalogItem::new(
                id.clone(),
                acme.clone(),
                item_number.clone(),
                Category::Locomotives,
                half_zero.clone(),
                Some("test description"),
                Some("test details"),
                Vec::new(),
                PowerMethod::DC,
                Some(DeliveryDate::ByYear(2000)),
                1,
                Metadata::created_at(now),
            );

            assert_eq!(&id, catalog_item.id());
            assert_eq!(&acme, catalog_item.brand());
            assert_eq!(&item_number, catalog_item.item_number());
            assert_eq!(Category::Locomotives, catalog_item.category());
            assert_eq!(Some("test description"), catalog_item.description());
            assert_eq!(Some("test details"), catalog_item.details());
            assert_eq!(PowerMethod::DC, catalog_item.power_method());
            assert_eq!(&half_zero, catalog_item.scale());
            assert_eq!(Some(&DeliveryDate::ByYear(2000)), catalog_item.delivery_date());
            assert_eq!(1, catalog_item.count());
            assert_eq!(&Metadata::created_at(now), catalog_item.metadata());
        }
    }
}
