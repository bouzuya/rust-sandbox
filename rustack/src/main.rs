use std::{
    collections::BTreeMap,
    fmt::Debug,
    io::{BufRead, BufReader},
};

use anyhow::Context as _;

type NativeOpFn = fn(&mut Vm) -> anyhow::Result<()>;

#[derive(Debug)]
struct Vm {
    stack: Vec<Value>,
    vars: BTreeMap<String, Value>,
    blocks: Vec<Vec<Value>>,
}

impl Vm {
    fn new() -> Self {
        let fns: [(&str, NativeOpFn); 10] = [
            ("+", add),
            ("-", sub),
            ("*", mul),
            ("/", div),
            ("<", lt),
            ("def", op_def),
            ("dup", dup),
            ("exch", exch),
            ("if", op_if),
            ("puts", puts),
        ];
        Self {
            stack: vec![],
            vars: fns
                .into_iter()
                .map(|(k, v)| (k.to_owned(), Value::Native(NativeOp(v))))
                .collect::<BTreeMap<String, Value>>(),
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
    Native(NativeOp),
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
        std::fmt::Display::fmt(
            &match self {
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
                Value::Native(_) => "<NativeOp>".to_owned(),
            },
            f,
        )
    }
}

#[derive(Clone)]
struct NativeOp(NativeOpFn);

impl PartialEq for NativeOp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for NativeOp {}

impl std::fmt::Debug for NativeOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<NativeOp>")
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
        let value = Value::Block(block);
        if let Some(block) = vm.blocks.last_mut() {
            block.push(value);
        } else {
            vm.stack.push(value);
        }
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
        Value::Op(op) => {
            let val = vm
                .vars
                .get(op.as_str())
                .context("{:?} is not a defined operation")?;
            match val {
                Value::Block(block) => {
                    for code in block.clone() {
                        eval(code.clone(), vm)?;
                    }
                }
                Value::Native(native) => (native.0)(vm)?,
                _ => vm.stack.push(val.clone()),
            }
        }
        _ => vm.stack.push(code.clone()),
    }
    Ok(())
}

macro_rules! impl_op {
    {$name:ident,$op:tt} => {
        fn $name(vm: &mut Vm) -> anyhow::Result<()> {
             let rhs = vm.stack.pop().context("rhs is none")?.as_num()?;
             let lhs = vm.stack.pop().context("lhs is none")?.as_num()?;
             vm.stack.push(Value::Num(lhs $op rhs));
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

fn dup(vm: &mut Vm) -> anyhow::Result<()> {
    let val = vm.stack.last().context("val is none")?.clone();
    vm.stack.push(val);
    Ok(())
}

fn exch(vm: &mut Vm) -> anyhow::Result<()> {
    let val1 = vm.stack.pop().context("val1 is none")?;
    let val2 = vm.stack.pop().context("val2 is none")?;
    vm.stack.push(val1);
    vm.stack.push(val2);
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
    fn test_double() -> anyhow::Result<()> {
        use Value::*;
        assert_eq!(
            parse_batch(Cursor::new("/double { 2 * } def 10 double"))?,
            vec![Num(20)]
        );
        Ok(())
    }

    #[test]
    fn test_dup() -> anyhow::Result<()> {
        use Value::*;
        assert_eq!(parse_batch(Cursor::new("1 dup"))?, vec![Num(1), Num(1)]);
        Ok(())
    }

    #[test]
    fn test_exch() -> anyhow::Result<()> {
        use Value::*;
        assert_eq!(parse_batch(Cursor::new("1 2 exch"))?, vec![Num(2), Num(1)]);
        Ok(())
    }

    #[test]
    fn test_factorial() -> anyhow::Result<()> {
        use Value::*;
        assert_eq!(
            parse_batch(Cursor::new(
                r#"
/factorial { 1 factorial_int } def

/factorial_int {
    /acc exch def
    /n exch def
    { n 2 < }
    { acc }
    {
      n 1 -
      acc n *
      factorial_int
    }
    if
} def

10 factorial"#
            ))?,
            vec![Num(3628800)]
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
    fn test_square() -> anyhow::Result<()> {
        use Value::*;
        assert_eq!(
            parse_batch(Cursor::new("/square { dup * } def 10 square"))?,
            vec![Num(100)]
        );
        Ok(())
    }

    #[test]
    fn test_sub() -> anyhow::Result<()> {
        use Value::*;
        assert_eq!(parse_batch(Cursor::new("1 2 -"))?, vec![Num(-1)]);
        Ok(())
    }

    #[test]
    fn test_vec2sqlen() -> anyhow::Result<()> {
        use Value::*;
        assert_eq!(
            parse_batch(Cursor::new(
                "/square { dup * } def /vec2sqlen { square exch square exch + } def 1 2 vec2sqlen"
            ))?,
            vec![Num(5)]
        );
        Ok(())
    }
}
