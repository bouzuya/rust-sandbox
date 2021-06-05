use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Set {
    pub key: String,
    pub value: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let set = Set {
            key: "2021-02-03".to_string(),
            value: 50.1,
        };
        assert_eq!(set.key, "2021-02-03".to_string());
        assert_eq!(set.value, 50.1);
    }
}
