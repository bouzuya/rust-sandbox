/// A `changefreq` child entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq, strum::AsRefStr, strum::EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Changefreq {
    Always,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Yearly,
    Never,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        use Changefreq::*;
        for (v, s) in [
            (Always, "always"),
            (Hourly, "hourly"),
            (Daily, "daily"),
            (Weekly, "weekly"),
            (Monthly, "monthly"),
            (Yearly, "yearly"),
            (Never, "never"),
        ] {
            assert_eq!(Changefreq::try_from(s)?, v);
            assert_eq!(v.as_ref(), s);
        }
        Ok(())
    }
}
