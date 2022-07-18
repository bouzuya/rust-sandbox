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
    let response_body = response.body();
    let attendance_table = parse_my_page_attendances_html(&response_body);
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
    let response = client.get(url, &[("Cookie", cookie)])?;
    Ok(response)
}

#[derive(Debug)]
struct AttendanceTable {
    rows: Vec<AttendanceTableRow>,
}

impl AttendanceTable {
    fn new(rows: Vec<AttendanceTableRow>) -> Self {
        Self { rows }
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

    let selector = Selector::parse(r#"#app"#).unwrap();
    let selected = document.select(&selector).next().unwrap();
    let app = selected.value();
    let _aggregation_tables_props = app
        .attr("data-aggregation-tables-props")
        .unwrap()
        .to_string();
    let daily_attendances_table_props = app.attr("data-daily-attendances-table-props").unwrap();
    let rows = parse_daily_attendances_table_props(daily_attendances_table_props);
    let _table_navigation_props = app.attr("data-table-navigation-props").unwrap().to_string();
    AttendanceTable::new(rows)
}

fn parse_daily_attendances_table_props(s: &str) -> Vec<AttendanceTableRow> {
    #[derive(Debug, serde::Deserialize)]
    struct DailyAttendancesTableProps {
        table_data: DailyAttendancesTablePropsTableData,
    }
    #[derive(Debug, serde::Deserialize)]
    struct DailyAttendancesTablePropsTableData {
        rows: Vec<DailyAttendancesTablePropsTableDataRow>,
    }
    #[derive(Debug, serde::Deserialize)]
    struct DailyAttendancesTablePropsTableDataRow {
        clock_in_times: Vec<String>,
        clock_out_times: Vec<String>,
        date_string: String,
        day_type: String,
        // display_pattern: DailyAttendancesTablePropsTableDataRowDisplayPattern,
        // prescribed_working_time: String,  // 所定
        // total_break_time: Option<String>, // 休憩
        total_working_time: Option<String>, // 総労働
    }
    // #[derive(Debug, serde::Deserialize)]
    // struct DailyAttendancesTablePropsTableDataRowDisplayPattern {
    //     name: Option<String>,
    // }

    let props: DailyAttendancesTableProps =
        serde_json::from_str::<'_, DailyAttendancesTableProps>(s).unwrap();
    let mut rows = vec![];
    for row in props.table_data.rows {
        rows.push(AttendanceTableRow::new(
            row.date_string,
            row.day_type,
            row.clock_in_times
                .get(0)
                .map(|s| s.to_owned())
                .unwrap_or_default(),
            row.clock_out_times
                .get(0)
                .map(|s| s.to_owned())
                .unwrap_or_default(),
            row.total_working_time.unwrap_or_default(),
        ));
    }
    rows
}

fn print_attendance_table(attendance_table: &AttendanceTable) {
    for row in attendance_table.rows.iter() {
        println!(
            "{:>2} {:8} {} {} {}",
            row.day, row.classification, row.clock_in, row.clock_out, row.hour
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let json = include_str!("test_daily_attendances_table_props.json");
        parse_daily_attendances_table_props(json);
        Ok(())
    }
}
