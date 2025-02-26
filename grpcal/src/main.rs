use std::{collections::BTreeMap, str::FromStr};

use anyhow::Context;
use tokio::sync::Mutex;
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};

mod grpcal {
    tonic::include_proto!("grpcal");
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct EventId(uuid::Uuid);

impl EventId {
    fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl std::str::FromStr for EventId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(uuid::Uuid::from_str(s).map(Self)?)
    }
}

impl std::fmt::Display for EventId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

#[derive(Clone, Debug)]
struct Event {
    date_time: String,
    id: EventId,
    summary: String,
}

impl From<Event> for grpcal::CreateEventResponse {
    fn from(event: Event) -> Self {
        Self {
            event: Some(grpcal::Event::from(event)),
        }
    }
}

impl From<Event> for grpcal::Event {
    fn from(
        Event {
            date_time,
            id,
            summary,
        }: Event,
    ) -> Self {
        Self {
            date_time,
            id: id.to_string(),
            summary,
        }
    }
}

impl From<Event> for grpcal::GetEventResponse {
    fn from(event: Event) -> Self {
        Self {
            event: Some(grpcal::Event::from(event)),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("deserialize")]
    Deserialize(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("read")]
    Read(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("serialize")]
    Serialize(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("write")]
    Write(#[source] Box<dyn std::error::Error + Send + Sync>),
}

#[tonic::async_trait]
trait EventStorage {
    async fn find(&self, event_id: &EventId) -> Result<Option<Event>, Error>;

    async fn find_all(&self) -> Result<Vec<Event>, Error>;

    async fn store(&self, event: Event) -> Result<(), Error>;
}

#[cfg(feature = "memory")]
#[derive(Debug)]
struct InMemoryEventStorage {
    data: Mutex<BTreeMap<EventId, Event>>,
}

#[cfg(feature = "memory")]
impl InMemoryEventStorage {
    fn new() -> Self {
        Self {
            data: Mutex::new(Default::default()),
        }
    }
}

#[cfg(feature = "memory")]
#[tonic::async_trait]
impl EventStorage for InMemoryEventStorage {
    async fn find(&self, event_id: &EventId) -> Result<Option<Event>, Error> {
        let data = self.data.lock().await;
        let event = data.get(event_id).cloned();
        Ok(event)
    }

    async fn find_all(&self) -> Result<Vec<Event>, Error> {
        let data = self.data.lock().await;
        let events = data.values().cloned().collect::<Vec<Event>>();
        Ok(events)
    }

    async fn store(&self, event: Event) -> Result<(), Error> {
        let mut data = self.data.lock().await;
        data.insert(event.id, event);
        Ok(())
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct EventStorageData {
    events: BTreeMap<String, EventData>,
}
#[derive(serde::Deserialize, serde::Serialize)]
struct EventData {
    date_time: String,
    id: String,
    summary: String,
}

#[cfg(feature = "fs")]
#[derive(Debug)]
struct FileSystemEventStorage {
    cache: Mutex<BTreeMap<EventId, Event>>,
    path: std::path::PathBuf,
}

#[cfg(feature = "fs")]
impl FileSystemEventStorage {
    async fn new(path: std::path::PathBuf) -> Result<Self, Error> {
        let deserialized = if tokio::fs::try_exists(path.as_path())
            .await
            .map_err(Into::into)
            .map_err(Error::Read)?
        {
            let stored = tokio::fs::read_to_string(path.as_path())
                .await
                .map_err(Into::into)
                .map_err(Error::Read)?;
            let deserialized = serde_json::from_str::<EventStorageData>(&stored)
                .map_err(Into::into)
                .map_err(Error::Deserialize)?;
            deserialized
        } else {
            EventStorageData {
                events: BTreeMap::default(),
            }
        };
        let cache = Mutex::new(
            deserialized
                .events
                .into_iter()
                .map(|(k, v)| {
                    Ok((
                        EventId::from_str(&k)
                            .map_err(Into::into)
                            .map_err(Error::Deserialize)?,
                        Event {
                            date_time: v.date_time,
                            id: EventId::from_str(&v.id)
                                .map_err(Into::into)
                                .map_err(Error::Deserialize)?,
                            summary: v.summary,
                        },
                    ))
                })
                .collect::<Result<BTreeMap<EventId, Event>, Error>>()?,
        );
        Ok(Self { cache, path })
    }
}

#[cfg(feature = "fs")]
#[tonic::async_trait]
impl EventStorage for FileSystemEventStorage {
    async fn find(&self, event_id: &EventId) -> Result<Option<Event>, Error> {
        let data = self.cache.lock().await;
        let event = data.get(event_id).cloned();
        Ok(event)
    }

    async fn find_all(&self) -> Result<Vec<Event>, Error> {
        let data = self.cache.lock().await;
        let events = data.values().cloned().collect::<Vec<Event>>();
        Ok(events)
    }

    async fn store(&self, event: Event) -> Result<(), Error> {
        let mut deserialized = if tokio::fs::try_exists(self.path.as_path())
            .await
            .map_err(Into::into)
            .map_err(Error::Read)?
        {
            let stored = tokio::fs::read_to_string(self.path.as_path())
                .await
                .map_err(Into::into)
                .map_err(Error::Read)?;
            let deserialized = serde_json::from_str::<EventStorageData>(&stored)
                .map_err(Into::into)
                .map_err(Error::Deserialize)?;
            deserialized
        } else {
            EventStorageData {
                events: BTreeMap::default(),
            }
        };
        deserialized.events.insert(
            event.id.to_string(),
            EventData {
                date_time: event.date_time.clone(),
                id: event.id.to_string(),
                summary: event.summary.clone(),
            },
        );
        let serialized = serde_json::to_string(&deserialized)
            .map_err(Into::into)
            .map_err(Error::Serialize)?;
        tokio::fs::write(self.path.as_path(), serialized.as_bytes())
            .await
            .map_err(Into::into)
            .map_err(Error::Write)?;
        let mut data = self.cache.lock().await;
        data.insert(event.id, event);
        Ok(())
    }
}

struct Server {
    event_storage: Box<dyn EventStorage + Send + Sync>,
}

#[tonic::async_trait]
impl grpcal::grpcal_service_server::GrpcalService for Server {
    #[tracing::instrument(skip(self))]
    async fn create_event(
        &self,
        request: tonic::Request<grpcal::CreateEventRequest>,
    ) -> Result<tonic::Response<grpcal::CreateEventResponse>, tonic::Status> {
        let grpcal::CreateEventRequest { date_time, summary } = request.into_inner();
        let event = Event {
            date_time,
            id: EventId::new(),
            summary,
        };
        self.event_storage
            .store(event.clone())
            .await
            .map_err(|_| tonic::Status::unavailable("event storage store"))?;
        Ok(tonic::Response::new(grpcal::CreateEventResponse::from(
            event,
        )))
    }

    #[tracing::instrument(skip(self))]
    async fn get_event(
        &self,
        request: tonic::Request<grpcal::GetEventRequest>,
    ) -> Result<tonic::Response<grpcal::GetEventResponse>, tonic::Status> {
        let grpcal::GetEventRequest { id } = request.into_inner();
        let id = EventId::from_str(&id).map_err(|_| tonic::Status::invalid_argument("id"))?;
        let event = self
            .event_storage
            .find(&id)
            .await
            .map_err(|_| tonic::Status::unavailable("event storage find"))?;
        event
            .map(grpcal::GetEventResponse::from)
            .map(tonic::Response::new)
            .ok_or_else(|| tonic::Status::not_found("event not found"))
    }

    #[tracing::instrument(skip(self))]
    async fn list_events(
        &self,
        _request: tonic::Request<grpcal::ListEventsRequest>,
    ) -> Result<tonic::Response<grpcal::ListEventsResponse>, tonic::Status> {
        let events = self
            .event_storage
            .find_all()
            .await
            .map_err(|_| tonic::Status::unavailable("event storage find_all"))?;
        let events = events
            .into_iter()
            .map(grpcal::Event::from)
            .collect::<Vec<grpcal::Event>>();
        Ok(tonic::Response::new(grpcal::ListEventsResponse { events }))
    }
}

#[derive(clap::Parser)]
struct Cli {
    #[arg(long)]
    endpoint: Option<String>,
    #[command(subcommand)]
    subcommand: Subcommand,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    /// Create an event
    Create(CreateSubcommand),
    /// Get the event
    Get(GetSubcommand),
    /// List events
    List,
    /// Run server
    Server,
}

#[derive(clap::Args)]
struct CreateSubcommand {
    #[arg(long)]
    date_time: String,
    #[arg(long)]
    summary: String,
}

#[derive(clap::Args)]
struct GetSubcommand {
    id: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = <Cli as clap::Parser>::parse();

    match cli.subcommand {
        Subcommand::Create(CreateSubcommand { date_time, summary }) => {
            let endpoint = cli
                .endpoint
                // TODO: read from config
                .unwrap_or_else(|| "http://localhost:3000".to_owned());
            let channel = tonic::transport::Endpoint::from_shared(endpoint)?
                .connect()
                .await?;
            let mut client = grpcal::grpcal_service_client::GrpcalServiceClient::new(channel);
            let response = client
                .create_event(grpcal::CreateEventRequest { date_time, summary })
                .await?;
            let event = response.into_inner().event.context("event not found")?;
            println!("{} {} {}", event.id, event.date_time, event.summary);
        }
        Subcommand::Get(GetSubcommand { id }) => {
            let endpoint = cli
                .endpoint
                // TODO: read from config
                .unwrap_or_else(|| "http://localhost:3000".to_owned());
            let channel = tonic::transport::Endpoint::from_shared(endpoint)?
                .connect()
                .await?;
            let mut client = grpcal::grpcal_service_client::GrpcalServiceClient::new(channel);
            let response = client.get_event(grpcal::GetEventRequest { id }).await?;
            let event = response.into_inner().event.context("event not found")?;
            println!("{} {} {}", event.id, event.date_time, event.summary);
        }
        Subcommand::List => {
            let endpoint = cli
                .endpoint
                // TODO: read from config
                .unwrap_or_else(|| "http://localhost:3000".to_owned());
            let channel = tonic::transport::Endpoint::from_shared(endpoint)?
                .connect()
                .await?;
            let mut client = grpcal::grpcal_service_client::GrpcalServiceClient::new(channel);
            let response = client.list_events(grpcal::ListEventsRequest {}).await?;
            for event in response.into_inner().events {
                println!("{} {} {}", event.id, event.date_time, event.summary);
            }
        }
        Subcommand::Server => {
            tracing_subscriber::registry()
                .with(tracing_subscriber::EnvFilter::from_default_env())
                .with(tracing_subscriber::fmt::layer().with_span_events(
                    tracing_subscriber::fmt::format::FmtSpan::NEW
                        | tracing_subscriber::fmt::format::FmtSpan::CLOSE,
                ))
                .init();
            tonic::transport::Server::builder()
                .trace_fn(|_http_request| tracing::info_span!("info_span"))
                .add_service(grpcal::grpcal_service_server::GrpcalServiceServer::new(
                    Server {
                        event_storage: new_event_storage().await?,
                    },
                ))
                .serve("0.0.0.0:3000".parse()?)
                .await?;
        }
    }
    Ok(())
}

#[cfg(feature = "fs")]
async fn new_event_storage() -> anyhow::Result<Box<dyn EventStorage + Send + Sync>> {
    Ok(Box::new(
        FileSystemEventStorage::new(std::path::PathBuf::from("./events.json")).await?,
    ))
}

#[cfg(feature = "memory")]
async fn new_event_storage() -> anyhow::Result<Box<dyn EventStorage + Send + Sync>> {
    Ok(Box::new(InMemoryEventStorage::new()))
}
