pub(super) async fn execute() -> anyhow::Result<()> {
    let config = crate::config::Config::load().await?;
    let page_id = crate::page_id::PageId::new();
    let path = crate::page_io::PageIo::create_page(&config, &page_id)?;
    println!("Created new page: {}", path.display());
    Ok(())
}
