fn main() {
    let input = "123world";
    println!("source: {:?}, parsed: {:?}", input, source(input));
}

#[derive(Debug, Eq, PartialEq)]
enum Token {
    Ident,
    Number,
}

fn ident(input: &str) -> (&str, Option<Token>) {
    let mut chars = input.chars();
    let mut token = None;
    if let Some('a'..='z' | 'A'..='Z') = chars.clone().next() {
        chars.next();
        while matches!(
            chars.clone().next(),
            Some('a'..='z' | 'A'..='Z' | '0'..='9')
        ) {
            chars.next();
        }
        token = Some(Token::Ident);
    }
    (chars.as_str(), token)
}

fn number(input: &str) -> (&str, Option<Token>) {
    let mut chars = input.chars();
    let mut token = None;
    if let Some('-' | '+' | '.' | '0'..='9') = chars.clone().next() {
        chars.next();
        while matches!(chars.clone().next(), Some('.' | '0'..='9')) {
            chars.next();
        }
        token = Some(Token::Number);
    }
    (chars.as_str(), token)
}

fn source(mut input: &str) -> Vec<Token> {
    let mut tokens = vec![];
    while !input.is_empty() {
        input = if let (next_input, Some(token)) = token(input) {
            tokens.push(token);
            next_input
        } else {
            break;
        }
    }
    tokens
}

fn token(input: &str) -> (&str, Option<Token>) {
    if let (input, Some(token)) = ident(whitespace(input)) {
        return (input, Some(token));
    }

    if let (input, Some(token)) = number(whitespace(input)) {
        return (input, Some(token));
    }

    (input, None)
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
        assert_eq!(ident("Adam"), ("", Some(Token::Ident)));
        assert_eq!(ident("abc"), ("", Some(Token::Ident)));
        assert_eq!(ident("123abc"), ("123abc", None));
        assert_eq!(ident("abc123"), ("", Some(Token::Ident)));
        assert_eq!(ident("abc123 "), (" ", Some(Token::Ident)));
    }

    #[test]
    fn test_number() {
        assert_eq!(number("123.45 "), (" ", Some(Token::Number)));
        assert_eq!(number("123"), ("", Some(Token::Number)));
        assert_eq!(number("+123.4"), ("", Some(Token::Number)));
        assert_eq!(number("-456.7"), ("", Some(Token::Number)));
        assert_eq!(number(".0"), ("", Some(Token::Number)));
        assert_eq!(number("..0"), ("", Some(Token::Number))); // OK ?????
        assert_eq!(number("123.456.789"), ("", Some(Token::Number))); // OK ?????
        assert_eq!(number("+123.4abc "), ("abc ", Some(Token::Number)));
    }

    #[test]
    fn test_source() {
        assert_eq!(source("123world"), vec![Token::Number, Token::Ident]);
        assert_eq!(source("Hello world"), vec![Token::Ident, Token::Ident]);
        assert_eq!(source("      world"), vec![Token::Ident]);
    }

    #[test]
    fn test_whitespace() {
        assert_eq!(whitespace("   "), "");
        assert_eq!(whitespace(" abc "), "abc ");
    }
}
