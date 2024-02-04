use std::{
    collections::HashMap,
    ops::{Add, Range},
    path::Path,
    str::FromStr as _,
};

use chrono::{prelude::Utc, DateTime, Duration};
use firestore_path::{DatabaseId, DatabaseName, ProjectId};
use google_api_proto::google::firestore::v1::{
    firestore_client::FirestoreClient, value::ValueType, ListDocumentsRequest, MapValue, Value,
};
use google_authz::{Credentials, GoogleAuthz};
use tonic::transport::Channel;

#[derive(Debug, clap::Parser)]
struct Args {
    account_id: String,
    #[arg(env, long)]
    google_application_credentials: String,
    #[arg(long)]
    project_id: String,
    #[arg(long)]
    yyyy_mm: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = <Args as clap::Parser>::parse();

    let date_time_range = build_date_time_range(&cli.yyyy_mm)?;
    let events = list_event_documents(
        &cli.google_application_credentials,
        &cli.project_id,
        &cli.account_id,
    )
    .await?;

    let (categories, transactions) = build_state(events)?;
    let filtered = filter_and_sort_transactions(transactions, &date_time_range);

    let message = filtered
        .into_iter()
        .map(|(date, _, amount, category_id, comment)| {
            let category_name = categories.get(&category_id).expect("category_id contains");
            format!("{}\t{}\t{}\t{}", date, amount, category_name, comment)
        })
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", message);

    Ok(())
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
enum EventDocumentData {
    #[serde(rename_all = "camelCase")]
    AccountCreated { at: String },
    #[serde(rename_all = "camelCase")]
    AccountDeleted { at: String },
    #[serde(rename_all = "camelCase")]
    AccountUpdated { at: String },
    #[serde(rename_all = "camelCase")]
    CategoryAdded {
        at: String,
        category_id: String,
        name: String,
    },
    #[serde(rename_all = "camelCase")]
    CategoryDeleted { at: String, category_id: String },
    #[serde(rename_all = "camelCase")]
    CategoryUpdated {
        at: String,
        category_id: String,
        name: String,
    },
    #[serde(rename_all = "camelCase")]
    OwnerAdded { at: String },
    #[serde(rename_all = "camelCase")]
    OwnerRemoved { at: String },
    #[serde(rename_all = "camelCase")]
    TransactionAdded {
        amount: String,
        at: String,
        category_id: String,
        comment: String,
        date: String,
        transaction_id: String,
    },
    #[serde(rename_all = "camelCase")]
    TransactionDeleted { at: String, transaction_id: String },
    #[serde(rename_all = "camelCase")]
    TransactionUpdated {
        amount: String,
        at: String,
        category_id: String,
        comment: String,
        date: String,
        transaction_id: String,
    },
}

impl EventDocumentData {
    fn at(&self) -> DateTime<Utc> {
        DateTime::parse_from_rfc3339(match self {
            EventDocumentData::AccountCreated { at }
            | EventDocumentData::AccountDeleted { at }
            | EventDocumentData::AccountUpdated { at }
            | EventDocumentData::CategoryAdded { at, .. }
            | EventDocumentData::CategoryDeleted { at, .. }
            | EventDocumentData::CategoryUpdated { at, .. }
            | EventDocumentData::OwnerAdded { at }
            | EventDocumentData::OwnerRemoved { at }
            | EventDocumentData::TransactionAdded { at, .. }
            | EventDocumentData::TransactionDeleted { at, .. }
            | EventDocumentData::TransactionUpdated { at, .. } => at,
        })
        .expect("at to be rfc3339 format")
        .to_utc()
    }
}

type Transaction = (String, String, String, String, String);

async fn build_firestore_client(
    google_application_credentials: &str,
) -> anyhow::Result<FirestoreClient<GoogleAuthz<Channel>>> {
    let channel = Channel::from_static("https://firestore.googleapis.com")
        .connect()
        .await?;
    let credentials = Credentials::builder()
        .json_file(Path::new(&google_application_credentials))
        .build()
        .await?;
    let google_authz = GoogleAuthz::builder(channel)
        .credentials(credentials)
        .build()
        .await;
    let client = FirestoreClient::new(google_authz);
    Ok(client)
}

fn build_date_time_range(yyyy_mm: &str) -> anyhow::Result<Range<DateTime<Utc>>> {
    let start_inclusive =
        DateTime::parse_from_rfc3339(&format!("{}-01T00:00:00+09:00", yyyy_mm))?.to_utc();
    let end_exclusive = {
        let d = start_inclusive.add(Duration::days(40));
        DateTime::parse_from_rfc3339(&format!("{}-01T00:00:00+09:00", d.format("%Y-%m")))?.to_utc()
    };
    Ok(start_inclusive..end_exclusive)
}

async fn list_event_documents(
    google_application_credentials: &str,
    project_id: &str,
    account_id: &str,
) -> anyhow::Result<Vec<EventDocumentData>> {
    let mut client = build_firestore_client(google_application_credentials).await?;
    let project_id = ProjectId::from_str(project_id)?;
    let database_id = DatabaseId::from_str("(default)")?;
    let database_name = DatabaseName::new(project_id, database_id);
    let event_collection_name = database_name
        .collection("accounts")?
        .doc(account_id)?
        .collection("events")?;
    let event_documents = client
        .list_documents(ListDocumentsRequest {
            parent: event_collection_name
                .parent()
                .map(|d| d.to_string())
                .unwrap_or_else(|| event_collection_name.root_document_name().to_string()),
            collection_id: event_collection_name.collection_id().to_string(),
            ..Default::default()
        })
        .await?
        .into_inner();
    let mut events = vec![];
    for event_document in event_documents.documents {
        let parsed = serde_firestore_value::from_value::<'_, EventDocumentData>(&Value {
            value_type: Some(ValueType::MapValue(MapValue {
                fields: event_document.fields,
            })),
        })?;
        events.push(parsed);
    }
    events.sort_by_key(|e| e.at());
    Ok(events)
}

fn build_state(
    events: Vec<EventDocumentData>,
) -> anyhow::Result<(HashMap<String, String>, HashMap<String, Transaction>)> {
    let mut categories = HashMap::new();
    let mut transactions = HashMap::new();
    for event in events.into_iter() {
        match event {
            EventDocumentData::AccountCreated { .. }
            | EventDocumentData::AccountDeleted { .. }
            | EventDocumentData::AccountUpdated { .. }
            | EventDocumentData::OwnerAdded { .. }
            | EventDocumentData::OwnerRemoved { .. } => {
                // do nothing
            }
            EventDocumentData::CategoryAdded {
                category_id, name, ..
            } => {
                categories.insert(category_id, name);
            }
            EventDocumentData::CategoryDeleted { category_id, .. } => {
                categories.remove(&category_id);
            }
            EventDocumentData::CategoryUpdated {
                category_id, name, ..
            } => {
                categories.insert(category_id, name);
            }
            EventDocumentData::TransactionAdded {
                amount,
                at,
                category_id,
                comment,
                date,
                transaction_id,
                ..
            } => {
                transactions.insert(transaction_id, (date, at, amount, category_id, comment));
            }
            EventDocumentData::TransactionDeleted { transaction_id, .. } => {
                transactions.remove(&transaction_id);
            }
            EventDocumentData::TransactionUpdated {
                amount,
                at,
                category_id,
                comment,
                date,
                transaction_id,
                ..
            } => {
                transactions.insert(transaction_id, (date, at, amount, category_id, comment));
            }
        }
    }
    Ok((categories, transactions))
}

fn filter_and_sort_transactions(
    transactions: HashMap<String, (String, String, String, String, String)>,
    date_time_range: &Range<DateTime<Utc>>,
) -> Vec<(String, String, String, String, String)> {
    let mut filtered = transactions
        .into_values()
        .filter(|(date, _, _, _, _)| {
            let date = DateTime::parse_from_rfc3339(&format!("{}T00:00:00+09:00", date))
                .expect("date to be YYYY-MM-DD format")
                .to_utc();
            date_time_range.contains(&date)
        })
        .collect::<Vec<_>>();
    filtered.sort();
    filtered
}
