use crate::{Digit2, Digit4};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OptionalDate(Option<Digit4>, Option<Digit2>, Option<Digit2>);

impl OptionalDate {
    pub fn from_yyyy(yyyy: Digit4) -> Self {
        Self(Some(yyyy), None, None)
    }

    pub fn from_yyyymm(yyyy: Digit4, mm: Digit2) -> Self {
        Self(Some(yyyy), Some(mm), None)
    }

    pub fn from_yyyymmdd(yyyy: Digit4, mm: Digit2, dd: Digit2) -> Self {
        Self(Some(yyyy), Some(mm), Some(dd))
    }

    pub fn year(&self) -> Option<Digit4> {
        self.0
    }

    pub fn month(&self) -> Option<Digit2> {
        self.1
    }

    pub fn day_of_month(&self) -> Option<Digit2> {
        self.2
    }
}

impl std::fmt::Display for OptionalDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.0, &self.1, &self.2) {
            (None, _, _) => unreachable!(),
            (Some(yyyy), None, None) => write!(f, "{}", yyyy),
            (Some(_), None, Some(_)) => unreachable!(),
            (Some(yyyy), Some(mm), None) => write!(f, "{}-{}", yyyy, mm),
            (Some(yyyy), Some(mm), Some(dd)) => write!(f, "{}-{}-{}", yyyy, mm, dd),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let yyyy = Digit4::try_from(2021)?;
        let mm = Digit2::try_from(2)?;
        let dd = Digit2::try_from(3)?;

        {
            let d = OptionalDate::from_yyyy(yyyy);
            assert_eq!(d.year(), Some(yyyy));
            assert_eq!(d.month(), None);
            assert_eq!(d.day_of_month(), None);
            assert_eq!(d.to_string(), "2021");
        }

        {
            let d = OptionalDate::from_yyyymm(yyyy, mm);
            assert_eq!(d.year(), Some(yyyy));
            assert_eq!(d.month(), Some(mm));
            assert_eq!(d.day_of_month(), None);
            assert_eq!(d.to_string(), "2021-02");
        }

        {
            let d = OptionalDate::from_yyyymmdd(yyyy, mm, dd);
            assert_eq!(d.year(), Some(yyyy));
            assert_eq!(d.month(), Some(mm));
            assert_eq!(d.day_of_month(), Some(dd));
            assert_eq!(d.to_string(), "2021-02-03");
        }

        Ok(())
    }
}
