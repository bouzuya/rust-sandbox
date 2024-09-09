use anyhow::Context as _;

#[derive(Debug, Eq, PartialEq)]
enum Value<'src> {
    Num(i32),
    Op(&'src str),
    Block(Vec<Value<'src>>),
}

impl<'src> Value<'src> {
    fn as_num(&self) -> anyhow::Result<i32> {
        match self {
            Value::Num(num) => Ok(*num),
            _ => anyhow::bail!("Value is not a number"),
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
    let mut stack = vec![];
    let words = line.split(' ').collect::<Vec<_>>();
    let mut words = &words[..];

    while let Some((&word, mut rest)) = words.split_first() {
        if word.is_empty() {
            break;
        }
        if word == "{" {
            let value;
            (value, rest) = parse_block(rest)?;
            stack.push(value);
        } else {
            match word.parse::<i32>() {
                Ok(parsed) => stack.push(Value::Num(parsed)),
                Err(_) => match word {
                    "+" => add(&mut stack)?,
                    "-" => sub(&mut stack)?,
                    "*" => mul(&mut stack)?,
                    "/" => div(&mut stack)?,
                    _ => anyhow::bail!("{:#?} could not be parsed", word),
                },
            }
        }
        words = rest;
    }
    Ok(stack)
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
        } else {
            let word = match word {
                "+" | "-" | "*" | "/" => word,
                _ => anyhow::bail!("{:#?} could not be parsed", word),
            };
            tokens.push(Value::Op(word));
        }
        words = rest;
    }

    Ok((Value::Block(tokens), words))
}

fn add(stack: &mut Vec<Value>) -> anyhow::Result<()> {
    let lhs = stack.pop().context("lhs is none")?.as_num()?;
    let rhs = stack.pop().context("rhs is none")?.as_num()?;
    stack.push(Value::Num(lhs + rhs));
    Ok(())
}

fn sub(stack: &mut Vec<Value>) -> anyhow::Result<()> {
    let lhs = stack.pop().context("lhs is none")?.as_num()?;
    let rhs = stack.pop().context("rhs is none")?.as_num()?;
    stack.push(Value::Num(lhs - rhs));
    Ok(())
}

fn mul(stack: &mut Vec<Value>) -> anyhow::Result<()> {
    let lhs = stack.pop().context("lhs is none")?.as_num()?;
    let rhs = stack.pop().context("rhs is none")?.as_num()?;
    stack.push(Value::Num(lhs * rhs));
    Ok(())
}

fn div(stack: &mut Vec<Value>) -> anyhow::Result<()> {
    let lhs = stack.pop().context("lhs is none")?.as_num()?;
    let rhs = stack.pop().context("rhs is none")?.as_num()?;
    stack.push(Value::Num(lhs / rhs));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_split_first() {
        assert_eq!(
            ["1", "2", "+"].split_first(),
            Some((&"1", ["2", "+"].as_slice()))
        );
        assert_eq!([""].split_first(), Some((&"", [].as_slice())));
    }
}
