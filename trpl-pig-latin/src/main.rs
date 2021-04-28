use std::{env, error::Error, io};

fn to_pig_latin(word: &str) -> String {
    assert!(!word.is_empty());
    let mut chars = word.chars().collect::<Vec<char>>();
    chars.push('-');
    let first_char = *chars.first().unwrap();
    if "aiueo".chars().any(|c| c == first_char) {
        chars.push('h');
    } else {
        chars.rotate_left(1);
    }
    chars.push('a');
    chars.push('y');
    chars.iter().collect()
}

fn print_pig_latin(message: &str, w: &mut impl io::Write) -> io::Result<()> {
    for word in message.split(' ') {
        if word.is_empty() {
            continue;
        }
        writeln!(w, "{}", to_pig_latin(&word))?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let message = env::args().nth(1).expect("Usage: trpl-pig-latin <message>");
    print_pig_latin(&message, &mut io::stdout())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_pig_latin() {
        assert_eq!(to_pig_latin("first"), "irst-fay".to_owned());
        assert_eq!(to_pig_latin("apple"), "apple-hay".to_owned());
    }

    #[test]
    fn test_print_pig_latin() {
        let mut b = vec![];
        assert!(print_pig_latin("first apple", &mut b).is_ok());
        assert_eq!(b, b"irst-fay\napple-hay\n");
    }
}
