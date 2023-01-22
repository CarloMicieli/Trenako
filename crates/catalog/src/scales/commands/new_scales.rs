use crate::common::TrackGauge;
use crate::scales::scale_id::ScaleId;
use crate::scales::scale_request::ScaleRequest;
use crate::scales::scale_response::ScaleCreated;
use async_trait::async_trait;
use chrono::Utc;
use common::metadata::Metadata;
use itertools::Itertools;
use rust_decimal::Decimal;
use std::result;
use thiserror::Error;

pub type Result<R> = result::Result<R, ScaleCreationError>;

pub async fn create_new_scale<R: NewScaleRepository>(request: ScaleRequest, repo: R) -> Result<ScaleCreated> {
    let scale_id = ScaleId::new(&request.name);

    if repo.exists_already(&scale_id).await? {
        return Err(ScaleCreationError::ScaleAlreadyExists(scale_id));
    }

    let command = NewScaleCommand::try_from(request)?;
    repo.insert(&command).await?;

    Ok(ScaleCreated {
        scale_id,
        created_at: *command.metadata.created(),
    })
}

#[derive(Debug, Error)]
pub enum ScaleCreationError {
    #[error("Error while interacting with the database: {0}")]
    Database(#[from] sqlx::error::Error),

    #[error("the scale request is not valid")]
    InvalidRequest,

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

#[derive(Debug, Clone, Default)]
pub struct ScaleCommandPayload {
    pub name: String,
    pub ratio: Decimal,
    pub gauge_millimeters: Option<Decimal>,
    pub gauge_inches: Option<Decimal>,
    pub track_gauge: TrackGauge,
    pub description: Option<String>,
    pub standards: Option<String>,
}

impl TryFrom<ScaleRequest> for ScaleCommandPayload {
    type Error = ScaleCreationError;

    fn try_from(request: ScaleRequest) -> result::Result<Self, Self::Error> {
        let standards = if request.standards.is_empty() {
            None
        } else {
            #[allow(unstable_name_collisions)]
            let s: String = request
                .standards
                .iter()
                .map(|s| s.to_string())
                .intersperse(",".to_string())
                .collect();

            Some(s)
        };

        let (track_gauge, gauge_inches, gauge_millimeters) = (
            request.gauge.track_gauge,
            Some(request.gauge.inches),
            Some(request.gauge.millimeters),
        );

        Ok(ScaleCommandPayload {
            name: request.name,
            ratio: *request.ratio,
            gauge_millimeters,
            gauge_inches,
            track_gauge,
            description: request.description.italian().map(String::to_string),
            standards,
        })
    }
}

/// The persistence related functionality for the new scales creation
#[async_trait]
pub trait NewScaleRepository {
    /// Checks if a scale with the input id already exists
    async fn exists_already(&self, scale_id: &ScaleId) -> Result<bool>;

    /// Inserts a new scale
    async fn insert(&self, new_scale: &NewScaleCommand) -> Result<()>;
}

#[cfg(test)]
mod test {
    use super::*;

    mod new_scale_command {
        use super::*;
        use crate::scales::ratio::Ratio;
        use crate::scales::scale_gauge::Gauge;
        use crate::scales::standard::Standard;
        use chrono::TimeZone;
        use common::in_memory::InMemoryRepository;
        use common::localized_text::LocalizedText;
        use pretty_assertions::assert_eq;

        #[tokio::test]
        async fn it_should_create_a_new_scale() {
            let repo = InMemoryNewScaleRepository::new();

            let ratio = Decimal::from_str_exact("87").unwrap();
            let request = new_scale("H0", ratio);
            let result = create_new_scale(request, repo).await;

            let created = result.expect("result is an error");

            assert_eq!(ScaleId::new("H0"), created.scale_id);
        }

        #[tokio::test]
        async fn it_should_return_an_error_when_the_scale_already_exists() {
            let repo = InMemoryNewScaleRepository::of(new_scale_cmd_with_name("H0"));

            let ratio = Decimal::from_str_exact("87").unwrap();
            let request = new_scale("H0", ratio);

            let result = create_new_scale(request, repo).await;

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
                ratio: Ratio::try_from(ratio).unwrap(),
                gauge: Gauge::new(TrackGauge::Standard, gauge_mm, gauge_in).unwrap(),
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

        struct InMemoryNewScaleRepository(InMemoryRepository<ScaleId, NewScaleCommand>);

        impl InMemoryNewScaleRepository {
            fn new() -> Self {
                InMemoryNewScaleRepository(InMemoryRepository::empty())
            }

            fn of(command: NewScaleCommand) -> Self {
                let id = ScaleId::new(&command.scale_id);
                InMemoryNewScaleRepository(InMemoryRepository::of(id, command))
            }
        }

        #[async_trait]
        impl NewScaleRepository for InMemoryNewScaleRepository {
            async fn exists_already(&self, scale_id: &ScaleId) -> Result<bool> {
                Ok(self.0.contains(scale_id))
            }

            async fn insert(&self, new_scale: &NewScaleCommand) -> Result<()> {
                let id = ScaleId::new(&new_scale.scale_id);
                self.0.add(id, new_scale.clone());
                Ok(())
            }
        }
    }
}
