fn main() {
    let input = "123world";
    println!("source: {:?}, parsed: {:?}", input, source(input));
}

#[derive(Debug, PartialEq)]
enum TokenTree<'a> {
    Token(Token<'a>),
    Tree(Vec<TokenTree<'a>>),
}

#[derive(Debug, PartialEq)]
enum Token<'a> {
    Ident(&'a str),
    Number(f64),
    LParen,
    RParen,
}

trait CharsExt {
    fn peek(&self) -> Option<char>;
}

impl<'a> CharsExt for std::str::Chars<'a> {
    fn peek(&self) -> Option<char> {
        self.clone().next()
    }
}

fn ident(input: &str) -> (&str, Option<Token>) {
    let mut chars = input.chars();
    let mut token = None;
    if let Some('a'..='z' | 'A'..='Z') = chars.peek() {
        chars.next();
        while matches!(chars.peek(), Some('a'..='z' | 'A'..='Z' | '0'..='9')) {
            chars.next();
        }
        token = Some(Token::Ident(&input[..input.len() - chars.as_str().len()]));
    }
    (chars.as_str(), token)
}

fn lparen(input: &str) -> (&str, Option<Token>) {
    let mut chars = input.chars();
    let mut token = None;
    if let Some('(') = chars.peek() {
        chars.next();
        token = Some(Token::LParen);
    }
    (chars.as_str(), token)
}

fn number(input: &str) -> (&str, Option<Token>) {
    let mut chars = input.chars();
    let mut token = None;
    if let Some('-' | '+' | '.' | '0'..='9') = chars.peek() {
        chars.next();
        while matches!(chars.clone().next(), Some('.' | '0'..='9')) {
            chars.next();
        }
        let v = input[..input.len() - chars.as_str().len()]
            .parse::<f64>()
            .expect("FIXME");
        token = Some(Token::Number(v));
    }
    (chars.as_str(), token)
}

fn rparen(input: &str) -> (&str, Option<Token>) {
    let mut chars = input.chars();
    let mut token = None;
    if let Some(')') = chars.peek() {
        chars.next();
        token = Some(Token::RParen);
    }
    (chars.as_str(), token)
}

fn source(mut input: &str) -> (&str, TokenTree) {
    let mut tokens = vec![];
    while !input.is_empty() {
        input = if let (next_input, Some(token)) = token(input) {
            match token {
                Token::LParen => {
                    let (next_input, tt) = source(next_input);
                    tokens.push(tt);
                    next_input
                }
                Token::RParen => return (next_input, TokenTree::Tree(tokens)),
                _ => {
                    tokens.push(TokenTree::Token(token));
                    next_input
                }
            }
        } else {
            break;
        }
    }
    (input, TokenTree::Tree(tokens))
}

fn token(input: &str) -> (&str, Option<Token>) {
    let input = whitespace(input);

    if let (input, Some(token)) = ident(input) {
        return (input, Some(token));
    }

    if let (input, Some(token)) = number(input) {
        return (input, Some(token));
    }

    if let (input, Some(token)) = lparen(input) {
        return (input, Some(token));
    }

    if let (input, Some(token)) = rparen(input) {
        return (input, Some(token));
    }

    (input, None)
}

fn whitespace(input: &str) -> &str {
    let mut chars = input.chars();
    while matches!(chars.peek(), Some(' ')) {
        chars.next();
    }
    chars.as_str()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ident() {
        assert_eq!(ident("Adam"), ("", Some(Token::Ident("Adam"))));
        assert_eq!(ident("abc"), ("", Some(Token::Ident("abc"))));
        assert_eq!(ident("123abc"), ("123abc", None));
        assert_eq!(ident("abc123"), ("", Some(Token::Ident("abc123"))));
        assert_eq!(ident("abc123 "), (" ", Some(Token::Ident("abc123"))));
    }

    #[test]
    fn test_number() {
        assert_eq!(number("123.45 "), (" ", Some(Token::Number(123.45))));
        assert_eq!(number("123"), ("", Some(Token::Number(123.0))));
        assert_eq!(number("+123.4"), ("", Some(Token::Number(123.4))));
        assert_eq!(number("-456.7"), ("", Some(Token::Number(-456.7))));
        assert_eq!(number(".0"), ("", Some(Token::Number(0.0))));
        // assert_eq!(number("..0"), ("", Some(Token::Number(_)))); // panic. OK ?????
        // assert_eq!(number("123.456.789"), ("", Some(Token::Number(_)))); // panic. OK ?????
        assert_eq!(number("+123.4abc "), ("abc ", Some(Token::Number(123.4))));
    }

    #[test]
    fn test_source() {
        assert_eq!(
            source("123world"),
            (
                "",
                TokenTree::Tree(
                    vec![Token::Number(123.0), Token::Ident("world")]
                        .into_iter()
                        .map(TokenTree::Token)
                        .collect()
                )
            )
        );
        assert_eq!(
            source("Hello world"),
            (
                "",
                TokenTree::Tree(
                    vec![Token::Ident("Hello"), Token::Ident("world")]
                        .into_iter()
                        .map(TokenTree::Token)
                        .collect()
                )
            )
        );
        assert_eq!(
            source("      world"),
            (
                "",
                TokenTree::Tree(
                    vec![Token::Ident("world")]
                        .into_iter()
                        .map(TokenTree::Token)
                        .collect()
                )
            )
        );
        assert_eq!(
            source("(123 456 world)"),
            (
                "",
                TokenTree::Tree(vec![TokenTree::Tree(
                    vec![
                        Token::Number(123.0),
                        Token::Number(456.0),
                        Token::Ident("world")
                    ]
                    .into_iter()
                    .map(TokenTree::Token)
                    .collect()
                )])
            )
        );
        assert_eq!(
            source("((car cdr) cdr)"),
            (
                "",
                TokenTree::Tree(vec![TokenTree::Tree(vec![
                    TokenTree::Tree(
                        vec![Token::Ident("car"), Token::Ident("cdr")]
                            .into_iter()
                            .map(TokenTree::Token)
                            .collect()
                    ),
                    TokenTree::Token(Token::Ident("cdr")),
                ])])
            )
        );
        assert_eq!(
            source("()())))((()))"),
            (
                "))((()))", // OK ???
                TokenTree::Tree(vec![TokenTree::Tree(vec![]), TokenTree::Tree(vec![])])
            )
        );
    }

    #[test]
    fn test_whitespace() {
        assert_eq!(whitespace("   "), "");
        assert_eq!(whitespace(" abc "), "abc ");
    }
}
