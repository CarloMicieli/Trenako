use std::fmt;
use std::fmt::Formatter;
use std::str;
use uuid::Uuid;

/// A unique identifier for a rolling stock
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, Serialize, Deserialize)]
pub struct RollingStockId(Uuid);

impl RollingStockId {
    /// Create a new random rolling stock id
    pub fn new() -> Self {
        RollingStockId::default()
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for RollingStockId {
    fn default() -> Self {
        let id = Uuid::new_v4();
        RollingStockId(id)
    }
}

impl fmt::Display for RollingStockId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

type Error = &'static str;

impl str::FromStr for RollingStockId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = Uuid::try_parse(s).map_err(|_| "invalid rolling stock id")?;
        Ok(RollingStockId(id))
    }
}

impl From<Uuid> for RollingStockId {
    fn from(id: Uuid) -> Self {
        RollingStockId(id)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod rolling_stock_ids {
        use std::str::FromStr;

        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_new_rolling_stock_id_from_str() {
            let id = "3302b9a7-252c-4b41-8de2-eb71efb1888e"
                .parse::<RollingStockId>()
                .unwrap();
            assert_eq!(
                RollingStockId(Uuid::from_str("3302b9a7-252c-4b41-8de2-eb71efb1888e").unwrap()),
                id
            );
        }

        #[test]
        fn it_should_create_new_rolling_stock_id_from_uuid() {
            let uuid = Uuid::new_v4();
            let rolling_stock_id: RollingStockId = uuid.into();
            assert_eq!(uuid, rolling_stock_id.value());
        }

        #[test]
        fn it_should_fail_to_parse_invalid_values_as_rolling_stocks() {
            let result = "invalid value".parse::<RollingStockId>();
            assert!(result.is_err());
        }
    }
}
