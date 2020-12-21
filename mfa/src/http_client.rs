use anyhow::{Context, Result};
use reqwest::{
    blocking::{Client, Response},
    redirect::Policy,
};

#[derive(Debug)]
pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .redirect(Policy::none())
            .build()
            .with_context(|| "http client build")?;
        Ok(Self { client })
    }

    pub fn get(&self, url: &str) -> Result<HttpResponse> {
        let request = self.client.get(url);
        let response = request.send()?;
        Ok(HttpResponse::of(response))
    }
}

#[derive(Debug)]
pub struct HttpResponse {
    response: Response,
}

impl HttpResponse {
    fn of(response: Response) -> Self {
        Self { response }
    }
}
