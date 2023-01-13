use std::borrow::Cow;

// TODO: Error
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
#[error("error")]
pub struct Error;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Priority<'a>(Cow<'a, str>);

impl<'a> Priority<'a> {
    pub(crate) fn into_inner(self) -> Cow<'a, str> {
        self.0
    }
}

impl<'a> TryFrom<&'a str> for Priority<'a> {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        value.parse::<f64>().map_err(|_| Error)?;
        Ok(Self(Cow::Borrowed(value)))
    }
}

impl<'a> TryFrom<f64> for Priority<'a> {
    type Error = Error;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Ok(Self(Cow::Owned(value.to_string())))
    }
}
