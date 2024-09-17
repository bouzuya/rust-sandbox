use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, char, multispace0};
use nom::combinator::recognize;
use nom::error::ParseError;
use nom::multi::{fold_many0, many0};
use nom::number::complete::recognize_float;
use nom::sequence::{delimited, pair};
use nom::IResult;
use nom::Parser;

fn main() {
    let input = "123world";
    println!(
        "source: {:?}, parsed: {:?}",
        input,
        expr(input).map(|(_, expr)| eval(expr))
    );
}

#[derive(Clone, Debug, PartialEq)]
enum Expression<'a> {
    Ident(&'a str),
    NumLiteral(f64),
    Add(Box<Expression<'a>>, Box<Expression<'a>>),
    Sub(Box<Expression<'a>>, Box<Expression<'a>>),
    Mul(Box<Expression<'a>>, Box<Expression<'a>>),
    Div(Box<Expression<'a>>, Box<Expression<'a>>),
}

fn expr(input: &str) -> IResult<&str, Expression> {
    let (rest, lhs) = term(input)?;
    fold_many0(
        pair(space_delimited(alt((char('+'), char('-')))), term),
        move || lhs.clone(),
        |lhs, (op, rhs)| match op {
            '+' => Expression::Add(Box::new(lhs), Box::new(rhs)),
            '-' => Expression::Sub(Box::new(lhs), Box::new(rhs)),
            _ => panic!("'+' or '-'"),
        },
    )(rest)
}

fn eval(expr: Expression) -> f64 {
    match expr {
        Expression::Ident("pi") => std::f64::consts::PI,
        Expression::Ident(id) => panic!("Unknown name {:?}", id),
        Expression::NumLiteral(n) => n,
        Expression::Add(lhs, rhs) => eval(*lhs) + eval(*rhs),
        Expression::Sub(lhs, rhs) => eval(*lhs) - eval(*rhs),
        Expression::Mul(lhs, rhs) => eval(*lhs) * eval(*rhs),
        Expression::Div(lhs, rhs) => eval(*lhs) / eval(*rhs),
    }
}

fn factor(input: &str) -> IResult<&str, Expression> {
    alt((number, ident, paren))(input)
}

fn ident(input: &str) -> IResult<&str, Expression> {
    let (rest, value) = space_delimited(recognize(pair(alpha1, many0(alphanumeric1))))(input)?;
    Ok((rest, Expression::Ident(value)))
}

fn lparen(input: &str) -> IResult<&str, &str> {
    tag("(")(input)
}

fn number(input: &str) -> IResult<&str, Expression> {
    let (rest, float_as_str) = space_delimited(recognize_float)(input)?;
    Ok((
        rest,
        Expression::NumLiteral(float_as_str.parse::<f64>().expect("FIXME")),
    ))
}

fn paren(input: &str) -> IResult<&str, Expression> {
    space_delimited(delimited(lparen, expr, rparen))(input)
}

fn rparen(input: &str) -> IResult<&str, &str> {
    tag(")")(input)
}

fn space_delimited<'a, O, E>(
    f: impl Parser<&'a str, O, E>,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    E: ParseError<&'a str>,
{
    delimited(multispace0, f, multispace0)
}

fn term(input: &str) -> IResult<&str, Expression> {
    let (input, init) = factor(input)?;
    fold_many0(
        pair(space_delimited(alt((char('*'), char('/')))), factor),
        move || init.clone(),
        |lhs, (op, rhs): (char, Expression)| match op {
            '*' => Expression::Mul(Box::new(lhs), Box::new(rhs)),
            '/' => Expression::Div(Box::new(lhs), Box::new(rhs)),
            _ => panic!("Multiplicative expression should have '*' or '/' operator"),
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr() {
        assert_eq!(expr("hello"), Ok(("", Expression::Ident("hello"))));
        assert_eq!(expr("123"), Ok(("", Expression::NumLiteral(123.0))));
        assert_eq!(
            expr("1+2"),
            Ok((
                "",
                Expression::Add(
                    Box::new(Expression::NumLiteral(1.0)),
                    Box::new(Expression::NumLiteral(2.0))
                )
            ))
        );

        assert_eq!(
            expr("Hello world"),
            Ok(("world", Expression::Ident("Hello")))
        );
        assert_eq!(
            expr("123world"),
            Ok(("world", Expression::NumLiteral(123.0)))
        );
    }

    #[test]
    fn test_eval() {
        assert_eq!(expr("123").map(|(_, expr)| eval(expr)), Ok(123.0));
        assert_eq!(expr("(123 + 456)").map(|(_, expr)| eval(expr)), Ok(579.0));
        assert_eq!(
            expr("10 + (100 + 1)").map(|(_, expr)| eval(expr)),
            Ok(111.0)
        );
        assert_eq!(
            expr("((1 + 2) + (3 + 4)) + 5 + 6").map(|(_, expr)| eval(expr)),
            Ok(21.0)
        );
        assert_eq!(
            expr("2 * pi").map(|(_, expr)| eval(expr)),
            Ok(2.0 * std::f64::consts::PI)
        );
        assert_eq!(
            expr("10 - (100 + 1)").map(|(_, expr)| eval(expr)),
            Ok(-91.0)
        );
        assert_eq!(
            expr("(3 + 7) / (2 + 3)").map(|(_, expr)| eval(expr)),
            Ok(2.0)
        );
    }

    #[test]
    fn test_ident() {
        assert_eq!(ident("Adam"), Ok(("", Expression::Ident("Adam"))));
        assert_eq!(ident("abc"), Ok(("", Expression::Ident("abc"))));
        assert!(ident("123abc").is_err());
        assert_eq!(ident("abc123"), Ok(("", Expression::Ident("abc123"))));
        assert_eq!(ident("abc123 "), Ok(("", Expression::Ident("abc123"))));
    }

    #[test]
    fn test_number() {
        assert_eq!(number("123.45 "), Ok(("", Expression::NumLiteral(123.45))));
        assert_eq!(number("123"), Ok(("", Expression::NumLiteral(123.0))));
        assert_eq!(number("+123.4"), Ok(("", Expression::NumLiteral(123.4))));
        assert_eq!(number("-456.7"), Ok(("", Expression::NumLiteral(-456.7))));
        assert_eq!(number(".0"), Ok(("", Expression::NumLiteral(0.0))));
        assert_eq!(
            number("+123.4abc "),
            Ok(("abc ", Expression::NumLiteral(123.4)))
        );
    }
}
