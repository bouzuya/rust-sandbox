use crate::page_id::PageId;

#[derive(Clone)]
pub struct PageMeta {
    pub links: std::collections::BTreeSet<PageId>,
    pub title: Option<String>,
}

impl PageMeta {
    pub fn from_markdown(md: &str) -> Self {
        let mut page_meta = PageMeta {
            links: Default::default(),
            title: Default::default(),
        };
        let mut broken_page_links = vec![];
        let parser = pulldown_cmark::Parser::new_with_broken_link_callback(
            &md,
            pulldown_cmark::Options::empty(),
            Some(|link: pulldown_cmark::BrokenLink<'_>| {
                match <PageId as std::str::FromStr>::from_str(&link.reference) {
                    Err(_) => None,
                    Ok(page_id) => {
                        broken_page_links.push(page_id.clone());
                        None
                    }
                }
            }),
        );
        let parser = pulldown_cmark::TextMergeStream::new(parser);
        let mut page_links = vec![];
        let mut in_h1 = false;
        for event in parser {
            match event {
                pulldown_cmark::Event::Start(pulldown_cmark::Tag::Heading {
                    level: pulldown_cmark::HeadingLevel::H1,
                    ..
                }) => {
                    in_h1 = true;
                }
                pulldown_cmark::Event::Start(pulldown_cmark::Tag::Link {
                    link_type,
                    dest_url,
                    title: _,
                    id: _,
                }) => {
                    match link_type {
                        pulldown_cmark::LinkType::Inline
                        | pulldown_cmark::LinkType::Reference
                        | pulldown_cmark::LinkType::Collapsed
                        | pulldown_cmark::LinkType::Shortcut => {
                            if let Some(stripped) = dest_url.strip_prefix('/') {
                                match <PageId as std::str::FromStr>::from_str(stripped) {
                                    Err(_) => { /* do nothing */ }
                                    Ok(page_id) => {
                                        page_links.push(page_id);
                                    }
                                }
                            }
                        }
                        pulldown_cmark::LinkType::ReferenceUnknown
                        | pulldown_cmark::LinkType::CollapsedUnknown
                        | pulldown_cmark::LinkType::ShortcutUnknown
                        | pulldown_cmark::LinkType::Autolink
                        | pulldown_cmark::LinkType::Email
                        | pulldown_cmark::LinkType::WikiLink { .. } => {
                            // do nothing
                        }
                    }
                }
                pulldown_cmark::Event::Text(text) => {
                    if in_h1 {
                        match page_meta.title {
                            None => {
                                page_meta.title = Some(text.to_string());
                            }
                            Some(_) => {
                                // do nothing
                            }
                        }
                    }
                }
                pulldown_cmark::Event::End(pulldown_cmark::TagEnd::Heading(
                    pulldown_cmark::HeadingLevel::H1,
                )) => {
                    in_h1 = false;
                }
                _ => { /* ignore other events */ }
            }
        }

        page_meta.links = broken_page_links
            .into_iter()
            .chain(page_links.into_iter())
            .collect::<std::collections::BTreeSet<PageId>>();

        page_meta
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_markdown_title_field() {
        let md = "# Title\n\nSome content.";
        let page_meta = PageMeta::from_markdown(md);
        assert_eq!(page_meta.title, Some("Title".to_string()));

        let md = "No title here.";
        let page_meta = PageMeta::from_markdown(md);
        assert_eq!(page_meta.title, None);

        let md = "# First Title\n\n# Second Title";
        let page_meta = PageMeta::from_markdown(md);
        assert_eq!(page_meta.title, Some("First Title".to_string()));
    }

    #[test]
    fn test_from_markdown_links_field() -> anyhow::Result<()> {
        fn id(s: &str) -> anyhow::Result<PageId> {
            Ok(<PageId as std::str::FromStr>::from_str(s)?)
        }

        fn set<I>(iter: I) -> std::collections::BTreeSet<PageId>
        where
            I: IntoIterator<Item = PageId>,
        {
            iter.into_iter()
                .collect::<std::collections::BTreeSet<PageId>>()
        }

        // inline link
        let md = "[foo](/19700102T151617Z)";
        let page_meta = PageMeta::from_markdown(md);
        assert_eq!(page_meta.links, set([id("19700102T151617Z")?]));

        // reference link
        let md = "[foo][bar]\n\n[bar]: /19700102T151617Z";
        let page_meta = PageMeta::from_markdown(md);
        assert_eq!(page_meta.links, set([id("19700102T151617Z")?]));

        // reference link (broken)
        let md = "[foo][19700102T151617Z]";
        let page_meta = PageMeta::from_markdown(md);
        assert_eq!(page_meta.links, set([id("19700102T151617Z")?]));

        // collapsed link
        let md = "[foo][]\n\n[foo]: /19700102T151617Z";
        let page_meta = PageMeta::from_markdown(md);
        assert_eq!(page_meta.links, set([id("19700102T151617Z")?]));

        // collapsed link (broken)
        let md = "[19700102T151617Z][]";
        let page_meta = PageMeta::from_markdown(md);
        assert_eq!(page_meta.links, set([id("19700102T151617Z")?]));

        // shortcut link
        let md = "[foo]\n\n[foo]: /19700102T151617Z";
        let page_meta = PageMeta::from_markdown(md);
        assert_eq!(page_meta.links, set([id("19700102T151617Z")?]));

        // shortcut link (broken)
        let md = "[19700102T151617Z]";
        let page_meta = PageMeta::from_markdown(md);
        assert_eq!(page_meta.links, set([id("19700102T151617Z")?]));

        // (duplicate)
        let md = "[19700102T151617Z]\n\n[19700102T151617Z]";
        let page_meta = PageMeta::from_markdown(md);
        assert_eq!(page_meta.links, set([id("19700102T151617Z")?]));

        // (two different)
        let md = "[19700102T151617Z]\n\n[19710102T151617Z]";
        let page_meta = PageMeta::from_markdown(md);
        assert_eq!(
            page_meta.links,
            set([id("19700102T151617Z")?, id("19710102T151617Z")?])
        );

        Ok(())
    }
}
