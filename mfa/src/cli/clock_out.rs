use crate::http_client::{HttpClient, HttpResponse};
use anyhow::{ensure, Context, Result};
use chrono::{DateTime, Utc};
use dirs::cache_dir;
use std::{
    fs::{create_dir_all, read_to_string},
    path::PathBuf,
};

pub fn clock_out() -> Result<()> {
    let session_file = get_session_file()?;
    let cookie = read_to_string(session_file)?;
    let response = get_my_page(&cookie)?;
    let html = response.body();
    let (authenticity_token, web_time_recorder_form) =
        parse_my_page(&html).with_context(|| "parse_my_page")?;
    let utc: DateTime<Utc> = Utc::now();
    let date = utc.format("%Y/%m/%d").to_string();
    let user_time = utc.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
    let web_time_recorder_form = WebTimeRecorderForm {
        event: "clock_out".to_string(),
        date,
        user_time,
        office_location_id: web_time_recorder_form.office_location_id,
    };
    let response =
        post_my_page_web_time_recorder(&cookie, &authenticity_token, &web_time_recorder_form)?;
    println!("{:?}", response);
    Ok(())
}

fn get_session_file() -> Result<PathBuf> {
    let cache_dir = cache_dir().with_context(|| "dirs::cache_dir")?;
    let app_cache_dir = cache_dir.join("rust-sandbox-mfa");
    if !app_cache_dir.is_dir() {
        ensure!(!app_cache_dir.exists(), "cache_dir is not dir");
        create_dir_all(&app_cache_dir).with_context(|| "fs::create_dir_all(cache_dir)")?;
    }
    Ok(app_cache_dir.join("session"))
}

fn get_my_page(cookie: &str) -> Result<HttpResponse> {
    let url = "https://attendance.moneyforward.com/my_page";
    let client = HttpClient::new()?;
    let response = client.get(url, &[("Cookie", &cookie)])?;
    Ok(response)
}

fn parse_my_page(s: &str) -> Option<(String, WebTimeRecorderForm)> {
    use scraper::html::Select;
    use scraper::node::Element;
    use scraper::{ElementRef, Html, Selector};

    let document = Html::parse_document(s);

    let authenticity_token = {
        let selector = Selector::parse(r#"input[name="authenticity_token"]"#).unwrap();
        let mut select: Select = document.select(&selector);
        select.next().and_then(|input: ElementRef| {
            let element: &Element = input.value();
            let value: Option<&str> = element.attr("value");
            value.map(|s| s.to_string())
        })
    }?;

    let office_location_id = {
        let selector =
            Selector::parse(r#"input[name="web_time_recorder_form[office_location_id]"]"#).unwrap();
        let mut select: Select = document.select(&selector);
        select.next().and_then(|input: ElementRef| {
            let element: &Element = input.value();
            let value: Option<&str> = element.attr("value");
            value.map(|s| s.to_string())
        })
    };

    let web_time_recorder_form = WebTimeRecorderForm {
        event: "".to_string(),
        date: "".to_string(),
        user_time: "".to_string(),
        office_location_id: office_location_id?,
    };

    Some((authenticity_token, web_time_recorder_form))
}

#[derive(Debug)]
struct WebTimeRecorderForm {
    event: String,
    date: String,
    user_time: String,
    office_location_id: String,
}

fn post_my_page_web_time_recorder(
    cookie: &str,
    authenticity_token: &str,
    web_time_recorder_form: &WebTimeRecorderForm,
) -> Result<HttpResponse> {
    let url = "https://attendance.moneyforward.com/my_page/web_time_recorder";
    let client = HttpClient::new()?;
    let body = [
        ("authenticity_token", authenticity_token),
        (
            "web_time_recorder_form[event]",
            &web_time_recorder_form.event,
        ),
        ("web_time_recorder_form[date]", &web_time_recorder_form.date),
        (
            "web_time_recorder_form[user_time]",
            &web_time_recorder_form.user_time,
        ),
        (
            "web_time_recorder_form[office_location_id]",
            &web_time_recorder_form.office_location_id,
        ),
    ];
    let response = client.post(url, &[("Cookie", cookie)], &body)?;
    ensure!(
        response.status() == 302,
        "post_employee_session status: {}",
        response.status()
    );
    Ok(response)
}
