use ordered_float::NotNan;

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct LatLng {
    pub latitude: NotNan<f64>,
    pub longitude: NotNan<f64>,
}

#[cfg(test)]
mod tests {
    use crate::firestore_rest::tests::serde_test;

    use super::*;

    #[test]
    fn serde_tests() -> anyhow::Result<()> {
        serde_test(
            LatLng {
                latitude: NotNan::new(123.456)?,
                longitude: NotNan::new(789.012)?,
            },
            r#"{"latitude":123.456,"longitude":789.012}"#,
        )?;
        Ok(())
    }
}
