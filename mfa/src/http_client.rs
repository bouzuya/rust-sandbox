use anyhow::Result;
use reqwest::{
    blocking::{Client, Request, Response},
    redirect::Policy,
};

#[derive(Debug)]
pub struct HttpClient(Client);

impl HttpClient {
    pub fn new() -> Result<Self> {
        let client = Client::builder().redirect(Policy::none()).build()?;
        Ok(Self(client))
    }

    pub fn execute(&self, request: HttpRequest) -> Result<HttpResponse> {
        let response = self.0.execute(request.0)?;
        Ok(HttpResponse::of(response)?)
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
pub struct HttpResponse {
    body: String,
    cookie: String,
    status: u16,
}

impl HttpResponse {
    fn of(response: Response) -> Result<Self> {
        let cookie = response
            .cookies()
            .map(|c| format!("{}={}", c.name(), c.value()))
            .collect::<Vec<String>>()
            .join("; ");
        let status = response.status().as_u16();
        let body = response.text()?;
        Ok(Self {
            body,
            cookie,
            status,
        })
    }

    pub fn body(&self) -> String {
        self.body.to_string()
    }

    pub fn cookie(&self) -> String {
        self.cookie.to_string()
    }

    pub fn status(&self) -> u16 {
        self.status
    }
}
