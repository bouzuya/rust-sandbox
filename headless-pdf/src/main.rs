use std::collections::HashMap;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("initialize browser")]
    InitializeBrowser(#[source] anyhow::Error),
    #[error("navigate to")]
    NavigateTo(#[source] anyhow::Error),
    #[error("new tab")]
    NewTab(#[source] anyhow::Error),
    #[error("print to pdf")]
    PrintToPdf(#[source] anyhow::Error),
    #[error("render template")]
    RenderTemplate(#[source] handlebars::RenderError),
    #[error("wait until navigated")]
    WaitUntilNavigated(#[source] anyhow::Error),
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Error::InitializeBrowser(_)
            | Error::NavigateTo(_)
            | Error::NewTab(_)
            | Error::PrintToPdf(_)
            | Error::RenderTemplate(_)
            | Error::WaitUntilNavigated(_) => {
                axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

async fn html_show(
    axum::extract::State(App { db }): axum::extract::State<App>,
    axum::extract::Path((id,)): axum::extract::Path<(String,)>,
) -> Result<axum::response::Html<String>, Error> {
    let db = db.lock().await;
    let html = db.get(&id).unwrap();
    Ok(axum::response::Html(html.to_owned()))
}

#[derive(serde::Deserialize)]
struct CreatePdfRequestBody {
    data: serde_json::Value,
    template: String,
}

async fn pdf_create(
    axum::extract::State(App { db }): axum::extract::State<App>,
    axum::Json(CreatePdfRequestBody { data, template }): axum::Json<CreatePdfRequestBody>,
) -> Result<Vec<u8>, Error> {
    let id = {
        let handlebars = handlebars::Handlebars::new();
        let html = handlebars
            .render_template(&template, &data)
            .map_err(Error::RenderTemplate)?;
        let id = {
            let mut bytes = [0u8; 96];
            rand::RngCore::fill_bytes(&mut rand::rngs::OsRng, &mut bytes);
            let encoded = hex::encode(&bytes);
            encoded
        };
        let mut db = db.lock().await;
        db.insert(id.clone(), html);
        id
    };

    let browser = headless_chrome::Browser::default().map_err(Error::InitializeBrowser)?;
    let tab = browser.new_tab().map_err(Error::NewTab)?;
    tab.navigate_to(&format!("http://localhost:3000/htmls/{}", id))
        .map_err(Error::NavigateTo)?;
    tab.wait_until_navigated()
        .map_err(Error::WaitUntilNavigated)?;
    let pdf = tab.print_to_pdf(None).map_err(Error::PrintToPdf)?;

    {
        let mut db = db.lock().await;
        db.remove(&id);
    }

    Ok(pdf)
}

#[derive(Clone, Default)]
struct App {
    db: std::sync::Arc<tokio::sync::Mutex<HashMap<String, String>>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let router = axum::Router::new()
        .route("/", axum::routing::get(|| async { "OK" }))
        .route("/htmls/{id}", axum::routing::get(html_show))
        .route("/pdfs", axum::routing::post(pdf_create))
        .with_state(App::default());
    let listener = tokio::net::TcpListener::bind(("0.0.0.0", 3000_u16)).await?;
    axum::serve(listener, router).await?;
    Ok(())
}
