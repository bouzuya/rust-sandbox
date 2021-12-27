#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Credentials {
    hatena_api_key: String,
    hatena_blog_id: String,
    hatena_id: String,
}

impl Credentials {
    pub fn new(hatena_api_key: String, hatena_blog_id: String, hatena_id: String) -> Self {
        Self {
            hatena_api_key,
            hatena_blog_id,
            hatena_id,
        }
    }

    pub fn hatena_api_key(&self) -> &str {
        self.hatena_api_key.as_str()
    }

    pub fn hatena_blog_id(&self) -> &str {
        self.hatena_blog_id.as_str()
    }

    pub fn hatena_id(&self) -> &str {
        self.hatena_id.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn credentials_test() -> anyhow::Result<()> {
        let hatena_api_key = "hatena_api_key1";
        let hatena_blog_id = "hatena_blog_id1";
        let hatena_id = "hatena_id1";
        let credentials = Credentials::new(
            hatena_api_key.to_string(),
            hatena_blog_id.to_string(),
            hatena_id.to_string(),
        );
        assert_eq!(credentials.hatena_api_key(), hatena_api_key);
        assert_eq!(credentials.hatena_blog_id(), hatena_blog_id);
        assert_eq!(credentials.hatena_id(), hatena_id);
        assert_eq!(credentials.clone(), credentials);
        Ok(())
    }
}
