#[derive(Debug)]
pub struct PageMeta {
    pub title: Option<String>,
}

impl PageMeta {
    pub fn from_markdown(md: &str) -> Self {
        let mut page_meta = PageMeta { title: None };
        let parser = pulldown_cmark::Parser::new(&md);
        let parser = pulldown_cmark::TextMergeStream::new(parser);
        let mut in_h1 = false;
        for event in parser {
            match event {
                pulldown_cmark::Event::Start(pulldown_cmark::Tag::Heading {
                    level: pulldown_cmark::HeadingLevel::H1,
                    ..
                }) => {
                    in_h1 = true;
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

        page_meta
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_markdown() {
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
}
