use anyhow::{ensure, Context, Result};
use dirs::cache_dir;
use std::{
    fs::{create_dir_all, read_to_string},
    path::PathBuf,
};

use crate::http_client::{HttpClient, HttpResponse};

pub fn list() -> Result<()> {
    let session_file = get_session_file()?;
    let cookie = read_to_string(session_file)?;
    let response = get_my_page_attendances(&cookie)?;
    let attendance_table = parse_my_page_attendances_html(&response.body());
    print_attendance_table(&attendance_table);
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

fn get_my_page_attendances(cookie: &str) -> Result<HttpResponse> {
    let url = "https://attendance.moneyforward.com/my_page/attendances";
    let client = HttpClient::new()?;
    let response = client.get(url, &[("Cookie", &cookie)])?;
    Ok(response)
}

#[derive(Debug)]
struct AttendanceTable {
    month_range: String,
    rows: Vec<AttendanceTableRow>,
}

impl AttendanceTable {
    fn new(month_range: String, rows: Vec<AttendanceTableRow>) -> Self {
        Self { month_range, rows }
    }
}

#[derive(Debug)]
struct AttendanceTableRow {
    day: String,
    classification: String,
    clock_in: String,
    clock_out: String,
    hour: String,
}

impl AttendanceTableRow {
    fn new(
        day: String,
        classification: String,
        clock_in: String,
        clock_out: String,
        hour: String,
    ) -> Self {
        Self {
            day,
            classification,
            clock_in,
            clock_out,
            hour,
        }
    }
}

fn parse_my_page_attendances_html(s: &str) -> AttendanceTable {
    use scraper::{Html, Selector};

    let document = Html::parse_document(s);

    let month_range = {
        let selector = Selector::parse(r#"div.attendance-table-header-month-range"#).unwrap();
        let selected = document.select(&selector).next().unwrap();

        let text = selected.text().collect::<String>();
        text
    };

    let mut rows = vec![];
    let tr_selector =
        Selector::parse(r#"tr.attendance-table-row-,tr.attendance-table-row-error"#).unwrap();
    for tr in document.select(&tr_selector) {
        let day = {
            let day_td_selector = Selector::parse(r#"td.column-day"#).unwrap();
            let day_td = tr.select(&day_td_selector).next().unwrap();
            let text_day_selector = Selector::parse(r#"span.attendance-table-text-day"#).unwrap();
            let text_day = day_td.select(&text_day_selector).next().unwrap();
            let day = text_day.text().collect::<String>();
            day
        };

        let classification = {
            let classification_td_selector =
                Selector::parse(r#"td.column-classification"#).unwrap();
            let classification_td = tr.select(&classification_td_selector).next().unwrap();
            let classification = classification_td.text().collect::<String>();
            classification
        };

        let (clock_in, clock_out) = {
            let attendance_td_selector = Selector::parse(r#"td.column-attendance"#).unwrap();
            let mut select = tr.select(&attendance_td_selector);
            let clock_in_td = select.next().unwrap();
            let clock_in = clock_in_td.text().collect::<String>();
            let clock_out_td = select.next().unwrap();
            let clock_out = clock_out_td.text().collect::<String>();
            (clock_in, clock_out)
        };

        let hour = {
            let hour_td_selector = Selector::parse(r#"td.column-hour"#).unwrap();
            let hour_td = tr.select(&hour_td_selector).next().unwrap();
            let hour = hour_td.text().collect::<String>();
            hour
        };

        let row = AttendanceTableRow::new(day, classification, clock_in, clock_out, hour);

        rows.push(row);
    }

    AttendanceTable::new(month_range, rows)
}

fn print_attendance_table(attendance_table: &AttendanceTable) {
    println!("{}", attendance_table.month_range);
    for row in attendance_table.rows.iter() {
        println!(
            "{:>2} {:8} {} {} {}",
            row.day, row.classification, row.clock_in, row.clock_out, row.hour
        );
    }
}
