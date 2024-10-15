// use std::collections::BTreeMap;
// use std::io::Read;
// use std::ops::ControlFlow;

// use nom::branch::alt;
// use nom::bytes::complete::tag;
// use nom::character::complete::{alpha1, alphanumeric1, char, multispace0, multispace1, none_of};
// use nom::combinator::{opt, recognize};
// use nom::error::ParseError;
// use nom::multi::{fold_many0, many0, separated_list0};
// use nom::number::complete::recognize_float;
// use nom::sequence::{delimited, pair, preceded, terminated};
// use nom::Parser;
// use nom::{Finish, IResult};

use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, multispace0, multispace1},
    combinator::{map_res, opt, recognize},
    multi::{fold_many0, many0, separated_list0},
    number::complete::recognize_float,
    sequence::{delimited, pair, preceded, terminated},
    IResult,
};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    match args[1].as_str() {
        "w" => {
            let writer = std::fs::File::create("bytecode.bin").unwrap();
            let mut writer = std::io::BufWriter::new(writer);
            write_program(args[2].as_str(), &mut writer, true).unwrap()
        }
        "r" => {
            let reader = std::fs::File::open("bytecode.bin").unwrap();
            let mut reader = std::io::BufReader::new(reader);
            let byte_code = read_program(&mut reader).unwrap();
            let result = byte_code.interpret("main", &[]);
            println!("result {:?}", result);
        }
        _ => println!("Please specify w or r as an argument"),
    }
}

#[derive(Debug)]
struct FnDef {
    args: Vec<String>,
    literals: Vec<Value>,
    instructions: Vec<Instruction>,
}

impl FnDef {
    fn deserialize(reader: &mut impl std::io::Read) -> std::io::Result<Self> {
        let args = Self::read_args(reader)?;
        let literals = Self::read_literals(reader)?;
        let instructions = Self::read_instructions(reader)?;
        Ok(Self {
            args,
            literals,
            instructions,
        })
    }

    fn read_args(reader: &mut impl std::io::Read) -> std::io::Result<Vec<String>> {
        let num_args = deserialize_size(reader)?;
        let mut args = Vec::with_capacity(num_args);
        for _ in 0..num_args {
            let mut buf = vec![0u8; deserialize_size(reader)?];
            reader.read_exact(&mut buf)?;
            let s = String::from_utf8(buf).unwrap();
            args.push(s);
        }
        Ok(args)
    }

    fn read_literals(reader: &mut impl std::io::Read) -> std::io::Result<Vec<Value>> {
        let num_literals = deserialize_size(reader)?;
        let mut literals = Vec::with_capacity(num_literals);
        for _ in 0..num_literals {
            literals.push(Value::deserialize(reader)?);
        }
        Ok(literals)
    }

    fn read_instructions(reader: &mut impl std::io::Read) -> std::io::Result<Vec<Instruction>> {
        let num_instructions = deserialize_size(reader)?;
        let mut instructions = Vec::with_capacity(num_instructions);
        for _ in 0..num_instructions {
            let instruction = Instruction::deserialize(reader)?;
            instructions.push(instruction);
        }
        Ok(instructions)
    }

    fn serialize(&self, writer: &mut impl std::io::Write) -> std::io::Result<()> {
        Self::write_args(&self.args, writer)?;
        Self::write_literals(&self.literals, writer)?;
        Self::write_insts(&self.instructions, writer)?;
        Ok(())
    }

    fn write_args(args: &[String], writer: &mut impl std::io::Write) -> std::io::Result<()> {
        serialize_size(args.len(), writer)?;
        for arg in args {
            serialize_str(arg, writer)?;
        }
        Ok(())
    }

    fn write_insts(
        instructions: &[Instruction],
        writer: &mut impl std::io::Write,
    ) -> std::io::Result<()> {
        serialize_size(instructions.len(), writer)?;
        for instruction in instructions {
            instruction.serialize(writer)?;
        }
        Ok(())
    }

    fn write_literals(literals: &[Value], writer: &mut impl std::io::Write) -> std::io::Result<()> {
        serialize_size(literals.len(), writer)?;
        for value in literals {
            value.serialize(writer)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct LoopStackUnderflowError;

impl std::fmt::Display for LoopStackUnderflowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A break statement outside loop")
    }
}

impl std::error::Error for LoopStackUnderflowError {}

struct LoopFrame {
    start: StkIdx,
    break_ips: Vec<InstPtr>,
    continue_ips: Vec<(InstPtr, usize)>,
}

impl LoopFrame {
    fn new(start: StkIdx) -> Self {
        Self {
            start,
            break_ips: vec![],
            continue_ips: vec![],
        }
    }
}

#[derive(Clone, Debug, Default)]
enum Target {
    #[default]
    Temp,
    Literal(usize),
    Local(String),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct StkIdx(usize);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct InstPtr(usize);

#[repr(u8)]
enum ValueKind {
    F64,
    Str,
}

#[derive(Clone, Debug)]
enum Value {
    F64(f64),
    Str(String),
}

impl Value {
    fn coerce_f64(&self) -> f64 {
        match self {
            Self::F64(v) => *v,
            _ => panic!("Coercion failed: {:?} cannot be coerced to f64", self),
        }
    }

    fn coerce_str(&self) -> String {
        match self {
            Self::Str(v) => v.clone(),
            _ => panic!("Coercion failed: {:?} cannot be coerced to str", self),
        }
    }

    fn deserialize(reader: &mut impl std::io::Read) -> std::io::Result<Self> {
        const F64: u8 = ValueKind::F64 as u8;
        const STR: u8 = ValueKind::Str as u8;

        let mut kind_buf = [0u8; 1];
        reader.read_exact(&mut kind_buf)?;
        match kind_buf[0] {
            F64 => {
                let mut buf = [0u8; std::mem::size_of::<f64>()];
                reader.read_exact(&mut buf)?;
                Ok(Value::F64(f64::from_le_bytes(buf)))
            }
            STR => Ok(Value::Str(deserialize_str(reader)?)),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "ValueKind {} does not match to any known value",
                    kind_buf[0]
                ),
            )),
        }
    }

    fn kind(&self) -> ValueKind {
        match self {
            Value::F64(_) => ValueKind::F64,
            Value::Str(_) => ValueKind::Str,
        }
    }

    fn serialize(&self, writer: &mut impl std::io::Write) -> std::io::Result<()> {
        let kind = self.kind() as u8;
        writer.write_all(&[kind])?;
        match self {
            Value::F64(v) => {
                writer.write_all(&v.to_le_bytes())?;
            }
            Value::Str(v) => {
                serialize_str(v, writer)?;
            }
        }
        Ok(())
    }
}

fn deserialize_str(reader: &mut impl std::io::Read) -> std::io::Result<String> {
    let size = deserialize_size(reader)?;
    let mut buf = vec![0u8; size];
    reader.read_exact(&mut buf)?;
    let s = String::from_utf8(buf).unwrap();
    Ok(s)
}

fn serialize_str(s: &str, writer: &mut impl std::io::Write) -> std::io::Result<()> {
    serialize_size(s.len(), writer)?;
    writer.write_all(s.as_bytes())?;
    Ok(())
}

#[derive(Debug)]
struct ByteCode {
    funcs: BTreeMap<String, FnDef>,
}

impl ByteCode {
    fn new() -> Self {
        Self {
            funcs: BTreeMap::new(),
        }
    }

    fn interpret(
        &self,
        fn_name: &str,
        args: &[Value],
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let fn_def = self
            .funcs
            .get(fn_name)
            .ok_or_else(|| format!("Function {:?} was not found", fn_name))?;
        let mut stack = args.to_vec();
        let mut ip = 0;

        while ip < fn_def.instructions.len() {
            let instruction = &fn_def.instructions[ip];
            match instruction.op {
                OpCode::LoadLiteral => {
                    stack.push(fn_def.literals[instruction.arg0 as usize].clone())
                }
                OpCode::Store => {
                    let idx = stack.len() - instruction.arg0 as usize - 1;
                    stack[idx] = stack.pop().expect("Store needs an argument");
                }
                OpCode::Dup => {
                    let top = stack.last().unwrap().clone();
                    stack.extend((0..instruction.arg0).map(|_| top.clone()));
                }
                OpCode::Copy => {
                    let idx = stack.len() - 1 - instruction.arg0 as usize;
                    stack.push(stack[idx].clone());
                }
                OpCode::Add => {
                    let rhs = stack.pop().expect("Stack underflow").coerce_f64();
                    let lhs = stack.pop().expect("Stack underflow").coerce_f64();
                    stack.push(Value::F64(lhs + rhs));
                }
                OpCode::Sub => {
                    let rhs = stack.pop().expect("Stack underflow").coerce_f64();
                    let lhs = stack.pop().expect("Stack underflow").coerce_f64();
                    stack.push(Value::F64(lhs - rhs));
                }
                OpCode::Mul => {
                    let rhs = stack.pop().expect("Stack underflow").coerce_f64();
                    let lhs = stack.pop().expect("Stack underflow").coerce_f64();
                    stack.push(Value::F64(lhs * rhs));
                }
                OpCode::Div => {
                    let rhs = stack.pop().expect("Stack underflow").coerce_f64();
                    let lhs = stack.pop().expect("Stack underflow").coerce_f64();
                    stack.push(Value::F64(lhs / rhs));
                }
                OpCode::Call => {
                    let args = &stack[stack.len() - instruction.arg0 as usize..];
                    let name = &stack[stack.len() - instruction.arg0 as usize - 1];
                    let name = name.coerce_str();
                    let res = match name.as_str() {
                        "sqrt" => unary_fn(f64::sqrt)(args),
                        "sin" => unary_fn(f64::sin)(args),
                        "cos" => unary_fn(f64::cos)(args),
                        "tan" => unary_fn(f64::tan)(args),
                        "asin" => unary_fn(f64::asin)(args),
                        "acos" => unary_fn(f64::acos)(args),
                        "atan" => unary_fn(f64::atan)(args),
                        "atan2" => binary_fn(f64::atan2)(args),
                        "pow" => binary_fn(f64::powf)(args),
                        "exp" => unary_fn(f64::exp)(args),
                        "log" => binary_fn(f64::log)(args),
                        "log10" => unary_fn(f64::log10)(args),
                        "print" => print_fn(args),
                        _ => self.interpret(&name, args)?,
                    };
                    stack.resize(stack.len() - instruction.arg0 as usize - 1, Value::F64(0.0));
                    stack.push(res);
                }
                OpCode::Jmp => {
                    ip = instruction.arg0 as usize;
                    continue;
                }
                OpCode::Jf => {
                    let cond = stack.pop().expect("Jf needs an argument");
                    if cond.coerce_f64() == 0.0 {
                        ip = instruction.arg0 as usize;
                        continue;
                    }
                }
                OpCode::Lt => {
                    let rhs = stack.pop().expect("Stack underflow").coerce_f64();
                    let lhs = stack.pop().expect("Stack underflow").coerce_f64();
                    stack.push(Value::F64((lhs < rhs) as i32 as f64));
                }
                OpCode::Pop => {
                    stack.resize(stack.len() - instruction.arg0 as usize, Value::F64(0.0))
                }
            }

            ip += 1;
        }
        Ok(stack.pop().ok_or_else(|| "Stack underflow".to_owned())?)
    }

    fn read_funcs(&mut self, reader: &mut impl std::io::Read) -> std::io::Result<()> {
        let num_funcs = deserialize_size(reader)?;
        let mut funcs = BTreeMap::new();
        for _ in 0..num_funcs {
            let mut buf = vec![0u8; deserialize_size(reader)?];
            reader.read_exact(&mut buf)?;
            let name = String::from_utf8(buf).unwrap();
            funcs.insert(name, FnDef::deserialize(reader)?);
        }
        self.funcs = funcs;
        Ok(())
    }
}

fn unary_fn(f: fn(f64) -> f64) -> impl Fn(&[Value]) -> Value {
    move |args| {
        let arg = args
            .first()
            .expect("function missing argument")
            .coerce_f64();
        Value::F64(f(arg))
    }
}

fn binary_fn(f: fn(f64, f64) -> f64) -> impl Fn(&[Value]) -> Value {
    move |args| {
        let mut args = args.into_iter();
        let lhs = args
            .next()
            .expect("function missing the first argument")
            .coerce_f64();
        let rhs = args
            .next()
            .expect("function missing the second argument")
            .coerce_f64();
        Value::F64(f(lhs, rhs))
    }
}

fn print_fn(args: &[Value]) -> Value {
    for arg in args {
        print!("{:?} ", arg);
    }
    println!();
    Value::F64(0.0)
}

fn deserialize_size(reader: &mut impl std::io::Read) -> std::io::Result<usize> {
    let mut buf = [0u8; std::mem::size_of::<u32>()];
    reader.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf) as usize)
}

struct Compiler {
    literals: Vec<Value>,
    instructions: Vec<Instruction>,
    target_stack: Vec<Target>,
    funcs: BTreeMap<String, FnDef>,
    loop_stack: Vec<LoopFrame>,
}

impl Compiler {
    fn new() -> Self {
        Self {
            literals: vec![],
            instructions: vec![],
            target_stack: vec![],
            funcs: BTreeMap::new(),
            loop_stack: vec![],
        }
    }

    fn add_copy_inst(&mut self, StkIdx(stk_idx): StkIdx) -> InstPtr {
        let inst = self.add_inst(OpCode::Copy, (self.target_stack.len() - stk_idx - 1) as u8);
        self.target_stack.push(Target::Temp);
        inst
    }

    fn add_fn(&mut self, name: String, args: &[&str]) {
        self.funcs.insert(
            name,
            FnDef {
                args: args.iter().map(|arg| arg.to_string()).collect(),
                literals: std::mem::take(&mut self.literals),
                instructions: std::mem::take(&mut self.instructions),
            },
        );
    }

    fn add_inst(&mut self, op: OpCode, arg0: u8) -> InstPtr {
        let addr = self.instructions.len();
        self.instructions.push(Instruction { op, arg0 });
        InstPtr(addr)
    }

    fn add_jf_inst(&mut self) -> InstPtr {
        let inst = self.add_inst(OpCode::Jf, 0);
        self.target_stack.pop();
        inst
    }

    fn add_pop_until_inst(&mut self, StkIdx(stk_idx): StkIdx) -> Option<InstPtr> {
        if self.target_stack.len() <= stk_idx {
            return None;
        }
        let inst = self.add_inst(OpCode::Pop, (self.target_stack.len() - stk_idx - 1) as u8);
        self.target_stack.resize(stk_idx + 1, Target::Temp);
        Some(inst)
    }

    fn add_store_inst(&mut self, StkIdx(stk_idx): StkIdx) -> InstPtr {
        let inst = self.add_inst(OpCode::Store, (self.target_stack.len() - stk_idx - 1) as u8);
        self.target_stack.pop();
        inst
    }

    fn add_literal(&mut self, value: Value) -> u8 {
        let addr = self.literals.len();
        if addr > u8::MAX as usize {
            panic!("Too many literals");
        }
        self.literals.push(value);
        addr as u8
    }

    fn add_load_literal_inst(&mut self, id: u8) -> InstPtr {
        let inst = self.add_inst(OpCode::LoadLiteral, id);
        self.target_stack.push(Target::Literal(id as usize));
        inst
    }

    fn bin_op(
        &mut self,
        op: OpCode,
        lhs: &Expression,
        rhs: &Expression,
    ) -> Result<StkIdx, Box<dyn std::error::Error>> {
        let lhs = self.compile_expr(lhs)?;
        let rhs = self.compile_expr(rhs)?;
        self.add_copy_inst(lhs);
        self.add_copy_inst(rhs);
        self.add_inst(op, 0);
        self.target_stack.pop();
        self.target_stack.pop();
        self.target_stack.push(Target::Temp);
        Ok(self.stack_top())
    }

    fn coerce_stack(&mut self, target: StkIdx) {
        if target.0 < self.target_stack.len() - 1 {
            self.add_store_inst(target);
            self.add_pop_until_inst(target);
        } else if self.target_stack.len() - 1 < target.0 {
            for _ in self.target_stack.len() - 1..target.0 {
                self.add_copy_inst(self.stack_top());
            }
        }
    }

    fn compile(&mut self, stmts: &Statements) -> Result<(), Box<dyn std::error::Error>> {
        let name = "main";
        self.compile_stmts(stmts)?;
        self.add_fn(name.to_string(), &[]);
        Ok(())
    }

    fn compile_expr(&mut self, ex: &Expression) -> Result<StkIdx, Box<dyn std::error::Error>> {
        match ex {
            Expression::NumLiteral(n) => {
                let id = self.add_literal(Value::F64(*n));
                self.add_load_literal_inst(id);
                Ok(self.stack_top())
            }
            Expression::Ident("pi") => {
                let id = self.add_literal(Value::F64(std::f64::consts::PI));
                self.add_load_literal_inst(id);
                Ok(self.stack_top())
            }
            Expression::Ident(ident) => self
                .target_stack
                .iter()
                .enumerate()
                .find(|(_, target)| {
                    if let Target::Local(id) = target {
                        id == ident
                    } else {
                        false
                    }
                })
                .map(|(index, _)| Ok(StkIdx(index)))
                .unwrap_or_else(|| Err(format!("Variable not found: {}", ident).into())),
            Expression::FnInvoke(name, args) => {
                let name = self.add_literal(Value::Str(name.to_string()));
                let args = args
                    .iter()
                    .map(|arg| self.compile_expr(arg))
                    .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?;
                self.add_load_literal_inst(name);
                for arg in &args {
                    self.add_copy_inst(*arg);
                }

                self.add_inst(OpCode::Call, args.len() as u8);
                self.target_stack
                    .resize(self.target_stack.len() - args.len(), Target::Temp);
                Ok(self.stack_top())
            }
            Expression::Add(lhs, rhs) => self.bin_op(OpCode::Add, lhs, rhs),
            Expression::Sub(lhs, rhs) => self.bin_op(OpCode::Sub, lhs, rhs),
            Expression::Mul(lhs, rhs) => self.bin_op(OpCode::Mul, lhs, rhs),
            Expression::Div(lhs, rhs) => self.bin_op(OpCode::Div, lhs, rhs),
            Expression::Gt(lhs, rhs) => self.bin_op(OpCode::Lt, rhs, lhs),
            Expression::Lt(lhs, rhs) => self.bin_op(OpCode::Lt, lhs, rhs),
            Expression::If(cond, true_branch, false_branch) => {
                let cond = self.compile_expr(cond)?;
                self.add_copy_inst(cond);
                let jf_inst = self.add_jf_inst();
                let stack_size_before = self.target_stack.len();
                self.compile_stmts(true_branch)?;
                self.coerce_stack(StkIdx(stack_size_before + 1));
                let jmp_inst = self.add_inst(OpCode::Jmp, 0);
                self.fixup_jmp(jf_inst);
                self.target_stack.resize(stack_size_before, Target::Temp);
                if let Some(false_branch) = false_branch.as_ref() {
                    self.compile_stmts(&false_branch)?;
                }
                self.coerce_stack(StkIdx(stack_size_before + 1));
                self.fixup_jmp(jmp_inst);
                Ok(self.stack_top())
            }
        }
    }

    fn compile_stmts(
        &mut self,
        stmts: &Statements,
    ) -> Result<Option<StkIdx>, Box<dyn std::error::Error>> {
        let mut last_result = None;
        for stmt in stmts {
            match stmt {
                Statement::Expression(expr) => {
                    last_result = Some(self.compile_expr(expr)?);
                }
                Statement::VarDef(name, expr) => {
                    let expr = self.compile_expr(expr)?;
                    let expr = if matches!(self.target_stack[expr.0], Target::Local(_)) {
                        self.add_copy_inst(expr);
                        self.stack_top()
                    } else {
                        expr
                    };
                    self.target_stack[expr.0] = Target::Local(name.to_string());
                }
                Statement::VarAssign(name, expr) => {
                    let expr = self.compile_expr(expr)?;
                    let (id, _) = self
                        .target_stack
                        .iter()
                        .enumerate()
                        .find(|(_, target)| {
                            if let Target::Local(target) = target {
                                target == name
                            } else {
                                false
                            }
                        })
                        .ok_or_else(|| format!("Variable name not found: {}", name))?;
                    self.add_copy_inst(expr);
                    self.add_store_inst(StkIdx(id));
                }
                Statement::For {
                    loop_var,
                    start,
                    end,
                    stmts,
                } => {
                    let stk_start = self.compile_expr(start)?;
                    let stk_end = self.compile_expr(end)?;
                    self.add_copy_inst(stk_start);
                    let stk_loop_var = self.stack_top();
                    self.target_stack[stk_loop_var.0] = Target::Local(loop_var.to_string());
                    let inst_check_exit = self.instructions.len();
                    self.add_copy_inst(stk_loop_var);
                    self.add_copy_inst(stk_end);
                    self.target_stack.pop();
                    self.add_inst(OpCode::Lt, 0);
                    let jf_inst = self.add_jf_inst();
                    self.loop_stack.push(LoopFrame::new(stk_loop_var));
                    self.compile_stmts(stmts)?;
                    self.fixup_continues()?;
                    let one = self.add_literal(Value::F64(1.0));
                    self.add_copy_inst(stk_loop_var);
                    self.add_load_literal_inst(one);
                    self.add_inst(OpCode::Add, 0);
                    self.target_stack.pop();
                    self.add_store_inst(stk_loop_var);
                    self.add_pop_until_inst(stk_loop_var);
                    self.add_inst(OpCode::Jmp, inst_check_exit as u8);
                    self.fixup_jmp(jf_inst);
                    self.fixup_breaks()?;
                }
                Statement::Break => {
                    let start = self
                        .loop_stack
                        .last()
                        .map(|loop_frame| loop_frame.start)
                        .ok_or(LoopStackUnderflowError)?;
                    self.add_pop_until_inst(start);

                    let loop_frame = self.loop_stack.last_mut().ok_or(LoopStackUnderflowError)?;
                    let break_ip = self.instructions.len();
                    loop_frame.break_ips.push(InstPtr(break_ip));
                    self.add_inst(OpCode::Jmp, 0);
                }
                Statement::Continue => {
                    let start = self
                        .loop_stack
                        .last()
                        .map(|frame| frame.start)
                        .ok_or(LoopStackUnderflowError)?;
                    self.add_pop_until_inst(start);

                    let loop_frame = self.loop_stack.last_mut().ok_or(LoopStackUnderflowError)?;
                    let continue_ip = self.instructions.len();
                    loop_frame
                        .continue_ips
                        .push((InstPtr(continue_ip), self.target_stack.len()));
                    self.add_inst(OpCode::Dup, 0);
                    self.add_inst(OpCode::Jmp, 0);
                }
                Statement::FnDef { name, args, stmts } => {
                    let literals = std::mem::take(&mut self.literals);
                    let instructions = std::mem::take(&mut self.instructions);
                    let target_stack = std::mem::take(&mut self.target_stack);

                    self.target_stack = args
                        .iter()
                        .map(|arg| Target::Local(arg.to_string()))
                        .collect::<Vec<Target>>();

                    self.compile_stmts(stmts)?;

                    self.add_fn(name.to_string(), args);
                    self.literals = literals;
                    self.instructions = instructions;
                    self.target_stack = target_stack;
                }
            }
        }
        Ok(last_result)
    }

    fn disasm(&self, writer: &mut impl std::io::Write) -> std::io::Result<()> {
        use OpCode::*;

        writeln!(writer, "Literals [{}]", self.literals.len())?;
        for (i, literal) in self.literals.iter().enumerate() {
            writeln!(writer, " [{}] {:?}", i, literal)?;
        }

        writeln!(writer, "Instructions [{}]", self.instructions.len())?;
        for (index, it) in self.instructions.iter().enumerate() {
            match it.op {
                LoadLiteral => {
                    writeln!(
                        writer,
                        " [{}] {:?} {} ({:?})",
                        index, it.op, it.arg0, self.literals[it.arg0 as usize]
                    )?;
                }
                Store => {
                    writeln!(writer, " [{}] {:?} {}", index, it.op, it.arg0)?;
                }
                Copy => {
                    writeln!(writer, " [{}] {:?} {}", index, it.op, it.arg0)?;
                }
                Dup => {
                    writeln!(writer, " [{}] {:?} {}", index, it.op, it.arg0)?;
                }
                Add => {
                    writeln!(writer, " [{}] {:?}", index, it.op)?;
                }
                Sub => {
                    writeln!(writer, " [{}] {:?}", index, it.op)?;
                }
                Mul => {
                    writeln!(writer, " [{}] {:?}", index, it.op)?;
                }
                Div => {
                    writeln!(writer, " [{}] {:?}", index, it.op)?;
                }
                Call => {
                    writeln!(writer, " [{}] {:?} {}", index, it.op, it.arg0)?;
                }
                Jmp => {
                    writeln!(writer, " [{}] {:?} {}", index, it.op, it.arg0)?;
                }
                Jf => {
                    writeln!(writer, " [{}] {:?} {}", index, it.op, it.arg0)?;
                }
                Lt => {
                    writeln!(writer, " [{}] {:?} {}", index, it.op, it.arg0)?;
                }
                Pop => {
                    writeln!(writer, " [{}] {:?} {}", index, it.op, it.arg0)?;
                }
            }
        }

        Ok(())
    }

    fn fixup_breaks(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let loop_frame = self.loop_stack.pop().ok_or(LoopStackUnderflowError)?;
        let break_jmp_addr = self.instructions.len();
        for ip in loop_frame.break_ips {
            self.instructions[ip.0].arg0 = break_jmp_addr as u8;
        }
        Ok(())
    }

    fn fixup_continues(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let loop_frame = self.loop_stack.last().ok_or(LoopStackUnderflowError)?;
        let continue_jmp_addr = self.instructions.len();
        for (ip, stk) in &loop_frame.continue_ips {
            self.instructions[ip.0].arg0 = (self.target_stack.len() - stk) as u8;
            self.instructions[ip.0 + 1].arg0 = continue_jmp_addr as u8;
        }
        Ok(())
    }

    fn fixup_jmp(&mut self, InstPtr(ip): InstPtr) {
        self.instructions[ip].arg0 = self.instructions.len() as u8;
    }

    fn stack_top(&self) -> StkIdx {
        StkIdx(self.target_stack.len() - 1)
    }

    fn write_funcs(&self, writer: &mut impl std::io::Write) -> std::io::Result<()> {
        serialize_size(self.funcs.len(), writer)?;
        for (name, func) in &self.funcs {
            serialize_str(name, writer)?;
            func.serialize(writer)?;
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
    Store,
    Copy,
    Dup,
    Add,
    Sub,
    Mul,
    Div,
    Call,
    Jmp,
    Jf,
    Lt,
    Pop,
}

impl From<u8> for OpCode {
    fn from(o: u8) -> Self {
        const LOAD_LITERAL: u8 = OpCode::LoadLiteral as u8;
        const STORE: u8 = OpCode::Store as u8;
        const COPY: u8 = OpCode::Copy as u8;
        const DUP: u8 = OpCode::Dup as u8;
        const ADD: u8 = OpCode::Add as u8;
        const SUB: u8 = OpCode::Sub as u8;
        const MUL: u8 = OpCode::Mul as u8;
        const DIV: u8 = OpCode::Div as u8;
        const CALL: u8 = OpCode::Call as u8;
        const JMP: u8 = OpCode::Jmp as u8;
        const JF: u8 = OpCode::Jf as u8;
        const LT: u8 = OpCode::Lt as u8;
        const POP: u8 = OpCode::Pop as u8;
        match o {
            LOAD_LITERAL => OpCode::LoadLiteral,
            STORE => OpCode::Store,
            COPY => OpCode::Copy,
            DUP => OpCode::Dup,
            ADD => OpCode::Add,
            SUB => OpCode::Sub,
            MUL => OpCode::Mul,
            DIV => OpCode::Mul,
            CALL => OpCode::Call,
            JMP => OpCode::Jmp,
            JF => OpCode::Jf,
            LT => OpCode::Lt,
            POP => OpCode::Pop,
            _ => panic!("OpCode \"{:02X}\" unrecognized!", o),
        }
    }
}

#[derive(Debug)]
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

fn read_program(reader: &mut impl std::io::Read) -> std::io::Result<ByteCode> {
    let mut byte_code = ByteCode::new();
    byte_code.read_funcs(reader)?;
    Ok(byte_code)
}

fn write_program(
    source: &str,
    writer: &mut impl std::io::Write,
    disasm: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut compiler = Compiler::new();

    let stmts = statements_finish(source)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    compiler.compile(&stmts)?;

    if disasm {
        compiler.disasm(&mut std::io::stdout())?;
    }

    compiler.write_funcs(writer)?;

    println!(
        "Written {} lietrals and {} instructions",
        compiler.literals.len(),
        compiler.instructions.len()
    );
    Ok(())
}

#[derive(Clone, Debug, PartialEq)]
enum Expression<'a> {
    Ident(&'a str),
    NumLiteral(f64),
    FnInvoke(&'a str, Vec<Expression<'a>>),
    Add(Box<Expression<'a>>, Box<Expression<'a>>),
    Sub(Box<Expression<'a>>, Box<Expression<'a>>),
    Mul(Box<Expression<'a>>, Box<Expression<'a>>),
    Div(Box<Expression<'a>>, Box<Expression<'a>>),
    Gt(Box<Expression<'a>>, Box<Expression<'a>>),
    Lt(Box<Expression<'a>>, Box<Expression<'a>>),
    If(
        Box<Expression<'a>>,
        Box<Statements<'a>>,
        Option<Box<Statements<'a>>>,
    ),
}

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
    Break,
    Continue,
    FnDef {
        name: &'a str,
        args: Vec<&'a str>,
        stmts: Statements<'a>,
    },
    // Return(Expression<'a>),
}

type Statements<'a> = Vec<Statement<'a>>;

fn space_delimited<'src, O, E>(
    f: impl nom::Parser<&'src str, O, E>,
) -> impl FnMut(&'src str) -> IResult<&'src str, O, E>
where
    E: nom::error::ParseError<&'src str>,
{
    delimited(multispace0, f, multispace0)
}

fn factor(i: &str) -> IResult<&str, Expression> {
    alt((number, func_call, ident, parens))(i)
}

fn func_call(i: &str) -> IResult<&str, Expression> {
    let (r, ident) = space_delimited(identifier)(i)?;
    let (r, args) = space_delimited(delimited(
        tag("("),
        many0(delimited(multispace0, expr, space_delimited(opt(tag(","))))),
        tag(")"),
    ))(r)?;
    Ok((r, Expression::FnInvoke(ident, args)))
}

fn ident(input: &str) -> IResult<&str, Expression> {
    let (r, res) = space_delimited(identifier)(input)?;
    Ok((r, Expression::Ident(res)))
}

fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    ))(input)
}

fn number(input: &str) -> IResult<&str, Expression> {
    let (r, v) = space_delimited(recognize_float)(input)?;
    Ok((
        r,
        Expression::NumLiteral(v.parse().map_err(|_| {
            nom::Err::Error(nom::error::Error {
                input,
                code: nom::error::ErrorKind::Digit,
            })
        })?),
    ))
}

fn parens(i: &str) -> IResult<&str, Expression> {
    space_delimited(delimited(tag("("), expr, tag(")")))(i)
}

fn term(i: &str) -> IResult<&str, Expression> {
    let (i, init) = factor(i)?;

    fold_many0(
        pair(space_delimited(alt((char('*'), char('/')))), factor),
        move || init.clone(),
        |acc, (op, val): (char, Expression)| match op {
            '*' => Expression::Mul(Box::new(acc), Box::new(val)),
            '/' => Expression::Div(Box::new(acc), Box::new(val)),
            _ => panic!("Multiplicative expression should have '*' or '/' operator"),
        },
    )(i)
}

fn num_expr(i: &str) -> IResult<&str, Expression> {
    let (i, init) = term(i)?;

    fold_many0(
        pair(space_delimited(alt((char('+'), char('-')))), term),
        move || init.clone(),
        |acc, (op, val): (char, Expression)| match op {
            '+' => Expression::Add(Box::new(acc), Box::new(val)),
            '-' => Expression::Sub(Box::new(acc), Box::new(val)),
            _ => {
                panic!("Additive expression should have '+' or '-' operator")
            }
        },
    )(i)
}

fn cond_expr(i: &str) -> IResult<&str, Expression> {
    let (i, first) = num_expr(i)?;
    let (i, cond) = space_delimited(alt((char('<'), char('>'))))(i)?;
    let (i, second) = num_expr(i)?;
    Ok((
        i,
        match cond {
            '<' => Expression::Lt(Box::new(first), Box::new(second)),
            '>' => Expression::Gt(Box::new(first), Box::new(second)),
            _ => unreachable!(),
        },
    ))
}

fn open_brace(i: &str) -> IResult<&str, ()> {
    let (i, _) = space_delimited(char('{'))(i)?;
    Ok((i, ()))
}

fn close_brace(i: &str) -> IResult<&str, ()> {
    let (i, _) = space_delimited(char('}'))(i)?;
    Ok((i, ()))
}

fn if_expr(i: &str) -> IResult<&str, Expression> {
    let (i, _) = space_delimited(tag("if"))(i)?;
    let (i, cond) = expr(i)?;
    let (i, t_case) = delimited(open_brace, statements, close_brace)(i)?;
    let (i, f_case) = opt(preceded(
        space_delimited(tag("else")),
        alt((
            delimited(open_brace, statements, close_brace),
            map_res(
                if_expr,
                |v| -> Result<Vec<Statement>, nom::error::Error<&str>> {
                    Ok(vec![Statement::Expression(v)])
                },
            ),
        )),
    ))(i)?;

    Ok((
        i,
        Expression::If(Box::new(cond), Box::new(t_case), f_case.map(Box::new)),
    ))
}

fn expr(i: &str) -> IResult<&str, Expression> {
    alt((if_expr, cond_expr, num_expr))(i)
}

fn var_def(i: &str) -> IResult<&str, Statement> {
    let (i, _) = delimited(multispace0, tag("var"), multispace1)(i)?;
    let (i, name) = space_delimited(identifier)(i)?;
    let (i, _) = space_delimited(char('='))(i)?;
    let (i, expr) = space_delimited(expr)(i)?;
    Ok((i, Statement::VarDef(name, expr)))
}

fn var_assign(i: &str) -> IResult<&str, Statement> {
    let (i, name) = space_delimited(identifier)(i)?;
    let (i, _) = space_delimited(char('='))(i)?;
    let (i, expr) = space_delimited(expr)(i)?;
    Ok((i, Statement::VarAssign(name, expr)))
}

fn expr_statement(i: &str) -> IResult<&str, Statement> {
    let (i, res) = expr(i)?;
    Ok((i, Statement::Expression(res)))
}

fn for_statement(i: &str) -> IResult<&str, Statement> {
    let (i, _) = space_delimited(tag("for"))(i)?;
    let (i, loop_var) = space_delimited(identifier)(i)?;
    let (i, _) = space_delimited(tag("in"))(i)?;
    let (i, start) = space_delimited(expr)(i)?;
    let (i, _) = space_delimited(tag("to"))(i)?;
    let (i, end) = space_delimited(expr)(i)?;
    let (i, stmts) = delimited(open_brace, statements, close_brace)(i)?;
    Ok((
        i,
        Statement::For {
            loop_var,
            start,
            end,
            stmts,
        },
    ))
}

fn fn_def_statement(i: &str) -> IResult<&str, Statement> {
    let (i, _) = space_delimited(tag("fn"))(i)?;
    let (i, name) = space_delimited(identifier)(i)?;
    let (i, _) = space_delimited(tag("("))(i)?;
    let (i, args) = separated_list0(char(','), space_delimited(identifier))(i)?;
    let (i, _) = space_delimited(tag(")"))(i)?;
    let (i, stmts) = delimited(open_brace, statements, close_brace)(i)?;
    Ok((i, Statement::FnDef { name, args, stmts }))
}

fn break_stmt(input: &str) -> IResult<&str, Statement> {
    let (r, _) = space_delimited(tag("break"))(input)?;
    Ok((r, Statement::Break))
}

fn continue_statement(i: &str) -> IResult<&str, Statement> {
    let (i, _) = space_delimited(tag("continue"))(i)?;
    Ok((i, Statement::Continue))
}

fn general_statement<'a>(last: bool) -> impl Fn(&'a str) -> IResult<&'a str, Statement> {
    let terminator = move |i| -> IResult<&str, ()> {
        let mut semicolon = pair(tag(";"), multispace0);
        if last {
            Ok((opt(semicolon)(i)?.0, ()))
        } else {
            Ok((semicolon(i)?.0, ()))
        }
    };
    move |input: &str| {
        alt((
            terminated(var_def, terminator),
            terminated(var_assign, terminator),
            fn_def_statement,
            for_statement,
            terminated(break_stmt, terminator),
            terminated(continue_statement, terminator),
            terminated(expr_statement, terminator),
        ))(input)
    }
}

pub(crate) fn last_statement(input: &str) -> IResult<&str, Statement> {
    general_statement(true)(input)
}

pub(crate) fn statement(input: &str) -> IResult<&str, Statement> {
    general_statement(false)(input)
}

fn statements(i: &str) -> IResult<&str, Statements> {
    let (r, mut v) = many0(statement)(i)?;
    let (r, last) = opt(last_statement)(r)?;
    let (r, _) = opt(multispace0)(r)?;
    if let Some(last) = last {
        v.push(last);
    }
    Ok((r, v))
}

fn statements_finish(i: &str) -> Result<Statements, nom::error::Error<&str>> {
    use nom::Finish;
    let (_, res) = statements(i).finish()?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr() {
        assert_eq!(
            expr("1 + 2"),
            Ok((
                "",
                Expression::Add(
                    Box::new(Expression::NumLiteral(1.0)),
                    Box::new(Expression::NumLiteral(2.0))
                )
            ))
        );
        assert_eq!(
            expr("1 - 2"),
            Ok((
                "",
                Expression::Sub(
                    Box::new(Expression::NumLiteral(1.0)),
                    Box::new(Expression::NumLiteral(2.0))
                )
            ))
        );
        assert_eq!(
            expr("1 * 2 + 3 / 4 - 5"),
            Ok((
                "",
                Expression::Sub(
                    Box::new(Expression::Add(
                        Box::new(Expression::Mul(
                            Box::new(Expression::NumLiteral(1.0)),
                            Box::new(Expression::NumLiteral(2.0))
                        )),
                        Box::new(Expression::Div(
                            Box::new(Expression::NumLiteral(3.0)),
                            Box::new(Expression::NumLiteral(4.0)),
                        ))
                    )),
                    Box::new(Expression::NumLiteral(5.0))
                )
            ))
        );

        // assert_eq!(
        //     expr("if 1 { 2 }"),
        //     Ok((
        //         "",
        //         Expression::If(
        //             Box::new(Expression::NumLiteral(1.0)),
        //             Box::new(Expression::NumLiteral(2.0)),
        //             None
        //         )
        //     ))
        // );
        // assert_eq!(
        //     expr("if 1 < 2 { 3 } else { 4 }"),
        //     Ok((
        //         "",
        //         Expression::If(
        //             Box::new(Expression::Lt(
        //                 Box::new(Expression::NumLiteral(1.0)),
        //                 Box::new(Expression::NumLiteral(2.0)),
        //             )),
        //             Box::new(Expression::NumLiteral(3.0)),
        //             Some(Box::new(Expression::NumLiteral(4.0))),
        //         )
        //     ))
        // );
        // assert_eq!(
        //     expr("if 1 < 2 { 3 } else if 4 < 5 { 6 }"),
        //     Ok((
        //         "",
        //         Expression::If(
        //             Box::new(Expression::Lt(
        //                 Box::new(Expression::NumLiteral(1.0)),
        //                 Box::new(Expression::NumLiteral(2.0)),
        //             )),
        //             Box::new(Expression::NumLiteral(3.0)),
        //             Some(Box::new(Expression::If(
        //                 Box::new(Expression::Lt(
        //                     Box::new(Expression::NumLiteral(4.0)),
        //                     Box::new(Expression::NumLiteral(5.0)),
        //                 )),
        //                 Box::new(Expression::NumLiteral(6.0)),
        //                 None
        //             )))
        //         )
        //     ))
        // );
    }

    #[test]
    fn test_statements_finish() {
        assert_eq!(
            statements_finish("var a = 10; for i in 0 to 3 { print(a + i) }"),
            Ok(vec![
                Statement::VarDef("a", Expression::NumLiteral(10.0)),
                Statement::For {
                    loop_var: "i",
                    start: Expression::NumLiteral(0.0),
                    end: Expression::NumLiteral(3.0),
                    stmts: vec![Statement::Expression(Expression::FnInvoke(
                        "print",
                        vec![Expression::Add(
                            Box::new(Expression::Ident("a")),
                            Box::new(Expression::Ident("i")),
                        )]
                    ))]
                }
            ])
        );
    }
}
