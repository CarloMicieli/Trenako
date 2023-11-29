//! the new brand creation command

use crate::brands::brand_id::BrandId;
use crate::brands::brand_kind::BrandKind;
use crate::brands::brand_request::BrandRequest;
use crate::brands::brand_response::BrandCreated;
use crate::brands::brand_status::BrandStatus;
use crate::brands::commands::repositories::NewBrandRepository;
use chrono::Utc;
use common::address::Address;
use common::contacts::{ContactInformation, MailAddress, PhoneNumber, WebsiteUrl};
use common::localized_text::LocalizedText;
use common::metadata::Metadata;
use common::organizations::OrganizationEntityType;
use common::queries::errors::DatabaseError;
use common::socials::{Handler, Socials};
use common::unit_of_work::{Database, UnitOfWork};
use std::result;
use thiserror::Error;
use validator::{Validate, ValidationErrors};

pub type Result<R> = result::Result<R, BrandCreationError>;

pub async fn create_new_brand<'db, U, Repo, DB>(request: BrandRequest, repo: Repo, db: DB) -> Result<BrandCreated>
where
    U: UnitOfWork<'db>,
    Repo: NewBrandRepository<'db, U>,
    DB: Database<'db, U>,
{
    let brand_id = BrandId::new(&request.name);

    let mut unit_of_work = db.begin().await?;

    if repo.exists(&brand_id, &mut unit_of_work).await? {
        return Err(BrandCreationError::BrandAlreadyExists(brand_id));
    }

    let command = NewBrandCommand::try_from(request)?;
    repo.insert(&command, &mut unit_of_work).await?;

    unit_of_work.commit().await?;

    Ok(BrandCreated {
        brand_id,
        created_at: *command.metadata.created(),
    })
}

#[derive(Debug, Error)]
pub enum BrandCreationError {
    #[error("The brand request is not valid")]
    InvalidRequest(ValidationErrors),

    #[error("The brand already exists (id: {0})")]
    BrandAlreadyExists(BrandId),

    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

/// It represents the command to create a new model railway brand
#[derive(Debug, Clone)]
pub struct NewBrandCommand {
    pub brand_id: BrandId,
    pub payload: BrandCommandPayload,
    pub metadata: Metadata,
}

impl TryFrom<BrandRequest> for NewBrandCommand {
    type Error = BrandCreationError;

    fn try_from(value: BrandRequest) -> result::Result<Self, Self::Error> {
        validate_request(&value)?;
        let brand_id = BrandId::new(&value.name);
        let payload = BrandCommandPayload::try_from(value)?;
        let metadata = Metadata::created_at(Utc::now());
        Ok(NewBrandCommand {
            brand_id,
            payload,
            metadata,
        })
    }
}

fn validate_request(request: &BrandRequest) -> result::Result<(), BrandCreationError> {
    request.validate().map_err(BrandCreationError::InvalidRequest)
}

#[derive(Debug, Clone, Default)]
pub struct BrandCommandPayload {
    pub name: String,
    pub registered_company_name: Option<String>,
    pub organization_entity_type: Option<OrganizationEntityType>,
    pub group_name: Option<String>,
    pub description: LocalizedText,
    pub kind: BrandKind,
    pub status: Option<BrandStatus>,
    pub contact_email: Option<MailAddress>,
    pub contact_website_url: Option<WebsiteUrl>,
    pub contact_phone: Option<PhoneNumber>,
    pub address_street_address: Option<String>,
    pub address_extended_address: Option<String>,
    pub address_city: Option<String>,
    pub address_region: Option<String>,
    pub address_postal_code: Option<String>,
    pub address_country: Option<String>,
    pub facebook_handler: Option<Handler>,
    pub instagram_handler: Option<Handler>,
    pub linkedin_handler: Option<Handler>,
    pub twitter_handler: Option<Handler>,
    pub youtube_handler: Option<Handler>,
}

impl TryFrom<BrandRequest> for BrandCommandPayload {
    type Error = BrandCreationError;

    fn try_from(request: BrandRequest) -> result::Result<Self, Self::Error> {
        let contacts = request.contact_info.unwrap_or_default();
        let ContactInformation {
            email,
            website_url,
            phone,
        } = contacts;

        let socials = request.socials.unwrap_or_default();
        let Socials {
            facebook,
            instagram,
            linkedin,
            twitter,
            youtube,
        } = socials;

        let (
            address_street_address,
            address_extended_address,
            address_city,
            address_region,
            address_postal_code,
            address_country,
        ) = if let Some(Address {
            street_address,
            extended_address,
            city,
            region,
            postal_code,
            country,
        }) = request.address
        {
            (
                Some(street_address),
                extended_address,
                Some(city),
                region,
                Some(postal_code),
                Some(country.alpha2().to_string()),
            )
        } else {
            (None, None, None, None, None, None)
        };

        let value = BrandCommandPayload {
            name: request.name,
            registered_company_name: request.registered_company_name,
            organization_entity_type: request.organization_entity_type,
            group_name: request.group_name,
            description: request.description,
            kind: request.kind,
            status: request.status,
            contact_email: email,
            contact_website_url: website_url,
            contact_phone: phone,
            address_street_address,
            address_extended_address,
            address_city,
            address_region,
            address_postal_code,
            address_country,
            facebook_handler: facebook,
            instagram_handler: instagram,
            linkedin_handler: linkedin,
            twitter_handler: twitter,
            youtube_handler: youtube,
        };

        Ok(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod new_brand_command {
        use super::*;
        use crate::brands::commands::repositories::in_memory::InMemoryBrandRepository;
        use chrono::TimeZone;
        use common::localized_text::LocalizedText;
        use common::unit_of_work::noop::NoOpDatabase;
        use isocountry::CountryCode;
        use pretty_assertions::assert_eq;

        #[tokio::test]
        async fn it_should_create_a_new_brand() {
            let repo = InMemoryBrandRepository::empty();

            let request = new_brand("ACME");
            let db = NoOpDatabase;
            let result = create_new_brand(request, repo, db).await;

            let created = result.expect("result is an error");

            assert_eq!(BrandId::new("ACME"), created.brand_id);
        }

        #[tokio::test]
        async fn it_should_return_an_error_when_the_brand_already_exists() {
            let new_brand_cmd = new_brand_cmd_with_name("ACME");
            let repo = InMemoryBrandRepository::with(new_brand_cmd);

            let request = new_brand("ACME");
            let db = NoOpDatabase;
            let result = create_new_brand(request, repo, db).await;

            match result {
                Err(BrandCreationError::BrandAlreadyExists(id)) => assert_eq!(BrandId::new("ACME"), id),
                _ => panic!("BrandAlreadyExists is expected (found: {:?})", result),
            }
        }

        fn new_brand(name: &str) -> BrandRequest {
            let address = Address::builder()
                .street_address("Rue Morgue 22")
                .city("London")
                .postal_code("1H2 4BB")
                .country(CountryCode::GBR)
                .build()
                .unwrap();

            let contact_info = ContactInformation::builder()
                .email("mail@mail.com")
                .phone("+14152370800")
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

            BrandRequest {
                name: String::from(name),
                registered_company_name: Some(String::from("A cool brand ltd.")),
                organization_entity_type: Some(OrganizationEntityType::LimitedCompany),
                group_name: Some(String::from("Group Corp.")),
                description: LocalizedText::with_italian("La descrizione va qui"),
                address: Some(address),
                contact_info: Some(contact_info),
                kind: BrandKind::Industrial,
                status: Some(BrandStatus::Active),
                socials: Some(socials),
            }
        }

        fn new_brand_cmd_with_name(name: &str) -> NewBrandCommand {
            let brand_id = BrandId::new(name);
            let payload = BrandCommandPayload {
                name: String::from(name),
                ..BrandCommandPayload::default()
            };
            let metadata = Metadata::created_at(Utc.with_ymd_and_hms(1988, 11, 25, 0, 0, 0).unwrap());

            NewBrandCommand {
                brand_id,
                payload,
                metadata,
            }
        }
    }
}
