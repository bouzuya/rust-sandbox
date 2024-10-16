use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, multispace0, multispace1, none_of},
    combinator::{cut, map_res, opt, recognize},
    multi::{fold_many0, many0, separated_list0},
    number::complete::recognize_float,
    sequence::{delimited, pair, preceded, terminated},
    IResult,
};
use nom_locate::LocatedSpan;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    match args[1].as_str() {
        "w" => {
            let writer = std::fs::File::create("bytecode.bin").unwrap();
            let mut writer = std::io::BufWriter::new(writer);
            write_program(
                args[2].as_str(),
                &std::fs::read_to_string(args[2].as_str()).unwrap(),
                &mut writer,
                true,
            )
            .unwrap()
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
struct FnByteCode {
    args: Vec<String>,
    literals: Vec<Value>,
    instructions: Vec<Instruction>,
}

impl FnByteCode {
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
    I64,
    F64,
    Str,
}

#[derive(Clone, Debug)]
enum Value {
    I64(i64),
    F64(f64),
    Str(String),
}

impl Value {
    fn coerce_i64(&self) -> i64 {
        match self {
            Self::I64(v) => *v,
            _ => panic!("Coercion failed: {:?} cannot be coerced to i64", self),
        }
    }

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
            Value::I64(_) => ValueKind::I64,
            Value::F64(_) => ValueKind::F64,
            Value::Str(_) => ValueKind::Str,
        }
    }

    fn serialize(&self, writer: &mut impl std::io::Write) -> std::io::Result<()> {
        let kind = self.kind() as u8;
        writer.write_all(&[kind])?;
        match self {
            Value::I64(v) => {
                writer.write_all(&v.to_le_bytes())?;
            }
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

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::I64(v) => write!(f, "{}", v),
            Value::F64(v) => write!(f, "{}", v),
            Value::Str(v) => write!(f, "{}", v),
        }
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
    funcs: BTreeMap<String, FnByteCode>,
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
                    let res = self.interpret(&name, args)?;
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
                OpCode::Ret => {
                    return Ok(stack
                        .get(stack.len() - instruction.arg0 as usize - 1)
                        .ok_or_else(|| "Stack underflow".to_owned())?
                        .clone());
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
            funcs.insert(name, FnByteCode::deserialize(reader)?);
        }
        self.funcs = funcs;
        Ok(())
    }
}

fn unary_fn<'a>(f: fn(f64) -> f64) -> FnDecl<'a> {
    FnDecl::Native(NativeFn {
        args: vec![("lhs", TypeDecl::F64), ("rhs", TypeDecl::F64)],
        ret_type: TypeDecl::F64,
        code: Box::new(move |args| {
            Value::F64(f(args
                .into_iter()
                .next()
                .expect("function missing argument")
                .coerce_f64()))
        }),
    })
}

fn binary_fn<'a>(f: fn(f64, f64) -> f64) -> FnDecl<'a> {
    FnDecl::Native(NativeFn {
        args: vec![("lhs", TypeDecl::F64), ("rhs", TypeDecl::F64)],
        ret_type: TypeDecl::F64,
        code: Box::new(move |args| {
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
        }),
    })
}

fn print_fn(args: &[Value]) -> Value {
    for arg in args {
        print!("{:?} ", arg);
    }
    println!();
    Value::F64(0.0)
}

fn dbg_fn(values: &[Value]) -> Value {
    println!("dbg: {:?}", values[0]);
    Value::I64(0)
}

fn puts_fn(args: &[Value]) -> Value {
    for arg in args {
        print!("{}", arg);
    }
    Value::F64(0.)
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
    funcs: BTreeMap<String, FnByteCode>,
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

    fn add_fn(&mut self, name: String, args: &[(Span, TypeDecl)]) {
        self.funcs.insert(
            name,
            FnByteCode {
                args: args.iter().map(|(arg, _)| arg.to_string()).collect(),
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
        match &ex.expr {
            ExprEnum::NumLiteral(n) => {
                let id = self.add_literal(Value::F64(*n));
                self.add_load_literal_inst(id);
                Ok(self.stack_top())
            }
            ExprEnum::StrLiteral(s) => {
                let id = self.add_literal(Value::Str(s.clone()));
                self.add_load_literal_inst(id);
                Ok(self.stack_top())
            }
            ExprEnum::Ident(ident) => self
                .target_stack
                .iter()
                .enumerate()
                .find(|(_, target)| {
                    if let Target::Local(id) = target {
                        id == ident.fragment()
                    } else {
                        false
                    }
                })
                .map(|(index, _)| Ok(StkIdx(index)))
                .unwrap_or_else(|| Err(format!("Variable not found: {}", ident).into())),
            ExprEnum::FnInvoke(name, args) => {
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
            ExprEnum::Add(lhs, rhs) => self.bin_op(OpCode::Add, lhs, rhs),
            ExprEnum::Sub(lhs, rhs) => self.bin_op(OpCode::Sub, lhs, rhs),
            ExprEnum::Mul(lhs, rhs) => self.bin_op(OpCode::Mul, lhs, rhs),
            ExprEnum::Div(lhs, rhs) => self.bin_op(OpCode::Div, lhs, rhs),
            ExprEnum::Gt(lhs, rhs) => self.bin_op(OpCode::Lt, rhs, lhs),
            ExprEnum::Lt(lhs, rhs) => self.bin_op(OpCode::Lt, lhs, rhs),
            ExprEnum::If(cond, true_branch, false_branch) => {
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
                Statement::VarDef { name, expr, .. } => {
                    let expr = self.compile_expr(expr)?;
                    let expr = if matches!(self.target_stack[expr.0], Target::Local(_)) {
                        self.add_copy_inst(expr);
                        self.stack_top()
                    } else {
                        expr
                    };
                    self.target_stack[expr.0] = Target::Local(name.to_string());
                }
                Statement::VarAssign { name, expr, .. } => {
                    let expr = self.compile_expr(expr)?;
                    let (id, _) = self
                        .target_stack
                        .iter()
                        .enumerate()
                        .find(|(_, target)| {
                            if let Target::Local(id) = target {
                                id == name.fragment()
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
                    ..
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
                Statement::FnDef {
                    name, args, stmts, ..
                } => {
                    let literals = std::mem::take(&mut self.literals);
                    let instructions = std::mem::take(&mut self.instructions);
                    let target_stack = std::mem::take(&mut self.target_stack);

                    self.target_stack = args
                        .iter()
                        .map(|arg| Target::Local(arg.0.to_string()))
                        .collect::<Vec<Target>>();

                    self.compile_stmts(stmts)?;

                    self.add_fn(name.to_string(), args);
                    self.literals = literals;
                    self.instructions = instructions;
                    self.target_stack = target_stack;
                }
                Statement::Return(expr) => {
                    let res = self.compile_expr(expr)?;
                    self.add_inst(OpCode::Ret, (self.target_stack.len() - res.0 - 1) as u8);
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
                Ret => {
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
    Ret,
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
        const RET: u8 = OpCode::Ret as u8;
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
            RET => OpCode::Ret,
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
    source_file: &str,
    source: &str,
    writer: &mut impl std::io::Write,
    disasm: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut compiler = Compiler::new();

    let stmts = statements_finish(Span::new(source)).map_err(|e| {
        format!(
            "{}:{}:{}: {}",
            source_file,
            e.input.location_line(),
            e.input.get_utf8_column(),
            e
        )
    })?;

    match type_check(&stmts, &mut TypeCheckContext::new()) {
        Ok(_) => println!("Typecheck Ok"),
        Err(e) => {
            return Err(format!(
                "{}:{}:{}: {}",
                source_file,
                e.span.location_line(),
                e.span.get_utf8_column(),
                e
            )
            .into());
        }
    }

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

enum FnDef {
    User(FnByteCode),
    Native(NativeFn<'static>),
}

type Functions<'a> = BTreeMap<String, FnDecl<'a>>;

fn standard_functions<'a>() -> Functions<'a> {
    let mut funcs = Functions::new();
    funcs.insert("sqrt".to_string(), unary_fn(f64::sqrt));
    funcs.insert("sin".to_string(), unary_fn(f64::sin));
    funcs.insert("cos".to_string(), unary_fn(f64::cos));
    funcs.insert("tan".to_string(), unary_fn(f64::tan));
    funcs.insert("asin".to_string(), unary_fn(f64::asin));
    funcs.insert("acos".to_string(), unary_fn(f64::acos));
    funcs.insert("atan".to_string(), unary_fn(f64::atan));
    funcs.insert("atan2".to_string(), binary_fn(f64::atan2));
    funcs.insert("pow".to_string(), binary_fn(f64::powf));
    funcs.insert("exp".to_string(), unary_fn(f64::exp));
    funcs.insert("log".to_string(), binary_fn(f64::log));
    funcs.insert("log10".to_string(), unary_fn(f64::log10));
    funcs.insert(
        "print".to_string(),
        FnDecl::Native(NativeFn {
            args: vec![("arg", TypeDecl::Any)],
            ret_type: TypeDecl::Any,
            code: Box::new(print_fn),
        }),
    );
    funcs.insert(
        "dbg".to_string(),
        FnDecl::Native(NativeFn {
            args: vec![("arg", TypeDecl::Any)],
            ret_type: TypeDecl::Any,
            code: Box::new(dbg_fn),
        }),
    );
    funcs.insert(
        "puts".to_string(),
        FnDecl::Native(NativeFn {
            args: vec![("arg", TypeDecl::Any)],
            ret_type: TypeDecl::Any,
            code: Box::new(puts_fn),
        }),
    );
    funcs.insert(
        "i64".to_string(),
        FnDecl::Native(NativeFn {
            args: vec![("arg", TypeDecl::Any)],
            ret_type: TypeDecl::I64,
            code: Box::new(move |args| {
                Value::I64(
                    args.first()
                        .expect("function missing argument")
                        .coerce_i64(),
                )
            }),
        }),
    );
    funcs.insert(
        "f64".to_string(),
        FnDecl::Native(NativeFn {
            args: vec![("arg", TypeDecl::Any)],
            ret_type: TypeDecl::F64,
            code: Box::new(move |args| {
                Value::F64(
                    args.first()
                        .expect("function missing argument")
                        .coerce_f64(),
                )
            }),
        }),
    );
    funcs.insert(
        "str".to_string(),
        FnDecl::Native(NativeFn {
            args: vec![("arg", TypeDecl::Any)],
            ret_type: TypeDecl::Str,
            code: Box::new(move |args| {
                Value::Str(
                    args.first()
                        .expect("function missing argument")
                        .coerce_str(),
                )
            }),
        }),
    );
    funcs
}

type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TypeDecl {
    Any,
    F64,
    I64,
    Str,
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
        (F64 | I64, F64) => F64,
        (F64, I64) => F64,
        (I64, I64) => I64,
        (Str, Str) => Str,
        _ => {
            return Err(TypeCheckError::new(
                format!("{:?} cannot be assigned to {:?}", value, target),
                span,
            ))
        }
    })
}

pub struct TypeCheckContext<'a, 'b> {
    vars: BTreeMap<&'a str, TypeDecl>,
    funcs: BTreeMap<String, FnDecl<'a>>,
    super_context: Option<&'b TypeCheckContext<'a, 'b>>,
}

impl<'a, 'b> TypeCheckContext<'a, 'b> {
    pub fn new() -> Self {
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

    fn get_fn(&self, name: &str) -> Option<&FnDecl<'a>> {
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
            "{}\nlocation: {}:{}: {}",
            self.msg,
            self.span.location_line(),
            self.span.get_utf8_column(),
            self.span.fragment()
        )
    }
}

impl<'a> std::error::Error for TypeCheckError<'a> {}

impl<'a> TypeCheckError<'a> {
    fn new(msg: String, span: Span<'a>) -> Self {
        Self { msg, span }
    }
}

fn tc_binary_op<'a>(
    lhs: &Expression<'a>,
    rhs: &Expression<'a>,
    ctx: &mut TypeCheckContext<'a, '_>,
    op: &str,
) -> Result<TypeDecl, TypeCheckError<'a>> {
    let lhst = tc_expr(lhs, ctx)?;
    let rhst = tc_expr(rhs, ctx)?;
    binary_op_type(&lhst, &rhst).map_err(|_| {
        TypeCheckError::new(
            format!(
                "Operation {op} between incompatible type: {:?} and {:?}",
                lhst, rhst,
            ),
            lhs.span,
        )
    })
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

fn tc_binary_cmp<'a>(
    lhs: &Expression<'a>,
    rhs: &Expression<'a>,
    ctx: &mut TypeCheckContext<'a, '_>,
    op: &str,
) -> Result<TypeDecl, TypeCheckError<'a>> {
    use TypeDecl::*;
    let lhst = tc_expr(lhs, ctx)?;
    let rhst = tc_expr(rhs, ctx)?;
    Ok(match (&lhst, &rhst) {
        (Any, _) => I64,
        (_, Any) => I64,
        (F64, F64) => I64,
        (I64, I64) => I64,
        (Str, Str) => I64,
        _ => {
            return Err(TypeCheckError::new(
                format!(
                    "Operation {op} between incompatible type: {:?} and {:?}",
                    lhst, rhst,
                ),
                lhs.span,
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
        NumLiteral(_val) => TypeDecl::F64,
        StrLiteral(_val) => TypeDecl::Str,
        Ident(str) => ctx.get_var(str).ok_or_else(|| {
            TypeCheckError::new(format!("Variable {:?} not found in scope", str), e.span)
        })?,
        FnInvoke(str, args) => {
            let args_ty = args
                .iter()
                .map(|v| Ok((tc_expr(v, ctx)?, v.span)))
                .collect::<Result<Vec<_>, _>>()?;
            let func = ctx.get_fn(**str).ok_or_else(|| {
                TypeCheckError::new(format!("function {} is not defined", str), *str)
            })?;
            let args_decl = func.args();
            for ((arg_ty, arg_span), decl) in args_ty.iter().zip(args_decl.iter()) {
                tc_coerce_type(&arg_ty, &decl.1, *arg_span)?;
            }
            func.ret_type()
        }
        Add(lhs, rhs) => tc_binary_op(&lhs, &rhs, ctx, "Add")?,
        Sub(lhs, rhs) => tc_binary_op(&lhs, &rhs, ctx, "Sub")?,
        Mul(lhs, rhs) => tc_binary_op(&lhs, &rhs, ctx, "Mult")?,
        Div(lhs, rhs) => tc_binary_op(&lhs, &rhs, ctx, "Div")?,
        Lt(lhs, rhs) => tc_binary_cmp(&lhs, &rhs, ctx, "LT")?,
        Gt(lhs, rhs) => tc_binary_cmp(&lhs, &rhs, ctx, "GT")?,
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
                            "Conditional expression doesn't have the \
            compatible types in true and false branch: \
            {:?} and {:?}",
                            true_type, false_type
                        ),
                        calc_offset(true_span, false_span),
                    )
                })?
            } else {
                true_type
            }
        }
    })
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
                let init_type = tc_expr(expr, ctx)?;
                let target = ctx.vars.get(**name).expect("Variable not found");
                tc_coerce_type(&init_type, target, expr.span)?;
            }
            Statement::FnDef {
                name,
                args,
                ret_type,
                stmts,
            } => {
                // Function declaration needs to be added first to allow recursive calls
                ctx.funcs.insert(
                    name.to_string(),
                    FnDecl::User(UserFn {
                        args: args.clone(),
                        ret_type: *ret_type,
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
            Statement::Break => {
                // TODO: check types in break out site. For now we disallow break with values like Rust.
            }
            Statement::Continue => (),
        }
    }
    Ok(res)
}

enum FnDecl<'a> {
    User(UserFn<'a>),
    Native(NativeFn<'a>),
}

impl<'a> FnDecl<'a> {
    fn args(&self) -> Vec<(&'a str, TypeDecl)> {
        match self {
            Self::User(user) => user
                .args
                .iter()
                .map(|(name, ty)| (*name.fragment(), *ty))
                .collect(),
            Self::Native(code) => code.args.clone(),
        }
    }

    fn ret_type(&self) -> TypeDecl {
        match self {
            Self::User(user) => user.ret_type,
            Self::Native(native) => native.ret_type,
        }
    }
}

struct UserFn<'a> {
    args: Vec<(Span<'a>, TypeDecl)>,
    ret_type: TypeDecl,
}

struct NativeFn<'a> {
    args: Vec<(&'a str, TypeDecl)>,
    ret_type: TypeDecl,
    code: Box<dyn Fn(&[Value]) -> Value>,
}

#[derive(Debug, PartialEq, Clone)]
enum ExprEnum<'a> {
    Ident(Span<'a>),
    NumLiteral(f64),
    StrLiteral(String),
    FnInvoke(Span<'a>, Vec<Expression<'a>>),
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

#[derive(Debug, PartialEq, Clone)]
struct Expression<'a> {
    pub(crate) expr: ExprEnum<'a>,
    pub(crate) span: Span<'a>,
}

impl<'a> Expression<'a> {
    fn new(expr: ExprEnum<'a>, span: Span<'a>) -> Self {
        Self { expr, span }
    }
}

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
    Break,
    Continue,
    FnDef {
        name: Span<'a>,
        args: Vec<(Span<'a>, TypeDecl)>,
        ret_type: TypeDecl,
        stmts: Statements<'a>,
    },
    Return(Expression<'a>),
}

impl<'a> Statement<'a> {
    fn span(&self) -> Option<Span<'a>> {
        use Statement::*;
        Some(match self {
            Expression(ex) => ex.span,
            VarDef { span, .. } => *span,
            VarAssign { span, .. } => *span,
            For { span, .. } => *span,
            FnDef { name, stmts, .. } => calc_offset(*name, stmts.span()),
            Return(ex) => ex.span,
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
        self.iter().find_map(|stmt| stmt.span()).unwrap()
    }
}

fn space_delimited<'a, O, E>(
    f: impl nom::Parser<Span<'a>, O, E>,
) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, O, E>
where
    E: nom::error::ParseError<Span<'a>>,
{
    delimited(multispace0, f, multispace0)
}

fn calc_offset<'a>(i: Span<'a>, r: Span<'a>) -> Span<'a> {
    use nom::{InputTake, Offset};
    i.take(i.offset(&r))
}

fn factor(i: Span) -> IResult<Span, Expression> {
    alt((str_literal, num_literal, func_call, ident, parens))(i)
}

fn func_call(i: Span) -> IResult<Span, Expression> {
    let (r, ident) = space_delimited(identifier)(i)?;
    let (r, args) = space_delimited(delimited(
        tag("("),
        many0(delimited(multispace0, expr, space_delimited(opt(tag(","))))),
        tag(")"),
    ))(r)?;
    Ok((
        r,
        Expression {
            expr: ExprEnum::FnInvoke(ident, args),
            span: i,
        },
    ))
}

fn ident(i: Span) -> IResult<Span, Expression> {
    let (r, res) = space_delimited(identifier)(i)?;
    Ok((
        r,
        Expression {
            expr: ExprEnum::Ident(res),
            span: i,
        },
    ))
}

fn identifier(i: Span) -> IResult<Span, Span> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    ))(i)
}

fn str_literal(i: Span) -> IResult<Span, Expression> {
    let (r0, _) = preceded(multispace0, char('\"'))(i)?;
    let (r, val) = many0(none_of("\""))(r0)?;
    let (r, _) = terminated(char('"'), multispace0)(r)?;
    Ok((
        r,
        Expression::new(
            ExprEnum::StrLiteral(
                val.iter()
                    .collect::<String>()
                    .replace("\\\\", "\\")
                    .replace("\\n", "\n"),
            ),
            i,
        ),
    ))
}

fn num_literal(i: Span) -> IResult<Span, Expression> {
    let (r, v) = space_delimited(recognize_float)(i)?;
    Ok((
        r,
        Expression::new(
            ExprEnum::NumLiteral(v.parse().map_err(|_| {
                nom::Err::Error(nom::error::Error {
                    input: i,
                    code: nom::error::ErrorKind::Digit,
                })
            })?),
            v,
        ),
    ))
}

fn parens(i: Span) -> IResult<Span, Expression> {
    space_delimited(delimited(tag("("), expr, tag(")")))(i)
}

fn term(i: Span) -> IResult<Span, Expression> {
    let (r, init) = factor(i)?;

    let res = fold_many0(
        pair(space_delimited(alt((char('*'), char('/')))), factor),
        move || init.clone(),
        |acc, (op, val): (char, Expression)| {
            let span = calc_offset(i, acc.span);
            match op {
                '*' => Expression::new(ExprEnum::Mul(Box::new(acc), Box::new(val)), span),
                '/' => Expression::new(ExprEnum::Div(Box::new(acc), Box::new(val)), span),
                _ => panic!(
                    "Multiplicative expression should have '*' \
            or '/' operator"
                ),
            }
        },
    )(r);
    res
}

fn num_expr(i: Span) -> IResult<Span, Expression> {
    let (r, init) = term(i)?;

    let res = fold_many0(
        pair(space_delimited(alt((char('+'), char('-')))), term),
        move || init.clone(),
        |acc, (op, val): (char, Expression)| {
            let span = calc_offset(i, acc.span);
            match op {
                '+' => Expression::new(ExprEnum::Add(Box::new(acc), Box::new(val)), span),
                '-' => Expression::new(ExprEnum::Sub(Box::new(acc), Box::new(val)), span),
                _ => panic!("Additive expression should have '+' or '-' operator"),
            }
        },
    )(r);
    res
}

fn cond_expr(i0: Span) -> IResult<Span, Expression> {
    let (i, first) = num_expr(i0)?;
    let (i, cond) = space_delimited(alt((char('<'), char('>'))))(i)?;
    let (i, second) = num_expr(i)?;
    let span = calc_offset(i0, i);
    Ok((
        i,
        match cond {
            '<' => Expression::new(ExprEnum::Lt(Box::new(first), Box::new(second)), span),
            '>' => Expression::new(ExprEnum::Gt(Box::new(first), Box::new(second)), span),
            _ => unreachable!(),
        },
    ))
}

fn open_brace(i: Span) -> IResult<Span, ()> {
    let (i, _) = space_delimited(char('{'))(i)?;
    Ok((i, ()))
}

fn close_brace(i: Span) -> IResult<Span, ()> {
    let (i, _) = space_delimited(char('}'))(i)?;
    Ok((i, ()))
}

fn if_expr(i0: Span) -> IResult<Span, Expression> {
    let (i, _) = space_delimited(tag("if"))(i0)?;
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
        Expression::new(
            ExprEnum::If(Box::new(cond), Box::new(t_case), f_case.map(Box::new)),
            calc_offset(i0, i),
        ),
    ))
}

fn expr(i: Span) -> IResult<Span, Expression> {
    alt((if_expr, cond_expr, num_expr))(i)
}

fn var_def(i: Span) -> IResult<Span, Statement> {
    let span = i;
    let (i, _) = delimited(multispace0, tag("var"), multispace1)(i)?;
    let (i, (name, td, expr)) = cut(|i| {
        let (i, name) = space_delimited(identifier)(i)?;
        let (i, _) = space_delimited(char(':'))(i)?;
        let (i, td) = type_decl(i)?;
        let (i, _) = space_delimited(char('='))(i)?;
        let (i, ex) = space_delimited(expr)(i)?;
        let (i, _) = space_delimited(char(';'))(i)?;
        Ok((i, (name, td, ex)))
    })(i)?;
    Ok((
        i,
        Statement::VarDef {
            span: calc_offset(span, i),
            name,
            td,
            expr,
        },
    ))
}

fn var_assign(i: Span) -> IResult<Span, Statement> {
    let span = i;
    let (i, name) = space_delimited(identifier)(i)?;
    let (i, _) = space_delimited(char('='))(i)?;
    let (i, expr) = space_delimited(expr)(i)?;
    let (i, _) = space_delimited(char(';'))(i)?;
    Ok((
        i,
        Statement::VarAssign {
            span: calc_offset(span, i),
            name,
            expr,
        },
    ))
}

fn expr_statement(i: Span) -> IResult<Span, Statement> {
    let (i, res) = expr(i)?;
    Ok((i, Statement::Expression(res)))
}

fn for_statement(i: Span) -> IResult<Span, Statement> {
    let i0 = i;
    let (i, _) = space_delimited(tag("for"))(i)?;
    let (i, (loop_var, start, end, stmts)) = cut(|i| {
        let (i, loop_var) = space_delimited(identifier)(i)?;
        let (i, _) = space_delimited(tag("in"))(i)?;
        let (i, start) = space_delimited(expr)(i)?;
        let (i, _) = space_delimited(tag("to"))(i)?;
        let (i, end) = space_delimited(expr)(i)?;
        let (i, stmts) = delimited(open_brace, statements, close_brace)(i)?;
        Ok((i, (loop_var, start, end, stmts)))
    })(i)?;
    Ok((
        i,
        Statement::For {
            span: calc_offset(i0, i),
            loop_var,
            start,
            end,
            stmts,
        },
    ))
}

fn type_decl(i: Span) -> IResult<Span, TypeDecl> {
    let (i, td) = space_delimited(identifier)(i)?;
    Ok((
        i,
        match *td.fragment() {
            "i64" => TypeDecl::I64,
            "f64" => TypeDecl::F64,
            "str" => TypeDecl::Str,
            _ => {
                return Err(nom::Err::Failure(nom::error::Error::new(
                    td,
                    nom::error::ErrorKind::Verify,
                )));
            }
        },
    ))
}

fn argument(i: Span) -> IResult<Span, (Span, TypeDecl)> {
    let (i, ident) = space_delimited(identifier)(i)?;
    let (i, _) = char(':')(i)?;
    let (i, td) = type_decl(i)?;

    Ok((i, (ident, td)))
}

fn fn_def_statement(i: Span) -> IResult<Span, Statement> {
    let (i, _) = space_delimited(tag("fn"))(i)?;
    let (i, (name, args, ret_type, stmts)) = cut(|i| {
        let (i, name) = space_delimited(identifier)(i)?;
        let (i, _) = space_delimited(tag("("))(i)?;
        let (i, args) = separated_list0(char(','), space_delimited(argument))(i)?;
        let (i, _) = space_delimited(tag(")"))(i)?;
        let (i, _) = space_delimited(tag("->"))(i)?;
        let (i, ret_type) = type_decl(i)?;
        let (i, stmts) = delimited(open_brace, statements, close_brace)(i)?;
        Ok((i, (name, args, ret_type, stmts)))
    })(i)?;
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

fn return_statement(i: Span) -> IResult<Span, Statement> {
    let (i, _) = space_delimited(tag("return"))(i)?;
    let (i, ex) = space_delimited(expr)(i)?;
    Ok((i, Statement::Return(ex)))
}

fn break_statement(i: Span) -> IResult<Span, Statement> {
    let (i, _) = space_delimited(tag("break"))(i)?;
    Ok((i, Statement::Break))
}

fn continue_statement(i: Span) -> IResult<Span, Statement> {
    let (i, _) = space_delimited(tag("continue"))(i)?;
    Ok((i, Statement::Continue))
}

fn general_statement<'a>(last: bool) -> impl Fn(Span<'a>) -> IResult<Span<'a>, Statement> {
    let terminator = move |i| -> IResult<Span, ()> {
        let mut semicolon = pair(tag(";"), multispace0);
        if last {
            Ok((opt(semicolon)(i)?.0, ()))
        } else {
            Ok((semicolon(i)?.0, ()))
        }
    };
    move |input| {
        alt((
            var_def,
            var_assign,
            fn_def_statement,
            for_statement,
            terminated(return_statement, terminator),
            terminated(break_statement, terminator),
            terminated(continue_statement, terminator),
            terminated(expr_statement, terminator),
        ))(input)
    }
}

pub(crate) fn last_statement(i: Span) -> IResult<Span, Statement> {
    general_statement(true)(i)
}

pub(crate) fn statement(i: Span) -> IResult<Span, Statement> {
    general_statement(false)(i)
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
    use nom::Finish;
    let (_, res) = statements(i).finish()?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr() {
        assert!(expr(Span::new("1 + 2")).is_ok());
        assert!(expr(Span::new("1 - 2")).is_ok());
        assert!(expr(Span::new("1 * 2 + 3 / 4 - 5")).is_ok());
        assert!(expr(Span::new("if 1 { 2 }")).is_ok());
        assert!(expr(Span::new("if 1 < 2 { 3 } else { 4 }")).is_ok());
        assert!(expr(Span::new("if 1 < 2 { 3 } else if 4 < 5 { 6 }")).is_ok());
    }

    #[test]
    fn test_statements_finish() {
        assert!(statements_finish(Span::new(
            "var a: i64 = 10; for i in 0 to 3 { print(a + i) }"
        ))
        .is_ok());
    }
}
