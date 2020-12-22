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

    pub fn get(&self, url: &str, headers: &[(&str, &str)]) -> Result<HttpResponse> {
        let mut request_builder = self.0.get(url);
        for &(key, value) in headers.iter() {
            request_builder = request_builder.header(key, value);
        }
        let request = request_builder.build()?;
        let response = self.0.execute(request)?;
        Ok(HttpResponse::of(response)?)
    }

    pub fn post(
        &self,
        url: &str,
        headers: &[(&str, &str)],
        body: &[(&str, &str)],
    ) -> Result<HttpResponse> {
        let mut request_builder = self.0.post(url);
        for &(key, value) in headers.iter() {
            request_builder = request_builder.header(key, value);
        }
        let request = request_builder.form(body).build()?;
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
