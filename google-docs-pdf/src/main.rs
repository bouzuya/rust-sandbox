mod google_docs_client;
mod google_drive_client;
mod token_source;

use anyhow::Context;

use crate::{
    google_docs_client::{v1::documents::Document, GoogleDocsClient},
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

    let google_drive_client = GoogleDriveClient::new(token_source.clone());
    if false {
        example_v3_files_get(&google_drive_client, &args.document_id).await?;
    }
    if false {
        example_v3_files_copy(
            &google_drive_client,
            &args.document_id,
            "new docs1",
            &args.parent_folder_id,
        )
        .await?;
    }

    let copied_document_id = {
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
        let file = serde_json::from_str::<File>(&copied)?;
        file.id.context("file.id is None")?
    };

    let google_docs_client = GoogleDocsClient::new(token_source.clone());
    let document = google_docs_client
        .v1_documents_get(&copied_document_id)
        .await?;
    // println!("{}", document);
    let document = serde_json::from_str::<Document>(&document)?;
    // println!(
    //     "{}",
    //     document
    //         .body
    //         .unwrap()
    //         .content
    //         .unwrap()
    //         .iter()
    //         .map(|e| format!("{:?}", e))
    //         .collect::<Vec<String>>()
    //         .join("\n")
    // );
    // println!("{:?}", document);

    let body_content = document
        .body
        .context("document.body is None")?
        .content
        .context("document.body.content is None")?;
    let matched_paragraph_element = body_content
        .iter()
        .filter_map(|structural_element| structural_element.content.as_ref())
        .filter_map(
            |structural_element_content| match structural_element_content {
                google_docs_client::v1::documents::StructuralElementContent::Paragraph(
                    paragraph,
                ) => Some(paragraph),
                _ => None,
            },
        )
        .filter_map(|paragraph| paragraph.elements.as_ref())
        .flat_map(|paragraph_elements| paragraph_elements.iter())
        .find(
            |paragraph_element| match paragraph_element.content.as_ref() {
                Some(google_docs_client::v1::documents::ParagraphElementContent::TextRun(
                    text_run,
                )) => match text_run.content.as_ref() {
                    None => false,
                    Some(s) => s.trim() == "placeholder",
                },
                _ => false,
            },
        );
    println!("{:?}", matched_paragraph_element);
    let start_index = matched_paragraph_element
        .context("matched_paragraph_element is None")?
        .start_index
        .unwrap_or(0);
    let end_index = matched_paragraph_element
        .context("matched_paragraph_element is None")?
        .end_index
        .unwrap_or(0);
    println!("{:?}, {:?}", start_index, end_index);

    // insert text
    // google_docs_client
    //     .v1_documents_batch_update(
    //         &copied_document_id,
    //         &google_docs_client::BatchUpdateRequestBody {
    //             requests: Some(vec![google_docs_client::v1::documents::request::Request {
    //                 request: Some(google_docs_client::v1::documents::request::RequestRequest::InsertText(
    //                         google_docs_client::v1::documents::request::InsertTextRequest {
    //                             text: Some("Hello, World!".to_string()),
    //                             insertion_location: Some(google_docs_client::v1::documents::request::InsertTextRequestInsertionLocation::Location(
    //                                 google_docs_client::v1::documents::request::Location {
    //                                     index: Some(index),
    //                                     segment_id: None,
    //                                 },
    //                             )),
    //                         },
    //                     )),
    //             }]),
    //         },
    //     )
    //     .await?;

    google_docs_client
        .v1_documents_batch_update(
            &copied_document_id,
            &google_docs_client::BatchUpdateRequestBody {
                requests: Some(vec![google_docs_client::v1::documents::request::Request {
                    request: Some(google_docs_client::v1::documents::request::RequestRequest::DeleteContentRange(
                            google_docs_client::v1::documents::request::DeleteContentRangeRequest{
                                range: Some(google_docs_client::v1::documents::Range {
                                    segment_id: None,
                                    start_index: Some(start_index),
                                    end_index: Some(end_index - 1 /* 1 = '\n' */),
                                }),
                            },
                        )),
                }]),
            },
        )
        .await?;

    let pdf = google_drive_client
        .v3_files_export(&copied_document_id, "application/pdf")
        .await?;
    std::io::Write::write_all(&mut std::fs::File::create(&args.output)?, &pdf)?;
    Ok(())
}

async fn example_v3_files_copy(
    google_drive_client: &GoogleDriveClient,
    file_id: &str,
    name: &str,
    parent_folder_id: &str,
) -> anyhow::Result<()> {
    let copied = google_drive_client
        .v3_files_copy(
            file_id,
            &File {
                name: Some(name.to_string()),
                parents: Some(vec![parent_folder_id.to_string()]),
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
    println!("{}", copied);
    let copied = serde_json::from_str::<File>(&copied)?;
    println!("{:#?}", copied);
    Ok(())
}

async fn example_v3_files_get(
    google_drive_client: &GoogleDriveClient,
    file_id: &str,
) -> anyhow::Result<()> {
    let file = google_drive_client.v3_files_get(file_id).await?;
    // {
    //   "kind": "drive#file",
    //   "id": "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
    //   "name": "docs1",
    //   "mimeType": "application/vnd.google-apps.document"
    // }
    println!("{}", file);
    let file = serde_json::from_str::<File>(&file)?;
    println!("{:#?}", file);
    Ok(())
}
