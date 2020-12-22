use crate::http_client::{HttpClient, HttpResponse};
use anyhow::{bail, Context, Result};

pub fn log_in() -> Result<()> {
    let response = get_employee_session_new()?;
    let html = response.body();
    let authenticity_token =
        parse_employee_session_new_html(&html).with_context(|| "no authenticity_token")?;
    let employee_session_form = EmployeeSessionForm::from_input()?;
    let response = post_employee_session(
        &response.cookie(),
        &authenticity_token,
        &employee_session_form,
    )?;
    println!("{:?}", response);
    Ok(())
}

fn get_employee_session_new() -> Result<HttpResponse> {
    let url = "https://attendance.moneyforward.com/employee_session/new";
    let client = HttpClient::new()?;
    let response = client.get(url)?;
    if response.status() != 200 {
        bail!("get_employee_session_new status: {}", response.status())
    }
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

#[derive(Debug)]
struct EmployeeSessionForm {
    office_account_name: String,
    account_name_or_email: String,
    password: String,
}

impl EmployeeSessionForm {
    fn from_input() -> Result<Self> {
        let office_account_name = dialoguer::Input::<String>::new()
            .with_prompt("office_account_name")
            .interact()?;
        let account_name_or_email = dialoguer::Input::<String>::new()
            .with_prompt("account_name_or_email")
            .interact()?;
        let password = dialoguer::Password::new()
            .with_prompt("password")
            .interact()?;
        Ok(Self {
            office_account_name,
            account_name_or_email,
            password,
        })
    }
}

fn post_employee_session(
    cookie: &str,
    authenticity_token: &str,
    employee_session_form: &EmployeeSessionForm,
) -> Result<HttpResponse> {
    let url = "https://attendance.moneyforward.com/employee_session";
    let client = HttpClient::new()?;
    let body = [
        ("authenticity_token", authenticity_token),
        (
            "employee_session_form[office_account_name]",
            &employee_session_form.office_account_name,
        ),
        (
            "employee_session_form[account_name_or_email]",
            &employee_session_form.account_name_or_email,
        ),
        (
            "employee_session_form[password]",
            &employee_session_form.password,
        ),
    ];
    let response = client.post(url, cookie, &body)?;
    if response.status() != 302 {
        bail!("post_employee_session status: {}", response.status())
    }
    Ok(response)
}
