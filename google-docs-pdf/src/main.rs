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
                    Some(s) => s == "placeholder\n",
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

    let mut request_body = google_docs_client::BatchUpdateRequestBody {
        requests: Some(vec![]),
    };

    // delete placeholder without "\n"
    delete_content(&mut request_body, start_index, end_index - 1)?;
    let mut index = start_index;
    index = insert_text(&mut request_body, index, "Hello, World!\n")?;
    index = insert_text(&mut request_body, index, "あいうえお\n")?;
    index = insert_text(&mut request_body, index, "Hi\n")?;
    index = insert_inline_image(
        &mut request_body,
        index,
        "https://blog.bouzuya.net/images/favicon.png",
        100,
        100,
    )?;
    index = insert_text(&mut request_body, index, "\n")?;
    index = insert_inline_image(
        &mut request_body,
        index,
        "https://blog.bouzuya.net/images/favicon.png",
        50,
        100,
    )?;
    // remove last "\n"
    delete_content(&mut request_body, index, index + 1)?;
    replace_all_text(&mut request_body, "placeholder2", "Good bye, World!")?;

    google_docs_client
        .v1_documents_batch_update(&copied_document_id, &request_body)
        .await?;

    let pdf = google_drive_client
        .v3_files_export(&copied_document_id, "application/pdf")
        .await?;
    std::io::Write::write_all(&mut std::fs::File::create(&args.output)?, &pdf)?;
    Ok(())
}

fn delete_content(
    request_body: &mut google_docs_client::BatchUpdateRequestBody,
    start_index: usize,
    end_index: usize,
) -> anyhow::Result<()> {
    use google_docs_client::v1::documents::{
        request::{DeleteContentRangeRequest, Request, RequestRequest},
        Range,
    };
    request_body
        .requests
        .as_mut()
        .context("requests is None")?
        .push(Request {
            request: Some(RequestRequest::DeleteContentRange(
                DeleteContentRangeRequest {
                    range: Some(Range {
                        segment_id: None,
                        start_index: Some(start_index),
                        end_index: Some(end_index),
                    }),
                },
            )),
        });
    Ok(())
}

fn insert_inline_image(
    request_body: &mut google_docs_client::BatchUpdateRequestBody,
    index: usize,
    uri: &str,
    height: usize,
    width: usize,
) -> anyhow::Result<usize> {
    use google_docs_client::v1::documents::{
        request::{
            InsertInlineImageRequest, InsertInlineImageRequestInsertionLocation, Location, Request,
            RequestRequest,
        },
        Dimension, Size, Unit,
    };
    request_body
        .requests
        .as_mut()
        .context("requests is None")?
        .push(Request {
            request: Some(RequestRequest::InsertInlineImage(
                InsertInlineImageRequest {
                    uri: Some(uri.to_string()),
                    object_size: Some(Size {
                        height: Some(Dimension {
                            magnitude: Some(serde_json::Number::from(height)),
                            unit: Unit::Pt,
                        }),
                        width: Some(Dimension {
                            magnitude: Some(serde_json::Number::from(width)),
                            unit: Unit::Pt,
                        }),
                    }),
                    insertion_location: Some(InsertInlineImageRequestInsertionLocation::Location(
                        Location {
                            index: Some(index),
                            segment_id: None,
                        },
                    )),
                },
            )),
        });
    Ok(index + 1)
}

fn insert_text(
    request_body: &mut google_docs_client::BatchUpdateRequestBody,
    index: usize,
    text: &str,
) -> anyhow::Result<usize> {
    use google_docs_client::v1::documents::request::{
        InsertTextRequest, InsertTextRequestInsertionLocation, Location, Request, RequestRequest,
    };
    request_body
        .requests
        .as_mut()
        .context("requests is None")?
        .push(Request {
            request: Some(RequestRequest::InsertText(InsertTextRequest {
                text: Some(text.to_string()),
                insertion_location: Some(InsertTextRequestInsertionLocation::Location(Location {
                    index: Some(index),
                    segment_id: None,
                })),
            })),
        });
    Ok(index + text.chars().count())
}

fn replace_all_text(
    request_body: &mut google_docs_client::BatchUpdateRequestBody,
    search: &str,
    replace_text: &str,
) -> anyhow::Result<()> {
    use google_docs_client::v1::documents::request::{
        ReplaceAllTextRequest, ReplaceAllTextRequestCriteria, Request, RequestRequest,
        SubstringMatchCriteria,
    };
    request_body
        .requests
        .as_mut()
        .context("requests is None")?
        .push(Request {
            request: Some(RequestRequest::ReplaceAllText(ReplaceAllTextRequest {
                replace_text: Some(replace_text.to_string()),
                criteria: Some(ReplaceAllTextRequestCriteria::ContainsText(
                    SubstringMatchCriteria {
                        text: Some(search.to_string()),
                        match_case: None,
                    },
                )),
            })),
        });
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
