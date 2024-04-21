mod alignment;
mod auto_text;
mod background;
mod background_suggestion_state;
mod baseline_offset;
mod body;
mod bullet;
mod bullet_alignment;
mod bullet_suggestion_state;
mod color;
mod column_break;
mod column_separator_style;
mod content_alignment;
mod content_direction;
mod crop_properties;
mod crop_properties_suggestion_state;
mod dash_style;
mod dimension;
mod document;
mod document_style;
mod document_style_suggestion_state;
mod embedded_drawing_properties;
mod embedded_drawing_properties_suggestion_state;
mod embedded_object;
mod embedded_object_border;
mod embedded_object_border_suggestion_state;
mod embedded_object_properties;
mod embedded_object_suggestion_state;
mod equation;
mod footer;
mod footnote;
mod footnote_reference;
mod glyph_type;
mod header;
mod horizontal_rule;
mod image_properties;
mod image_properties_suggestion_state;
mod inline_object;
mod inline_object_element;
mod inline_object_properties;
mod inline_object_properties_suggestion_state;
mod link;
mod link_destrination;
mod linked_content_reference;
mod linked_content_reference_reference;
mod linked_content_reference_suggestion_state;
mod list;
mod list_properties;
mod list_properties_suggestion_state;
mod named_range;
mod named_ranges;
mod named_style;
mod named_style_suggestion_state;
mod named_style_type;
mod named_styles;
mod named_styles_suggestion_state;
mod nesting_level;
mod nesting_level_glyph_kind;
mod nesting_level_suggestion_state;
mod object_references;
mod optional_color;
mod page_break;
mod paragraph;
mod paragraph_border;
mod paragraph_element;
mod paragraph_element_content;
mod paragraph_style;
mod paragraph_style_suggestion_state;
mod person;
mod person_properties;
mod positioned_object;
mod positioned_object_layout;
mod positioned_object_positioning;
mod positioned_object_positioning_suggestion_state;
mod positioned_object_properties;
mod positioned_object_properties_suggestion_state;
mod property_state;
mod range;
mod rgb_color;
mod rich_link;
mod rich_link_properties;
mod section_break;
mod section_column_properties;
mod section_style;
mod section_type;
mod shading;
mod shading_suggestion_state;
mod sheets_chart_reference;
mod sheets_chart_reference_suggestion_state;
mod size;
mod size_suggestion_state;
mod spacing_mode;
mod structural_element;
mod structural_element_content;
mod suggested_bullet;
mod suggested_document_style;
mod suggested_inline_object_properties;
mod suggested_list_properties;
mod suggested_named_styles;
mod suggested_paragraph_style;
mod suggested_positioned_object_properties;
mod suggested_table_cell_style;
mod suggested_table_row_style;
mod suggested_text_style;
mod suggestions_view_mode;
mod tab_stop;
mod tab_stop_alignment;
mod table;
mod table_cell;
mod table_cell_border;
mod table_cell_style;
mod table_cell_style_suggestion_state;
mod table_column_properties;
mod table_of_contents;
mod table_row;
mod table_row_style;
mod table_row_style_suggestion_state;
mod table_style;
mod text_run;
mod text_style;
mod text_style_suggestion_state;
mod r#type;
mod unit;
pub mod v1;
mod weighted_font_family;
mod width_type;

use self::v1::documents::request::Request;

use crate::token_source::TokenSource;

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchUpdateRequestBody {
    pub requests: Option<Vec<Request>>,
    // TODO:
}

// <https://developers.google.com/docs/api/reference/rest>
// - batchUpdate
// - create
// - get
#[derive(Clone)]
pub struct GoogleDocsClient {
    client: reqwest::Client,
    service_endpoint: String,
    token_source: std::sync::Arc<dyn Send + Sync + TokenSource>,
}

impl GoogleDocsClient {
    const SERVICE_ENDPOINT: &'static str = "https://docs.googleapis.com";

    pub fn new<T: Send + Sync + TokenSource + 'static>(token_source: T) -> Self {
        Self {
            client: reqwest::Client::new(),
            service_endpoint: Self::SERVICE_ENDPOINT.to_string(),
            token_source: std::sync::Arc::new(token_source),
        }
    }

    // <https://developers.google.com/docs/api/reference/rest/v1/documents/batchUpdate>
    pub async fn v1_documents_batch_update<S: AsRef<str>>(
        &self,
        document_id: S,
        body: &BatchUpdateRequestBody,
    ) -> anyhow::Result<String> {
        let token = self.token_source.token().await?;
        let request = self
            .client
            .request(
                reqwest::Method::POST,
                format!(
                    "{}/v1/documents/{}:batchUpdate",
                    self.service_endpoint,
                    document_id.as_ref()
                ),
            )
            .header("Authorization", format!("Bearer {}", token))
            .json(body)
            .build()?;
        let response = self.client.execute(request).await?;
        if !response.status().is_success() {
            anyhow::bail!("{:?}", response.status());
        }
        Ok(response.text().await?)
    }
    // <https://developers.google.com/docs/api/reference/rest/v1/documents/get>
    pub async fn v1_documents_get<S: AsRef<str>>(&self, document_id: S) -> anyhow::Result<String> {
        let token = self.token_source.token().await?;
        let request = self
            .client
            .request(
                reqwest::Method::GET,
                format!(
                    "{}/v1/documents/{}",
                    self.service_endpoint,
                    document_id.as_ref()
                ),
            )
            .header("Authorization", format!("Bearer {}", token))
            .build()?;
        let response = self.client.execute(request).await?;
        if !response.status().is_success() {
            anyhow::bail!("{:?}", response.status());
        }
        Ok(response.text().await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub(crate) fn test_serde<
        T: std::fmt::Debug + PartialEq + serde::Serialize + serde::de::DeserializeOwned,
    >(
        s: &str,
        v: T,
    ) -> anyhow::Result<()> {
        assert_eq!(serde_json::from_str::<'_, T>(s)?, v);
        assert_eq!(
            serde_json::from_str::<'_, serde_json::Value>(&serde_json::to_string(&v)?)?,
            serde_json::from_str::<'_, serde_json::Value>(s)?
        );
        Ok(())
    }

    #[test]
    fn test() {
        fn assert_impls<T: Clone + Send + Sync>() {}
        assert_impls::<GoogleDocsClient>();
    }
}
