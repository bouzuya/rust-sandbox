mod google_docs_client;
mod google_drive_client;
mod token_source;

use crate::{
    google_docs_client::GoogleDocsClient,
    google_drive_client::{File, GoogleDriveClient},
    token_source::GoogleCloudAuthTokenSource,
};

#[derive(clap::Parser)]
struct Args {
    #[clap(long, env)]
    document_id: String,
    #[clap(long, env)]
    parent_folder_id: String,
    #[clap(long, env)]
    output: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = <Args as clap::Parser>::parse();

    let token_source = GoogleCloudAuthTokenSource::new([
        "https://www.googleapis.com/auth/documents",
        "https://www.googleapis.com/auth/drive",
    ])
    .await?;

    let google_docs_client = GoogleDocsClient::new(token_source.clone());
    let document = google_docs_client
        .v1_documents_get(&args.document_id)
        .await?;
    println!("{}", document);

    let google_drive_client = GoogleDriveClient::new(token_source);

    // let file = google_drive_client.v3_files_get(&args.document_id).await?;
    // // {
    // //   "kind": "drive#file",
    // //   "id": "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
    // //   "name": "docs1",
    // //   "mimeType": "application/vnd.google-apps.document"
    // // }
    // println!("{}", file);
    // let file = serde_json::from_str::<File>(&file)?;
    // println!("{:#?}", file);

    let copied = google_drive_client
        .v3_files_copy(
            &args.document_id,
            &File {
                name: Some("new docs1".to_string()),
                parents: Some(vec![args.parent_folder_id]),
                ..Default::default()
            },
        )
        .await?;
    // {
    //   "kind": "drive#file",
    //   "id": "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
    //   "name": "new docs1",
    //   "mimeType": "application/vnd.google-apps.document"
    // }
    // println!("{}", copied);
    let file = serde_json::from_str::<File>(&copied)?;
    println!("{:#?}", file);

    let pdf = google_drive_client
        .v3_files_export(&args.document_id, "application/pdf")
        .await?;
    std::io::Write::write_all(&mut std::fs::File::create(&args.output)?, &pdf)?;
    Ok(())
}
