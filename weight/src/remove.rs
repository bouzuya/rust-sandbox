#[derive(Debug, Eq, PartialEq)]
pub struct Remove {
    key: String,
}

impl Remove {
    pub fn new(key: String) -> Self {
        Self { key }
    }

    pub fn key(&self) -> String {
        self.key.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let remove = Remove::new("2021-02-03".to_string());
        assert_eq!(remove.key(), "2021-02-03".to_string());
    }
}
