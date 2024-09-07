use anyhow::Context as _;

fn main() -> anyhow::Result<()> {
    let mut stack = vec![];

    stack.push(42);
    stack.push(36);

    add(&mut stack)?;

    stack.push(22);

    add(&mut stack)?;

    println!("stack {:#?}", stack);
    Ok(())
}

fn add(stack: &mut Vec<i32>) -> anyhow::Result<()> {
    let lhs = stack.pop().context("lhs is none")?;
    let rhs = stack.pop().context("rhs is none")?;
    stack.push(lhs + rhs);
    Ok(())
}
