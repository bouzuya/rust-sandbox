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
    let mut args = std::env::args();
    args.next();
    match args.next().as_ref().map(|s| s as &str) {
        Some("w") => write_program("bytecode.bin").unwrap(),
        Some("r") => {
            let byte_code = read_program("bytecode.bin").unwrap();
            let result = byte_code.interpret();
            println!("result {:?}", result);
        }
        _ => println!("Please specify w or r as an argument"),
    }
}

struct ByteCode {
    literals: Vec<i64>,
    instructions: Vec<Instruction>,
}

impl ByteCode {
    fn new() -> Self {
        Self {
            literals: vec![],
            instructions: vec![],
        }
    }

    fn interpret(&self) -> Option<i64> {
        let mut stack = vec![];
        for instruction in &self.instructions {
            match instruction.op {
                OpCode::LoadLiteral => stack.push(self.literals[instruction.arg0 as usize]),
                OpCode::Add => {
                    let rhs = stack.pop().expect("Stack underflow");
                    let lhs = stack.pop().expect("Stack underflow");
                    stack.push(lhs + rhs);
                }
            }
        }
        stack.pop()
    }

    fn read_literals(&mut self, reader: &mut impl std::io::Read) -> std::io::Result<()> {
        let num_literals = deserialize_size(reader)?;
        for _ in 0..num_literals {
            let mut buf = [0u8; std::mem::size_of::<i64>()];
            reader.read_exact(&mut buf)?;
            self.literals.push(i64::from_le_bytes(buf));
        }
        Ok(())
    }

    fn read_instructions(&mut self, reader: &mut impl std::io::Read) -> std::io::Result<()> {
        let num_instructions = deserialize_size(reader)?;
        for _ in 0..num_instructions {
            let instruction = Instruction::deserialize(reader)?;
            self.instructions.push(instruction);
        }
        Ok(())
    }
}

fn deserialize_size(reader: &mut impl std::io::Read) -> std::io::Result<usize> {
    let mut buf = [0u8; std::mem::size_of::<u32>()];
    reader.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf) as usize)
}

struct Compiler {
    literals: Vec<i64>,
    instructions: Vec<Instruction>,
}

impl Compiler {
    fn new() -> Self {
        Self {
            literals: vec![],
            instructions: vec![],
        }
    }

    fn add_instruction(&mut self, op: OpCode, arg0: u8) -> u8 {
        let addr = self.instructions.len();
        self.instructions.push(Instruction { op, arg0 });
        addr as u8
    }

    fn add_literal(&mut self, value: i64) -> u8 {
        let addr = self.literals.len();
        self.literals.push(value);
        addr as u8
    }

    fn write_instructions(&self, writer: &mut impl std::io::Write) -> std::io::Result<()> {
        serialize_size(self.instructions.len(), writer)?;
        for instruction in &self.instructions {
            instruction.serialize(writer)?;
        }
        Ok(())
    }

    fn write_literals(&self, writer: &mut impl std::io::Write) -> std::io::Result<()> {
        serialize_size(self.literals.len(), writer)?;
        for literal in &self.literals {
            writer.write_all(&literal.to_le_bytes())?;
        }
        Ok(())
    }
}

fn serialize_size(size: usize, writer: &mut impl std::io::Write) -> std::io::Result<()> {
    writer.write_all(&(size as u32).to_le_bytes())
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
enum OpCode {
    LoadLiteral,
    Add,
}

impl From<u8> for OpCode {
    fn from(o: u8) -> Self {
        const LOAD_LITERAL: u8 = OpCode::LoadLiteral as u8;
        const ADD: u8 = OpCode::Add as u8;
        match o {
            LOAD_LITERAL => OpCode::LoadLiteral,
            ADD => OpCode::Add,
            _ => panic!("OpCode \"{:02X}\" unrecognized!", o),
        }
    }
}

#[repr(C)]
struct Instruction {
    op: OpCode,
    arg0: u8,
}

impl Instruction {
    fn new(op: OpCode, arg0: u8) -> Self {
        Self { op, arg0 }
    }

    fn deserialize(reader: &mut impl std::io::Read) -> Result<Self, std::io::Error> {
        let mut buf = [0u8; 2];
        reader.read_exact(&mut buf)?;
        Ok(Self::new(buf[0].into(), buf[1]))
    }

    fn serialize(&self, writer: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        writer.write_all(&[self.op as u8, self.arg0])?;
        Ok(())
    }
}

fn read_program(file: &str) -> std::io::Result<ByteCode> {
    let reader = std::fs::File::open(file)?;
    let mut reader = std::io::BufReader::new(reader);
    let mut byte_code = ByteCode::new();
    byte_code.read_literals(&mut reader)?;
    byte_code.read_instructions(&mut reader)?;
    Ok(byte_code)
}

fn write_program(file: &str) -> std::io::Result<()> {
    let mut compiler = Compiler::new();
    let arg = compiler.add_literal(512);
    compiler.add_instruction(OpCode::LoadLiteral, arg);
    let arg = compiler.add_literal(1024);
    compiler.add_instruction(OpCode::LoadLiteral, arg);
    compiler.add_instruction(OpCode::Add, 0);

    let writer = std::fs::File::create(file)?;

    let mut writer = std::io::BufWriter::new(writer);
    compiler.write_literals(&mut writer)?;
    compiler.write_instructions(&mut writer)?;

    println!("Written {} instructions", compiler.instructions.len());
    Ok(())
}

// ---

pub fn main1() {
    let mut buf = String::new();
    if !std::io::stdin().read_to_string(&mut buf).is_ok() {
        panic!("Failed to read from stdin");
    }
    let parsed_statements = match statements_finish(Span::new(&buf)) {
        Ok(parsed_statements) => parsed_statements,
        Err(e) => {
            eprintln!("Parse error: {:?}", e);
            return;
        }
    };

    let mut tc_ctx = TypeCheckContext::new();
    if let Err(err) = type_check(&parsed_statements, &mut tc_ctx) {
        println!("Type check error: {}", err);
        return;
    }
    println!("Type check OK");

    let mut frame = StackFrame::new();
    eval_stmts(&parsed_statements, &mut frame);
}

type Span<'a> = nom_locate::LocatedSpan<&'a str>;

pub struct TypeCheckContext<'a, 'b> {
    vars: BTreeMap<&'a str, TypeDecl>,
    funcs: BTreeMap<String, FnDef<'a>>,
    super_context: Option<&'b TypeCheckContext<'a, 'b>>,
}

impl<'a, 'b> TypeCheckContext<'a, 'b> {
    fn new() -> Self {
        Self {
            vars: BTreeMap::new(),
            funcs: standard_functions(),
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

    fn push_stack(super_ctx: &'b Self) -> Self {
        Self {
            vars: BTreeMap::new(),
            funcs: BTreeMap::new(),
            super_context: Some(super_ctx),
        }
    }
}

#[derive(Debug)]
pub struct TypeCheckError<'a> {
    msg: String,
    span: Span<'a>,
}

impl<'a> std::fmt::Display for TypeCheckError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\nlocation: {}:{}",
            self.msg,
            self.span.location_line(),
            self.span.get_utf8_column()
        )
    }
}

impl<'a> TypeCheckError<'a> {
    fn new(msg: String, span: Span<'a>) -> Self {
        Self { msg, span }
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
    VarDef {
        span: Span<'a>,
        name: Span<'a>,
        td: TypeDecl,
        expr: Expression<'a>,
    },
    VarAssign {
        span: Span<'a>,
        name: Span<'a>,
        expr: Expression<'a>,
    },
    For {
        span: Span<'a>,
        loop_var: Span<'a>,
        start: Expression<'a>,
        end: Expression<'a>,
        stmts: Statements<'a>,
    },
    FnDef {
        name: Span<'a>,
        args: Vec<(Span<'a>, TypeDecl)>,
        ret_type: TypeDecl,
        stmts: Statements<'a>,
    },
    Return(Expression<'a>),
    Break,
    Continue,
}

impl<'a> Statement<'a> {
    fn span(&self) -> Option<Span<'a>> {
        use Statement::*;
        Some(match self {
            Expression(expr) => expr.span,
            VarDef { span, .. } => *span,
            VarAssign { span, .. } => *span,
            For { span, .. } => *span,
            FnDef { name, stmts, .. } => calc_offset(*name, stmts.span()),
            Return(expr) => expr.span,
            Break | Continue => return None,
        })
    }
}

trait GetSpan<'a> {
    fn span(&self) -> Span<'a>;
}

type Statements<'a> = Vec<Statement<'a>>;

impl<'a> GetSpan<'a> for Statements<'a> {
    fn span(&self) -> Span<'a> {
        self.iter().find_map(|it| it.span()).unwrap()
    }
}

#[derive(Clone, Debug, PartialEq)]
enum ExprEnum<'a> {
    Ident(Span<'a>),
    NumLiteral(f64),
    StrLiteral(String),
    FnInvoke(Span<'a>, Vec<Expression<'a>>),
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

#[derive(Clone, Debug, PartialEq)]
struct Expression<'a> {
    expr: ExprEnum<'a>,
    span: Span<'a>,
}

impl<'a> Expression<'a> {
    fn new(expr: ExprEnum<'a>, span: Span<'a>) -> Self {
        Self { expr, span }
    }
}

enum FnDef<'a> {
    User(UserFn<'a>),
    Native(NativeFn<'a>),
}

impl<'a> FnDef<'a> {
    fn args(&self) -> Vec<(&'a str, TypeDecl)> {
        match self {
            Self::User(f) => f.args.iter().map(|it| (&**it.0, it.1)).collect(),
            Self::Native(f) => f.args.clone(),
        }
    }

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

    fn ret_type(&self) -> TypeDecl {
        match self {
            Self::User(f) => f.ret_type,
            Self::Native(f) => f.ret_type,
        }
    }
}

struct UserFn<'a> {
    args: Vec<(Span<'a>, TypeDecl)>,
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
        Self {
            vars: Variables::new(),
            funcs: standard_functions(),
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

fn argument(i: Span) -> IResult<Span, (Span, TypeDecl)> {
    let (i, ident) = space_delimited(identifier)(i)?;
    let (i, _) = char(':')(i)?;
    let (i, td) = type_decl(i)?;
    Ok((i, (ident, td)))
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

fn binary_op_type(lhs: &TypeDecl, rhs: &TypeDecl) -> Result<TypeDecl, ()> {
    use TypeDecl::*;
    Ok(match (lhs, rhs) {
        (Any, _) => Any,
        (_, Any) => Any,
        (I64, I64) => I64,
        (F64 | I64, F64 | I64) => F64,
        (Str, Str) => Str,
        _ => return Err(()),
    })
}

fn break_statement(i: Span) -> IResult<Span, Statement> {
    let (i, _) = space_delimited(tag("break"))(i)?;
    Ok((i, Statement::Break))
}

fn calc_offset<'a>(i: Span<'a>, r: Span<'a>) -> Span<'a> {
    use nom::{InputTake, Offset};
    i.take(i.offset(&r))
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

fn cond_expr(i: Span) -> IResult<Span, Expression> {
    let start = i;
    let (i, first) = num_expr(i)?;
    let (i, cond) = space_delimited(alt((char('<'), char('>'))))(i)?;
    let (i, second) = num_expr(i)?;
    Ok((
        i,
        match cond {
            '<' => Expression::new(
                ExprEnum::Lt(Box::new(first), Box::new(second)),
                calc_offset(start, i),
            ),
            '>' => Expression::new(
                ExprEnum::Gt(Box::new(first), Box::new(second)),
                calc_offset(start, i),
            ),
            _ => unreachable!(),
        },
    ))
}

fn continue_statement(i: Span) -> IResult<Span, Statement> {
    let (i, _) = space_delimited(tag("continue"))(i)?;
    Ok((i, Statement::Continue))
}

fn expr(i: Span) -> IResult<Span, Expression> {
    alt((if_expr, cond_expr, num_expr))(i)
}

fn expr_statement(i: Span) -> IResult<Span, Statement> {
    let (i, expr) = expr(i)?;
    Ok((i, Statement::Expression(expr)))
}

fn eval<'a>(expr: &Expression<'a>, frame: &mut StackFrame<'a>) -> EvalResult {
    let res = match &expr.expr {
        ExprEnum::Ident(id) => {
            if **id == "pi" {
                Value::F64(std::f64::consts::PI)
            } else {
                frame.vars.get(**id).cloned().expect("Unknown variable")
            }
        }
        ExprEnum::NumLiteral(n) => Value::F64(*n),
        ExprEnum::StrLiteral(s) => Value::Str(s.to_owned()),
        ExprEnum::Add(lhs, rhs) => eval(lhs, frame)? + eval(rhs, frame)?,
        ExprEnum::Sub(lhs, rhs) => eval(lhs, frame)? - eval(rhs, frame)?,
        ExprEnum::Mul(lhs, rhs) => eval(lhs, frame)? * eval(rhs, frame)?,
        ExprEnum::Div(lhs, rhs) => eval(lhs, frame)? / eval(rhs, frame)?,
        ExprEnum::Lt(lhs, rhs) => {
            let lhs = eval(lhs, frame)?;
            let rhs = eval(rhs, frame)?;
            if lhs < rhs {
                Value::I64(1)
            } else {
                Value::I64(0)
            }
        }
        ExprEnum::Gt(lhs, rhs) => {
            let lhs = eval(lhs, frame)?;
            let rhs = eval(rhs, frame)?;
            if lhs > rhs {
                Value::I64(1)
            } else {
                Value::I64(0)
            }
        }
        ExprEnum::If(cond, t_case, f_case) => {
            if coerce_i64(&eval(cond, frame)?) != 0 {
                eval_stmts(t_case, frame)?
            } else if let Some(f_case) = f_case {
                eval_stmts(f_case, frame)?
            } else {
                Value::I64(0)
            }
        }
        ExprEnum::FnInvoke(ident, args) => {
            let mut arg_vals = vec![];
            for arg in args {
                arg_vals.push(eval(arg, frame)?);
            }
            if let Some(func) = frame.get_fn(**ident) {
                func.call(&arg_vals, frame)
            } else {
                panic!("Unknown function {:?}", ident);
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
            Statement::VarDef { name, expr, .. } => {
                let value = eval(expr, frame)?;
                frame.vars.insert(name.to_string(), value);
            }
            Statement::VarAssign { name, expr, .. } => {
                if !frame.vars.contains_key(**name) {
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
                ..
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

fn factor(i: Span) -> IResult<Span, Expression> {
    alt((str_literal, num, func_call, ident, paren))(i)
}

fn fn_def_statement(i: Span) -> IResult<Span, Statement> {
    let (i, _) = delimited(multispace0, tag("fn"), multispace1)(i)?;
    let (i, name) = space_delimited(identifier)(i)?;
    let (i, _) = space_delimited(tag("("))(i)?;
    let (i, args) = separated_list0(char(','), space_delimited(argument))(i)?;
    let (i, _) = space_delimited(tag(")"))(i)?;
    let (i, _) = space_delimited(tag("->"))(i)?;
    let (i, ret_type) = space_delimited(type_decl)(i)?;
    let (i, stmts) = delimited(
        space_delimited(tag("{")),
        statements,
        space_delimited(tag("}")),
    )(i)?;
    Ok((
        i,
        Statement::FnDef {
            name,
            args,
            ret_type,
            stmts,
        },
    ))
}

fn for_statement(i: Span) -> IResult<Span, Statement> {
    let start_i = i;
    let (i, _) = space_delimited(tag("for"))(i)?;
    let (i, loop_var) = space_delimited(identifier)(i)?;
    let (i, _) = space_delimited(tag("in"))(i)?;
    let (i, start) = space_delimited(expr)(i)?;
    let (i, _) = space_delimited(tag("to"))(i)?;
    let (i, end) = space_delimited(expr)(i)?;
    let (i, stmts) = delimited(
        space_delimited(tag("{")),
        statements,
        space_delimited(tag("}")),
    )(i)?;
    Ok((
        i,
        Statement::For {
            span: calc_offset(start_i, i),
            loop_var,
            start,
            end,
            stmts,
        },
    ))
}

fn func_call(i: Span) -> IResult<Span, Expression> {
    let start = i;
    let (i, ident) = space_delimited(identifier)(i)?;
    let (i, args) = space_delimited(delimited(
        tag("("),
        many0(delimited(multispace0, expr, space_delimited(opt(tag(","))))),
        tag(")"),
    ))(i)?;
    Ok((i, Expression::new(ExprEnum::FnInvoke(ident, args), start)))
}

fn general_statement<'a>(last: bool) -> impl Fn(Span<'a>) -> IResult<Span<'a>, Statement> {
    let terminator = move |input| -> IResult<Span, ()> {
        let mut semicolon = pair(tag(";"), multispace0);
        if last {
            Ok((opt(semicolon)(input)?.0, ()))
        } else {
            Ok((semicolon(input)?.0, ()))
        }
    };
    move |input| {
        alt((
            fn_def_statement,
            for_statement,
            terminated(var_def_statement, terminator),
            terminated(var_assign_statement, terminator),
            terminated(break_statement, terminator),
            terminated(continue_statement, terminator),
            terminated(return_statement, terminator),
            terminated(expr_statement, terminator),
        ))(input)
    }
}

fn identifier(i: Span) -> IResult<Span, Span> {
    recognize(pair(alpha1, many0(alphanumeric1)))(i)
}

fn ident(i: Span) -> IResult<Span, Expression> {
    let (i, value) = space_delimited(identifier)(i)?;
    Ok((i, Expression::new(ExprEnum::Ident(value), value)))
}

fn if_expr(i: Span) -> IResult<Span, Expression> {
    let start = i;
    let (i, _) = space_delimited(tag("if"))(i)?;
    let (i, cond) = expr(i)?;
    let (i, t_case) = delimited(
        space_delimited(char('{')),
        statements,
        space_delimited(char('}')),
    )(i)?;
    let (i, f_case) = opt(preceded(
        space_delimited(tag("else")),
        delimited(
            space_delimited(char('{')),
            statements,
            space_delimited(char('}')),
        ),
    ))(i)?;
    Ok((
        i,
        Expression::new(
            ExprEnum::If(Box::new(cond), Box::new(t_case), f_case.map(Box::new)),
            calc_offset(start, i),
        ),
    ))
}

fn last_statement(i: Span) -> IResult<Span, Statement> {
    general_statement(true)(i)
}

fn num_expr(i: Span) -> IResult<Span, Expression> {
    use nom::Slice;
    let start = i;
    let (i, lhs) = term(i)?;
    let res = fold_many0(
        pair(space_delimited(alt((char('+'), char('-')))), term),
        move || lhs.clone(),
        |lhs, (op, rhs)| {
            let span = calc_offset(start, rhs.span.slice(rhs.span.len()..));
            match op {
                '+' => Expression::new(ExprEnum::Add(Box::new(lhs), Box::new(rhs)), span),
                '-' => Expression::new(ExprEnum::Sub(Box::new(lhs), Box::new(rhs)), span),
                _ => panic!("'+' or '-'"),
            }
        },
    )(i);
    res
}

fn number(i: Span) -> IResult<Span, Span> {
    recognize_float(i)
}

fn num(i: Span) -> IResult<Span, Expression> {
    let (i, value) = space_delimited(number)(i)?;
    Ok((
        i,
        Expression::new(
            ExprEnum::NumLiteral(value.parse::<f64>().expect("FIXME")),
            value,
        ),
    ))
}

fn paren(i: Span) -> IResult<Span, Expression> {
    space_delimited(delimited(char('('), expr, char(')')))(i)
}

fn print(arg: &Value) -> Value {
    println!("print: {}", arg);
    Value::I64(0)
}

fn print_debug(arg: &Value) -> Value {
    println!("dbg: {:?}", arg);
    Value::I64(0)
}

fn return_statement(i: Span) -> IResult<Span, Statement> {
    let (i, _) = space_delimited(tag("return"))(i)?;
    let (i, expr) = space_delimited(expr)(i)?;
    Ok((i, Statement::Return(expr)))
}

fn space_delimited<'a, O, E>(
    f: impl Parser<Span<'a>, O, E>,
) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, O, E>
where
    E: ParseError<Span<'a>>,
{
    delimited(multispace0, f, multispace0)
}

fn standard_functions<'a>() -> Functions<'a> {
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
    funcs
}

fn statement(i: Span) -> IResult<Span, Statement> {
    general_statement(true)(i)
}

fn statements(i: Span) -> IResult<Span, Statements> {
    let (i, mut stmts) = many0(statement)(i)?;
    let (i, last) = opt(last_statement)(i)?;
    let (i, _) = opt(multispace0)(i)?;
    if let Some(last) = last {
        stmts.push(last);
    }
    Ok((i, stmts))
}

fn statements_finish(i: Span) -> Result<Statements, nom::error::Error<Span>> {
    let (_, res) = statements(i).finish()?;
    Ok(res)
}

fn str_literal(i: Span) -> IResult<Span, Expression> {
    use nom::{Offset, Slice};
    let start = i;
    let (i, _) = preceded(multispace0, char('"'))(i)?;
    let s_start = start.offset(&i) - 1;
    let (i, val) = many0(none_of("\""))(i)?;
    let s_end = start.offset(&i) + 1;
    let (i, _) = terminated(char('"'), multispace0)(i)?;
    let span = start.slice(s_start..s_end);
    Ok((
        i,
        Expression::new(
            ExprEnum::StrLiteral(
                val.iter()
                    .collect::<String>()
                    .replace("\\\\", "\\")
                    .replace("\\n", "\n"),
            ),
            span,
        ),
    ))
}

fn tc_binary_cmp<'a>(
    lhs: &Expression<'a>,
    rhs: &Expression<'a>,
    ctx: &mut TypeCheckContext<'a, '_>,
    op: &str,
) -> Result<TypeDecl, TypeCheckError<'a>> {
    use TypeDecl::*;
    let lhs_t = tc_expr(lhs, ctx)?;
    let rhs_t = tc_expr(rhs, ctx)?;
    Ok(match (&lhs_t, &rhs_t) {
        (Any, _) => I64,
        (_, Any) => I64,
        (F64, F64) => I64,
        (I64, F64) => I64,
        (Str, Str) => I64,
        _ => {
            return Err(TypeCheckError::new(
                format!(
                    "Operation {} between incompatible type: {:?} and {:?}",
                    op, lhs_t, rhs_t
                ),
                lhs.span,
            ))
        }
    })
}

fn tc_binary_op<'a>(
    lhs: &Expression<'a>,
    rhs: &Expression<'a>,
    ctx: &mut TypeCheckContext<'a, '_>,
    op: &str,
) -> Result<TypeDecl, TypeCheckError<'a>> {
    let lhs_t = tc_expr(lhs, ctx)?;
    let rhs_t = tc_expr(rhs, ctx)?;
    binary_op_type(&lhs_t, &rhs_t).map_err(|_| {
        TypeCheckError::new(
            format!(
                "Operation {} between incompatible type: {:?} and {:?}",
                op, lhs_t, rhs_t
            ),
            lhs.span,
        )
    })
}

fn tc_coerce_type<'a>(
    value: &TypeDecl,
    target: &TypeDecl,
    span: Span<'a>,
) -> Result<TypeDecl, TypeCheckError<'a>> {
    use TypeDecl::*;
    Ok(match (value, target) {
        (_, Any) => value.clone(),
        (Any, _) => target.clone(),
        (F64, F64) => F64,
        (I64, F64) => F64,
        (F64, I64) => F64,
        (I64, I64) => I64,
        (Str, Str) => Str,
        _ => {
            return Err(TypeCheckError::new(
                format!(
                    "Type check error {:?} cannot be assigned to {:?}",
                    value, target
                ),
                span,
            ))
        }
    })
}

fn tc_expr<'a>(
    e: &Expression<'a>,
    ctx: &mut TypeCheckContext<'a, '_>,
) -> Result<TypeDecl, TypeCheckError<'a>> {
    use ExprEnum::*;
    Ok(match &e.expr {
        NumLiteral(_) => TypeDecl::F64,
        StrLiteral(_) => TypeDecl::Str,
        Ident(str) => ctx.get_var(str).ok_or_else(|| {
            TypeCheckError::new(format!("Variable {:?} not found in scope", str), e.span)
        })?,
        FnInvoke(name, args) => {
            let args_ty = args
                .iter()
                .map(|v| Ok((tc_expr(v, ctx)?, v.span)))
                .collect::<Result<Vec<_>, _>>()?;
            let func = ctx.get_fn(**name).ok_or_else(|| {
                TypeCheckError::new(format!("function {} is not defined", name), *name)
            })?;
            let args_decl = func.args();
            for ((arg_ty, arg_span), decl) in args_ty.iter().zip(args_decl.iter()) {
                tc_coerce_type(&arg_ty, &decl.1, *arg_span)?;
            }
            func.ret_type()
        }
        Add(lhs, rhs) => tc_binary_op(lhs, rhs, ctx, "Add")?,
        Sub(lhs, rhs) => tc_binary_op(lhs, rhs, ctx, "Sub")?,
        Mul(lhs, rhs) => tc_binary_op(lhs, rhs, ctx, "Mult")?,
        Div(lhs, rhs) => tc_binary_op(lhs, rhs, ctx, "Div")?,
        Lt(lhs, rhs) => tc_binary_cmp(lhs, rhs, ctx, "LT")?,
        Gt(lhs, rhs) => tc_binary_cmp(lhs, rhs, ctx, "GT")?,
        If(cond, true_branch, false_branch) => {
            tc_coerce_type(&tc_expr(cond, ctx)?, &TypeDecl::I64, cond.span)?;
            let true_type = type_check(true_branch, ctx)?;
            if let Some(false_branch) = false_branch {
                let false_type = type_check(false_branch, ctx)?;
                binary_op_type(&true_type, &false_type).map_err(|_| {
                    let true_span = true_branch.span();
                    let false_span = false_branch.span();
                    TypeCheckError::new(
                        format!(
                            "Conditional expression doesn't have the compatible types in true and false branch: {:?} and {:?}",
                            true_type, false_type
                        ),
                        calc_offset(true_span, false_span)
                    )
                })?
            } else {
                true_type
            }
        }
    })
}

fn term(i: Span) -> IResult<Span, Expression> {
    use nom::Slice;
    let start = i;
    let (i, init) = factor(i)?;
    let res = fold_many0(
        pair(space_delimited(alt((char('*'), char('/')))), factor),
        move || init.clone(),
        |lhs, (op, rhs): (char, Expression)| {
            let span = calc_offset(start, rhs.span.slice(rhs.span.len()..));
            match op {
                '*' => Expression::new(ExprEnum::Mul(Box::new(lhs), Box::new(rhs)), span),
                '/' => Expression::new(ExprEnum::Div(Box::new(lhs), Box::new(rhs)), span),
                _ => panic!("Multiplicative expression should have '*' or '/' operator"),
            }
        },
    )(i);
    res
}

fn type_check<'a>(
    stmts: &Vec<Statement<'a>>,
    ctx: &mut TypeCheckContext<'a, '_>,
) -> Result<TypeDecl, TypeCheckError<'a>> {
    let mut res = TypeDecl::Any;
    for stmt in stmts {
        match stmt {
            Statement::VarDef { name, td, expr, .. } => {
                let init_type = tc_expr(expr, ctx)?;
                let init_type = tc_coerce_type(&init_type, td, expr.span)?;
                ctx.vars.insert(**name, init_type);
            }
            Statement::VarAssign { name, expr, .. } => {
                let ty = tc_expr(expr, ctx)?;
                let var = ctx.vars.get(**name).expect("Variable not found");
                tc_coerce_type(&ty, var, expr.span)?;
            }
            Statement::FnDef {
                name,
                args,
                ret_type,
                stmts,
            } => {
                ctx.funcs.insert(
                    name.to_string(),
                    FnDef::User(UserFn {
                        args: args.clone(),
                        ret_type: *ret_type,
                        stmts: stmts.clone(),
                    }),
                );
                let mut subctx = TypeCheckContext::push_stack(ctx);
                for (arg, ty) in args.iter() {
                    subctx.vars.insert(arg, *ty);
                }
                let last_stmt = type_check(stmts, &mut subctx)?;
                tc_coerce_type(&last_stmt, &ret_type, stmts.span())?;
            }
            Statement::Expression(e) => {
                res = tc_expr(&e, ctx)?;
            }
            Statement::For {
                loop_var,
                start,
                end,
                stmts,
                ..
            } => {
                tc_coerce_type(&tc_expr(start, ctx)?, &TypeDecl::I64, start.span)?;
                tc_coerce_type(&tc_expr(end, ctx)?, &TypeDecl::I64, end.span)?;
                ctx.vars.insert(loop_var, TypeDecl::I64);
                res = type_check(stmts, ctx)?;
            }
            Statement::Return(e) => {
                return tc_expr(e, ctx);
            }
            Statement::Break => (),
            Statement::Continue => (),
        }
    }
    Ok(res)
}

fn type_decl(i: Span) -> IResult<Span, TypeDecl> {
    let (i, td) = space_delimited(identifier)(i)?;
    Ok((
        i,
        match *td {
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

fn var_assign_statement(i: Span) -> IResult<Span, Statement> {
    let start = i;
    let (i, name) = space_delimited(identifier)(i)?;
    let (i, _) = space_delimited(char('='))(i)?;
    let (i, expr) = space_delimited(expr)(i)?;
    Ok((
        i,
        Statement::VarAssign {
            span: calc_offset(start, i),
            name,
            expr,
        },
    ))
}

fn var_def_statement(i: Span) -> IResult<Span, Statement> {
    let start = i;
    let (i, _) = delimited(multispace0, tag("var"), multispace1)(i)?;
    let (i, name) = space_delimited(identifier)(i)?;
    let (i, _) = space_delimited(char(':'))(i)?;
    let (i, td) = type_decl(i)?;
    let (i, _) = space_delimited(char('='))(i)?;
    let (i, expr) = space_delimited(expr)(i)?;
    Ok((
        i,
        Statement::VarDef {
            span: calc_offset(start, i),
            name,
            td,
            expr,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_argument() {
        use nom::Slice;
        let span = Span::new("a: i64");
        assert_eq!(
            argument(span),
            Ok((span.slice(span.len()..), (span.slice(0..1), TypeDecl::I64)))
        );
    }

    #[test]
    fn test_break_statement() {
        use nom::Slice;
        let span = Span::new("break");
        assert_eq!(
            break_statement(span),
            Ok((span.slice(span.len()..), Statement::Break))
        );
    }

    #[test]
    fn test_coerce_f64() {
        assert_eq!(coerce_f64(&Value::F64(123.456)), 123.456);
        assert_eq!(coerce_f64(&Value::I64(123)), 123.0);
        // Value::Str => panic
    }

    #[test]
    fn test_coerce_i64() {
        assert_eq!(coerce_i64(&Value::F64(123.456)), 123);
        assert_eq!(coerce_i64(&Value::I64(123)), 123);
        // Value::Str => panic
    }

    #[test]
    fn test_coerce_str() {
        assert_eq!(coerce_str(&Value::F64(123.456)), "123.456");
        assert_eq!(coerce_str(&Value::I64(123)), "123");
        assert_eq!(coerce_str(&Value::Str("abc".to_owned())), "abc");
    }

    #[test]
    fn test_cond_expr() {
        use nom::Slice;
        let span = Span::new("1 < 2");
        assert_eq!(
            cond_expr(span),
            Ok((
                span.slice(span.len()..),
                Expression::new(
                    ExprEnum::Lt(
                        Box::new(Expression::new(ExprEnum::NumLiteral(1.0), span.slice(0..1))),
                        Box::new(Expression::new(ExprEnum::NumLiteral(2.0), span.slice(4..5)))
                    ),
                    span.slice(0..5)
                )
            ))
        );

        let span = Span::new("1 > 2");
        assert_eq!(
            cond_expr(span),
            Ok((
                span.slice(span.len()..),
                Expression::new(
                    ExprEnum::Gt(
                        Box::new(Expression::new(ExprEnum::NumLiteral(1.0), span.slice(0..1))),
                        Box::new(Expression::new(ExprEnum::NumLiteral(2.0), span.slice(4..5)))
                    ),
                    span.slice(0..5)
                )
            ))
        );

        let span = Span::new(" 1 < 2 ");
        assert_eq!(
            cond_expr(span),
            Ok((
                span.slice(span.len()..),
                Expression::new(
                    ExprEnum::Lt(
                        Box::new(Expression::new(ExprEnum::NumLiteral(1.0), span.slice(1..2))),
                        Box::new(Expression::new(ExprEnum::NumLiteral(2.0), span.slice(5..6)))
                    ),
                    // TODO: span.slice(1..6)
                    span.slice(0..7)
                )
            ))
        );
    }

    #[test]
    fn test_continue_statement() {
        use nom::Slice;
        let span = Span::new("continue");
        assert_eq!(
            continue_statement(span),
            Ok((span.slice(span.len()..), Statement::Continue))
        );
    }

    #[test]
    fn test_expr() {
        use nom::Slice;
        // TODO: if_expr
        // TODO: cond_expr
        let span = Span::new("hello");
        assert_eq!(
            expr(span),
            Ok((
                span.slice(span.len()..),
                Expression::new(ExprEnum::Ident(span), span)
            ))
        );

        let span = Span::new("123");
        assert_eq!(
            expr(span),
            Ok((
                span.slice(span.len()..),
                Expression::new(ExprEnum::NumLiteral(123.0), span)
            ))
        );

        let span = Span::new("1+2");
        assert_eq!(
            expr(span),
            Ok((
                span.slice(span.len()..),
                Expression::new(
                    ExprEnum::Add(
                        Box::new(Expression::new(ExprEnum::NumLiteral(1.0), span.slice(0..1))),
                        Box::new(Expression::new(ExprEnum::NumLiteral(2.0), span.slice(2..3)))
                    ),
                    span
                )
            ))
        );

        let span = Span::new("1+2+3");
        assert_eq!(
            expr(span),
            Ok((
                span.slice(span.len()..),
                Expression::new(
                    ExprEnum::Add(
                        Box::new(Expression::new(
                            ExprEnum::Add(
                                Box::new(Expression::new(
                                    ExprEnum::NumLiteral(1.0),
                                    span.slice(0..1)
                                )),
                                Box::new(Expression::new(
                                    ExprEnum::NumLiteral(2.0),
                                    span.slice(2..3)
                                ))
                            ),
                            span.slice(0..3),
                        )),
                        Box::new(Expression::new(ExprEnum::NumLiteral(3.0), span.slice(4..5)))
                    ),
                    span
                )
            ))
        );
    }

    // TODO: expr_statement
    // TODO: eval
    // TODO: eval_stmts
    // TODO: factor

    #[test]
    fn test_fn_def_statement() {
        use nom::Slice;
        let span = Span::new("fn f(a: i64, b: f64) -> str { \"abc\"; }");
        assert_eq!(
            fn_def_statement(span),
            Ok((
                span.slice(span.len()..),
                Statement::FnDef {
                    name: span.slice(3..4),
                    args: vec![
                        (span.slice(5..6), TypeDecl::I64),
                        (span.slice(13..14), TypeDecl::F64)
                    ],
                    ret_type: TypeDecl::Str,
                    stmts: vec![Statement::Expression(Expression::new(
                        ExprEnum::StrLiteral("abc".to_owned(),),
                        span.slice(30..35)
                    ))]
                }
            ))
        );
    }

    // TODO: for_statement
    // TODO: func_call
    // TODO: general_statement
    // TODO: identifier

    #[test]
    fn test_ident() {
        use nom::Slice;
        let span = Span::new("Adam");
        assert_eq!(
            ident(span),
            Ok((
                span.slice(span.len()..),
                Expression::new(ExprEnum::Ident(span), span)
            ))
        );

        let span = Span::new("abc");
        assert_eq!(
            ident(span),
            Ok((
                span.slice(span.len()..),
                Expression::new(ExprEnum::Ident(span), span)
            ))
        );

        let span = Span::new("123abc");
        assert!(ident(span).is_err());

        let span = Span::new(" abc ");
        assert_eq!(
            ident(span),
            Ok((
                span.slice(span.len()..),
                Expression::new(ExprEnum::Ident(span.slice(1..4)), span.slice(1..4))
            ))
        );
    }

    #[test]
    fn test_if_expr() {
        use nom::Slice;
        let span = Span::new("if 123 { 456; }");
        assert_eq!(
            if_expr(span),
            Ok((
                span.slice(span.len()..),
                Expression::new(
                    ExprEnum::If(
                        Box::new(Expression::new(
                            ExprEnum::NumLiteral(123.0),
                            span.slice(3..6)
                        )),
                        Box::new(vec![Statement::Expression(Expression::new(
                            ExprEnum::NumLiteral(456.0),
                            span.slice(9..12)
                        ))]),
                        None,
                    ),
                    span
                )
            ))
        );

        let span = Span::new("if 123 { 456; } else { 789; }");
        assert_eq!(
            if_expr(span),
            Ok((
                span.slice(span.len()..),
                Expression::new(
                    ExprEnum::If(
                        Box::new(Expression::new(
                            ExprEnum::NumLiteral(123.0),
                            span.slice(3..6)
                        )),
                        Box::new(vec![Statement::Expression(Expression::new(
                            ExprEnum::NumLiteral(456.0),
                            span.slice(9..12)
                        ))]),
                        Some(Box::new(vec![Statement::Expression(Expression::new(
                            ExprEnum::NumLiteral(789.0),
                            span.slice(23..26)
                        ))]))
                    ),
                    span
                )
            ))
        );
    }

    #[test]
    fn test_num() {
        use nom::Slice;
        let span = Span::new("123");
        assert_eq!(
            num(span),
            Ok((
                span.slice(span.len()..),
                Expression::new(ExprEnum::NumLiteral(123.0), span)
            ))
        );

        let span = Span::new("123.456");
        assert_eq!(
            num(span),
            Ok((
                span.slice(span.len()..),
                Expression::new(ExprEnum::NumLiteral(123.456), span)
            ))
        );

        let span = Span::new("+123.456");
        assert_eq!(
            num(span),
            Ok((
                span.slice(span.len()..),
                Expression::new(ExprEnum::NumLiteral(123.456), span)
            ))
        );

        let span = Span::new("-123.456");
        assert_eq!(
            num(span),
            Ok((
                span.slice(span.len()..),
                Expression::new(ExprEnum::NumLiteral(-123.456), span)
            ))
        );

        let span = Span::new(".0");
        assert_eq!(
            num(span),
            Ok((
                span.slice(span.len()..),
                Expression::new(ExprEnum::NumLiteral(0.0), span)
            ))
        );

        let span = Span::new(" 123.456 ");
        assert_eq!(
            num(span),
            Ok((
                span.slice(span.len()..),
                Expression::new(ExprEnum::NumLiteral(123.456), span.slice(1..8))
            ))
        );
    }

    #[test]
    fn test_return_statement() {
        use nom::Slice;
        let span = Span::new("return 123");
        assert_eq!(
            return_statement(span),
            Ok((
                span.slice(span.len()..),
                Statement::Return(Expression::new(
                    ExprEnum::NumLiteral(123.0),
                    span.slice(7..10)
                ))
            ))
        );
    }

    #[test]
    fn test_str_literal() {
        use nom::Slice;

        let span = Span::new("\"abc\n\\\"");
        assert_eq!(
            str_literal(span),
            Ok((
                span.slice(span.len()..),
                Expression::new(ExprEnum::StrLiteral("abc\n\\".to_owned()), span)
            ))
        );

        let span = Span::new(" \"abc\" ");
        assert_eq!(
            str_literal(span),
            Ok((
                span.slice(span.len()..),
                Expression::new(ExprEnum::StrLiteral("abc".to_owned()), span.slice(1..6))
            ))
        );
    }

    #[test]
    fn test_statements_finish() {
        use nom::Slice;
        let span = Span::new("123; 456;");
        assert_eq!(
            statements_finish(span),
            Ok(vec![
                Statement::Expression(Expression::new(
                    ExprEnum::NumLiteral(123.0),
                    span.slice(0..3)
                )),
                Statement::Expression(Expression::new(
                    ExprEnum::NumLiteral(456.0),
                    span.slice(5..8)
                ))
            ])
        );

        let span = Span::new("fn add(a: i64, b: i64) -> i64 { a + b; } add(1, 2);");
        assert_eq!(
            statements_finish(span),
            Ok(vec![
                Statement::FnDef {
                    name: span.slice(3..6),
                    args: vec![
                        (span.slice(7..8), TypeDecl::I64),
                        (span.slice(15..16), TypeDecl::I64)
                    ],
                    ret_type: TypeDecl::I64,
                    stmts: vec![Statement::Expression(Expression::new(
                        ExprEnum::Add(
                            Box::new(Expression::new(
                                ExprEnum::Ident(span.slice(32..33)),
                                span.slice(32..33)
                            )),
                            Box::new(Expression::new(
                                ExprEnum::Ident(span.slice(36..37)),
                                span.slice(36..37)
                            ))
                        ),
                        span.slice(32..37)
                    ))],
                },
                Statement::Expression(Expression::new(
                    ExprEnum::FnInvoke(
                        span.slice(41..44),
                        vec![
                            Expression::new(ExprEnum::NumLiteral(1.0), span.slice(45..46)),
                            Expression::new(ExprEnum::NumLiteral(2.0), span.slice(48..49))
                        ]
                    ),
                    // TODO: 41..51 add(1, 2); => 41..50 add(1, 2)
                    span.slice(41..51)
                ))
            ])
        );

        let span = Span::new("if 1 { 123; } else { 456; };");
        assert_eq!(
            statements_finish(span),
            Ok(vec![Statement::Expression(Expression::new(
                ExprEnum::If(
                    Box::new(Expression::new(ExprEnum::NumLiteral(1.0), span.slice(3..4))),
                    Box::new(vec![Statement::Expression(Expression::new(
                        ExprEnum::NumLiteral(123.0),
                        span.slice(7..10)
                    ))]),
                    Some(Box::new(vec![Statement::Expression(Expression::new(
                        ExprEnum::NumLiteral(456.0),
                        span.slice(21..24)
                    ))]))
                ),
                span.slice(0..27)
            ))])
        );

        let span = Span::new( "fn earlyreturn(a: i64, b: i64) -> i64 { if a < b { return a; }; b; } earlyreturn(1, 2);" );
        assert_eq!(
            statements_finish(span),
            Ok(vec![
                Statement::FnDef {
                    name: span.slice(3..14),
                    args: vec![
                        (span.slice(15..16), TypeDecl::I64),
                        (span.slice(23..24), TypeDecl::I64)
                    ],
                    ret_type: TypeDecl::I64,
                    stmts: vec![
                        Statement::Expression(Expression::new(
                            ExprEnum::If(
                                Box::new(Expression::new(
                                    ExprEnum::Lt(
                                        Box::new(Expression::new(
                                            ExprEnum::Ident(span.slice(43..44)),
                                            span.slice(43..44)
                                        )),
                                        Box::new(Expression::new(
                                            ExprEnum::Ident(span.slice(47..48)),
                                            span.slice(47..48)
                                        ))
                                    ),
                                    // TODO: 43..48
                                    span.slice(43..49)
                                )),
                                Box::new(vec![Statement::Return(Expression::new(
                                    ExprEnum::Ident(span.slice(58..59)),
                                    span.slice(58..59)
                                ))]),
                                None
                            ),
                            span.slice(40..62)
                        )),
                        Statement::Expression(Expression::new(
                            ExprEnum::Ident(span.slice(64..65)),
                            span.slice(64..65)
                        ))
                    ],
                },
                Statement::Expression(Expression::new(
                    ExprEnum::FnInvoke(
                        span.slice(69..80),
                        vec![
                            Expression::new(ExprEnum::NumLiteral(1.0), span.slice(81..82)),
                            Expression::new(ExprEnum::NumLiteral(2.0), span.slice(84..85))
                        ]
                    ),
                    // TODO: 69..86 `;`
                    span.slice(69..87)
                ))
            ])
        );

        let span = Span::new("1 * 2");
        assert_eq!(
            statements_finish(span),
            Ok(vec![Statement::Expression(Expression::new(
                ExprEnum::Mul(
                    Box::new(Expression::new(ExprEnum::NumLiteral(1.0), span.slice(0..1))),
                    Box::new(Expression::new(ExprEnum::NumLiteral(2.0), span.slice(4..5)))
                ),
                span.slice(0..5)
            ))])
        );

        let span = Span::new(
            "for i in 0 to 3 { for j in 0 to 3 { if j > 1 { break; }; print(i * 10 + j); } }",
        );
        assert_eq!(
            statements_finish(span),
            Ok(vec![Statement::For {
                span: span.slice(0..79),
                loop_var: span.slice(4..5),
                start: Expression::new(ExprEnum::NumLiteral(0.0), span.slice(9..10)),
                end: Expression::new(ExprEnum::NumLiteral(3.0), span.slice(14..15)),
                stmts: vec![Statement::For {
                    span: span.slice(18..78),
                    loop_var: span.slice(22..23),
                    start: Expression::new(ExprEnum::NumLiteral(0.0), span.slice(27..28)),
                    end: Expression::new(ExprEnum::NumLiteral(3.0), span.slice(32..33)),
                    stmts: vec![
                        Statement::Expression(Expression::new(
                            ExprEnum::If(
                                Box::new(Expression::new(
                                    ExprEnum::Gt(
                                        Box::new(Expression::new(
                                            ExprEnum::Ident(span.slice(39..40)),
                                            span.slice(39..40)
                                        )),
                                        Box::new(Expression::new(
                                            ExprEnum::NumLiteral(1.0),
                                            span.slice(43..44)
                                        ))
                                    ),
                                    // TODO: 39..44
                                    span.slice(39..45)
                                )),
                                Box::new(vec![Statement::Break]),
                                None
                            ),
                            span.slice(36..55)
                        )),
                        Statement::Expression(Expression::new(
                            ExprEnum::FnInvoke(
                                span.slice(57..62),
                                vec![Expression::new(
                                    ExprEnum::Add(
                                        Box::new(Expression::new(
                                            ExprEnum::Mul(
                                                Box::new(Expression::new(
                                                    ExprEnum::Ident(span.slice(63..64)),
                                                    span.slice(63..64)
                                                )),
                                                Box::new(Expression::new(
                                                    ExprEnum::NumLiteral(10.0),
                                                    span.slice(67..69)
                                                ))
                                            ),
                                            span.slice(63..69)
                                        )),
                                        Box::new(Expression::new(
                                            ExprEnum::Ident(span.slice(72..73)),
                                            span.slice(72..73)
                                        ))
                                    ),
                                    span.slice(63..73)
                                )]
                            ),
                            span.slice(57..79)
                        ))
                    ]
                }],
            }])
        );
    }

    #[test]
    fn test_type_check() {
        fn f<'a>(s: &'a str) -> Result<TypeDecl, TypeCheckError<'a>> {
            type_check(
                &statements_finish(Span::new(s)).expect("valid input"),
                &mut TypeCheckContext::new(),
            )
        }
        assert!(f("fn add(a: i64, b: i64) -> i64 { return a + b; }").is_ok());
        assert!(f("fn add(a: i64, b: str) -> i64 { return a + b; }").is_err());
    }

    #[test]
    fn test_var_def_statement() {
        use nom::Slice;
        let span = Span::new("var i : i64 = 123");
        assert_eq!(
            var_def_statement(span),
            Ok((
                span.slice(span.len()..),
                Statement::VarDef {
                    span,
                    name: span.slice(4..5),
                    td: TypeDecl::I64,
                    expr: Expression::new(ExprEnum::NumLiteral(123.0), span.slice(14..17))
                }
            ))
        );

        let span = Span::new("var f:f64=456.7");
        assert_eq!(
            var_def_statement(span),
            Ok((
                span.slice(span.len()..),
                Statement::VarDef {
                    span,
                    name: span.slice(4..5),
                    td: TypeDecl::F64,
                    expr: Expression::new(ExprEnum::NumLiteral(456.7), span.slice(10..15))
                }
            ))
        );

        let span = Span::new("var s: str = \"abc\"");
        assert_eq!(
            var_def_statement(span),
            Ok((
                span.slice(span.len()..),
                Statement::VarDef {
                    span,
                    name: span.slice(4..5),
                    td: TypeDecl::Str,
                    expr: Expression::new(
                        ExprEnum::StrLiteral("abc".to_owned()),
                        span.slice(13..18)
                    )
                }
            ))
        );
    }

    #[test]
    fn test_nom_located_span() {
        use nom::{InputTake, Offset, Slice};
        let span = Span::new("abc def");
        assert_eq!(
            format!("{:?}", span),
            r#"LocatedSpan { offset: 0, line: 1, fragment: "abc def", extra: () }"#
        );
        assert_eq!(
            format!("{:?}", span.take(1)),
            r#"LocatedSpan { offset: 0, line: 1, fragment: "a", extra: () }"#
        );
        assert_eq!(
            format!("{:?}", span.slice(1..3)),
            r#"LocatedSpan { offset: 1, line: 1, fragment: "bc", extra: () }"#
        );
        assert_eq!(
            format!("{:?}", span.slice(1..3).take(1)),
            r#"LocatedSpan { offset: 1, line: 1, fragment: "b", extra: () }"#
        );
        let start = span.slice(2..); // c def
        let end = span.slice(4..); // def
        assert_eq!(start.offset(&end), 2_usize);
        assert_eq!(start.take(2), span.slice(2..4));
        // calc_offset = start.take(start.offset(end))
        assert_eq!(
            format!("{:?}", calc_offset(start, end)),
            r#"LocatedSpan { offset: 2, line: 1, fragment: "c ", extra: () }"#
        );
    }
}
