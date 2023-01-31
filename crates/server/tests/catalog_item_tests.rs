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
use rust_decimal_macros::dec;
use serde_json::json;
use sqlx::PgPool;
use std::str::FromStr;

pub mod common;

const API_CATALOG_ITEMS: &str = "/api/catalog-items";

#[tokio::test]
async fn it_should_create_a_new_locomotive() {
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
                "length_over_buffers" : {
                  "millimeters" : 220.0,
                  "inches": 8.66142
                },
                "technical_specifications" : {
                  "coupling" : {
                    "socket" : "NEM_362",
                    "close_couplers" : "YES",
                    "digital_shunting" : "NO"
                  },
                  "flywheel_fitted" : "NO",
                  "metal_body" : "NO",
                  "minimum_radius": 360.0,
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

        let saved = fetch_saved_catalog_item(catalog_item_id.clone(), &pg_pool).await;
        let item = saved.catalog_item;

        assert_eq!(BrandId::new("ACME"), item.brand_id);
        assert_eq!("123456", item.item_number);
        assert_eq!(Category::Locomotives, item.category);
        assert_eq!(ScaleId::new("H0"), item.scale_id);
        assert_eq!(PowerMethod::DC, item.power_method);
        assert_eq!(
            Some(String::from(
                "Locomotiva elettrica E 402A 015 nella livrea di origine rosso/bianco, pantografi 52 Sommerfeldt"
            )),
            item.description_it
        );
        assert_eq!(Some(String::from("Motore a 5 poli")), item.details_it);
        assert_eq!(Some(DeliveryDate::by_year(2005).to_string()), item.delivery_date);
        assert_eq!(Some(AvailabilityStatus::Available), item.availability_status);
        assert_eq!(1, item.count);

        let rs = saved.rolling_stocks.get(0).expect("no saved rolling stock found");

        assert_ne!("", rs.rolling_stock_id.to_string());
        assert_eq!(RailwayId::new("FS"), rs.railway_id);
        assert_eq!(RollingStockCategory::Locomotive, rs.rolling_stock_category);
        assert_eq!("Vb", rs.epoch);
        assert_eq!(Some("rosso/bianco".to_string()), rs.livery);
        assert_eq!(Some(dec!(220)), rs.length_over_buffers_mm);
        assert_eq!(Some(dec!(8.66142)), rs.length_over_buffers_in);
        assert_eq!(Some("E402 A".to_string()), rs.type_name);
        assert_eq!(Some("E402 026".to_string()), rs.road_number);
        assert_eq!(Some("PRIMA SERIE".to_string()), rs.series);
        assert_eq!(Some("Milano Smistamento".to_string()), rs.depot);
        assert_eq!(Some(DccInterface::Mtc21), rs.dcc_interface);
        assert_eq!(Some(Control::DccReady), rs.control);
        assert_eq!(None, rs.electric_multiple_unit_type);
        assert_eq!(None, rs.freight_car_type);
        assert_eq!(Some(LocomotiveType::ElectricLocomotive), rs.locomotive_type);
        assert_eq!(None, rs.passenger_car_type);
        assert_eq!(None, rs.railcar_type);
        assert_eq!(None, rs.service_level);
        assert_eq!(Some(false), rs.is_dummy);
        assert_eq!(Some(dec!(360.0)), rs.minimum_radius);
        assert_eq!(Some(CouplingSocket::Nem362), rs.coupling_socket);
        assert_eq!(Some(FeatureFlag::Yes), rs.close_couplers);
        assert_eq!(Some(FeatureFlag::No), rs.digital_shunting_coupling);
        assert_eq!(Some(FeatureFlag::No), rs.flywheel_fitted);
        assert_eq!(Some(FeatureFlag::No), rs.metal_body);
        assert_eq!(Some(FeatureFlag::No), rs.interior_lights);
        assert_eq!(Some(FeatureFlag::Yes), rs.lights);
        assert_eq!(Some(FeatureFlag::No), rs.spring_buffers);
    })
    .await
}

#[tokio::test]
async fn it_should_create_a_new_electric_multiple_unit() {
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
            "category" : "ELECTRIC_MULTIPLE_UNITS",
            "scale" : "H0",
            "power_method" : "AC",
            "description" : {
                "it" : "Elettromotrice Ale.540 013 e rimorchiata Le.760 003"
            },
            "details" : {
                "it" : "il modello è dotato di illuminazione interna di serie"
            },
            "delivery_date" : "2005",
            "availability_status" : "AVAILABLE",
            "count" : 2,
            "rolling_stocks": [{
                "category" : "ELECTRIC_MULTIPLE_UNIT",
                "type_name" : "ALe 540",
                "road_number" : "ALe 540 013",
                "series" : "SECONDA SERIE",
                "electric_multiple_unit_type" : "POWER_CAR",
                "railway" : "FS",
                "epoch" : "IVa",
                "livery" : "castano/isabella",
                "depot" : "Milano Smistamento",
                "dcc_interface" : "PLUX_22",
                "control" : "DCC_READY",
                "length_over_buffers" : {
                  "millimeters" : 310.0,
                },
                "technical_specifications" : {
                  "coupling" : {
                    "socket" : "NEM_362",
                    "close_couplers" : "YES",
                    "digital_shunting" : "NO"
                  },
                  "flywheel_fitted" : "NO",
                  "metal_body" : "NO",
                  "minimum_radius": 360.0,
                  "interior_lights" : "YES",
                  "lights" : "YES",
                  "spring_buffers" : "NO"
                },
                "is_dummy" : false
              },
            {
                "category" : "ELECTRIC_MULTIPLE_UNIT",
                "type_name" : "Le 760",
                "road_number" : "Le 760 010",
                "series" : "SECONDA SERIE",
                "electric_multiple_unit_type" : "TRAILER_CAR",
                "railway" : "FS",
                "epoch" : "IVa",
                "livery" : "castano/isabella",
                "control" : "NO_DCC",
                "length_over_buffers" : {
                  "millimeters" : 310.0,
                },
                "technical_specifications" : {
                  "coupling" : {
                    "socket" : "NEM_362",
                    "close_couplers" : "YES",
                    "digital_shunting" : "NO"
                  },
                  "flywheel_fitted" : "NO",
                  "metal_body" : "NO",
                  "minimum_radius": 360.0,
                  "interior_lights" : "YES",
                  "lights" : "YES",
                  "spring_buffers" : "NO"
                },
                "is_dummy" : true
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

        let saved = fetch_saved_catalog_item(catalog_item_id.clone(), &pg_pool).await;
        let item = saved.catalog_item;

        assert_eq!(BrandId::new("ACME"), item.brand_id);
        assert_eq!("123456", item.item_number);
        assert_eq!(Category::ElectricMultipleUnits, item.category);
        assert_eq!(ScaleId::new("H0"), item.scale_id);
        assert_eq!(PowerMethod::AC, item.power_method);
        assert_eq!(
            Some(String::from("Elettromotrice Ale.540 013 e rimorchiata Le.760 003")),
            item.description_it
        );
        assert_eq!(
            Some(String::from("il modello è dotato di illuminazione interna di serie")),
            item.details_it
        );
        assert_eq!(Some(DeliveryDate::by_year(2005).to_string()), item.delivery_date);
        assert_eq!(Some(AvailabilityStatus::Available), item.availability_status);
        assert_eq!(2, item.count);

        assert_eq!(2, saved.rolling_stocks.len());

        let rs1 = saved.rolling_stocks.get(0).unwrap();

        assert_ne!("", rs1.rolling_stock_id.to_string());
        assert_eq!(RailwayId::new("FS"), rs1.railway_id);
        assert_eq!(RollingStockCategory::ElectricMultipleUnit, rs1.rolling_stock_category);
        assert_eq!("IVa", rs1.epoch);
        assert_eq!(Some("castano/isabella".to_string()), rs1.livery);
        assert_eq!(Some(dec!(310)), rs1.length_over_buffers_mm);
        assert_eq!(None, rs1.length_over_buffers_in);
        assert_eq!(Some("ALe 540".to_string()), rs1.type_name);
        assert_eq!(Some("ALe 540 013".to_string()), rs1.road_number);
        assert_eq!(Some("SECONDA SERIE".to_string()), rs1.series);
        assert_eq!(Some("Milano Smistamento".to_string()), rs1.depot);
        assert_eq!(Some(DccInterface::Plux22), rs1.dcc_interface);
        assert_eq!(Some(Control::DccReady), rs1.control);
        assert_eq!(
            Some(ElectricMultipleUnitType::PowerCar),
            rs1.electric_multiple_unit_type
        );
        assert_eq!(None, rs1.freight_car_type);
        assert_eq!(None, rs1.locomotive_type);
        assert_eq!(None, rs1.passenger_car_type);
        assert_eq!(None, rs1.railcar_type);
        assert_eq!(None, rs1.service_level);
        assert_eq!(Some(false), rs1.is_dummy);
        assert_eq!(Some(dec!(360.0)), rs1.minimum_radius);
        assert_eq!(Some(CouplingSocket::Nem362), rs1.coupling_socket);
        assert_eq!(Some(FeatureFlag::Yes), rs1.close_couplers);
        assert_eq!(Some(FeatureFlag::No), rs1.digital_shunting_coupling);
        assert_eq!(Some(FeatureFlag::No), rs1.flywheel_fitted);
        assert_eq!(Some(FeatureFlag::No), rs1.metal_body);
        assert_eq!(Some(FeatureFlag::Yes), rs1.interior_lights);
        assert_eq!(Some(FeatureFlag::Yes), rs1.lights);
        assert_eq!(Some(FeatureFlag::No), rs1.spring_buffers);

        let rs2 = saved.rolling_stocks.get(1).unwrap();

        assert_ne!("", rs2.rolling_stock_id.to_string());
        assert_eq!(RailwayId::new("FS"), rs2.railway_id);
        assert_eq!(RollingStockCategory::ElectricMultipleUnit, rs2.rolling_stock_category);
        assert_eq!("IVa", rs2.epoch);
        assert_eq!(Some("castano/isabella".to_string()), rs2.livery);
        assert_eq!(Some(dec!(310)), rs2.length_over_buffers_mm);
        assert_eq!(None, rs2.length_over_buffers_in);
        assert_eq!(Some("Le 760".to_string()), rs2.type_name);
        assert_eq!(Some("Le 760 010".to_string()), rs2.road_number);
        assert_eq!(Some("SECONDA SERIE".to_string()), rs2.series);
        assert_eq!(None, rs2.depot);
        assert_eq!(None, rs2.dcc_interface);
        assert_eq!(Some(Control::NoDcc), rs2.control);
        assert_eq!(
            Some(ElectricMultipleUnitType::TrailerCar),
            rs2.electric_multiple_unit_type
        );
        assert_eq!(None, rs2.freight_car_type);
        assert_eq!(None, rs2.locomotive_type);
        assert_eq!(None, rs2.passenger_car_type);
        assert_eq!(None, rs2.railcar_type);
        assert_eq!(None, rs2.service_level);
        assert_eq!(Some(true), rs2.is_dummy);
        assert_eq!(Some(dec!(360.0)), rs2.minimum_radius);
        assert_eq!(Some(CouplingSocket::Nem362), rs2.coupling_socket);
        assert_eq!(Some(FeatureFlag::Yes), rs2.close_couplers);
        assert_eq!(Some(FeatureFlag::No), rs2.digital_shunting_coupling);
        assert_eq!(Some(FeatureFlag::No), rs2.flywheel_fitted);
        assert_eq!(Some(FeatureFlag::No), rs2.metal_body);
        assert_eq!(Some(FeatureFlag::Yes), rs2.interior_lights);
        assert_eq!(Some(FeatureFlag::Yes), rs2.lights);
        assert_eq!(Some(FeatureFlag::No), rs2.spring_buffers);
    })
    .await
}

#[tokio::test]
async fn it_should_create_a_new_railcar() {
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
            "category" : "RAILCARS",
            "scale" : "H0",
            "power_method" : "DC",
            "description" : {
                "it" : "Automotrice FS ALn 668"
            },
            "details" : {
                "it" : "Verde lichene/giallo coloniale con mantice frontale, motorizzata + folle"
            },
            "delivery_date" : "2008",
            "availability_status" : "AVAILABLE",
            "count" : 2,
            "rolling_stocks": [{
                "category" : "RAILCAR",
                "type_name" : "ALn 668",
                "road_number" : "ALn 668 1449",
                "series" : "SERIE 1400",
                "railcar_type" : "POWER_CAR",
                "railway" : "FS",
                "epoch" : "IIIb",
                "livery" : "verde lichene/giallo coloniale",
                "dcc_interface" : "PLUX_22",
                "control" : "DCC_READY",
                "length_over_buffers" : {
                  "millimeters" : 310.0,
                },
                "technical_specifications" : {
                  "coupling" : {
                    "socket" : "NEM_362",
                    "close_couplers" : "YES",
                    "digital_shunting" : "NO"
                  },
                  "flywheel_fitted" : "NO",
                  "metal_body" : "NO",
                  "minimum_radius": 360.0,
                  "interior_lights" : "NO",
                  "lights" : "YES",
                  "spring_buffers" : "NO"
                },
                "is_dummy" : false
              },
              {
                "category" : "RAILCAR",
                "type_name" : "ALn 668",
                "road_number" : "ALn 668 1456",
                "series" : "SERIE 1400",
                "railcar_type" : "POWER_CAR",
                "railway" : "FS",
                "epoch" : "IIIb",
                "livery" : "verde lichene/giallo coloniale",
                "control" : "NO_DCC",
                "length_over_buffers" : {
                  "millimeters" : 310.0,
                },
                "technical_specifications" : {
                  "coupling" : {
                    "socket" : "NEM_362",
                    "close_couplers" : "YES",
                    "digital_shunting" : "NO"
                  },
                  "flywheel_fitted" : "NO",
                  "metal_body" : "NO",
                  "minimum_radius": 360.0,
                  "interior_lights" : "NO",
                  "lights" : "YES",
                  "spring_buffers" : "NO"
                },
                "is_dummy" : true
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

        let saved = fetch_saved_catalog_item(catalog_item_id.clone(), &pg_pool).await;
        let item = saved.catalog_item;

        assert_eq!(BrandId::new("ACME"), item.brand_id);
        assert_eq!("123456", item.item_number);
        assert_eq!(Category::Railcars, item.category);
        assert_eq!(ScaleId::new("H0"), item.scale_id);
        assert_eq!(PowerMethod::DC, item.power_method);
        assert_eq!(Some(String::from("Automotrice FS ALn 668")), item.description_it);
        assert_eq!(
            Some(String::from(
                "Verde lichene/giallo coloniale con mantice frontale, motorizzata + folle"
            )),
            item.details_it
        );
        assert_eq!(Some(DeliveryDate::by_year(2008).to_string()), item.delivery_date);
        assert_eq!(Some(AvailabilityStatus::Available), item.availability_status);
        assert_eq!(2, item.count);

        assert_eq!(2, saved.rolling_stocks.len());

        let rs1 = saved.rolling_stocks.get(0).unwrap();

        assert_ne!("", rs1.rolling_stock_id.to_string());
        assert_eq!(RailwayId::new("FS"), rs1.railway_id);
        assert_eq!(RollingStockCategory::Railcar, rs1.rolling_stock_category);
        assert_eq!("IIIb", rs1.epoch);
        assert_eq!(Some("verde lichene/giallo coloniale".to_string()), rs1.livery);
        assert_eq!(Some(dec!(310)), rs1.length_over_buffers_mm);
        assert_eq!(None, rs1.length_over_buffers_in);
        assert_eq!(Some("ALn 668".to_string()), rs1.type_name);
        assert_eq!(Some("ALn 668 1449".to_string()), rs1.road_number);
        assert_eq!(Some("SERIE 1400".to_string()), rs1.series);
        assert_eq!(None, rs1.depot);
        assert_eq!(Some(DccInterface::Plux22), rs1.dcc_interface);
        assert_eq!(Some(Control::DccReady), rs1.control);
        assert_eq!(None, rs1.electric_multiple_unit_type);
        assert_eq!(None, rs1.freight_car_type);
        assert_eq!(None, rs1.locomotive_type);
        assert_eq!(None, rs1.passenger_car_type);
        assert_eq!(Some(RailcarType::PowerCar), rs1.railcar_type);
        assert_eq!(None, rs1.service_level);
        assert_eq!(Some(false), rs1.is_dummy);
        assert_eq!(Some(dec!(360.0)), rs1.minimum_radius);
        assert_eq!(Some(CouplingSocket::Nem362), rs1.coupling_socket);
        assert_eq!(Some(FeatureFlag::Yes), rs1.close_couplers);
        assert_eq!(Some(FeatureFlag::No), rs1.digital_shunting_coupling);
        assert_eq!(Some(FeatureFlag::No), rs1.flywheel_fitted);
        assert_eq!(Some(FeatureFlag::No), rs1.metal_body);
        assert_eq!(Some(FeatureFlag::No), rs1.interior_lights);
        assert_eq!(Some(FeatureFlag::Yes), rs1.lights);
        assert_eq!(Some(FeatureFlag::No), rs1.spring_buffers);

        let rs2 = saved.rolling_stocks.get(1).unwrap();

        assert_ne!("", rs2.rolling_stock_id.to_string());
        assert_eq!(RailwayId::new("FS"), rs2.railway_id);
        assert_eq!(RollingStockCategory::Railcar, rs2.rolling_stock_category);
        assert_eq!("IIIb", rs2.epoch);
        assert_eq!(Some("verde lichene/giallo coloniale".to_string()), rs2.livery);
        assert_eq!(Some(dec!(310)), rs2.length_over_buffers_mm);
        assert_eq!(None, rs2.length_over_buffers_in);
        assert_eq!(Some("ALn 668".to_string()), rs2.type_name);
        assert_eq!(Some("ALn 668 1456".to_string()), rs2.road_number);
        assert_eq!(Some("SERIE 1400".to_string()), rs2.series);
        assert_eq!(None, rs2.depot);
        assert_eq!(None, rs2.dcc_interface);
        assert_eq!(Some(Control::NoDcc), rs2.control);
        assert_eq!(None, rs2.electric_multiple_unit_type);
        assert_eq!(None, rs2.freight_car_type);
        assert_eq!(None, rs2.locomotive_type);
        assert_eq!(None, rs2.passenger_car_type);
        assert_eq!(Some(RailcarType::PowerCar), rs2.railcar_type);
        assert_eq!(None, rs2.service_level);
        assert_eq!(Some(true), rs2.is_dummy);
        assert_eq!(Some(dec!(360.0)), rs2.minimum_radius);
        assert_eq!(Some(CouplingSocket::Nem362), rs2.coupling_socket);
        assert_eq!(Some(FeatureFlag::Yes), rs2.close_couplers);
        assert_eq!(Some(FeatureFlag::No), rs2.digital_shunting_coupling);
        assert_eq!(Some(FeatureFlag::No), rs2.flywheel_fitted);
        assert_eq!(Some(FeatureFlag::No), rs2.metal_body);
        assert_eq!(Some(FeatureFlag::No), rs2.interior_lights);
        assert_eq!(Some(FeatureFlag::Yes), rs2.lights);
        assert_eq!(Some(FeatureFlag::No), rs2.spring_buffers);
    })
    .await
}

#[tokio::test]
async fn it_should_create_a_new_passenger_car() {
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
            "category" : "PASSENGER_CARS",
            "scale" : "H0",
            "power_method" : "DC",
            "description" : {
                "it" : "Carrozza passeggeri"
            },
            "details" : {
                "it" : "porte dorate, carrelli MD50"
            },
            "delivery_date" : "2005",
            "availability_status" : "ANNOUNCED",
            "count" : 1,
            "rolling_stocks": [{
                "category" : "PASSENGER_CAR",
                "type_name" : "UIC-X",
                "road_number" : "50 83 10-88 076-2 A",
                "series" : "TIPO 1964",
                "passenger_car_type" : "COMPARTMENT_COACH",
                "railway" : "FS",
                "epoch" : "IVb/V",
                "livery" : "rosso fegato/grigio beige",
                "length_over_buffers" : {
                  "millimeters" : 303.0
                },
                "service_level" : "FIRST_CLASS",
                "technical_specifications" : {
                  "coupling" : {
                    "socket" : "NEM_362",
                    "close_couplers" : "YES",
                    "digital_shunting" : "NO"
                  },
                  "flywheel_fitted" : "NO",
                  "metal_body" : "NO",
                  "minimum_radius": 360.0,
                  "interior_lights" : "NO",
                  "lights" : "NO",
                  "spring_buffers" : "NO"
                }
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

        let saved = fetch_saved_catalog_item(catalog_item_id.clone(), &pg_pool).await;
        let item = saved.catalog_item;

        assert_eq!(BrandId::new("ACME"), item.brand_id);
        assert_eq!("123456", item.item_number);
        assert_eq!(Category::PassengerCars, item.category);
        assert_eq!(ScaleId::new("H0"), item.scale_id);
        assert_eq!(PowerMethod::DC, item.power_method);
        assert_eq!(Some(String::from("Carrozza passeggeri")), item.description_it);
        assert_eq!(Some(String::from("porte dorate, carrelli MD50")), item.details_it);
        assert_eq!(Some(DeliveryDate::by_year(2005).to_string()), item.delivery_date);
        assert_eq!(Some(AvailabilityStatus::Announced), item.availability_status);
        assert_eq!(1, item.count);

        let rs = saved.rolling_stocks.get(0).expect("no saved rolling stock found");

        assert_ne!("", rs.rolling_stock_id.to_string());
        assert_eq!(RailwayId::new("FS"), rs.railway_id);
        assert_eq!(RollingStockCategory::PassengerCar, rs.rolling_stock_category);
        assert_eq!("IVb/V", rs.epoch);
        assert_eq!(Some("rosso fegato/grigio beige".to_string()), rs.livery);
        assert_eq!(Some(dec!(303)), rs.length_over_buffers_mm);
        assert_eq!(None, rs.length_over_buffers_in);
        assert_eq!(Some("UIC-X".to_string()), rs.type_name);
        assert_eq!(Some("50 83 10-88 076-2 A".to_string()), rs.road_number);
        assert_eq!(Some("TIPO 1964".to_string()), rs.series);
        assert_eq!(None, rs.electric_multiple_unit_type);
        assert_eq!(None, rs.freight_car_type);
        assert_eq!(None, rs.locomotive_type);
        assert_eq!(Some(PassengerCarType::CompartmentCoach), rs.passenger_car_type);
        assert_eq!(None, rs.railcar_type);
        assert_eq!(Some(ServiceLevel::FirstClass), rs.service_level);
        assert_eq!(Some(false), rs.is_dummy);
        assert_eq!(Some(dec!(360.0)), rs.minimum_radius);
        assert_eq!(Some(CouplingSocket::Nem362), rs.coupling_socket);
        assert_eq!(Some(FeatureFlag::Yes), rs.close_couplers);
        assert_eq!(Some(FeatureFlag::No), rs.digital_shunting_coupling);
        assert_eq!(Some(FeatureFlag::No), rs.flywheel_fitted);
        assert_eq!(Some(FeatureFlag::No), rs.metal_body);
        assert_eq!(Some(FeatureFlag::No), rs.interior_lights);
        assert_eq!(Some(FeatureFlag::No), rs.lights);
        assert_eq!(Some(FeatureFlag::No), rs.spring_buffers);
    })
    .await
}

#[tokio::test]
async fn it_should_create_a_new_freight_car() {
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
            "category" : "FREIGHT_CARS",
            "scale" : "H0",
            "power_method" : "DC",
            "description" : {
                "it" : "Carro FS Hbbillns coperto livrea livrea XMPR grigio/verde"
            },
            "details" : {
                "it" : ""
            },
            "delivery_date" : "2005",
            "availability_status" : "ANNOUNCED",
            "count" : 1,
            "rolling_stocks": [{
                "category" : "FREIGHT_CAR",
                "type_name" : "Hbbillns",
                "road_number" : "21 83 245 7 266-6 Hbbillns",
                "freight_car_type" : "SLIDING_WALL_BOXCARS",
                "railway" : "FS",
                "epoch" : "V",
                "livery" : "XMPR",
                "length_over_buffers" : {
                  "millimeters" : 180.0
                },
                "technical_specifications" : {
                  "coupling" : {
                    "socket" : "NEM_362",
                    "close_couplers" : "YES",
                    "digital_shunting" : "NO"
                  },
                  "flywheel_fitted" : "NO",
                  "metal_body" : "NO",
                  "minimum_radius": 360.0,
                  "interior_lights" : "NO",
                  "lights" : "NO",
                  "spring_buffers" : "NO"
                }
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

        let saved = fetch_saved_catalog_item(catalog_item_id.clone(), &pg_pool).await;
        let item = saved.catalog_item;

        assert_eq!(BrandId::new("ACME"), item.brand_id);
        assert_eq!("123456", item.item_number);
        assert_eq!(Category::FreightCars, item.category);
        assert_eq!(ScaleId::new("H0"), item.scale_id);
        assert_eq!(PowerMethod::DC, item.power_method);
        assert_eq!(
            Some(String::from(
                "Carro FS Hbbillns coperto livrea livrea XMPR grigio/verde"
            )),
            item.description_it
        );
        assert_eq!(Some(String::from("")), item.details_it);
        assert_eq!(Some(DeliveryDate::by_year(2005).to_string()), item.delivery_date);
        assert_eq!(Some(AvailabilityStatus::Announced), item.availability_status);
        assert_eq!(1, item.count);

        let rs = saved.rolling_stocks.get(0).expect("no saved rolling stock found");

        assert_ne!("", rs.rolling_stock_id.to_string());
        assert_eq!(RailwayId::new("FS"), rs.railway_id);
        assert_eq!(RollingStockCategory::FreightCar, rs.rolling_stock_category);
        assert_eq!("V", rs.epoch);
        assert_eq!(Some("XMPR".to_string()), rs.livery);
        assert_eq!(Some(dec!(180)), rs.length_over_buffers_mm);
        assert_eq!(None, rs.length_over_buffers_in);
        assert_eq!(Some("Hbbillns".to_string()), rs.type_name);
        assert_eq!(Some("21 83 245 7 266-6 Hbbillns".to_string()), rs.road_number);
        assert_eq!(None, rs.series);
        assert_eq!(None, rs.electric_multiple_unit_type);
        assert_eq!(Some(FreightCarType::SlidingWallBoxcars), rs.freight_car_type);
        assert_eq!(None, rs.locomotive_type);
        assert_eq!(None, rs.passenger_car_type);
        assert_eq!(None, rs.railcar_type);
        assert_eq!(None, rs.service_level);
        assert_eq!(Some(false), rs.is_dummy);
        assert_eq!(Some(dec!(360.0)), rs.minimum_radius);
        assert_eq!(Some(CouplingSocket::Nem362), rs.coupling_socket);
        assert_eq!(Some(FeatureFlag::Yes), rs.close_couplers);
        assert_eq!(Some(FeatureFlag::No), rs.digital_shunting_coupling);
        assert_eq!(Some(FeatureFlag::No), rs.flywheel_fitted);
        assert_eq!(Some(FeatureFlag::No), rs.metal_body);
        assert_eq!(Some(FeatureFlag::No), rs.interior_lights);
        assert_eq!(Some(FeatureFlag::No), rs.lights);
        assert_eq!(Some(FeatureFlag::No), rs.spring_buffers);
    })
    .await
}

async fn fetch_saved_catalog_item(catalog_item_id: CatalogItemId, pg_pool: &PgPool) -> Saved {
    let catalog_item = sqlx::query_as!(
        SavedCatalogItem,
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
    .fetch_one(pg_pool)
    .await
    .expect("Failed to fetch saved catalog item.");

    let rolling_stocks = sqlx::query_as!(
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
    .fetch_all(pg_pool)
    .await
    .expect("Failed to fetch saved rolling stock(s).");

    Saved {
        catalog_item,
        rolling_stocks,
    }
}

#[derive(Debug)]
struct Saved {
    catalog_item: SavedCatalogItem,
    rolling_stocks: Vec<SavedRollingStock>,
}

#[derive(Debug)]
struct SavedCatalogItem {
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
