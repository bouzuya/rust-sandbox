use time::format_description::well_known::Iso8601;

// TODO: Error
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
#[error("error")]
pub struct Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Lastmod(LastmodInner);

impl std::fmt::Display for Lastmod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LastmodInner {
    Date(time::Date),
    OffsetDateTime(time::OffsetDateTime),
}

impl std::fmt::Display for LastmodInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LastmodInner::Date(d) => {
                let format = time::macros::format_description!("[year]-[month]-[day]");
                d.format(&format).expect("formattable date").fmt(f)
            }
            LastmodInner::OffsetDateTime(odt) => odt
                .format(&Iso8601::DEFAULT)
                .expect("formattable offset date time")
                .fmt(f),
        }
    }
}

impl TryFrom<&str> for Lastmod {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() == 10 {
            let format = time::macros::format_description!("[year]-[month]-[day]");
            time::Date::parse(value, &format)
                .map(LastmodInner::Date)
                .map(Self)
                .map_err(|_| Error)
        } else {
            time::OffsetDateTime::parse(value, &Iso8601::DEFAULT)
                .map(LastmodInner::OffsetDateTime)
                .map(Self)
                .map_err(|_| Error)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let lastmod = Lastmod::try_from("2005-01-01")?;
        assert_eq!(
            lastmod,
            Lastmod(LastmodInner::Date(time::macros::date!(2005 - 01 - 01)))
        );
        assert_eq!(lastmod.to_string(), "2005-01-01");

        let lastmod = Lastmod::try_from("2004-12-23")?;
        assert_eq!(
            lastmod,
            Lastmod(LastmodInner::Date(time::macros::date!(2004 - 12 - 23)))
        );
        assert_eq!(lastmod.to_string(), "2005-01-01");

        let lastmod = Lastmod::try_from("2004-12-23T18:00:15+00:00")?;
        assert_eq!(
            lastmod,
            Lastmod(LastmodInner::OffsetDateTime(
                time::macros::datetime!(2004-12-23 18:00:15 +00:00)
            ))
        );
        assert_eq!(lastmod.to_string(), "2004-12-23T18:00:15+00:00");

        let lastmod = Lastmod::try_from("2004-11-23")?;
        assert_eq!(
            lastmod,
            Lastmod(LastmodInner::Date(time::macros::date!(2004 - 11 - 23)))
        );
        assert_eq!(lastmod.to_string(), "2004-11-23");
        Ok(())
    }
}
