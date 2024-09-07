use anyhow::Context as _;

fn main() -> anyhow::Result<()> {
    let mut stack = vec![];

    for line in std::io::stdin().lines() {
        let line = line?;
        let words = line.split(' ').collect::<Vec<_>>();
        println!("Line: {:#?}", words);

        for word in words {
            match word.parse::<i32>() {
                Ok(parsed) => stack.push(parsed),
                Err(_) => match word {
                    "+" => add(&mut stack)?,
                    "-" => sub(&mut stack)?,
                    "*" => mul(&mut stack)?,
                    "/" => div(&mut stack)?,
                    _ => anyhow::bail!("{:#?} could not be parsed", word),
                },
            }
        }
    }
    println!("stack {:#?}", stack);
    Ok(())
}

fn add(stack: &mut Vec<i32>) -> anyhow::Result<()> {
    let lhs = stack.pop().context("lhs is none")?;
    let rhs = stack.pop().context("rhs is none")?;
    stack.push(lhs + rhs);
    Ok(())
}

fn sub(stack: &mut Vec<i32>) -> anyhow::Result<()> {
    let lhs = stack.pop().context("lhs is none")?;
    let rhs = stack.pop().context("rhs is none")?;
    stack.push(lhs - rhs);
    Ok(())
}

fn mul(stack: &mut Vec<i32>) -> anyhow::Result<()> {
    let lhs = stack.pop().context("lhs is none")?;
    let rhs = stack.pop().context("rhs is none")?;
    stack.push(lhs * rhs);
    Ok(())
}

fn div(stack: &mut Vec<i32>) -> anyhow::Result<()> {
    let lhs = stack.pop().context("lhs is none")?;
    let rhs = stack.pop().context("rhs is none")?;
    stack.push(lhs / rhs);
    Ok(())
}
