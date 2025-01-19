#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let email = "".to_owned();
    let password = "".to_owned();

    let browser = headless_chrome::Browser::default()?;

    let tab = browser.new_tab()?;
    tab.set_user_agent(
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/132.0.0.0 Safari/537.36",
        Some("en-US"),
        Some("Linux x86_64")
    )?;

    tab.navigate_to("https://order.yodobashi.com/yc/login/index.html")?;
    tab.wait_until_navigated()?;

    tab.wait_for_element("input[name=\"memberId\"]")?.click()?;
    tab.type_str(&email)?;

    tab.wait_for_element("input[name=\"password\"]")?.click()?;
    tab.type_str(&password)?;

    tab.press_key("Enter")?;

    tab.wait_until_navigated()?;

    tokio::time::sleep(std::time::Duration::from_secs(10)).await;

    let pdf = tab.print_to_pdf(None)?;
    std::fs::write("screenshot.pdf", pdf)?;

    Ok(())
}
