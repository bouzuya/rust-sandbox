use code_point::CodePoint;
use std::{convert::TryFrom, env, error::Error, io};

fn run(char_or_code_point: &str, f: &mut dyn io::Write) -> io::Result<()> {
    let chars = char_or_code_point.chars().collect::<Vec<char>>();
    if chars.len() == 1 {
        let c = chars[0];
        let code_point = CodePoint::from(c);
        writeln!(f, "{}", code_point)
    } else {
        let code_point_string = char_or_code_point;
        let code_point = CodePoint::try_from(code_point_string).expect("code-point is not valid");
        let c = char::from(code_point);
        writeln!(f, "{}", c)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let char_or_code_point = env::args().nth(1).expect("char or code-point");
    run(&char_or_code_point, &mut io::stdout())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_to_code_point() {
        let mut b = vec![];
        run("A", &mut b).unwrap();
        assert_eq!(b, b"U+0041\n");
    }

    #[test]
    fn test_code_point_to_char() {
        let mut b = vec![];
        run("U+0041", &mut b).unwrap();
        assert_eq!(b, b"A\n");
    }
}
