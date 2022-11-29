use rust_decimal::Decimal;

#[derive(Debug, Deserialize, Clone)]
pub struct CatalogItem {
    pub brand: String,
    #[serde(rename = "itemNumber")]
    pub item_number: String,
    pub description: String,
    #[serde(rename = "powerMethod")]
    pub power_method: String,
    pub scale: String,
    #[serde(rename = "deliveryDate")]
    pub delivery_date: Option<String>,
    pub count: u8,
    #[serde(rename = "rollingStocks")]
    pub rolling_stocks: Vec<RollingStock>,
    pub version: u8,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RollingStock {
    #[serde(rename = "typeName")]
    pub type_name: String,
    #[serde(rename = "roadNumber")]
    pub road_number: Option<String>,
    pub series: Option<String>,
    pub railway: String,
    pub epoch: String,
    #[serde(default)]
    pub category: String,
    #[serde(rename = "subCategory")]
    pub sub_category: Option<String>,
    pub depot: Option<String>,
    pub length: Option<u32>,
    pub livery: Option<String>,
    #[serde(rename = "serviceLevel")]
    pub service_level: Option<String>,
    pub control: Option<String>,
    #[serde(rename = "dccInterface")]
    pub dcc_interface: Option<String>,
    #[serde(rename = "techSpecs")]
    pub tech_specs: TechSpecs,
    pub dummy: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TechSpecs {
    #[serde(rename = "minimumRadius")]
    pub minimum_radius: Decimal,
    #[serde(rename = "flywheelFitted")]
    pub flywheel_fitted: String,
    #[serde(rename = "closeCouplers")]
    pub close_couplers: String,
    #[serde(rename = "metalBody")]
    pub metal_body: String,
    #[serde(rename = "interiorLights")]
    pub interior_lights: String,
    pub lights: String,
    #[serde(rename = "springBuffers")]
    pub spring_buffers: String,
}
