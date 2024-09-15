fn main() {
    println!("Hello, world!");
}

fn ident(input: &str) -> &str {
    let mut chars = input.chars();
    if let Some('a'..='z' | 'A'..='Z') = chars.clone().next() {
        chars.next();
        while matches!(
            chars.clone().next(),
            Some('a'..='z' | 'A'..='Z' | '0'..='9')
        ) {
            chars.next();
        }
    }
    chars.as_str()
}

fn number(input: &str) -> &str {
    let mut chars = input.chars();
    if let Some('-' | '+' | '.' | '0'..='9') = chars.clone().next() {
        chars.next();
        while matches!(chars.clone().next(), Some('.' | '0'..='9')) {
            chars.next();
        }
    }
    chars.as_str()
}

fn whitespace(input: &str) -> &str {
    let mut chars = input.chars();
    while matches!(chars.clone().next(), Some(' ')) {
        chars.next();
    }
    chars.as_str()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ident() {
        assert_eq!(ident("Adam"), "");
        assert_eq!(ident("abc"), "");
        assert_eq!(ident("123abc"), "123abc");
        assert_eq!(ident("abc123"), "");
        assert_eq!(ident("abc123 "), " ");
    }

    #[test]
    fn test_number() {
        assert_eq!(number("123.45 "), " ");
        assert_eq!(number("123"), "");
        assert_eq!(number("+123.4"), "");
        assert_eq!(number("-456.7"), "");
        assert_eq!(number(".0"), "");
        assert_eq!(number("..0"), ""); // OK ?????
        assert_eq!(number("123.456.789"), ""); // OK ?????
        assert_eq!(number("+123.4abc "), "abc ");
    }

    #[test]
    fn test_whitespace() {
        assert_eq!(whitespace("   "), "");
        assert_eq!(whitespace(" abc "), "abc ");
    }
}
