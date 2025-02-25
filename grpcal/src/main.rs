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

#[derive(Debug)]
struct EventStorage {
    data: Mutex<BTreeMap<EventId, Event>>,
}

impl EventStorage {
    async fn find(&self, event_id: &EventId) -> Option<Event> {
        let data = self.data.lock().await;
        data.get(event_id).cloned()
    }

    async fn find_all(&self) -> Vec<Event> {
        let data = self.data.lock().await;
        data.values().cloned().collect::<Vec<Event>>()
    }

    async fn store(&self, event: Event) {
        let mut data = self.data.lock().await;
        data.insert(event.id, event);
    }
}

#[derive(Debug)]
struct Server {
    event_storage: EventStorage,
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
        self.event_storage.store(event.clone()).await;
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
        let event = self.event_storage.find(&id).await;
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
        let events = self.event_storage.find_all().await;
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
                        data: Default::default(),
                    },
                ))
                .serve("0.0.0.0:3000".parse()?)
                .await?;
        }
    }
    Ok(())
}
