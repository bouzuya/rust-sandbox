use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

use anyhow::Context as _;

#[derive(Debug)]
struct Vm {
    stack: Vec<Value>,
    vars: HashMap<String, Value>,
    blocks: Vec<Vec<Value>>,
}

impl Vm {
    fn new() -> Self {
        Self {
            stack: vec![],
            vars: HashMap::new(),
            blocks: vec![],
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Value {
    Num(i32),
    Op(String),
    Sym(String),
    Block(Vec<Value>),
}

impl Value {
    fn as_block(&self) -> anyhow::Result<&Vec<Value>> {
        match self {
            Value::Block(val) => Ok(val),
            _ => anyhow::bail!("Value is not a block"),
        }
    }

    fn as_num(&self) -> anyhow::Result<i32> {
        match self {
            Value::Num(num) => Ok(*num),
            _ => anyhow::bail!("Value is not a number"),
        }
    }

    fn as_sym(&self) -> anyhow::Result<&str> {
        match self {
            Value::Sym(sym) => Ok(sym.as_str()),
            _ => anyhow::bail!("Value is not a symbol"),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Num(num) => num.to_string(),
            Value::Op(op) => op.clone(),
            Value::Sym(sym) => sym.clone(),
            Value::Block(block) => {
                let mut s = String::from("{");
                for val in block {
                    s.push_str(&val.to_string());
                    s.push(' ');
                }
                s.push('}');
                s
            }
        }
        .fmt(f)
    }
}

fn main() -> anyhow::Result<()> {
    if let Some(f) = std::env::args()
        .nth(1)
        .and_then(|f| std::fs::File::open(f).ok())
    {
        parse_batch(BufReader::new(f))?;
    } else {
        parse_interactive()?;
    }
    Ok(())
}

fn parse_batch(source: impl BufRead) -> anyhow::Result<Vec<Value>> {
    let mut vm = Vm::new();
    for line in source.lines().map_while(Result::ok) {
        for word in line.split(' ') {
            parse_word(word, &mut vm)?;
        }
    }
    Ok(vm.stack)
}

fn parse_interactive() -> anyhow::Result<()> {
    let mut vm = Vm::new();
    for line in std::io::stdin().lines().map_while(Result::ok) {
        for word in line.split(' ') {
            parse_word(word, &mut vm)?;
        }
        println!("stack: {:#?}", vm.stack);
    }
    Ok(())
}

fn parse_word(word: &str, vm: &mut Vm) -> anyhow::Result<()> {
    if word.is_empty() {
        return Ok(());
    }
    if word == "{" {
        vm.blocks.push(vec![]);
    } else if word == "}" {
        let block = vm.blocks.pop().context("block is none")?;
        vm.stack.push(Value::Block(block));
    } else {
        let value = match word.parse::<i32>() {
            Ok(parsed) => Value::Num(parsed),
            Err(_) => {
                if let Some(word) = word.strip_prefix('/') {
                    Value::Sym(word.to_owned())
                } else {
                    Value::Op(word.to_owned())
                }
            }
        };
        eval(value, vm)?;
    }

    Ok(())
}

fn eval(code: Value, vm: &mut Vm) -> anyhow::Result<()> {
    if let Some(block) = vm.blocks.last_mut() {
        block.push(code);
        return Ok(());
    }
    match code {
        Value::Op(op) => match op.as_str() {
            "+" => add(&mut vm.stack)?,
            "-" => sub(&mut vm.stack)?,
            "*" => mul(&mut vm.stack)?,
            "/" => div(&mut vm.stack)?,
            "<" => lt(vm)?,
            "def" => op_def(vm)?,
            "if" => op_if(vm)?,
            "puts" => puts(vm)?,
            op => {
                let val = vm.vars.get(op).context("{:?} is not a defined operation")?;
                vm.stack.push(val.clone());
            }
        },
        _ => vm.stack.push(code.clone()),
    }
    Ok(())
}

macro_rules! impl_op {
    {$name:ident,$op:tt} => {
        fn $name(stack: &mut Vec<Value>) -> anyhow::Result<()> {
             let rhs = stack.pop().context("rhs is none")?.as_num()?;
             let lhs = stack.pop().context("lhs is none")?.as_num()?;
             stack.push(Value::Num(lhs $op rhs));
             Ok(())
        }
    }
}

impl_op!(add, +);
impl_op!(sub, -);
impl_op!(mul, *);
impl_op!(div, /);

fn lt(vm: &mut Vm) -> anyhow::Result<()> {
    let rhs = vm.stack.pop().context("lhs is none")?.as_num()?;
    let lhs = vm.stack.pop().context("rhs is none")?.as_num()?;
    vm.stack.push(Value::Num(if lhs < rhs { 1 } else { 0 }));
    Ok(())
}

fn op_def(vm: &mut Vm) -> anyhow::Result<()> {
    let val = vm.stack.pop().context("val is none")?;
    let sym = vm.stack.pop().context("sym is none")?;
    let sym = sym.as_sym()?;
    vm.vars.insert(sym.to_owned(), val);
    Ok(())
}

fn op_if(vm: &mut Vm) -> anyhow::Result<()> {
    let false_branch = vm
        .stack
        .pop()
        .context("false_branch is none")?
        .as_block()?
        .clone();
    let true_branch = vm
        .stack
        .pop()
        .context("true_branch is none")?
        .as_block()?
        .clone();
    let cond = vm.stack.pop().context("cond is none")?.as_block()?.clone();
    for code in cond {
        eval(code, vm)?;
    }

    let cond_result = vm.stack.pop().context("cond_result is none")?.as_num()?;
    if cond_result != 0 {
        for code in true_branch {
            eval(code, vm)?;
        }
    } else {
        for code in false_branch {
            eval(code, vm)?;
        }
    }
    Ok(())
}

fn puts(vm: &mut Vm) -> anyhow::Result<()> {
    let val = vm.stack.pop().context("val is none")?;
    println!("{}", val);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_def() -> anyhow::Result<()> {
        use Value::*;
        assert_eq!(
            parse_batch(Cursor::new("/x 10 def /y 20 def { x y < } { x } { y } if"))?,
            vec![Num(10)]
        );
        Ok(())
    }

    #[test]
    fn test_group() -> anyhow::Result<()> {
        use Value::*;
        assert_eq!(
            parse_batch(Cursor::new("1 2 + { 3 4 }"))?,
            vec![Num(3), Block(vec![Num(3), Num(4)])]
        );
        Ok(())
    }

    #[test]
    fn test_if_false() -> anyhow::Result<()> {
        use Value::*;
        assert_eq!(
            parse_batch(Cursor::new("{ 1 -1 + } { 100 } { -100 } if"))?,
            vec![Num(-100)]
        );
        Ok(())
    }

    #[test]
    fn test_if_true() -> anyhow::Result<()> {
        use Value::*;
        assert_eq!(
            parse_batch(Cursor::new("{ 1 1 + } { 100 } { -100 } if"))?,
            vec![Num(100)]
        );
        Ok(())
    }

    #[test]
    fn test_split_first() {
        assert_eq!(
            ["1", "2", "+"].split_first(),
            Some((&"1", ["2", "+"].as_slice()))
        );
        assert_eq!([""].split_first(), Some((&"", [].as_slice())));
    }

    #[test]
    fn test_sub() -> anyhow::Result<()> {
        use Value::*;
        assert_eq!(parse_batch(Cursor::new("1 2 -"))?, vec![Num(-1)]);
        Ok(())
    }
}
