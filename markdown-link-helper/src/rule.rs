use regex::Regex;
use std::convert::TryFrom;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("pattern is invalid")]
    InvalidPattern,
}

pub struct Rule {
    pattern: Regex,
    replace: String,
}

impl Rule {
    pub fn apply(&self, s: &str) -> Option<String> {
        if self.pattern.is_match(s) {
            Some(self.pattern.replace(s, &self.replace).to_string())
        } else {
            None
        }
    }
}

impl TryFrom<(&str, &str)> for Rule {
    type Error = Error;

    fn try_from((p, r): (&str, &str)) -> Result<Self, Self::Error> {
        Regex::new(p)
            .map(|pattern| Self {
                pattern,
                replace: r.to_owned(),
            })
            .map_err(|_| Error::InvalidPattern)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let rule = Rule::try_from((
            r"^(\d{4})-(\d{2})-(\d{2})$",
            "[$1-$2-$3]: https://blog.bouzuya.net/$1/$2/$3/",
        ))?;
        assert_eq!(
            rule.apply("2021-04-29"),
            Some("[2021-04-29]: https://blog.bouzuya.net/2021/04/29/".to_owned())
        );
        Ok(())
    }
}
