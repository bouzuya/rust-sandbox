use std::collections::BTreeMap;

use tokio::sync::Mutex;
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};

mod grpcal {
    tonic::include_proto!("grpcal");
}

#[derive(Clone, Debug)]
struct Event {
    date_time: String,
    id: String,
    summary: String,
}

impl From<Event> for grpcal::CreateEventResponse {
    fn from(
        Event {
            date_time,
            id,
            summary,
        }: Event,
    ) -> Self {
        Self {
            date_time,
            id,
            summary,
        }
    }
}

impl From<Event> for grpcal::GetEventResponse {
    fn from(
        Event {
            date_time,
            id,
            summary,
        }: Event,
    ) -> Self {
        Self {
            date_time,
            id,
            summary,
        }
    }
}

#[derive(Debug)]
struct Server {
    data: Mutex<BTreeMap<String, Event>>,
}

#[tonic::async_trait]
impl grpcal::grpcal_server::Grpcal for Server {
    #[tracing::instrument(skip(self))]
    async fn create_event(
        &self,
        request: tonic::Request<grpcal::CreateEventRequest>,
    ) -> Result<tonic::Response<grpcal::CreateEventResponse>, tonic::Status> {
        let grpcal::CreateEventRequest { date_time, summary } = request.into_inner();
        let mut data = self.data.lock().await;
        let id = data.len().to_string();
        let event = Event {
            date_time,
            id,
            summary,
        };
        data.insert(event.id.clone(), event.clone());
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
        let data = self.data.lock().await;
        data.get(&id)
            .cloned()
            .map(grpcal::GetEventResponse::from)
            .map(tonic::Response::new)
            .ok_or_else(|| tonic::Status::not_found("event not found"))
    }

    #[tracing::instrument(skip(self))]
    async fn hello(
        &self,
        request: tonic::Request<grpcal::HelloRequest>,
    ) -> Result<tonic::Response<grpcal::HelloResponse>, tonic::Status> {
        let grpcal::HelloRequest { name } = request.into_inner();
        let message = format!("Hello, {}!", name);
        Ok(tonic::Response::new(grpcal::HelloResponse { message }))
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer().with_span_events(
            tracing_subscriber::fmt::format::FmtSpan::NEW
                | tracing_subscriber::fmt::format::FmtSpan::CLOSE,
        ))
        .init();

    tonic::transport::Server::builder()
        .trace_fn(|_http_request| tracing::info_span!("info_span"))
        .add_service(grpcal::grpcal_server::GrpcalServer::new(Server {
            data: Default::default(),
        }))
        .serve("0.0.0.0:3000".parse().unwrap())
        .await
        .unwrap()
}
