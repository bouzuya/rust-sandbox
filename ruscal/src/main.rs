use std::collections::BTreeMap;
use std::io::Read;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, char, multispace0};
use nom::combinator::{opt, recognize};
use nom::error::ParseError;
use nom::multi::{fold_many0, many0, separated_list0};
use nom::number::complete::recognize_float;
use nom::sequence::{delimited, pair};
use nom::Parser;
use nom::{Finish, IResult};

fn main() {
    let mut variables = BTreeMap::new();

    let mut buf = String::new();
    if std::io::stdin().read_to_string(&mut buf).is_ok() {
        let parsed_statements = match statements(&buf) {
            Ok(parsed_statements) => parsed_statements,
            Err(e) => {
                eprintln!("Parse error: {:?}", e);
                return;
            }
        };

        for statement in parsed_statements {
            match statement {
                Statement::Expression(expr) => {
                    println!("eval: {:?}", eval(expr, &variables));
                }
                Statement::VarDef(name, expr) => {
                    let value = eval(expr, &variables);
                    variables.insert(name, value);
                }
                Statement::VarAssign(name, expr) => {
                    if !variables.contains_key(name) {
                        panic!("Variable is not defined");
                    }

                    let value = eval(expr, &variables);
                    variables.insert(name, value);
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Statement<'a> {
    Expression(Expression<'a>),
    VarDef(&'a str, Expression<'a>),
    VarAssign(&'a str, Expression<'a>),
}

type Statements<'a> = Vec<Statement<'a>>;

#[derive(Clone, Debug, PartialEq)]
enum Expression<'a> {
    Ident(&'a str),
    NumLiteral(f64),
    FnInvoke(&'a str, Vec<Expression<'a>>),
    Add(Box<Expression<'a>>, Box<Expression<'a>>),
    Sub(Box<Expression<'a>>, Box<Expression<'a>>),
    Mul(Box<Expression<'a>>, Box<Expression<'a>>),
    Div(Box<Expression<'a>>, Box<Expression<'a>>),
}

fn binary_fn(f: fn(f64, f64) -> f64) -> impl Fn(Vec<Expression>, &BTreeMap<&str, f64>) -> f64 {
    move |args, variables| {
        let mut args = args.into_iter();
        let lhs = eval(
            args.next().expect("function missing the first argument"),
            variables,
        );
        let rhs = eval(
            args.next().expect("function missing the second argument"),
            variables,
        );
        f(lhs, rhs)
    }
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

fn expr_statement(input: &str) -> IResult<&str, Statement> {
    let (input, expr) = expr(input)?;
    Ok((input, Statement::Expression(expr)))
}

fn eval(expr: Expression, variables: &BTreeMap<&str, f64>) -> f64 {
    match expr {
        Expression::Ident("pi") => std::f64::consts::PI,
        Expression::Ident(id) => *variables.get(id).expect("Unknown variable"),
        Expression::NumLiteral(n) => n,
        Expression::Add(lhs, rhs) => eval(*lhs, variables) + eval(*rhs, variables),
        Expression::Sub(lhs, rhs) => eval(*lhs, variables) - eval(*rhs, variables),
        Expression::Mul(lhs, rhs) => eval(*lhs, variables) * eval(*rhs, variables),
        Expression::Div(lhs, rhs) => eval(*lhs, variables) / eval(*rhs, variables),
        Expression::FnInvoke(ident, args) => match ident {
            "sqrt" => unary_fn(f64::sqrt)(args, variables),
            "sin" => unary_fn(f64::sin)(args, variables),
            "cos" => unary_fn(f64::cos)(args, variables),
            "tan" => unary_fn(f64::tan)(args, variables),
            "asin" => unary_fn(f64::asin)(args, variables),
            "acos" => unary_fn(f64::acos)(args, variables),
            "atan" => unary_fn(f64::atan)(args, variables),
            "atan2" => binary_fn(f64::atan2)(args, variables),
            "pow" => binary_fn(f64::powf)(args, variables),
            "exp" => unary_fn(f64::exp)(args, variables),
            "log" => binary_fn(f64::log)(args, variables),
            "log10" => unary_fn(f64::log10)(args, variables),
            fn_name => panic!("unknown func name {}", fn_name),
        },
    }
}

fn factor(input: &str) -> IResult<&str, Expression> {
    alt((number, func_call, ident, paren))(input)
}

fn func_call(input: &str) -> IResult<&str, Expression> {
    let (input, ident) = space_delimited(identifier)(input)?;
    let (input, args) = space_delimited(delimited(
        tag("("),
        many0(delimited(multispace0, expr, space_delimited(opt(tag(","))))),
        tag(")"),
    ))(input)?;
    Ok((input, Expression::FnInvoke(ident, args)))
}

fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(alpha1, many0(alphanumeric1)))(input)
}

fn ident(input: &str) -> IResult<&str, Expression> {
    let (rest, value) = space_delimited(identifier)(input)?;
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

fn statement(input: &str) -> IResult<&str, Statement> {
    alt((var_def, var_assign, expr_statement))(input)
}

fn statements(input: &str) -> Result<Statements, nom::error::Error<&str>> {
    let (_, res) = separated_list0(tag(";"), statement)(input).finish()?;
    Ok(res)
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

fn unary_fn(f: fn(f64) -> f64) -> impl Fn(Vec<Expression>, &BTreeMap<&str, f64>) -> f64 {
    move |args, variables| {
        let mut args = args.into_iter();
        f(eval(
            args.next().expect("function missing argument"),
            variables,
        ))
    }
}

fn var_assign(input: &str) -> IResult<&str, Statement> {
    let (input, name) = space_delimited(identifier)(input)?;
    let (input, _) = space_delimited(char('='))(input)?;
    let (input, expr) = space_delimited(expr)(input)?;
    Ok((input, Statement::VarAssign(name, expr)))
}

fn var_def(input: &str) -> IResult<&str, Statement> {
    let (input, _) = space_delimited(tag("var"))(input)?;
    let (input, name) = space_delimited(identifier)(input)?;
    let (input, _) = space_delimited(char('='))(input)?;
    let (input, expr) = space_delimited(expr)(input)?;
    Ok((input, Statement::VarDef(name, expr)))
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
        let variables = BTreeMap::new();
        assert_eq!(
            expr("123").map(|(_, expr)| eval(expr, &variables)),
            Ok(123.0)
        );
        assert_eq!(
            expr("(123 + 456)").map(|(_, expr)| eval(expr, &variables)),
            Ok(579.0)
        );
        assert_eq!(
            expr("10 + (100 + 1)").map(|(_, expr)| eval(expr, &variables)),
            Ok(111.0)
        );
        assert_eq!(
            expr("((1 + 2) + (3 + 4)) + 5 + 6").map(|(_, expr)| eval(expr, &variables)),
            Ok(21.0)
        );
        assert_eq!(
            expr("2 * pi").map(|(_, expr)| eval(expr, &variables)),
            Ok(2.0 * std::f64::consts::PI)
        );
        assert_eq!(
            expr("10 - (100 + 1)").map(|(_, expr)| eval(expr, &variables)),
            Ok(-91.0)
        );
        assert_eq!(
            expr("(3 + 7) / (2 + 3)").map(|(_, expr)| eval(expr, &variables)),
            Ok(2.0)
        );
        assert_eq!(
            expr("sqrt(2) / 2").map(|(_, expr)| eval(expr, &variables)),
            Ok(0.7071067811865476)
        );
        assert_eq!(
            expr("sin(pi / 4)").map(|(_, expr)| eval(expr, &variables)),
            Ok(0.7071067811865475)
        );
        assert_eq!(
            expr("atan2(1, 1)").map(|(_, expr)| eval(expr, &variables)),
            Ok(0.7853981633974483)
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
