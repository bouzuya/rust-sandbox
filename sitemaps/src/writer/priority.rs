use ordered_float::NotNan;

// TODO: Error
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
#[error("error")]
pub struct Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Priority(NotNan<f64>);

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl TryFrom<&str> for Priority {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.parse::<f64>().map_err(|_| Error)?)
    }
}

impl TryFrom<f64> for Priority {
    type Error = Error;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        NotNan::new(value).map(Self).map_err(|_| Error)
    }
}
