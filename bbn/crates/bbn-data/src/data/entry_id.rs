use date_range::date::Date;

pub type EntryKey = Date;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct EntryId {
    date: Date,
    id_title: Option<String>,
}

#[derive(Debug, thiserror::Error)]
#[error("parse entry id")]
pub struct ParseEntryId;

impl std::str::FromStr for EntryId {
    type Err = ParseEntryId;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s.len() == 10 {
            Self {
                date: Date::from_str(s).map_err(|_| ParseEntryId)?,
                id_title: None,
            }
        } else {
            Self {
                date: Date::from_str(&s.chars().take(10).collect::<String>())
                    .map_err(|_| ParseEntryId)?,
                id_title: Some(s.chars().skip(11).collect::<String>()),
            }
        })
    }
}

impl std::fmt::Display for EntryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.date,
            self.id_title
                .as_ref()
                .map(|s| format!("-{s}"))
                .unwrap_or_default()
        )
    }
}

impl EntryId {
    pub fn new(date: Date, id_title: Option<String>) -> Self {
        Self { date, id_title }
    }

    pub fn date(&self) -> &Date {
        &self.date
    }

    pub fn id_title(&self) -> Option<&str> {
        self.id_title.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn string_conversion_test() -> anyhow::Result<()> {
        let f = |s: &str| -> anyhow::Result<()> {
            assert_eq!(EntryId::from_str(s)?.to_string(), s.to_string());
            Ok(())
        };
        f("2021-07-06")?;
        f("2021-07-06-id")?;
        Ok(())
    }

    #[test]
    fn date_test() -> anyhow::Result<()> {
        let date = Date::from_str("2021-07-06")?;
        assert_eq!(EntryId::new(date, None).date(), &date);
        Ok(())
    }

    #[test]
    fn id_title_test() -> anyhow::Result<()> {
        let date = Date::from_str("2021-07-06")?;
        let id_title = "id_title";
        assert_eq!(
            EntryId::new(date, Some(id_title.to_string())).id_title(),
            Some(id_title)
        );
        Ok(())
    }
}
