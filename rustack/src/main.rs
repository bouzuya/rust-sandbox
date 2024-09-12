use std::collections::HashMap;

use anyhow::Context as _;

#[derive(Debug)]
struct Vm<'src> {
    stack: Vec<Value<'src>>,
    vars: HashMap<&'src str, Value<'src>>,
}

impl<'src> Vm<'src> {
    fn new() -> Self {
        Self {
            stack: vec![],
            vars: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Value<'src> {
    Num(i32),
    Op(&'src str),
    Sym(&'src str),
    Block(Vec<Value<'src>>),
}

impl<'src> Value<'src> {
    fn as_block(&self) -> anyhow::Result<&Vec<Value<'src>>> {
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

    fn as_sym(&self) -> anyhow::Result<&'src str> {
        match self {
            Value::Sym(sym) => Ok(sym),
            _ => anyhow::bail!("Value is not a symbol"),
        }
    }
}

fn main() -> anyhow::Result<()> {
    for line in std::io::stdin().lines() {
        let line = line?;
        let stack = parse(&line)?;
        println!("stack {:#?}", stack);
    }

    Ok(())
}

fn parse(line: &str) -> anyhow::Result<Vec<Value>> {
    let mut vm = Vm::new();
    let words = line.split(' ').collect::<Vec<_>>();
    let mut words = &words[..];

    while let Some((&word, mut rest)) = words.split_first() {
        if word.is_empty() {
            break;
        }
        if word == "{" {
            let value;
            (value, rest) = parse_block(rest)?;
            vm.stack.push(value);
        } else {
            let value = match word.parse::<i32>() {
                Ok(parsed) => Value::Num(parsed),
                Err(_) => {
                    if word.starts_with('/') {
                        Value::Sym(&word[1..])
                    } else {
                        Value::Op(word)
                    }
                }
            };
            eval(value, &mut vm)?;
            println!("vm {:#?}", vm);
        }
        words = rest;
    }
    Ok(vm.stack)
}

fn parse_block<'src, 'a>(input: &'a [&'src str]) -> anyhow::Result<(Value<'src>, &'a [&'src str])> {
    let mut tokens = vec![];
    let mut words = input;

    while let Some((&word, mut rest)) = words.split_first() {
        if word.is_empty() {
            break;
        }
        if word == "{" {
            let value;
            (value, rest) = parse_block(rest)?;
            tokens.push(value);
        } else if word == "}" {
            return Ok((Value::Block(tokens), rest));
        } else if let Ok(value) = word.parse::<i32>() {
            tokens.push(Value::Num(value));
        } else if word.starts_with('/') {
            tokens.push(Value::Sym(&word[1..]));
        } else {
            // op or sym
            tokens.push(Value::Op(word));
        }
        words = rest;
    }

    Ok((Value::Block(tokens), words))
}

fn eval<'src>(code: Value<'src>, vm: &mut Vm<'src>) -> anyhow::Result<()> {
    match code {
        Value::Op("+") => add(&mut vm.stack)?,
        Value::Op("-") => sub(&mut vm.stack)?,
        Value::Op("*") => mul(&mut vm.stack)?,
        Value::Op("/") => div(&mut vm.stack)?,
        Value::Op("<") => lt(vm)?,
        Value::Op("def") => op_def(vm)?,
        Value::Op("if") => op_if(vm)?,
        Value::Op(op) => {
            let val = vm.vars.get(op).context("{:?} is not a defined operation")?;
            vm.stack.push(val.clone());
        }
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

fn lt<'src>(vm: &mut Vm<'src>) -> anyhow::Result<()> {
    let rhs = vm.stack.pop().context("lhs is none")?.as_num()?;
    let lhs = vm.stack.pop().context("rhs is none")?.as_num()?;
    vm.stack.push(Value::Num(if lhs < rhs { 1 } else { 0 }));
    Ok(())
}

fn op_def<'src>(vm: &mut Vm<'src>) -> anyhow::Result<()> {
    let val = vm.stack.pop().context("val is none")?;
    let sym = vm.stack.pop().context("sym is none")?.as_sym()?;
    vm.vars.insert(sym, val);
    Ok(())
}

fn op_if<'src>(vm: &mut Vm<'src>) -> anyhow::Result<()> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_def() -> anyhow::Result<()> {
        use Value::*;
        assert_eq!(
            parse("/x 10 def /y 20 def { x y < } { x } { y } if")?,
            vec![Num(10)]
        );
        Ok(())
    }

    #[test]
    fn test_group() -> anyhow::Result<()> {
        use Value::*;
        assert_eq!(
            parse("1 2 + { 3 4 }")?,
            vec![Num(3), Block(vec![Num(3), Num(4)])]
        );
        Ok(())
    }

    #[test]
    fn test_if_false() -> anyhow::Result<()> {
        use Value::*;
        assert_eq!(parse("{ 1 -1 + } { 100 } { -100 } if")?, vec![Num(-100)]);
        Ok(())
    }

    #[test]
    fn test_if_true() -> anyhow::Result<()> {
        use Value::*;
        assert_eq!(parse("{ 1 1 + } { 100 } { -100 } if")?, vec![Num(100)]);
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
        assert_eq!(parse("1 2 -")?, vec![Num(-1)]);
        Ok(())
    }
}
