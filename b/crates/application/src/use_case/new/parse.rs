use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::char,
    combinator::all_consuming,
    error::ParseError,
    multi::many0,
    sequence::delimited,
    FindSubstring, InputTake,
};

#[derive(Debug, Eq, PartialEq)]
pub enum Token<'a> {
    Str(&'a str),
    Var(&'a str),
}

pub fn parse<'a>(input: &'a str) -> Result<Vec<Token<'a>>, &'static str> {
    template(input)
        .map(|(_, tokens)| tokens)
        .map_err(|_| "parse error")
}

fn template(input: &str) -> nom::IResult<&str, Vec<Token>> {
    all_consuming(many0(template_token))(input)
}

fn template_token(input: &str) -> nom::IResult<&str, Token> {
    alt((template_non_block, template_block))(input)
}

fn template_non_block(input: &str) -> nom::IResult<&str, Token> {
    let (input, s) = match input.find_substring("{{") {
        None => {
            if input.is_empty() {
                Err(nom::Err::Error(nom::error::Error::from_error_kind(
                    input,
                    nom::error::ErrorKind::TakeUntil,
                )))
            } else {
                Ok(input.take_split(input.len()))
            }
        }
        Some(0) => Err(nom::Err::Error(nom::error::Error::from_error_kind(
            input,
            nom::error::ErrorKind::TakeUntil,
        ))),
        Some(index) => Ok(input.take_split(index)),
    }?;
    Ok((input, Token::Str(s)))
}

fn template_block(input: &str) -> nom::IResult<&str, Token> {
    let (input, v) = delimited(tag("{{"), template_expr, tag("}}"))(input)?;
    Ok((input, v))
}

fn template_expr(input: &str) -> nom::IResult<&str, Token> {
    let (input, expr) = alt((template_str, template_var))(input)?;
    Ok((input, expr))
}

fn template_str(input: &str) -> nom::IResult<&str, Token> {
    let (input, s) = delimited(char('"'), tag("{{"), char('"'))(input)?;
    Ok((input, Token::Str(s)))
}

fn template_var(input: &str) -> nom::IResult<&str, Token> {
    let (input, v) = is_a("_0123456789abcdefghijklmnopqrstuvwxyz")(input)?;
    Ok((input, Token::Var(v)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn template_test() {
        assert_eq!(template(""), Ok(("", vec![])));
        assert_eq!(
            template("abc{{foo}}def{{bar}}"),
            Ok((
                "",
                vec![
                    Token::Str("abc"),
                    Token::Var("foo"),
                    Token::Str("def"),
                    Token::Var("bar"),
                ]
            ))
        );
        assert!(template("abc{{foo}}def{{bar").is_err());
    }

    #[test]
    fn template_token_test() {
        assert!(template_token("").is_err());
        assert_eq!(template_token("abc"), Ok(("", Token::Str("abc"))));
        assert_eq!(template_token("{{foo}}"), Ok(("", Token::Var("foo"))));
    }

    #[test]
    fn template_str_test() {
        assert!(template_non_block("").is_err());
        assert!(template_non_block("{{").is_err());
        assert_eq!(template_non_block("a{{"), Ok(("{{", Token::Str("a"))));
        assert_eq!(template_non_block("abc{{"), Ok(("{{", Token::Str("abc"))));
    }

    #[test]
    fn template_block_test() {
        assert!(template_block("").is_err());
        assert_eq!(template_block("{{_}}"), Ok(("", Token::Var("_"))));
        assert_eq!(template_block("{{0}}"), Ok(("", Token::Var("0"))));
        assert_eq!(template_block("{{a}}"), Ok(("", Token::Var("a"))));
        assert_eq!(template_block("{{abc}}"), Ok(("", Token::Var("abc"))));
        assert_eq!(template_block("{{abc}}bar"), Ok(("bar", Token::Var("abc"))));
        assert!(template_block("foo{{abc}}").is_err());
        assert_eq!(template_block(r#"{{"{{"}}"#), Ok(("", Token::Str("{{"))));
    }
}
