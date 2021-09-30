use nom::{
    branch::alt,
    bytes::{
        complete::{tag, take_while1},
        streaming::take_while,
    },
    combinator::map,
    sequence::delimited,
    IResult,
};

#[derive(Debug, Eq, PartialEq)]
pub struct TagParam(String);

impl std::fmt::Display for TagParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "tag:\"{}\"", self.0)
    }
}

fn is_ascii_alphanumeric_or_space(c: char) -> bool {
    c == ' ' || c.is_ascii_alphanumeric()
}

fn parse(s: &str) -> IResult<&str, TagParam> {
    let (s, _) = tag("tag:")(s)?;
    let (s, t) = map(
        alt((
            delimited(
                tag("\""),
                take_while(is_ascii_alphanumeric_or_space),
                tag("\""),
            ),
            take_while1(char::is_alphanumeric),
        )),
        |s: &str| TagParam(s.to_string()),
    )(s)?;
    Ok((s, t))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_conversion_test() -> anyhow::Result<()> {
        let s = "tag:\"abc def\"";
        let (_, t) = parse(s)?;
        assert_eq!(t.to_string(), s.to_string());
        Ok(())
    }
}
