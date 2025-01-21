fn main() {
    let browser = headless_chrome::Browser::default().unwrap();

    let tab = browser.new_tab().unwrap();

    tab.navigate_to("https://bouzuya.net/").unwrap();

    tab.wait_until_navigated().unwrap();

    let pdf = tab.print_to_pdf(None).unwrap();

    std::fs::write("test.pdf", pdf).unwrap();
}
