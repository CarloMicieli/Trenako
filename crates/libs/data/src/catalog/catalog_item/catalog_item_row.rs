//! the catalog items row definition

use catalog::brands::brand_id::BrandId;
use catalog::catalog_items::availability_status::AvailabilityStatus;
use catalog::catalog_items::catalog_item_id::CatalogItemId;
use catalog::catalog_items::category::Category;
use catalog::catalog_items::power_method::PowerMethod;
use catalog::scales::scale_id::ScaleId;
use chrono::{DateTime, Utc};

/// It represents the catalog item row definition
#[derive(Debug)]
pub struct CatalogItemRow {
    pub catalog_item_id: CatalogItemId,
    pub brand_id: BrandId,
    pub brand_display: String,
    pub item_number: String,
    pub category: Category,
    pub scale_id: ScaleId,
    pub scale_display: String,
    pub power_method: PowerMethod,
    pub epoch: String,
    pub description_en: Option<String>,
    pub description_it: Option<String>,
    pub details_en: Option<String>,
    pub details_it: Option<String>,
    pub delivery_date: Option<String>,
    pub availability_status: Option<AvailabilityStatus>,
    pub count: i32,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub last_modified_at: Option<DateTime<Utc>>,
}

#[cfg(test)]
pub mod test {
    use super::*;
    use catalog::catalog_items::epoch::Epoch;
    use catalog::catalog_items::item_number::ItemNumber;

    #[allow(dead_code)]
    pub fn new_catalog_item_row(
        brand: &str,
        item_number: &str,
        scale: &str,
        created_at: DateTime<Utc>,
    ) -> CatalogItemRow {
        let item_number = ItemNumber::new(item_number);
        CatalogItemRow {
            catalog_item_id: CatalogItemId::of(&BrandId::new(brand), &item_number),
            brand_id: BrandId::new(brand),
            brand_display: String::from(brand),
            item_number: item_number.value().to_string(),
            category: Category::Locomotives,
            scale_id: ScaleId::new(scale),
            scale_display: String::from(scale),
            power_method: PowerMethod::DC,
            epoch: Epoch::III.to_string(),
            description_en: None,
            description_it: None,
            details_en: None,
            details_it: None,
            delivery_date: None,
            availability_status: None,
            count: 1,
            version: 0,
            created_at,
            last_modified_at: None,
        }
    }
}
