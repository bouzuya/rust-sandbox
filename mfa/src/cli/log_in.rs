use crate::http_client::{HttpClient, HttpMethod, HttpResponse};
use anyhow::{Context, Result};

pub fn log_in() -> Result<()> {
    let response = get_employee_session_new()?;
    let html = response.body();
    let authenticity_token =
        parse_employee_session_new_html(&html).with_context(|| "no authenticity_token")?;
    println!("{:?}", response.status());
    println!("{:?}", response.body());
    println!("{:?}", response.cookie());
    println!("{:?}", authenticity_token);
    Ok(())
}

fn get_employee_session_new() -> Result<HttpResponse> {
    let url = "https://attendance.moneyforward.com/employee_session/new";
    let client = HttpClient::new()?;
    let request = client.request(HttpMethod::GET, url)?;
    let response = client.execute(request)?;
    Ok(response)
}

fn parse_employee_session_new_html(s: &str) -> Option<String> {
    use scraper::html::Select;
    use scraper::node::Element;
    use scraper::{ElementRef, Html, Selector};

    let document = Html::parse_document(s);
    let selector = Selector::parse(r#"input[name="authenticity_token"]"#).unwrap();
    let mut select: Select = document.select(&selector);
    select.next().and_then(|input: ElementRef| {
        let element: &Element = input.value();
        let value: Option<&str> = element.attr("value");
        value.map(|s| s.to_string())
    })
}
