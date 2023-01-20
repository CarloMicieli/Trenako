mod common;

use crate::common::{create_docker_test, spawn_app, IMAGE_NAME};
use ::common::address::Address;
use ::common::contacts::ContactInformation;
use ::common::localized_text::LocalizedText;
use ::common::organizations::OrganizationEntityType;
use ::common::socials::Socials;
use catalog::brands::brand_kind::BrandKind;
use catalog::brands::brand_request::BrandRequest;
use catalog::brands::brand_status::BrandStatus;
use isocountry::CountryCode;
use uuid::Uuid;

const API_BRANDS: &str = "/api/brands";

#[tokio::test]
async fn post_new_brands() {
    let test = create_docker_test();

    test.run_async(|ops| async move {
        let (_, port) = ops.handle(IMAGE_NAME).host_port(5432).unwrap();

        let sut = spawn_app(*port).await;
        let client = reqwest::Client::new();
        sut.run_database_migrations().await;

        let brand_name = Uuid::new_v4().to_string();
        let expected_location = format!("{}/{}", API_BRANDS, brand_name);

        let address = Address::builder()
            .street_address("Rue Morgue 22")
            .city("London")
            .postal_code("1H2 4BB")
            .country(CountryCode::GBR)
            .build()
            .unwrap();

        let contact_info = ContactInformation::builder()
            .email("mail@mail.com")
            .phone("555 1234")
            .website_url("https://www.site.com")
            .build()
            .unwrap();

        let socials = Socials::builder()
            .instagram("instagram_handler")
            .linkedin("linkedin_handler")
            .facebook("facebook_handler")
            .twitter("twitter_handler")
            .youtube("youtube_handler")
            .build()
            .unwrap();

        let request = BrandRequest {
            name: brand_name.clone(),
            registered_company_name: Some(String::from("A cool brand ltd.")),
            organization_entity_type: Some(OrganizationEntityType::LimitedCompany),
            group_name: Some(String::from("Group Corp.")),
            description: LocalizedText::with_italian("La descrizione va qui"),
            address: Some(address),
            contact_info: Some(contact_info),
            kind: BrandKind::Industrial,
            status: Some(BrandStatus::Active),
            socials: Some(socials)
        };

        let endpoint = sut.endpoint(API_BRANDS);
        let response = client
            .post(endpoint)
            .json(&request)
            .send()
            .await
            .expect("Failed to execute request.");

        assert!(response.status().is_success());
        assert_eq!(Some(0), response.content_length());
        assert_eq!(expected_location, response.headers()["Location"]);

        let pg_pool = sut.pg_pool();
        let saved = sqlx::query_as!(Saved,
                r#"SELECT
                    brand_id, name, registered_company_name, 
                    organization_entity_type as "organization_entity_type: OrganizationEntityType", group_name, 
                    description_it, kind as "kind: BrandKind", status as "status: BrandStatus",
                    contact_email, contact_website_url, contact_phone,
                    address_street_address, address_extended_address, address_city, address_region, address_postal_code, address_country,
                    socials_facebook, socials_instagram, socials_linkedin, socials_twitter, socials_youtube
                FROM brands WHERE name = $1"#, &brand_name)
            .fetch_one(&pg_pool)
            .await
            .expect("Failed to fetch saved brand.");

        assert_eq!(request.name, saved.brand_id);
        assert_eq!(request.name, saved.name);
        assert_eq!(request.kind, saved.kind);
        assert_eq!(request.description.italian(), saved.description_it.as_ref());
        assert_eq!(request.group_name, saved.group_name);
        assert_eq!(request.registered_company_name, saved.registered_company_name);
        assert_eq!(request.organization_entity_type, saved.organization_entity_type);
        assert_eq!(request.status, saved.status);
        assert_eq!(Some(String::from("mail@mail.com")), saved.contact_email);
        assert_eq!(Some(String::from("555 1234")), saved.contact_phone);
        assert_eq!(Some(String::from("https://www.site.com/")), saved.contact_website_url);
        assert_eq!(Some(String::from("Rue Morgue 22")), saved.address_street_address);
        assert_eq!(None, saved.address_extended_address);
        assert_eq!(Some(String::from("London")), saved.address_city);
        assert_eq!(None, saved.address_region);
        assert_eq!(Some(String::from("1H2 4BB")), saved.address_postal_code);
        assert_eq!(Some(String::from("GBR")), saved.address_country);
        assert_eq!(Some(String::from("facebook_handler")), saved.socials_facebook);
        assert_eq!(Some(String::from("instagram_handler")), saved.socials_instagram);
        assert_eq!(Some(String::from("linkedin_handler")), saved.socials_linkedin);
        assert_eq!(Some(String::from("twitter_handler")), saved.socials_twitter);
        assert_eq!(Some(String::from("youtube_handler")), saved.socials_youtube);
    })
    .await;
}

#[derive(Debug)]
struct Saved {
    brand_id: String,
    name: String,
    registered_company_name: Option<String>,
    organization_entity_type: Option<OrganizationEntityType>,
    group_name: Option<String>,
    description_it: Option<String>,
    kind: BrandKind,
    status: Option<BrandStatus>,
    contact_email: Option<String>,
    contact_website_url: Option<String>,
    contact_phone: Option<String>,
    address_street_address: Option<String>,
    address_extended_address: Option<String>,
    address_city: Option<String>,
    address_region: Option<String>,
    address_postal_code: Option<String>,
    address_country: Option<String>,
    socials_facebook: Option<String>,
    socials_instagram: Option<String>,
    socials_linkedin: Option<String>,
    socials_twitter: Option<String>,
    socials_youtube: Option<String>,
}
