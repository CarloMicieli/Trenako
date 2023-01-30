use crate::common::seeding::{seed_brands, seed_railways, seed_scales};
use crate::common::{create_docker_test, spawn_app, IMAGE_NAME};
use catalog::brands::brand_id::BrandId;
use catalog::catalog_items::availability_status::AvailabilityStatus;
use catalog::catalog_items::catalog_item_id::CatalogItemId;
use catalog::catalog_items::category::{
    Category, ElectricMultipleUnitType, FreightCarType, LocomotiveType, PassengerCarType, RailcarType,
    RollingStockCategory,
};
use catalog::catalog_items::control::{Control, DccInterface};
use catalog::catalog_items::delivery_date::DeliveryDate;
use catalog::catalog_items::power_method::PowerMethod;
use catalog::catalog_items::rolling_stock_id::RollingStockId;
use catalog::catalog_items::service_level::ServiceLevel;
use catalog::catalog_items::technical_specifications::{CouplingSocket, FeatureFlag};
use catalog::railways::railway_id::RailwayId;
use catalog::scales::scale_id::ScaleId;
use rust_decimal::Decimal;
use serde_json::json;
use std::str::FromStr;

pub mod common;

const API_CATALOG_ITEMS: &str = "/api/catalog-items";

#[tokio::test]
async fn post_new_catalog_items() {
    let test = create_docker_test();

    test.run_async(|ops| async move {
        let (_, port) = ops.handle(IMAGE_NAME).host_port(5432).unwrap();

        let sut = spawn_app(*port).await;
        sut.run_database_migrations().await;

        let pg_pool = sut.pg_pool();
        seed_brands(&pg_pool).await;
        seed_railways(&pg_pool).await;
        seed_scales(&pg_pool).await;

        let catalog_item_id = CatalogItemId::from_str("acme-123456").unwrap();
        let expected_location = format!("{API_CATALOG_ITEMS}/{catalog_item_id}");

        let request = json!({
            "brand" : "ACME",
            "item_number" : "123456",
            "category" : "LOCOMOTIVES",
            "scale" : "H0",
            "power_method" : "DC",
            "description" : {
                "it" : "Locomotiva elettrica E 402A 015 nella livrea di origine rosso/bianco, pantografi 52 Sommerfeldt"
            },
            "details" : {
                "it" : "Motore a 5 poli"
            },
            "delivery_date" : "2005",
            "availability_status" : "AVAILABLE",
            "count" : 1,
            "rolling_stocks": [{
                "category" : "LOCOMOTIVE",
                "class_name" : "E402 A",
                "road_number" : "E402 026",
                "series" : "PRIMA SERIE",
                "locomotive_type" : "ELECTRIC_LOCOMOTIVE",
                "railway" : "FS",
                "epoch" : "Vb",
                "livery" : "rosso/bianco",
                "depot" : "Milano Smistamento",
                "dcc_interface" : "MTC_21",
                "control" : "DCC_READY",
                "length_over_buffer" : {
                  "millimeters" : 220.0
                },
                "technical_specifications" : {
                  "coupling" : {
                    "socket" : "NEM_362",
                    "close_couplers" : "YES",
                    "digital_shunting" : "NO"
                  },
                  "flywheel_fitted" : "NO",
                  "metal_body" : "NO",
                  "interior_lights" : "NO",
                  "lights" : "YES",
                  "spring_buffers" : "NO"
                },
                "is_dummy" : false
              }]
        });

        let client = reqwest::Client::new();
        let endpoint = sut.endpoint(API_CATALOG_ITEMS);
        let response = client
            .post(endpoint)
            .json(&request)
            .send()
            .await
            .expect("Failed to execute request.");

        assert!(response.status().is_success());
        assert_eq!(Some(0), response.content_length());
        assert_eq!(expected_location, response.headers()["Location"]);

        let saved = sqlx::query_as!(
            Saved,
            r#"SELECT
                    item_number,
                    brand_id as "brand_id: BrandId",  
                    scale_id as "scale_id: ScaleId", 
                    category as "category: Category",
                    power_method as "power_method: PowerMethod",
                    description_it, 
                    details_it,
                    delivery_date, 
                    availability_status as "availability_status: AvailabilityStatus", 
                    count
                FROM catalog_items WHERE catalog_item_id = $1"#,
            catalog_item_id.clone() as CatalogItemId
        )
        .fetch_one(&pg_pool)
        .await
        .expect("Failed to fetch saved catalog item.");

        assert_eq!(BrandId::new("ACME"), saved.brand_id);
        assert_eq!("123456", saved.item_number);
        assert_eq!(Category::Locomotives, saved.category);
        assert_eq!(ScaleId::new("H0"), saved.scale_id);
        assert_eq!(PowerMethod::DC, saved.power_method);
        assert_eq!(
            Some(String::from(
                "Locomotiva elettrica E 402A 015 nella livrea di origine rosso/bianco, pantografi 52 Sommerfeldt"
            )),
            saved.description_it
        );
        assert_eq!(Some(String::from("Motore a 5 poli")), saved.details_it);
        assert_eq!(Some(DeliveryDate::by_year(2005).to_string()), saved.delivery_date);
        assert_eq!(Some(AvailabilityStatus::Available), saved.availability_status);
        assert_eq!(1, saved.count);

        let saved_rs = sqlx::query_as!(
            SavedRollingStock,
            r#"SELECT 
                  rolling_stock_id as "rolling_stock_id: RollingStockId",
                  railway_id as "railway_id: RailwayId",
                  rolling_stock_category as "rolling_stock_category: RollingStockCategory",
                  epoch,
                  livery,
                  length_over_buffers_mm,
                  length_over_buffers_in,
                  type_name,
                  road_number,
                  series,
                  depot,
                  dcc_interface as "dcc_interface: DccInterface",
                  control as "control: Control",
                  electric_multiple_unit_type as "electric_multiple_unit_type: ElectricMultipleUnitType",
                  freight_car_type as "freight_car_type: FreightCarType",
                  locomotive_type as "locomotive_type: LocomotiveType",
                  passenger_car_type as "passenger_car_type: PassengerCarType",
                  railcar_type as "railcar_type: RailcarType",
                  service_level as "service_level: ServiceLevel",
                  is_dummy,
                  minimum_radius,
                  coupling_socket as "coupling_socket: CouplingSocket",
                  close_couplers as "close_couplers: FeatureFlag",
                  digital_shunting_coupling as "digital_shunting_coupling: FeatureFlag",
                  flywheel_fitted as "flywheel_fitted: FeatureFlag",
                  metal_body as "metal_body: FeatureFlag",
                  interior_lights as "interior_lights: FeatureFlag",
                  lights as "lights: FeatureFlag",
                  spring_buffers as "spring_buffers: FeatureFlag"
                FROM rolling_stocks
                WHERE catalog_item_id = $1"#,
            catalog_item_id as CatalogItemId
        )
        .fetch_one(&pg_pool)
        .await
        .expect("Failed to fetch saved rolling stock.");

        assert_ne!("", saved_rs.rolling_stock_id.to_string());
        assert_eq!(RailwayId::new("FS"), saved_rs.railway_id);
        assert_eq!(RollingStockCategory::Locomotive, saved_rs.rolling_stock_category);
        assert_eq!("Vb", saved_rs.epoch);
        assert_eq!(Some("rosso/bianco".to_string()), saved_rs.livery);
        //assert_eq!(Some(dec!(220)), saved_rs.length_over_buffers_mm);
        //assert_eq!(Some(dec!(8.66142)), saved_rs.length_over_buffers_in);
        assert_eq!(None, saved_rs.length_over_buffers_mm);
        assert_eq!(None, saved_rs.length_over_buffers_in);
        assert_eq!(Some("E402 A".to_string()), saved_rs.type_name);
        assert_eq!(Some("E402 026".to_string()), saved_rs.road_number);
        assert_eq!(Some("PRIMA SERIE".to_string()), saved_rs.series);
        assert_eq!(Some("Milano Smistamento".to_string()), saved_rs.depot);
        assert_eq!(Some(DccInterface::Mtc21), saved_rs.dcc_interface);
        assert_eq!(Some(Control::DccReady), saved_rs.control);
        assert_eq!(None, saved_rs.electric_multiple_unit_type);
        assert_eq!(None, saved_rs.freight_car_type);
        assert_eq!(Some(LocomotiveType::ElectricLocomotive), saved_rs.locomotive_type);
        assert_eq!(None, saved_rs.passenger_car_type);
        assert_eq!(None, saved_rs.railcar_type);
        assert_eq!(None, saved_rs.service_level);
        assert_eq!(Some(false), saved_rs.is_dummy);
        assert_eq!(None, saved_rs.minimum_radius);
        assert_eq!(Some(CouplingSocket::Nem362), saved_rs.coupling_socket);
        assert_eq!(Some(FeatureFlag::Yes), saved_rs.close_couplers);
        assert_eq!(Some(FeatureFlag::No), saved_rs.digital_shunting_coupling);
        assert_eq!(Some(FeatureFlag::No), saved_rs.flywheel_fitted);
        assert_eq!(Some(FeatureFlag::No), saved_rs.metal_body);
        assert_eq!(Some(FeatureFlag::No), saved_rs.interior_lights);
        assert_eq!(Some(FeatureFlag::Yes), saved_rs.lights);
        assert_eq!(Some(FeatureFlag::No), saved_rs.spring_buffers);
    })
    .await
}

#[derive(Debug)]
struct Saved {
    brand_id: BrandId,
    item_number: String,
    category: Category,
    scale_id: ScaleId,
    power_method: PowerMethod,
    description_it: Option<String>,
    details_it: Option<String>,
    delivery_date: Option<String>,
    availability_status: Option<AvailabilityStatus>,
    count: i32,
}

#[derive(Debug)]
struct SavedRollingStock {
    rolling_stock_id: RollingStockId,
    railway_id: RailwayId,
    rolling_stock_category: RollingStockCategory,
    epoch: String,
    livery: Option<String>,
    length_over_buffers_mm: Option<Decimal>,
    length_over_buffers_in: Option<Decimal>,
    type_name: Option<String>,
    road_number: Option<String>,
    series: Option<String>,
    depot: Option<String>,
    dcc_interface: Option<DccInterface>,
    control: Option<Control>,
    electric_multiple_unit_type: Option<ElectricMultipleUnitType>,
    freight_car_type: Option<FreightCarType>,
    locomotive_type: Option<LocomotiveType>,
    passenger_car_type: Option<PassengerCarType>,
    railcar_type: Option<RailcarType>,
    service_level: Option<ServiceLevel>,
    is_dummy: Option<bool>,
    minimum_radius: Option<Decimal>,
    coupling_socket: Option<CouplingSocket>,
    close_couplers: Option<FeatureFlag>,
    digital_shunting_coupling: Option<FeatureFlag>,
    flywheel_fitted: Option<FeatureFlag>,
    metal_body: Option<FeatureFlag>,
    interior_lights: Option<FeatureFlag>,
    lights: Option<FeatureFlag>,
    spring_buffers: Option<FeatureFlag>,
}
