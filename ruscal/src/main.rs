use std::collections::BTreeMap;
use std::io::Read;
use std::ops::ControlFlow;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, char, multispace0, multispace1, none_of};
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

pub struct TypeCheckContext<'a> {
    vars: BTreeMap<&'a str, TypeDecl>,
    funcs: BTreeMap<String, FnDef<'a>>,
    super_context: Option<&'a TypeCheckContext<'a>>,
}

impl<'a> TypeCheckContext<'a> {
    fn new() -> Self {
        Self {
            vars: BTreeMap::new(),
            funcs: BTreeMap::new(),
            super_context: None,
        }
    }

    fn get_var(&self, name: &str) -> Option<TypeDecl> {
        if let Some(val) = self.vars.get(name) {
            Some(val.clone())
        } else {
            None
        }
    }

    fn get_fn(&self, name: &str) -> Option<&FnDef<'a>> {
        if let Some(val) = self.funcs.get(name) {
            Some(val)
        } else if let Some(super_ctx) = self.super_context {
            super_ctx.get_fn(name)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct TypeCheckError {
    msg: String,
}

impl<'src> std::fmt::Display for TypeCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg,)
    }
}

impl TypeCheckError {
    fn new(msg: String) -> Self {
        Self { msg }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum TypeDecl {
    Any,
    F64,
    I64,
    Str,
}

#[derive(Clone, Debug, PartialEq)]
enum Value {
    F64(f64),
    I64(i64),
    Str(String),
}

impl Value {
    fn as_i64(&self) -> Option<i64> {
        match self {
            Self::F64(val) => Some(*val as i64),
            Self::I64(val) => Some(*val),
            Self::Str(val) => val.parse().ok(),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::F64(v) => write!(f, "{}", v),
            Self::I64(v) => write!(f, "{}", v),
            Self::Str(v) => write!(f, "{}", v),
        }
    }
}

impl std::cmp::PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::F64(lhs), Self::F64(rhs)) => lhs.partial_cmp(rhs),
            (Self::F64(lhs), Self::I64(rhs)) => lhs.partial_cmp(&(*rhs as f64)),
            (Self::F64(_), Self::Str(_)) => None,
            (Self::I64(lhs), Self::F64(rhs)) => (*lhs as f64).partial_cmp(rhs),
            (Self::I64(lhs), Self::I64(rhs)) => lhs.partial_cmp(rhs),
            (Self::I64(_), Self::Str(_)) => None,
            (Self::Str(_), Self::F64(_)) => None,
            (Self::Str(_), Self::I64(_)) => None,
            (Self::Str(lhs), Self::Str(rhs)) => lhs.partial_cmp(rhs),
        }
    }
}

impl std::ops::Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        binary_op_str(
            &self,
            &rhs,
            |lhs, rhs| lhs + rhs,
            |lhs, rhs| lhs + rhs,
            |lhs, rhs| lhs.to_owned() + rhs,
        )
    }
}

impl std::ops::Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        binary_op_str(
            &self,
            &rhs,
            |lhs, rhs| lhs - rhs,
            |lhs, rhs| lhs - rhs,
            |_, _| panic!("Strings cannot be subtracted"),
        )
    }
}

impl std::ops::Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        binary_op_str(
            &self,
            &rhs,
            |lhs, rhs| lhs * rhs,
            |lhs, rhs| lhs * rhs,
            |_, _| panic!("Strings cannot be multiplied"),
        )
    }
}

impl std::ops::Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        binary_op_str(
            &self,
            &rhs,
            |lhs, rhs| lhs / rhs,
            |lhs, rhs| lhs / rhs,
            |_, _| panic!("Strings cannot be divided"),
        )
    }
}

enum BreakResult {
    Return(Value),
    Break,
    Continue,
}

type EvalResult = ControlFlow<BreakResult, Value>;

type Variables = BTreeMap<String, Value>;

type Functions<'a> = BTreeMap<String, FnDef<'a>>;

#[derive(Clone, Debug, PartialEq)]
enum Statement<'a> {
    Expression(Expression<'a>),
    VarDef(&'a str, TypeDecl, Expression<'a>),
    VarAssign(&'a str, Expression<'a>),
    For {
        loop_var: &'a str,
        start: Expression<'a>,
        end: Expression<'a>,
        stmts: Statements<'a>,
    },
    FnDef {
        name: &'a str,
        args: Vec<(&'a str, TypeDecl)>,
        ret_type: TypeDecl,
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
    StrLiteral(String),
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
    Native(NativeFn<'a>),
}

impl<'a> FnDef<'a> {
    fn call(&self, args: &[Value], frame: &StackFrame) -> Value {
        match self {
            Self::User(code) => {
                let mut new_frame = StackFrame::push_stack(frame);
                new_frame.vars = args
                    .iter()
                    .zip(code.args.iter())
                    .map(|(arg, (name, _))| (name.to_string(), arg.clone()))
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
    args: Vec<(&'a str, TypeDecl)>,
    ret_type: TypeDecl,
    stmts: Statements<'a>,
}

struct NativeFn<'a> {
    args: Vec<(&'a str, TypeDecl)>,
    ret_type: TypeDecl,
    code: Box<dyn Fn(&[Value]) -> Value>,
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
        funcs.insert(
            "print".to_owned(),
            FnDef::Native(NativeFn {
                args: vec![("arg", TypeDecl::Any)],
                ret_type: TypeDecl::Any,
                code: Box::new(move |args| {
                    let mut args = args.into_iter();
                    let arg = args.next().expect("function missing argument");
                    print(arg)
                }),
            }),
        );
        funcs.insert(
            "dbg".to_owned(),
            FnDef::Native(NativeFn {
                args: vec![("arg", TypeDecl::Any)],
                ret_type: TypeDecl::Any,
                code: Box::new(move |args| {
                    let mut args = args.into_iter();
                    let arg = args.next().expect("function missing argument");
                    print_debug(arg)
                }),
            }),
        );
        funcs.insert(
            "f64".to_owned(),
            FnDef::Native(NativeFn {
                args: vec![("arg", TypeDecl::Any)],
                ret_type: TypeDecl::F64,
                code: Box::new(move |args| {
                    Value::F64(coerce_f64(args.first().expect("function missing argument")))
                }),
            }),
        );
        funcs.insert(
            "i64".to_owned(),
            FnDef::Native(NativeFn {
                args: vec![("arg", TypeDecl::Any)],
                ret_type: TypeDecl::I64,
                code: Box::new(move |args| {
                    Value::I64(coerce_i64(args.first().expect("function missing argument")))
                }),
            }),
        );
        funcs.insert(
            "str".to_owned(),
            FnDef::Native(NativeFn {
                args: vec![("arg", TypeDecl::Any)],
                ret_type: TypeDecl::Str,
                code: Box::new(move |args| {
                    Value::Str(coerce_str(args.first().expect("function missing argument")))
                }),
            }),
        );
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

fn argument(input: &str) -> IResult<&str, (&str, TypeDecl)> {
    let (input, ident) = space_delimited(identifier)(input)?;
    let (input, _) = char(':')(input)?;
    let (input, td) = type_decl(input)?;
    Ok((input, (ident, td)))
}

fn binary_fn<'a>(f: fn(f64, f64) -> f64) -> FnDef<'a> {
    FnDef::Native(NativeFn {
        args: vec![("lhs", TypeDecl::F64), ("rhs", TypeDecl::F64)],
        ret_type: TypeDecl::F64,
        code: Box::new(move |args| {
            let mut args = args.into_iter();
            let lhs = args.next().expect("function missing the first argument");
            let rhs = args.next().expect("function missing the second argument");
            Value::F64(f(coerce_f64(lhs), coerce_f64(rhs)))
        }),
    })
}

fn binary_op_str(
    lhs: &Value,
    rhs: &Value,
    d: impl Fn(f64, f64) -> f64,
    i: impl Fn(i64, i64) -> i64,
    s: impl Fn(&str, &str) -> String,
) -> Value {
    use Value::*;
    match (lhs, rhs) {
        (F64(lhs), rhs) => F64(d(*lhs, coerce_f64(&rhs))),
        (lhs, F64(rhs)) => F64(d(coerce_f64(&lhs), *rhs)),
        (I64(lhs), I64(rhs)) => I64(i(*lhs, *rhs)),
        (Str(lhs), Str(rhs)) => Str(s(lhs, rhs)),
        _ => {
            panic!("Unsupported operator between {:?} and {:?}", lhs, rhs)
        }
    }
}

fn break_statement(input: &str) -> IResult<&str, Statement> {
    let (input, _) = space_delimited(tag("break"))(input)?;
    Ok((input, Statement::Break))
}

fn coerce_f64(a: &Value) -> f64 {
    match a {
        Value::F64(v) => *v as f64,
        Value::I64(v) => *v as f64,
        _ => panic!("The string cloud not be parsed as f64"),
    }
}

fn coerce_i64(a: &Value) -> i64 {
    match a {
        Value::F64(v) => *v as i64,
        Value::I64(v) => *v as i64,
        _ => panic!("The string cloud not be parsed as i64"),
    }
}

fn coerce_str(a: &Value) -> String {
    match a {
        Value::F64(v) => v.to_string(),
        Value::I64(v) => v.to_string(),
        Value::Str(v) => v.to_owned(),
    }
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
        Expression::Ident("pi") => Value::F64(std::f64::consts::PI),
        Expression::Ident(id) => frame.vars.get(*id).cloned().expect("Unknown variable"),
        Expression::NumLiteral(n) => Value::F64(*n),
        Expression::StrLiteral(s) => Value::Str(s.to_owned()),
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
            if coerce_i64(&eval(cond, frame)?) != 0 {
                eval_stmts(t_case, frame)?
            } else if let Some(f_case) = f_case {
                eval_stmts(f_case, frame)?
            } else {
                Value::I64(0)
            }
        }
        Expression::Lt(lhs, rhs) => {
            let lhs = eval(lhs, frame)?;
            let rhs = eval(rhs, frame)?;
            if lhs < rhs {
                Value::I64(1)
            } else {
                Value::I64(0)
            }
        }
        Expression::Gt(lhs, rhs) => {
            let lhs = eval(lhs, frame)?;
            let rhs = eval(rhs, frame)?;
            if lhs > rhs {
                Value::I64(1)
            } else {
                Value::I64(0)
            }
        }
    };
    EvalResult::Continue(res)
}

fn eval_stmts<'a>(stmts: &[Statement<'a>], frame: &mut StackFrame<'a>) -> EvalResult {
    let mut last_result = EvalResult::Continue(Value::I64(0));
    for statement in stmts {
        match statement {
            Statement::Expression(expr) => {
                last_result = EvalResult::Continue(eval(expr, frame)?);
            }
            Statement::VarDef(name, _type_decl, expr) => {
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
                let start = eval(start, frame)?
                    .as_i64()
                    .expect("Start needs to be an integer");
                let end = eval(end, frame)?
                    .as_i64()
                    .expect("End needs to be an integer");
                for i in start..end {
                    frame.vars.insert(loop_var.to_string(), Value::I64(i));
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
            Statement::FnDef {
                name,
                args,
                ret_type,
                stmts,
            } => {
                frame.funcs.insert(
                    name.to_string(),
                    FnDef::User(UserFn {
                        args: args.clone(),
                        ret_type: *ret_type,
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
    alt((str_literal, number, func_call, ident, paren))(input)
}

fn fn_def_statement(input: &str) -> IResult<&str, Statement> {
    let (input, _) = delimited(multispace0, tag("fn"), multispace1)(input)?;
    let (input, name) = space_delimited(identifier)(input)?;
    let (input, _) = space_delimited(tag("("))(input)?;
    let (input, args) = separated_list0(char(','), space_delimited(argument))(input)?;
    let (input, _) = space_delimited(tag(")"))(input)?;
    let (input, _) = space_delimited(tag("->"))(input)?;
    let (input, ret_type) = space_delimited(type_decl)(input)?;
    let (input, stmts) = delimited(
        space_delimited(tag("{")),
        statements,
        space_delimited(tag("}")),
    )(input)?;
    Ok((
        input,
        Statement::FnDef {
            name,
            args,
            ret_type,
            stmts,
        },
    ))
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

fn print(arg: &Value) -> Value {
    println!("print: {}", arg);
    Value::I64(0)
}

fn print_debug(arg: &Value) -> Value {
    println!("dbg: {:?}", arg);
    Value::I64(0)
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

fn general_statement<'a>(last: bool) -> impl Fn(&'a str) -> IResult<&'a str, Statement> {
    let terminator = move |input| -> IResult<&str, ()> {
        let mut semicolon = pair(tag(";"), multispace0);
        if last {
            Ok((opt(semicolon)(input)?.0, ()))
        } else {
            Ok((semicolon(input)?.0, ()))
        }
    };
    move |input| {
        alt((
            terminated(var_def, terminator),
            terminated(var_assign, terminator),
            fn_def_statement,
            for_statement,
            terminated(break_statement, terminator),
            terminated(continue_statement, terminator),
            terminated(return_statement, terminator),
            terminated(expr_statement, terminator),
        ))(input)
    }
}

fn statement(input: &str) -> IResult<&str, Statement> {
    general_statement(true)(input)
}

fn last_statement(input: &str) -> IResult<&str, Statement> {
    general_statement(true)(input)
}

fn statements(input: &str) -> IResult<&str, Statements> {
    let (input, mut stmts) = many0(statement)(input)?;
    let (input, last) = opt(last_statement)(input)?;
    let (input, _) = opt(multispace0)(input)?;
    if let Some(last) = last {
        stmts.push(last);
    }
    Ok((input, stmts))
}

fn statements_finish(input: &str) -> Result<Statements, nom::error::Error<&str>> {
    let (_, res) = statements(input).finish()?;
    Ok(res)
}

fn str_literal(input: &str) -> IResult<&str, Expression> {
    let (input, _) = preceded(multispace0, char('"'))(input)?;
    let (input, val) = many0(none_of("\""))(input)?;
    let (input, _) = terminated(char('"'), multispace0)(input)?;
    Ok((
        input,
        Expression::StrLiteral(
            val.iter()
                .collect::<String>()
                .replace("\\\\", "\\")
                .replace("\\n", "\n"),
        ),
    ))
}

fn tc_coerce_type<'a>(value: &TypeDecl, target: &TypeDecl) -> Result<TypeDecl, TypeCheckError> {
    use TypeDecl::*;
    Ok(match (value, target) {
        (_, Any) => value.clone(),
        (Any, _) => target.clone(),
        (F64, F64) => F64,
        (I64, F64) => F64,
        (F64, I64) => F64,
        (Str, Str) => Str,
        _ => {
            return Err(TypeCheckError::new(format!(
                "Type check error {:?} cannot be assigned to {:?}",
                value, target
            )))
        }
    })
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

fn type_decl(input: &str) -> IResult<&str, TypeDecl> {
    let (input, td) = space_delimited(identifier)(input)?;
    Ok((
        input,
        match td {
            "i64" => TypeDecl::I64,
            "f64" => TypeDecl::F64,
            "str" => TypeDecl::Str,
            _ => {
                panic!("Type annotation has unknown type: {}", td)
            }
        },
    ))
}

fn unary_fn<'a>(f: fn(f64) -> f64) -> FnDef<'a> {
    FnDef::Native(NativeFn {
        args: vec![("arg", TypeDecl::F64)],
        ret_type: TypeDecl::F64,
        code: Box::new(move |args| {
            let mut args = args.into_iter();
            let arg = args.next().expect("function missing argument");
            Value::F64(f(coerce_f64(arg)))
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
    let (input, _) = delimited(multispace0, tag("var"), multispace1)(input)?;
    let (input, name) = space_delimited(identifier)(input)?;
    let (input, _) = space_delimited(char(':'))(input)?;
    let (input, td) = type_decl(input)?;
    let (input, _) = space_delimited(char('='))(input)?;
    let (input, expr) = space_delimited(expr)(input)?;
    Ok((input, Statement::VarDef(name, td, expr)))
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
    fn test_fn_def_statement() {
        assert_eq!(
            fn_def_statement("fn f(a: i64, b: f64) -> str { \"abc\"; }"),
            Ok((
                "",
                Statement::FnDef {
                    name: "f",
                    args: vec![("a", TypeDecl::I64), ("b", TypeDecl::F64)],
                    ret_type: TypeDecl::Str,
                    stmts: vec![Statement::Expression(Expression::StrLiteral(
                        "abc".to_owned()
                    ))]
                }
            ))
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
    fn test_str_literal() {
        assert_eq!(
            str_literal("\"abc\n\\\""),
            Ok(("", Expression::StrLiteral("abc\n\\".to_owned())))
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
            statements_finish("fn add(a: i64, b: i64) -> i64 { a + b; } add(1, 2);"),
            Ok(vec![
                Statement::FnDef {
                    name: "add",
                    args: vec![("a", TypeDecl::I64), ("b", TypeDecl::I64)],
                    ret_type: TypeDecl::I64,
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
                "fn earlyreturn(a: i64, b: i64) -> i64 { if a < b { return a; }; b; } earlyreturn(1, 2);"
            ),
            Ok(vec![
                Statement::FnDef {
                    name: "earlyreturn",
                    args: vec![("a", TypeDecl::I64), ("b", TypeDecl::I64)],
                    ret_type: TypeDecl::I64,
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

        assert_eq!(
            statements_finish(
                r#"
var i = i64(123);
var f = f64(123.456);
var s = "Hello, world!";

print(i);
dbg(i);
print(f);
dbg(f);
print(s);
dbg(s);

print(i + f);
print(i / f);
print(s + s);
                "#
            )
            .unwrap()
            .len(),
            12
        );
    }

    #[test]
    fn test_var_def() {
        assert_eq!(
            var_def("var i : i64 = 123"),
            Ok((
                "",
                Statement::VarDef("i", TypeDecl::I64, Expression::NumLiteral(123.0))
            ))
        );
        assert_eq!(
            var_def("var f:f64=456.7"),
            Ok((
                "",
                Statement::VarDef("f", TypeDecl::F64, Expression::NumLiteral(456.7))
            ))
        );
        assert_eq!(
            var_def("var s: str = \"abc\""),
            Ok((
                "",
                Statement::VarDef("s", TypeDecl::Str, Expression::StrLiteral("abc".to_owned()))
            ))
        );
    }
}
