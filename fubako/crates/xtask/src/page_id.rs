#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PageId(String);

impl PageId {
    pub fn new() -> Self {
        let now = chrono::Utc::now();
        let s = now.format("%Y%m%dT%H%M%SZ").to_string();
        Self(s)
    }
}

impl std::fmt::Display for PageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::str::FromStr for PageId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        (s.len() == "00000000T000000Z".len()
            && s.chars().all(|c| matches!(c, '0'..='9' | 'T' | 'Z')))
        .then_some(Self(s.to_string()))
        .ok_or_else(|| anyhow::anyhow!("invalid ID format"))
    }
}

impl<'de> serde::de::Deserialize<'de> for PageId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'vi> serde::de::Visitor<'vi> for Visitor {
            type Value = PageId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string matching the ID format")
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                (v.len() == "00000000T000000Z".len()
                    && v.chars().all(|c| matches!(c, '0'..='9' | 'T' | 'Z')))
                .then_some(v)
                .map(PageId)
                .ok_or_else(|| E::custom("invalid ID format"))
            }
        }

        deserializer.deserialize_string(Visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_impl_page_id_new() {
        let page_id = PageId::new();
        assert_eq!(page_id.0.len(), "00000000T000000Z".len());
    }

    #[test]
    fn test_impl_display_for_page_id() -> anyhow::Result<()> {
        let s = "20240620T123456Z";
        let page_id = <PageId as std::str::FromStr>::from_str(s)?;
        assert_eq!(page_id.to_string(), s);
        Ok(())
    }

    #[test]
    fn test_impl_from_str_for_page_id() -> anyhow::Result<()> {
        let valid = "20240620T123456Z";
        let invalid = "2024-06-20T12:34:56Z";

        let page_id = <PageId as std::str::FromStr>::from_str(valid)?;
        assert_eq!(page_id.to_string(), valid);

        assert!(<PageId as std::str::FromStr>::from_str(invalid).is_err());
        Ok(())
    }

    #[test]
    fn test_impl_serde_deserialize_for_page_id() -> anyhow::Result<()> {
        // TODO
        Ok(())
    }
}
