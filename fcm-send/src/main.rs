mod fcm_client;

use std::collections::HashMap;

use fcm_client::{send, FcmClient, Message, WebpushConfig, WebpushConfigNotification};

#[derive(clap::Parser)]
struct Args {
    // GOOGLE_APPLICATION_CREDENTIALS
    #[arg(long)]
    body: Option<String>,
    #[arg(long)]
    data: Option<Vec<String>>,
    #[arg(long)]
    icon: Option<String>,
    #[arg(long)]
    title: String,
    #[arg(env, long)]
    token: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = <Args as clap::Parser>::parse();
    let client = FcmClient::new().await?;
    let response = client
        .send(
            send::PathParameters {
                parent: format!("projects/{}", client.project_id()),
            },
            send::RequestBody {
                message: Message {
                    webpush: Some(WebpushConfig {
                        notification: Some(WebpushConfigNotification {
                            body: args.body,
                            data: args.data.map(|data| {
                                data.into_iter()
                                    .filter_map(|s| {
                                        s.split_once('=').map(|(k, v)| (k.to_owned(), v.to_owned()))
                                    })
                                    .collect::<HashMap<String, String>>()
                            }),
                            icon: args.icon,
                            require_interaction: Some(true),
                            title: Some(args.title),
                            ..Default::default()
                        }),
                    }),
                    token: Some(args.token),
                    ..Default::default()
                },
            },
        )
        .await?;
    println!(
        "{}",
        response
            .0
            .name
            .expect("the response to include the name property")
    );
    Ok(())
}
