use crate::catalog::brands::routes;
use crate::commands::{handle_web_request, Command, IntoHttpResponse};
use actix_web::{web, HttpResponse, Responder};
use async_trait::async_trait;
use catalog::brands::brand_id::BrandId;
use catalog::brands::brand_kind::BrandKind;
use catalog::brands::brand_request::BrandRequest;
use catalog::brands::brand_response::BrandCreated;
use catalog::brands::brand_status::BrandStatus;
use catalog::brands::commands::new_brand::{BrandCreationError, NewBrandCommand, NewBrandRepository, Result};
use common::contacts::{MailAddress, PhoneNumber};
use common::organizations::OrganizationEntityType;
use common::socials::Handler;
use sqlx::PgPool;
use tracing_actix_web::RequestId;

pub async fn handler(
    request_id: RequestId,
    request: web::Json<BrandRequest>,
    pg_pool: web::Data<PgPool>,
) -> impl Responder {
    let command = NewBrandCommand::try_from(request.0).expect("The new brand request is invalid");
    handle_web_request(command, request_id, &pg_pool).await
}

#[async_trait]
impl Command for NewBrandCommand {
    type Output = BrandCreated;
    type Error = BrandCreationError;

    async fn execute(self, pg_pool: &PgPool) -> std::result::Result<Self::Output, Self::Error> {
        let repository = PgNewBrandRepository { pg_pool };
        self.handle(repository).await
    }
}

impl IntoHttpResponse for BrandCreated {
    fn into_http_response(self) -> HttpResponse {
        let location = format!("{}/{}", routes::BRAND_ROOT_API, self.brand_id);
        HttpResponse::Created().insert_header(("Location", location)).finish()
    }
}

#[allow(dead_code)]
struct PgNewBrandRepository<'repo> {
    pg_pool: &'repo PgPool,
}

#[async_trait]
impl<'repo> NewBrandRepository for PgNewBrandRepository<'repo> {
    async fn exists_already(&self, _brand_id: &BrandId) -> Result<bool> {
        Ok(false)
    }

    async fn insert(&self, new_brand: &NewBrandCommand) -> Result<()> {
        let mut transaction = self.pg_pool.begin().await?;
        let brand_id = &new_brand.brand_id;
        let request = &new_brand.payload;
        let metadata = &new_brand.metadata;

        sqlx::query!(
                r#"INSERT INTO brands (
                    brand_id,
                    name,
                    registered_company_name,
                    organization_entity_type,
                    group_name,
                    description_it,
                    kind,
                    status,
                    contact_email, contact_website_url, contact_phone,
                    address_street_address, address_extended_address, address_city, address_region, address_postal_code, address_country,
                    socials_facebook, socials_instagram, socials_linkedin, socials_twitter, socials_youtube,
                    created_at,
                    version
                )
                VALUES (
                    $1, $2, $3, $4, $5, $6,
                    $7, $8, $9, $10, $11, $12, 
                    $13, $14, $15, $16, $17, $18,
                    $19, $20, $21, $22, $23, $24
                )"#,
                brand_id as &BrandId,
                request.name,
                request.registered_company_name,
                request.organization_entity_type as Option<OrganizationEntityType>,
                request.group_name,
                request.description_it,
                request.kind as BrandKind,
                request.status as Option<BrandStatus>,
                request.contact_email.as_ref() as Option<&MailAddress>,
                request.contact_website_url.as_ref().map(|x| x.to_string()),
                request.contact_phone.as_ref() as Option<&PhoneNumber>,
                request.address_street_address,
                request.address_extended_address,
                request.address_city,
                request.address_region,
                request.address_postal_code,
                request.address_country,
                request.facebook_handler.as_ref() as Option<&Handler>,
                request.instagram_handler.as_ref() as Option<&Handler>,
                request.linkedin_handler.as_ref() as Option<&Handler>,
                request.twitter_handler.as_ref() as Option<&Handler>,
                request.youtube_handler.as_ref() as Option<&Handler>,
                metadata.created(),
                metadata.version() as i32
            )
            .execute(&mut *transaction)
            .await?;

        transaction.commit().await?;
        Ok(())
    }
}
