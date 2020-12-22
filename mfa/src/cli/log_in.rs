use anyhow::Result;

use crate::http_client::{HttpClient, HttpMethod, HttpResponse};

pub fn log_in() -> Result<()> {
    let response = get_employee_session_new()?;
    println!("{:?}", response.status());
    println!("{:?}", response.body());
    println!("{:?}", response.cookie());
    Ok(())
}

fn get_employee_session_new() -> Result<HttpResponse> {
    let url = "https://attendance.moneyforward.com/employee_session/new";
    let client = HttpClient::new()?;
    let request = client.request(HttpMethod::GET, url)?;
    let response = client.execute(request)?;
    Ok(response)
}
