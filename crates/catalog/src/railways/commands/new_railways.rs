use crate::common::TrackGauge;
use crate::railways::commands::repositories::RailwayRepository;
use crate::railways::period_of_activity::{PeriodOfActivity, RailwayStatus};
use crate::railways::railway_id::RailwayId;
use crate::railways::railway_request::RailwayRequest;
use crate::railways::railway_response::RailwayCreated;
use chrono::{NaiveDate, Utc};
use common::contacts::{ContactInformation, MailAddress, PhoneNumber, WebsiteUrl};
use common::metadata::Metadata;
use common::organizations::OrganizationEntityType;
use common::socials::{Handler, Socials};
use common::unit_of_work::{Database, UnitOfWork};
use rust_decimal::Decimal;
use std::result;
use thiserror::Error;

pub type Result<R> = result::Result<R, RailwayCreationError>;

pub async fn create_new_railway<'db, U: UnitOfWork<'db>, R: RailwayRepository<'db, U>, DB: Database<'db, U>>(
    request: RailwayRequest,
    repo: R,
    db: DB,
) -> Result<RailwayCreated> {
    let railway_id = RailwayId::new(&request.name);

    let mut unit_of_work = db.begin().await?;

    if repo.exists(&railway_id, &mut unit_of_work).await? {
        return Err(RailwayCreationError::RailwayAlreadyExists(railway_id));
    }

    let command = NewRailwayCommand::try_from(request)?;
    repo.insert(&command, &mut unit_of_work).await?;

    unit_of_work.commit().await?;

    Ok(RailwayCreated {
        railway_id,
        created_at: *command.metadata.created(),
    })
}

#[derive(Debug, Error)]
pub enum RailwayCreationError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),

    #[error("the railway request is not valid")]
    InvalidRequest,

    #[error("The railway already exists (id: {0})")]
    RailwayAlreadyExists(RailwayId),
}

/// It represents the command to create a new model railway company
#[derive(Debug, Clone)]
pub struct NewRailwayCommand {
    pub railway_id: RailwayId,
    pub payload: RailwayCommandPayload,
    pub metadata: Metadata,
}

impl TryFrom<RailwayRequest> for NewRailwayCommand {
    type Error = RailwayCreationError;

    fn try_from(value: RailwayRequest) -> result::Result<Self, Self::Error> {
        let railway_id = RailwayId::new(&value.name);
        let payload = RailwayCommandPayload::try_from(value)?;
        let metadata = Metadata::created_at(Utc::now());
        Ok(NewRailwayCommand {
            railway_id,
            payload,
            metadata,
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct RailwayCommandPayload {
    pub name: String,
    pub abbreviation: Option<String>,
    pub registered_company_name: Option<String>,
    pub organization_entity_type: Option<OrganizationEntityType>,
    pub description: Option<String>,
    pub country: String,
    pub operating_since: Option<NaiveDate>,
    pub operating_until: Option<NaiveDate>,
    pub status: Option<RailwayStatus>,
    pub gauge_m: Option<Decimal>,
    pub track_gauge: Option<TrackGauge>,
    pub headquarters: Option<String>,
    pub total_length_mi: Option<Decimal>,
    pub total_length_km: Option<Decimal>,
    pub contact_email: Option<MailAddress>,
    pub contact_website_url: Option<WebsiteUrl>,
    pub contact_phone: Option<PhoneNumber>,
    pub socials_facebook: Option<Handler>,
    pub socials_instagram: Option<Handler>,
    pub socials_linkedin: Option<Handler>,
    pub socials_twitter: Option<Handler>,
    pub socials_youtube: Option<Handler>,
}

impl TryFrom<RailwayRequest> for RailwayCommandPayload {
    type Error = RailwayCreationError;

    fn try_from(request: RailwayRequest) -> result::Result<Self, Self::Error> {
        let contacts = request.contact_info.unwrap_or_default();
        let ContactInformation {
            email,
            website_url,
            phone,
        } = contacts;

        let period_of_activity = request.period_of_activity.unwrap_or_default();
        let PeriodOfActivity {
            operating_since,
            operating_until,
            status,
        } = period_of_activity;

        let socials = request.socials.unwrap_or_default();
        let Socials {
            facebook,
            instagram,
            linkedin,
            twitter,
            youtube,
        } = socials;

        let (track_gauge, gauge_m) = if let Some(gauge) = request.gauge {
            (Some(gauge.track_gauge), Some(gauge.meters.quantity()))
        } else {
            (None, None)
        };

        Ok(RailwayCommandPayload {
            name: request.name,
            abbreviation: request.abbreviation,
            registered_company_name: request.registered_company_name,
            organization_entity_type: request.organization_entity_type,
            description: request.description.italian().map(String::to_string),
            country: request
                .country
                .expect("country code is required for railway requests")
                .alpha2()
                .to_string(),
            operating_since,
            operating_until,
            status: Some(status),
            track_gauge,
            gauge_m,
            headquarters: request.headquarters,
            total_length_mi: request.total_length.map(|tl| tl.miles.quantity()),
            total_length_km: request.total_length.map(|tl| tl.kilometers.quantity()),
            contact_email: email,
            contact_website_url: website_url,
            contact_phone: phone,
            socials_facebook: facebook,
            socials_instagram: instagram,
            socials_linkedin: linkedin,
            socials_twitter: twitter,
            socials_youtube: youtube,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod new_railway_command {
        use super::*;
        use crate::railways::commands::repositories::in_memory::InMemoryRailwayRepository;
        use chrono::TimeZone;
        use common::unit_of_work::noop::NoOpDatabase;
        use isocountry::CountryCode;
        use pretty_assertions::assert_eq;

        #[tokio::test]
        async fn it_should_create_a_new_railway() {
            let repo = InMemoryRailwayRepository::empty();

            let request = new_railway("FS");
            let db = NoOpDatabase;
            let result = create_new_railway(request, repo, db).await;

            let created = result.expect("result is an error");

            assert_eq!(RailwayId::new("FS"), created.railway_id);
        }

        #[tokio::test]
        async fn it_should_return_an_error_when_the_railway_already_exists() {
            let repo = InMemoryRailwayRepository::with(new_railway_cmd_with_name("FS"));

            let request = new_railway("FS");
            let db = NoOpDatabase;
            let result = create_new_railway(request, repo, db).await;

            match result {
                Err(RailwayCreationError::RailwayAlreadyExists(id)) => assert_eq!(RailwayId::new("FS"), id),
                _ => panic!("RailwayAlreadyExists expected"),
            }
        }

        fn new_railway(name: &str) -> RailwayRequest {
            RailwayRequest {
                name: name.to_string(),
                country: Some(CountryCode::ITA),
                ..RailwayRequest::default()
            }
        }

        fn new_railway_cmd_with_name(name: &str) -> NewRailwayCommand {
            let railway_id = RailwayId::new(name);
            let payload = RailwayCommandPayload {
                name: String::from(name),
                ..RailwayCommandPayload::default()
            };
            let metadata = Metadata::created_at(Utc.with_ymd_and_hms(1988, 11, 25, 0, 0, 0).unwrap());

            NewRailwayCommand {
                railway_id,
                payload,
                metadata,
            }
        }
    }
}
