use std::collections::BTreeMap;
use std::io::Read;
use std::ops::ControlFlow;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, char, multispace0};
use nom::combinator::{opt, recognize};
use nom::error::ParseError;
use nom::multi::{fold_many0, many0, separated_list0};
use nom::number::complete::recognize_float;
use nom::sequence::{delimited, pair, preceded, terminated};
use nom::Parser;
use nom::{Finish, IResult};

fn main() {
    let mut buf = String::new();
    if !std::io::stdin().read_to_string(&mut buf).is_ok() {
        panic!("Failed to read from stdin");
    }
    let parsed_statements = match statements_finish(&buf) {
        Ok(parsed_statements) => parsed_statements,
        Err(e) => {
            eprintln!("Parse error: {:?}", e);
            return;
        }
    };
    let mut frame = StackFrame::new();
    eval_stmts(&parsed_statements, &mut frame);
}

enum BreakResult {
    Return(f64),
    Break,
    Continue,
}

type EvalResult = ControlFlow<BreakResult, f64>;

type Variables = BTreeMap<String, f64>;

type Functions<'a> = BTreeMap<String, FnDef<'a>>;

#[derive(Clone, Debug, PartialEq)]
enum Statement<'a> {
    Expression(Expression<'a>),
    VarDef(&'a str, Expression<'a>),
    VarAssign(&'a str, Expression<'a>),
    For {
        loop_var: &'a str,
        start: Expression<'a>,
        end: Expression<'a>,
        stmts: Statements<'a>,
    },
    FnDef {
        name: &'a str,
        args: Vec<&'a str>,
        stmts: Statements<'a>,
    },
    Return(Expression<'a>),
    Break,
    Continue,
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
    If(
        Box<Expression<'a>>,
        Box<Statements<'a>>,
        Option<Box<Statements<'a>>>,
    ),
    Lt(Box<Expression<'a>>, Box<Expression<'a>>),
    Gt(Box<Expression<'a>>, Box<Expression<'a>>),
}

enum FnDef<'a> {
    User(UserFn<'a>),
    Native(NativeFn),
}

impl<'a> FnDef<'a> {
    fn call(&self, args: &[f64], frame: &StackFrame) -> f64 {
        match self {
            Self::User(code) => {
                let mut new_frame = StackFrame::push_stack(frame);
                new_frame.vars = args
                    .iter()
                    .zip(code.args.iter())
                    .map(|(arg, name)| (name.to_string(), *arg))
                    .collect();
                match eval_stmts(&code.stmts, &mut new_frame) {
                    ControlFlow::Continue(v) | ControlFlow::Break(BreakResult::Return(v)) => v,
                    ControlFlow::Break(BreakResult::Break) => {
                        panic!("Breaking outside loop is prohibited");
                    }
                    ControlFlow::Break(BreakResult::Continue) => {
                        panic!("Continuing outside loop is prohibited");
                    }
                }
            }
            Self::Native(code) => (code.code)(args),
        }
    }
}

struct UserFn<'a> {
    args: Vec<&'a str>,
    stmts: Statements<'a>,
}

struct NativeFn {
    code: Box<dyn Fn(&[f64]) -> f64>,
}

struct StackFrame<'a> {
    vars: Variables,
    funcs: Functions<'a>,
    uplevel: Option<&'a StackFrame<'a>>,
}

impl<'a> StackFrame<'a> {
    fn new() -> Self {
        let mut funcs = Functions::new();
        funcs.insert("sqrt".to_owned(), unary_fn(f64::sqrt));
        funcs.insert("sin".to_owned(), unary_fn(f64::sin));
        funcs.insert("cos".to_owned(), unary_fn(f64::cos));
        funcs.insert("tan".to_owned(), unary_fn(f64::tan));
        funcs.insert("asin".to_owned(), unary_fn(f64::asin));
        funcs.insert("acos".to_owned(), unary_fn(f64::acos));
        funcs.insert("atan".to_owned(), unary_fn(f64::atan));
        funcs.insert("atan2".to_owned(), binary_fn(f64::atan2));
        funcs.insert("pow".to_owned(), binary_fn(f64::powf));
        funcs.insert("exp".to_owned(), unary_fn(f64::exp));
        funcs.insert("log".to_owned(), binary_fn(f64::log));
        funcs.insert("log10".to_owned(), unary_fn(f64::log10));
        funcs.insert("print".to_owned(), unary_fn(print));
        Self {
            vars: Variables::new(),
            funcs,
            uplevel: None,
        }
    }

    fn get_fn(&self, name: &str) -> Option<&FnDef<'a>> {
        let mut next_frame = Some(self);
        while let Some(frame) = next_frame {
            if let Some(func) = frame.funcs.get(name) {
                return Some(func);
            }
            next_frame = frame.uplevel;
        }
        None
    }

    fn push_stack(uplevel: &'a Self) -> Self {
        Self {
            vars: BTreeMap::new(),
            funcs: BTreeMap::new(),
            uplevel: Some(uplevel),
        }
    }
}

fn binary_fn<'a>(f: fn(f64, f64) -> f64) -> FnDef<'a> {
    FnDef::Native(NativeFn {
        code: Box::new(move |args| {
            let mut args = args.into_iter();
            let lhs = args.next().expect("function missing the first argument");
            let rhs = args.next().expect("function missing the second argument");
            f(*lhs, *rhs)
        }),
    })
}

fn break_statement(input: &str) -> IResult<&str, Statement> {
    let (input, _) = space_delimited(tag("break"))(input)?;
    Ok((input, Statement::Break))
}

fn cond_expr(input: &str) -> IResult<&str, Expression> {
    let (input, first) = num_expr(input)?;
    let (input, cond) = space_delimited(alt((char('<'), char('>'))))(input)?;
    let (input, second) = num_expr(input)?;
    Ok((
        input,
        match cond {
            '<' => Expression::Lt(Box::new(first), Box::new(second)),
            '>' => Expression::Gt(Box::new(first), Box::new(second)),
            _ => unreachable!(),
        },
    ))
}

fn continue_statement(input: &str) -> IResult<&str, Statement> {
    let (input, _) = space_delimited(tag("continue"))(input)?;
    Ok((input, Statement::Continue))
}

fn expr(input: &str) -> IResult<&str, Expression> {
    alt((if_expr, cond_expr, num_expr))(input)
}

fn expr_statement(input: &str) -> IResult<&str, Statement> {
    let (input, expr) = expr(input)?;
    Ok((input, Statement::Expression(expr)))
}

fn eval<'a>(expr: &Expression<'a>, frame: &mut StackFrame<'a>) -> EvalResult {
    let res = match expr {
        Expression::Ident("pi") => std::f64::consts::PI,
        Expression::Ident(id) => *frame.vars.get(*id).expect("Unknown variable"),
        Expression::NumLiteral(n) => *n,
        Expression::Add(lhs, rhs) => eval(lhs, frame)? + eval(rhs, frame)?,
        Expression::Sub(lhs, rhs) => eval(lhs, frame)? - eval(rhs, frame)?,
        Expression::Mul(lhs, rhs) => eval(lhs, frame)? * eval(rhs, frame)?,
        Expression::Div(lhs, rhs) => eval(lhs, frame)? / eval(rhs, frame)?,
        Expression::FnInvoke(ident, args) => {
            let mut arg_vals = vec![];
            for arg in args {
                arg_vals.push(eval(arg, frame)?);
            }
            if let Some(func) = frame.get_fn(*ident) {
                func.call(&arg_vals, frame)
            } else {
                panic!("Unknown function {:?}", ident);
            }
        }
        Expression::If(cond, t_case, f_case) => {
            if eval(cond, frame)? != 0.0 {
                eval_stmts(t_case, frame)?
            } else if let Some(f_case) = f_case {
                eval_stmts(f_case, frame)?
            } else {
                EvalResult::Continue(0.0)?
            }
        }
        Expression::Lt(lhs, rhs) => {
            let lhs = eval(lhs, frame)?;
            let rhs = eval(rhs, frame)?;
            if lhs < rhs {
                1.0
            } else {
                0.0
            }
        }
        Expression::Gt(lhs, rhs) => {
            let lhs = eval(lhs, frame)?;
            let rhs = eval(rhs, frame)?;
            if lhs > rhs {
                1.0
            } else {
                0.0
            }
        }
    };
    EvalResult::Continue(res)
}

fn eval_stmts<'a>(stmts: &[Statement<'a>], frame: &mut StackFrame<'a>) -> EvalResult {
    let mut last_result = EvalResult::Continue(0.0);
    for statement in stmts {
        match statement {
            Statement::Expression(expr) => {
                last_result = EvalResult::Continue(eval(expr, frame)?);
            }
            Statement::VarDef(name, expr) => {
                let value = eval(expr, frame)?;
                frame.vars.insert(name.to_string(), value);
            }
            Statement::VarAssign(name, expr) => {
                if !frame.vars.contains_key(*name) {
                    panic!("Variable is not defined");
                }

                let value = eval(expr, frame)?;
                frame.vars.insert(name.to_string(), value);
            }
            Statement::For {
                loop_var,
                start,
                end,
                stmts,
            } => {
                let start = eval(start, frame)? as isize;
                let end = eval(end, frame)? as isize;
                for i in start..end {
                    frame.vars.insert(loop_var.to_string(), i as f64);
                    match eval_stmts(stmts, frame) {
                        EvalResult::Continue(val) => last_result = EvalResult::Continue(val),
                        EvalResult::Break(BreakResult::Return(val)) => {
                            return EvalResult::Break(BreakResult::Return(val))
                        }
                        EvalResult::Break(BreakResult::Break) => break,
                        EvalResult::Break(BreakResult::Continue) => continue,
                    }
                }
            }
            Statement::FnDef { name, args, stmts } => {
                frame.funcs.insert(
                    name.to_string(),
                    FnDef::User(UserFn {
                        args: args.clone(),
                        stmts: stmts.clone(),
                    }),
                );
            }
            Statement::Return(expr) => {
                return EvalResult::Break(BreakResult::Return(eval(expr, frame)?));
            }
            Statement::Break => {
                return EvalResult::Break(BreakResult::Break);
            }
            Statement::Continue => {
                return EvalResult::Break(BreakResult::Continue);
            }
        }
    }
    last_result
}

fn factor(input: &str) -> IResult<&str, Expression> {
    alt((number, func_call, ident, paren))(input)
}

fn fn_def_statement(input: &str) -> IResult<&str, Statement> {
    let (input, _) = space_delimited(tag("fn"))(input)?;
    let (input, name) = space_delimited(identifier)(input)?;
    let (input, _) = space_delimited(tag("("))(input)?;
    let (input, args) = separated_list0(char(','), space_delimited(identifier))(input)?;
    let (input, _) = space_delimited(tag(")"))(input)?;
    let (input, stmts) = delimited(
        space_delimited(tag("{")),
        statements,
        space_delimited(tag("}")),
    )(input)?;
    Ok((input, Statement::FnDef { name, args, stmts }))
}

fn for_statement(input: &str) -> IResult<&str, Statement> {
    let (input, _) = space_delimited(tag("for"))(input)?;
    let (input, loop_var) = space_delimited(identifier)(input)?;
    let (input, _) = space_delimited(tag("in"))(input)?;
    let (input, start) = space_delimited(expr)(input)?;
    let (input, _) = space_delimited(tag("to"))(input)?;
    let (input, end) = space_delimited(expr)(input)?;
    let (input, stmts) = delimited(
        space_delimited(tag("{")),
        statements,
        space_delimited(tag("}")),
    )(input)?;
    Ok((
        input,
        Statement::For {
            loop_var,
            start,
            end,
            stmts,
        },
    ))
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

fn if_expr(input: &str) -> IResult<&str, Expression> {
    let (input, _) = space_delimited(tag("if"))(input)?;
    let (input, cond) = expr(input)?;
    let (input, t_case) = delimited(
        space_delimited(char('{')),
        statements,
        space_delimited(char('}')),
    )(input)?;
    let (input, f_case) = opt(preceded(
        space_delimited(tag("else")),
        delimited(
            space_delimited(char('{')),
            statements,
            space_delimited(char('}')),
        ),
    ))(input)?;
    Ok((
        input,
        Expression::If(Box::new(cond), Box::new(t_case), f_case.map(Box::new)),
    ))
}

fn lparen(input: &str) -> IResult<&str, &str> {
    tag("(")(input)
}

fn num_expr(input: &str) -> IResult<&str, Expression> {
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

fn print(arg: f64) -> f64 {
    println!("print: {}", arg);
    0.0
}

fn return_statement(input: &str) -> IResult<&str, Statement> {
    let (input, _) = space_delimited(tag("return"))(input)?;
    let (input, expr) = space_delimited(expr)(input)?;
    Ok((input, Statement::Return(expr)))
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
    let terminator = move |input| -> IResult<&str, ()> {
        let mut semicolon = pair(tag(";"), multispace0);
        Ok((semicolon(input)?.0, ()))
    };
    alt((
        var_def,
        var_assign,
        fn_def_statement,
        for_statement,
        terminated(break_statement, terminator),
        terminated(continue_statement, terminator),
        terminated(return_statement, terminator),
        terminated(expr_statement, terminator),
    ))(input)
}

fn statements(input: &str) -> IResult<&str, Statements> {
    let (input, stmts) = many0(statement)(input)?;
    let (input, _) = opt(char(';'))(input)?;
    Ok((input, stmts))
}

fn statements_finish(input: &str) -> Result<Statements, nom::error::Error<&str>> {
    let (_, res) = statements(input).finish()?;
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

fn unary_fn<'a>(f: fn(f64) -> f64) -> FnDef<'a> {
    FnDef::Native(NativeFn {
        code: Box::new(move |args| {
            let mut args = args.into_iter();
            f(*args.next().expect("function missing argument"))
        }),
    })
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
    fn test_break_statement() {
        assert_eq!(break_statement("break"), Ok(("", Statement::Break)));
    }

    #[test]
    fn test_continue_statement() {
        assert_eq!(
            continue_statement("continue"),
            Ok(("", Statement::Continue))
        );
    }

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
    fn test_ident() {
        assert_eq!(ident("Adam"), Ok(("", Expression::Ident("Adam"))));
        assert_eq!(ident("abc"), Ok(("", Expression::Ident("abc"))));
        assert!(ident("123abc").is_err());
        assert_eq!(ident("abc123"), Ok(("", Expression::Ident("abc123"))));
        assert_eq!(ident("abc123 "), Ok(("", Expression::Ident("abc123"))));
    }

    #[test]
    fn test_if_expr() {
        assert_eq!(
            if_expr("if 123 { 456; }"),
            Ok((
                "",
                Expression::If(
                    Box::new(Expression::NumLiteral(123.0)),
                    Box::new(vec![Statement::Expression(Expression::NumLiteral(456.0))],),
                    None,
                )
            ))
        );
        assert_eq!(
            if_expr("if 123 { 456; } else { 789; }"),
            Ok((
                "",
                Expression::If(
                    Box::new(Expression::NumLiteral(123.0)),
                    Box::new(vec![Statement::Expression(Expression::NumLiteral(456.0))],),
                    Some(Box::new(vec![Statement::Expression(
                        Expression::NumLiteral(789.0)
                    )]))
                )
            ))
        );
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

    #[test]
    fn test_return_statement() {
        assert_eq!(
            return_statement("return 123"),
            Ok(("", Statement::Return(Expression::NumLiteral(123.0))))
        );
    }

    #[test]
    fn test_statements_finish() {
        assert_eq!(
            statements_finish("123; 456;"),
            Ok(vec![
                Statement::Expression(Expression::NumLiteral(123.0)),
                Statement::Expression(Expression::NumLiteral(456.0))
            ])
        );
        assert_eq!(
            statements_finish("fn add(a, b) { a + b; } add(1, 2);"),
            Ok(vec![
                Statement::FnDef {
                    name: "add",
                    args: vec!["a", "b"],
                    stmts: vec![Statement::Expression(Expression::Add(
                        Box::new(Expression::Ident("a")),
                        Box::new(Expression::Ident("b"))
                    ))],
                },
                Statement::Expression(Expression::FnInvoke(
                    "add",
                    vec![Expression::NumLiteral(1.0), Expression::NumLiteral(2.0)]
                ))
            ])
        );

        assert_eq!(
            statements_finish("if 1 { 123; } else { 456; };"),
            Ok(vec![Statement::Expression(Expression::If(
                Box::new(Expression::NumLiteral(1.0)),
                Box::new(vec![Statement::Expression(Expression::NumLiteral(123.0))]),
                Some(Box::new(vec![Statement::Expression(
                    Expression::NumLiteral(456.0)
                )]))
            ))])
        );

        assert_eq!(
            statements_finish(
                "fn earlyreturn(a, b) { if a < b { return a; }; b; } earlyreturn(1, 2);"
            ),
            Ok(vec![
                Statement::FnDef {
                    name: "earlyreturn",
                    args: vec!["a", "b"],
                    stmts: vec![
                        Statement::Expression(Expression::If(
                            Box::new(Expression::Lt(
                                Box::new(Expression::Ident("a")),
                                Box::new(Expression::Ident("b"))
                            )),
                            Box::new(vec![Statement::Return(Expression::Ident("a"))]),
                            None
                        )),
                        Statement::Expression(Expression::Ident("b"))
                    ],
                },
                Statement::Expression(Expression::FnInvoke(
                    "earlyreturn",
                    vec![Expression::NumLiteral(1.0), Expression::NumLiteral(2.0)]
                ))
            ])
        );

        assert_eq!(
            statements_finish(
                "for i in 0 to 3 { for j in 0 to 3 { if j > 1 { break; }; print(i * 10 + j); } }"
            ),
            Ok(vec![Statement::For {
                loop_var: "i",
                start: Expression::NumLiteral(0.0),
                end: Expression::NumLiteral(3.0),
                stmts: vec![Statement::For {
                    loop_var: "j",
                    start: Expression::NumLiteral(0.0),
                    end: Expression::NumLiteral(3.0),
                    stmts: vec![
                        Statement::Expression(Expression::If(
                            Box::new(Expression::Gt(
                                Box::new(Expression::Ident("j")),
                                Box::new(Expression::NumLiteral(1.0))
                            )),
                            Box::new(vec![Statement::Break]),
                            None
                        )),
                        Statement::Expression(Expression::FnInvoke(
                            "print",
                            vec![Expression::Add(
                                Box::new(Expression::Mul(
                                    Box::new(Expression::Ident("i")),
                                    Box::new(Expression::NumLiteral(10.0))
                                )),
                                Box::new(Expression::Ident("j"))
                            )]
                        ))
                    ]
                }],
            }])
        );
    }
}
