use ordered_float::NotNan;

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct LatLng {
    pub latitude: NotNan<f64>,
    pub longitude: NotNan<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_test() -> anyhow::Result<()> {
        let deserialized: LatLng =
            serde_json::from_str(r#"{"latitude":123.456,"longitude":789.012}"#)?;
        assert_eq!(
            deserialized,
            LatLng {
                latitude: NotNan::new(123.456)?,
                longitude: NotNan::new(789.012)?,
            }
        );
        Ok(())
    }

    #[test]
    fn serialize_test() -> anyhow::Result<()> {
        assert_eq!(
            serde_json::to_string(&LatLng {
                latitude: NotNan::new(123.456)?,
                longitude: NotNan::new(789.012)?,
            })?,
            r#"{"latitude":123.456,"longitude":789.012}"#
        );
        Ok(())
    }
}
