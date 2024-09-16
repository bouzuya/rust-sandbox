use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, multispace0};
use nom::combinator::recognize;
use nom::multi::{fold_many0, many0};
use nom::number::complete::recognize_float;
use nom::sequence::{delimited, pair};
use nom::IResult;

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
}

fn expr(input: &str) -> IResult<&str, Expression> {
    let (rest, lhs) = term(input)?;
    fold_many0(
        pair(delimited(multispace0, plus, multispace0), term),
        move || lhs.clone(),
        |lhs, (_, rhs)| Expression::Add(Box::new(lhs), Box::new(rhs)),
    )(rest)
}

fn eval(expr: Expression) -> f64 {
    match expr {
        Expression::Ident("pi") => std::f64::consts::PI,
        Expression::Ident(id) => panic!("Unknown name {:?}", id),
        Expression::NumLiteral(n) => n,
        Expression::Add(lhs, rhs) => eval(*lhs) + eval(*rhs),
    }
}

fn ident(input: &str) -> IResult<&str, Expression> {
    let (rest, value) = delimited(
        multispace0,
        recognize(pair(alpha1, many0(alphanumeric1))),
        multispace0,
    )(input)?;
    Ok((rest, Expression::Ident(value)))
}

fn lparen(input: &str) -> IResult<&str, &str> {
    tag("(")(input)
}

fn number(input: &str) -> IResult<&str, Expression> {
    let (rest, float_as_str) = delimited(multispace0, recognize_float, multispace0)(input)?;
    Ok((
        rest,
        Expression::NumLiteral(float_as_str.parse::<f64>().expect("FIXME")),
    ))
}

fn paren(input: &str) -> IResult<&str, Expression> {
    delimited(multispace0, delimited(lparen, expr, rparen), multispace0)(input)
}

fn plus(input: &str) -> IResult<&str, &str> {
    tag("+")(input)
}

fn rparen(input: &str) -> IResult<&str, &str> {
    tag(")")(input)
}

fn term(input: &str) -> IResult<&str, Expression> {
    alt((paren, token))(input)
}

fn token(input: &str) -> IResult<&str, Expression> {
    alt((ident, number))(input)
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
