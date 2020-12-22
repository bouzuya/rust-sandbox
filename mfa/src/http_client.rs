use anyhow::Result;
use reqwest::{
    blocking::{Client, Response},
    redirect::Policy,
};

#[derive(Debug)]
pub struct HttpClient(Client);

impl HttpClient {
    pub fn new() -> Result<Self> {
        let client = Client::builder().redirect(Policy::none()).build()?;
        Ok(Self(client))
    }

    pub fn get(&self, url: &str) -> Result<HttpResponse> {
        let request = self.0.get(url).build()?;
        let response = self.0.execute(request)?;
        Ok(HttpResponse::of(response)?)
    }

    pub fn post(&self, url: &str, cookie: &str, body: &[(&str, &str)]) -> Result<HttpResponse> {
        let request = self
            .0
            .post(url)
            .header("Cookie", cookie)
            .form(body)
            .build()?;
        let response = self.0.execute(request)?;
        Ok(HttpResponse::of(response)?)
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
