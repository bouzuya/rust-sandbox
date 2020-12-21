use anyhow::{Context, Result};
use reqwest::{
    blocking::{Client, Request, Response},
    redirect::Policy,
};

#[derive(Debug)]
pub struct HttpClient(Client);

impl HttpClient {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .redirect(Policy::none())
            .build()
            .with_context(|| "http client build")?;
        Ok(Self(client))
    }

    pub fn execute(&self, request: HttpRequest) -> Result<HttpResponse> {
        let response = self.0.execute(request.0)?;
        Ok(HttpResponse::of(response))
    }

    pub fn request(&self, method: HttpMethod, url: &str) -> Result<HttpRequest> {
        let request = match method {
            HttpMethod::GET => self.0.get(url).build()?,
        };
        Ok(HttpRequest::of(request))
    }
}

#[derive(Debug)]
pub enum HttpMethod {
    GET,
}

#[derive(Debug)]
pub struct HttpRequest(Request);

impl HttpRequest {
    fn of(request: Request) -> Self {
        Self(request)
    }
}

#[derive(Debug)]
pub struct HttpResponse(Response);

impl HttpResponse {
    fn of(response: Response) -> Self {
        Self(response)
    }
}
