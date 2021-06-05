use ordered_float::NotNan;
use thiserror::Error;

#[derive(Debug, Eq, PartialEq)]
pub struct Set {
    key: String,
    value: NotNan<f64>,
}

#[derive(Debug, Error)]
pub enum ParseSetError {
    #[error("not nan")]
    NotNan,
}

impl Set {
    pub fn new(key: String, value: f64) -> Result<Set, ParseSetError> {
        let value = NotNan::new(value).map_err(|_| ParseSetError::NotNan)?;
        Ok(Self { key, value })
    }

    pub fn key(&self) -> String {
        self.key.clone()
    }

    pub fn value(&self) -> f64 {
        self.value.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(Set::new("2021-02-03".to_string(), f64::NAN).is_err(), true);

        let set = Set::new("2021-02-03".to_string(), 50.1).unwrap();
        assert_eq!(set.key(), "2021-02-03".to_string());
        assert_eq!(set.value(), 50.1);
    }
}
