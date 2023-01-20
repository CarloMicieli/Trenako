mod common;

use crate::common::{create_docker_test, spawn_app, IMAGE_NAME};
use ::common::contacts::ContactInformation;
use ::common::localized_text::LocalizedText;
use ::common::organizations::OrganizationEntityType;
use ::common::socials::Socials;
use catalog::common::TrackGauge;
use catalog::railways::period_of_activity::{PeriodOfActivity, RailwayStatus};
use catalog::railways::railway_gauge::RailwayGauge;
use catalog::railways::railway_length::RailwayLength;
use catalog::railways::railway_request::RailwayRequest;
use chrono::NaiveDate;
use isocountry::CountryCode;
use rust_decimal::prelude::*;
use uuid::Uuid;

const API_RAILWAYS: &str = "/api/railways";

#[tokio::test]
async fn post_new_railways() {
    let test = create_docker_test();

    test.run_async(|ops| async move {
        let (_, port) = ops.handle(IMAGE_NAME).host_port(5432).unwrap();

        let sut = spawn_app(*port).await;
        let client = reqwest::Client::new();
        sut.run_database_migrations().await;

        let railway_name = Uuid::new_v4().to_string();
        let expected_location = format!("{}/{}", API_RAILWAYS, railway_name);

        let operating_since = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();
        let period_of_activity = PeriodOfActivity::active_railway(operating_since);

        let gauge_mt = Decimal::from_str("1.435").unwrap();
        let gauge = RailwayGauge::new(gauge_mt, TrackGauge::Standard);

        let total_length_km = Decimal::from_str("10000").unwrap();
        let total_length_mi = Decimal::from_str("621.371").unwrap();
        let length = RailwayLength::new(total_length_km, total_length_mi);
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

        let request = RailwayRequest {
            name: railway_name.clone(),
            abbreviation: Some(String::from("FS")),
            registered_company_name: Some(String::from("FS")),
            organization_entity_type: Some(OrganizationEntityType::StateOwnedEnterprise),
            description: LocalizedText::with_italian("Descrizione"),
            country: CountryCode::DEU,
            period_of_activity: Some(period_of_activity),
            gauge: Some(gauge),
            headquarters: Some(String::from("Some City")),
            total_length: Some(length),
            contact_info: Some(contact_info),
            socials: Some(socials),
        };

        let endpoint = sut.endpoint(API_RAILWAYS);
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
        let saved = sqlx::query_as!(
            Saved,
            r#"SELECT
                railway_id,
                name,
                abbreviation,
                registered_company_name,
                organization_entity_type as "organization_entity_type?: OrganizationEntityType",
                description_it,
                country,
                operating_since,
                operating_until,
                status as "status?: RailwayStatus",
                gauge_m,
                track_gauge as "track_gauge?: TrackGauge",
                headquarters,
                total_length_mi,
                total_length_km,
                contact_email,
                contact_website_url,
                contact_phone,
                socials_facebook,
                socials_instagram,
                socials_linkedin,
                socials_twitter,
                socials_youtube
            FROM railways WHERE name = $1"#,
            &railway_name
        )
        .fetch_one(&pg_pool)
        .await
        .expect("Failed to fetch saved railway.");

        assert_eq!(request.name, saved.railway_id);
        assert_eq!(request.name, saved.name);
        assert_eq!(request.abbreviation, saved.abbreviation);
        assert_eq!(request.registered_company_name, saved.registered_company_name);
        assert_eq!(request.organization_entity_type, saved.organization_entity_type);
        assert_eq!(request.description.italian(), saved.description_it.as_ref());
        assert_eq!(request.country.alpha3(), saved.country);
        assert_eq!(Some(operating_since), saved.operating_since);
        assert_eq!(None, saved.operating_until);
        assert_eq!(Some(RailwayStatus::Active), saved.status);
        assert_eq!(Some(String::from("Some City")), saved.headquarters);
        assert_eq!(Some(gauge_mt), saved.gauge_m);
        assert_eq!(Some(TrackGauge::Standard), saved.track_gauge);
        assert_eq!(Some(total_length_km), saved.total_length_km);
        assert_eq!(Some(total_length_mi), saved.total_length_mi);
        assert_eq!(Some(String::from("mail@mail.com")), saved.contact_email);
        assert_eq!(Some(String::from("555 1234")), saved.contact_phone);
        assert_eq!(Some(String::from("https://www.site.com/")), saved.contact_website_url);
        assert_eq!(Some(String::from("facebook_handler")), saved.socials_facebook);
        assert_eq!(Some(String::from("instagram_handler")), saved.socials_instagram);
        assert_eq!(Some(String::from("linkedin_handler")), saved.socials_linkedin);
        assert_eq!(Some(String::from("twitter_handler")), saved.socials_twitter);
        assert_eq!(Some(String::from("youtube_handler")), saved.socials_youtube);
    })
    .await;
}

struct Saved {
    railway_id: String,
    name: String,
    abbreviation: Option<String>,
    registered_company_name: Option<String>,
    organization_entity_type: Option<OrganizationEntityType>,
    description_it: Option<String>,
    country: String,
    operating_since: Option<NaiveDate>,
    operating_until: Option<NaiveDate>,
    status: Option<RailwayStatus>,
    gauge_m: Option<Decimal>,
    track_gauge: Option<TrackGauge>,
    headquarters: Option<String>,
    total_length_mi: Option<Decimal>,
    total_length_km: Option<Decimal>,
    contact_email: Option<String>,
    contact_website_url: Option<String>,
    contact_phone: Option<String>,
    socials_facebook: Option<String>,
    socials_instagram: Option<String>,
    socials_linkedin: Option<String>,
    socials_twitter: Option<String>,
    socials_youtube: Option<String>,
}
