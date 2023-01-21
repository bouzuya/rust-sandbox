use std::convert::TryInto;

use hatena_blog_api::{Entry, ListEntriesResponse};

use crate::hatena_blog::HatenaBlogEntryId;
use bbn_data::{DateTime, Timestamp};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HatenaBlogListEntriesResponse(String);

impl HatenaBlogListEntriesResponse {
    pub fn body(self) -> String {
        self.0
    }

    pub fn hatena_blog_entry_ids(
        self,
        since: Option<Timestamp>,
    ) -> anyhow::Result<Vec<HatenaBlogEntryId>> {
        let response = ListEntriesResponse::from(self.0);
        let (_, entries): (Option<String>, Vec<Entry>) = response.try_into()?;
        let filtered = entries
            .iter()
            .take_while(|entry| match since {
                None => true,
                Some(since) => since <= Timestamp::from(DateTime::from(entry.published)),
            })
            .map(|entry| HatenaBlogEntryId::from(entry.id.clone()))
            .collect::<Vec<HatenaBlogEntryId>>();
        Ok(filtered)
    }

    pub fn next_page(self, since: Option<Timestamp>) -> anyhow::Result<Option<String>> {
        let response = ListEntriesResponse::from(self.0);
        let (next, entries): (Option<String>, Vec<Entry>) = response.try_into()?;
        let filtered_len = entries
            .iter()
            .take_while(|entry| match since {
                None => true,
                Some(since) => since <= Timestamp::from(DateTime::from(entry.published)),
            })
            .count();
        Ok(match (next, filtered_len == entries.len()) {
            (None, _) | (Some(_), false) => None,
            (Some(page), true) => Some(page),
        })
    }
}

impl From<ListEntriesResponse> for HatenaBlogListEntriesResponse {
    fn from(response: ListEntriesResponse) -> Self {
        Self(response.to_string())
    }
}

impl From<String> for HatenaBlogListEntriesResponse {
    fn from(body: String) -> Self {
        Self(body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response_conversion_test() {
        let body = "response body";
        let response = ListEntriesResponse::from(body.to_string());
        assert_eq!(HatenaBlogListEntriesResponse::from(response).body(), body);
    }

    #[test]
    fn body_conversion_test() {
        let body = "response body";
        assert_eq!(
            HatenaBlogListEntriesResponse::from(body.to_string()).body(),
            body
        );
    }

    #[test]
    fn hatena_blog_entry_ids() {
        // TODO
    }

    #[test]
    fn next_page() {
        // TODO
    }
}
