use crate::common::TrackGauge;
use crate::scales::commands::repositories::NewScaleRepository;
use crate::scales::scale_id::ScaleId;
use crate::scales::scale_request::ScaleRequest;
use crate::scales::scale_response::ScaleCreated;
use crate::scales::standard::Standard;
use chrono::Utc;
use common::localized_text::LocalizedText;
use common::metadata::Metadata;
use common::queries::errors::DatabaseError;
use common::unit_of_work::{Database, UnitOfWork};
use rust_decimal::Decimal;
use std::result;
use thiserror::Error;
use validator::{Validate, ValidationErrors};

pub type Result<R> = result::Result<R, ScaleCreationError>;

pub async fn create_new_scale<'db, U, Repo, DB>(request: ScaleRequest, repo: Repo, db: DB) -> Result<ScaleCreated>
where
    U: UnitOfWork<'db>,
    Repo: NewScaleRepository<'db, U>,
    DB: Database<'db, U>,
{
    let scale_id = ScaleId::new(&request.name);

    let mut unit_of_work = db.begin().await?;

    if repo.exists(&scale_id, &mut unit_of_work).await? {
        return Err(ScaleCreationError::ScaleAlreadyExists(scale_id));
    }

    let command = NewScaleCommand::try_from(request)?;
    repo.insert(&command, &mut unit_of_work).await?;

    unit_of_work.commit().await?;

    Ok(ScaleCreated {
        scale_id,
        created_at: *command.metadata.created(),
    })
}

#[derive(Debug, Error)]
pub enum ScaleCreationError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),

    #[error("The scale request is not valid")]
    InvalidRequest(ValidationErrors),

    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),

    #[error("The scale already exists (id: {0})")]
    ScaleAlreadyExists(ScaleId),
}

/// It represents the command to create a new model railway scale
#[derive(Debug, Clone)]
pub struct NewScaleCommand {
    pub scale_id: ScaleId,
    pub payload: ScaleCommandPayload,
    pub metadata: Metadata,
}

impl TryFrom<ScaleRequest> for NewScaleCommand {
    type Error = ScaleCreationError;

    fn try_from(value: ScaleRequest) -> result::Result<Self, Self::Error> {
        validate_request(&value)?;
        let scale_id = ScaleId::new(&value.name);
        let payload = ScaleCommandPayload::try_from(value)?;
        let metadata = Metadata::created_at(Utc::now());
        Ok(NewScaleCommand {
            scale_id,
            payload,
            metadata,
        })
    }
}

fn validate_request(request: &ScaleRequest) -> result::Result<(), ScaleCreationError> {
    request.validate().map_err(ScaleCreationError::InvalidRequest)
}

#[derive(Debug, Clone, Default)]
pub struct ScaleCommandPayload {
    pub name: String,
    pub ratio: Decimal,
    pub gauge_millimeters: Option<Decimal>,
    pub gauge_inches: Option<Decimal>,
    pub track_gauge: TrackGauge,
    pub description: LocalizedText,
    pub standards: Vec<Standard>,
}

impl TryFrom<ScaleRequest> for ScaleCommandPayload {
    type Error = ScaleCreationError;

    fn try_from(request: ScaleRequest) -> result::Result<Self, Self::Error> {
        let scale_gauge = request.gauge.expect("gauge is required for scale requests");
        let (track_gauge, gauge_inches, gauge_millimeters) = (
            scale_gauge.track_gauge,
            Some(scale_gauge.inches.quantity()),
            Some(scale_gauge.millimeters.quantity()),
        );

        let ratio = request.ratio.expect("ratio is required for scale requests");
        Ok(ScaleCommandPayload {
            name: request.name,
            ratio: *ratio,
            gauge_millimeters,
            gauge_inches,
            track_gauge,
            description: request.description,
            standards: request.standards,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod new_scale_command {
        use super::*;
        use crate::scales::commands::repositories::in_memory::InMemoryScaleRepository;
        use crate::scales::ratio::Ratio;
        use crate::scales::scale_gauge::Gauge;
        use crate::scales::standard::Standard;
        use chrono::TimeZone;
        use common::localized_text::LocalizedText;
        use common::unit_of_work::noop::NoOpDatabase;
        use pretty_assertions::assert_eq;

        #[tokio::test]
        async fn it_should_create_a_new_scale() {
            let repo = InMemoryScaleRepository::empty();

            let ratio = Decimal::from_str_exact("87").unwrap();
            let request = new_scale("H0", ratio);

            let db = NoOpDatabase;
            let result = create_new_scale(request, repo, db).await;

            let created = result.expect("result is an error");

            assert_eq!(ScaleId::new("H0"), created.scale_id);
        }

        #[tokio::test]
        async fn it_should_return_an_error_when_the_scale_already_exists() {
            let repo = InMemoryScaleRepository::with(new_scale_cmd_with_name("H0"));

            let ratio = Decimal::from_str_exact("87").unwrap();
            let request = new_scale("H0", ratio);

            let db = NoOpDatabase;
            let result = create_new_scale(request, repo, db).await;

            match result {
                Err(ScaleCreationError::ScaleAlreadyExists(id)) => assert_eq!(ScaleId::new("H0"), id),
                _ => panic!("ScaleAlreadyExists is expected (found: {:?})", result),
            }
        }

        fn new_scale(name: &str, ratio: Decimal) -> ScaleRequest {
            let gauge_mm = Decimal::from_str_exact("16.5").unwrap();
            let gauge_in = Decimal::from_str_exact("0.65").unwrap();

            ScaleRequest {
                name: String::from(name),
                ratio: Ratio::try_from(ratio).ok(),
                gauge: Gauge::new(TrackGauge::Standard, gauge_mm, gauge_in).ok(),
                description: LocalizedText::with_italian("Descrizione"),
                standards: vec![Standard::NEM],
            }
        }

        fn new_scale_cmd_with_name(name: &str) -> NewScaleCommand {
            let scale_id = ScaleId::new(name);
            let payload = ScaleCommandPayload {
                name: String::from(name),
                ..ScaleCommandPayload::default()
            };
            let metadata = Metadata::created_at(Utc.with_ymd_and_hms(1988, 11, 25, 0, 0, 0).unwrap());
            NewScaleCommand {
                scale_id,
                payload,
                metadata,
            }
        }
    }
}
