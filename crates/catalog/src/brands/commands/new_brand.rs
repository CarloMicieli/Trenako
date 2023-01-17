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
use std::result;
use thiserror::Error;

pub type Result<R> = result::Result<R, BrandCreationError>;

#[derive(Debug, Error)]
pub enum BrandCreationError {
    #[error("Error while interacting with the database: {0}")]
    Database(#[from] sqlx::error::Error),

    #[error("Brand already exists")]
    BrandAlreadyExists,
}

/// It represents the command to create a new model railway brand
#[derive(Debug, Clone)]
pub struct NewBrandCommand {
    pub brand_id: BrandId,
    pub payload: BrandCommandPayload,
    pub metadata: Metadata,
}

impl NewBrandCommand {
    pub async fn handle<R: NewBrandRepository>(self, repo: R) -> Result<BrandCreated> {
        if repo.exists_already(&self.brand_id).await? {
            return Err(BrandCreationError::BrandAlreadyExists);
        }

        repo.insert(&self).await?;
        Ok(BrandCreated {
            brand_id: self.brand_id,
            created_at: *self.metadata.created(),
        })
    }
}

#[derive(Debug, Error)]
pub enum InvalidBrandRequest {
    #[error("the brand request is not valid")]
    InvalidRequest,
}

impl TryFrom<BrandRequest> for NewBrandCommand {
    type Error = InvalidBrandRequest;

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
    type Error = InvalidBrandRequest;

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
                Some(country.to_string()),
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
pub trait NewBrandRepository {
    /// Checks if a brand with the input id already exists
    async fn exists_already(&self, brand_id: &BrandId) -> Result<bool>;

    /// Inserts a new brand
    async fn insert(&self, new_brand: &NewBrandCommand) -> Result<()>;
}

#[cfg(test)]
mod test {
    use super::*;

    mod new_brand_command {
        use super::*;
        use chrono::TimeZone;
        use pretty_assertions::assert_eq;
        use std::cell::RefCell;
        use std::sync::Mutex;

        #[tokio::test]
        async fn it_should_create_a_new_brand() {
            let repo = InMemoryNewBrandRepository::new();
            let new_brand_cmd = new_brand_cmd_with_name("ACME");

            let result = new_brand_cmd.handle(repo).await;

            let expected = BrandCreated {
                brand_id: BrandId::new("ACME"),
                created_at: Utc.with_ymd_and_hms(1988, 11, 25, 0, 0, 0).unwrap(),
            };
            assert_eq!(expected, result.unwrap());
        }

        #[tokio::test]
        async fn it_should_return_an_error_when_the_brand_already_exists() {
            let repo = InMemoryNewBrandRepository::with_brand("ACME");
            let new_brand_cmd = new_brand_cmd_with_name("ACME");

            let result = new_brand_cmd.handle(repo).await;

            match result {
                Err(BrandCreationError::BrandAlreadyExists) => {}
                Err(why) => panic!("Unexpected error {:?}", why),
                Ok(_) => panic!("The result should be an error"),
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

        struct InMemoryNewBrandRepository {
            items: Mutex<RefCell<Vec<NewBrandCommand>>>,
        }

        impl InMemoryNewBrandRepository {
            fn new() -> Self {
                InMemoryNewBrandRepository {
                    items: Mutex::new(RefCell::new(Vec::new())),
                }
            }

            fn with_brand(name: &str) -> Self {
                let item = new_brand_cmd_with_name(name);
                InMemoryNewBrandRepository {
                    items: Mutex::new(RefCell::new(vec![item])),
                }
            }
        }

        #[async_trait]
        impl NewBrandRepository for InMemoryNewBrandRepository {
            async fn exists_already(&self, brand_id: &BrandId) -> Result<bool> {
                let items = self.items.lock().expect("Unable to acquire the lock");
                let result = items.borrow().iter().any(|it| it.brand_id == *brand_id);
                Ok(result)
            }

            async fn insert(&self, new_brand: &NewBrandCommand) -> Result<()> {
                let items = self.items.lock().expect("Unable to acquire the lock");
                items.borrow_mut().push(new_brand.clone());
                Ok(())
            }
        }
    }
}
