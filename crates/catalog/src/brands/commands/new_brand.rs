use crate::brands::brand_id::BrandId;
use crate::brands::brand_kind::BrandKind;
use crate::brands::brand_request::BrandRequest;
use crate::brands::brand_response::BrandCreated;
use crate::brands::brand_status::BrandStatus;
use async_trait::async_trait;
use chrono::Utc;
use common::address::Address;
use common::contacts::{ContactInformation, MailAddress, PhoneNumber, WebsiteUrl};
use common::metadata::Metadata;
use common::organizations::OrganizationEntityType;
use common::socials::{Handler, Socials};
use common::unit_of_work::{Database, UnitOfWork};
use std::result;
use thiserror::Error;

pub type Result<R> = result::Result<R, BrandCreationError>;

pub async fn create_new_brand<'db, U: UnitOfWork<'db>, R: NewBrandRepository<'db, U>, DB: Database<'db, U>>(
    request: BrandRequest,
    repo: R,
    db: DB,
) -> Result<BrandCreated> {
    let brand_id = BrandId::new(&request.name);

    let mut unit_of_work = db.begin().await?;

    if repo.exists_already(&brand_id, &mut unit_of_work).await? {
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
    InvalidRequest,

    #[error("The brand already exists (id: {0})")]
    BrandAlreadyExists(BrandId),

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

    fn try_from(request: BrandRequest) -> result::Result<Self, Self::Error> {
        Ok(NewBrandCommand {
            brand_id: BrandId::new(&request.name),
            payload: BrandCommandPayload::try_from(request)?,
            metadata: Metadata::created_at(Utc::now()),
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct BrandCommandPayload {
    pub name: String,
    pub registered_company_name: Option<String>,
    pub organization_entity_type: Option<OrganizationEntityType>,
    pub group_name: Option<String>,
    pub description_it: Option<String>,
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
                Some(country.alpha3().to_string()),
            )
        } else {
            (None, None, None, None, None, None)
        };

        let value = BrandCommandPayload {
            name: request.name,
            registered_company_name: request.registered_company_name,
            organization_entity_type: request.organization_entity_type,
            group_name: request.group_name,
            description_it: request.description.italian().map(String::to_string),
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

/// The persistence related functionality for the new brands creation
#[async_trait]
pub trait NewBrandRepository<'db, U: UnitOfWork<'db>> {
    /// Checks if a brand with the input id already exists
    async fn exists_already(&self, brand_id: &BrandId, unit_of_work: &mut U) -> Result<bool>;

    /// Inserts a new brand
    async fn insert(&self, new_brand: &NewBrandCommand, unit_of_work: &mut U) -> Result<()>;
}

#[cfg(test)]
mod test {
    use super::*;

    mod new_brand_command {
        use super::*;
        use chrono::TimeZone;
        use common::in_memory::InMemoryRepository;
        use common::localized_text::LocalizedText;
        use common::unit_of_work::noop::{NoOpDatabase, NoOpUnitOfWork};
        use isocountry::CountryCode;
        use pretty_assertions::assert_eq;

        #[tokio::test]
        async fn it_should_create_a_new_brand() {
            let repo = InMemoryNewBrandRepository::new();

            let request = new_brand("ACME");
            let db = NoOpDatabase;
            let result = create_new_brand(request, repo, db).await;

            let created = result.expect("result is an error");

            assert_eq!(BrandId::new("ACME"), created.brand_id);
        }

        #[tokio::test]
        async fn it_should_return_an_error_when_the_brand_already_exists() {
            let new_brand_cmd = new_brand_cmd_with_name("ACME");
            let repo = InMemoryNewBrandRepository::of(new_brand_cmd);

            let request = new_brand("ACME");
            let db = NoOpDatabase;
            let result = create_new_brand(request, repo, db).await;

            match result {
                Err(BrandCreationError::BrandAlreadyExists(id)) => assert_eq!(BrandId::new("ACME"), id),
                _ => panic!("BrandAlreadyExists is expected (found: {result:?})"),
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

        struct InMemoryNewBrandRepository(InMemoryRepository<BrandId, NewBrandCommand>);

        impl InMemoryNewBrandRepository {
            fn new() -> Self {
                InMemoryNewBrandRepository(InMemoryRepository::empty())
            }

            fn of(command: NewBrandCommand) -> Self {
                let id = BrandId::new(&command.brand_id);
                InMemoryNewBrandRepository(InMemoryRepository::of(id, command))
            }
        }

        #[async_trait]
        impl NewBrandRepository<'static, NoOpUnitOfWork> for InMemoryNewBrandRepository {
            async fn exists_already(&self, brand_id: &BrandId, _unit_of_work: &mut NoOpUnitOfWork) -> Result<bool> {
                Ok(self.0.contains(brand_id))
            }

            async fn insert(&self, new_brand: &NewBrandCommand, _unit_of_work: &mut NoOpUnitOfWork) -> Result<()> {
                let id = BrandId::new(&new_brand.brand_id);
                self.0.add(id, new_brand.clone());
                Ok(())
            }
        }
    }
}
