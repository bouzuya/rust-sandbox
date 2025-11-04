use anyhow::Context as _;

pub(crate) struct PageIo;

impl PageIo {
    pub(crate) fn create_page(
        config: &crate::Config,
        page_id: &crate::page_id::PageId,
    ) -> anyhow::Result<std::path::PathBuf> {
        let path_buf = Self::page_path(config, page_id);
        std::fs::create_dir_all(path_buf.parent().context("invalid path")?)?;
        std::fs::write(&path_buf, "")?;
        Ok(path_buf)
    }

    pub(crate) fn page_id(path: &std::path::Path) -> anyhow::Result<crate::page_id::PageId> {
        // check config.data_dir is a prefix of path?
        let file_stem = path.file_stem().context("file_stem")?;
        let page_id = file_stem.to_str().context("file_stem is not UTF-8")?;
        let page_id = <crate::page_id::PageId as std::str::FromStr>::from_str(page_id)
            .context("invalid ID in data dir")?;
        Ok(page_id)
    }

    fn page_path(config: &crate::Config, page_id: &crate::page_id::PageId) -> std::path::PathBuf {
        config
            .data_dir
            .join(page_id.to_string())
            .with_extension("md")
    }

    pub(crate) fn read_page_ids(
        config: &crate::Config,
    ) -> anyhow::Result<std::collections::BTreeSet<crate::page_id::PageId>> {
        let read_dir = std::fs::read_dir(&config.data_dir).context("data dir not found")?;
        let mut page_ids = std::collections::BTreeSet::new();
        for dir_entry in read_dir {
            let dir_entry = dir_entry.context("dir_entry")?;
            let path_buf = dir_entry.path();
            let file_stem = path_buf.file_stem().context("file_stem")?;
            let page_id = file_stem.to_str().context("file_stem is not UTF-8")?;
            let page_id = <crate::page_id::PageId as std::str::FromStr>::from_str(page_id)
                .context("invalid ID in data dir")?;
            page_ids.insert(page_id);
        }
        Ok(page_ids)
    }

    pub(crate) fn read_page_meta(
        config: &crate::Config,
        page_id: &crate::page_id::PageId,
    ) -> anyhow::Result<crate::page_meta::PageMeta> {
        let path_buf = Self::page_path(config, page_id);
        let md = std::fs::read_to_string(path_buf).context("read page")?;
        let page_meta = crate::page_meta::PageMeta::from_markdown(&md);
        Ok(page_meta)
    }

    pub(crate) fn read_page_content(
        config: &crate::Config,
        page_id: &crate::page_id::PageId,
    ) -> anyhow::Result<String> {
        let path = Self::page_path(config, &page_id);
        let md = std::fs::read_to_string(path).context("not found")?;
        let parser = pulldown_cmark::Parser::new_with_broken_link_callback(
            &md,
            pulldown_cmark::Options::empty(),
            Some(|broken_link: pulldown_cmark::BrokenLink<'_>| {
                match <crate::page_id::PageId as std::str::FromStr>::from_str(
                    &broken_link.reference,
                ) {
                    Err(_) => None,
                    Ok(page_id) => Some((
                        pulldown_cmark::CowStr::Boxed(page_id.to_string().into_boxed_str()),
                        pulldown_cmark::CowStr::Boxed(format!("/{page_id}").into_boxed_str()),
                    )),
                }
            }),
        );
        let mut html = String::new();
        pulldown_cmark::html::push_html(&mut html, parser);
        Ok(html)
    }
}
