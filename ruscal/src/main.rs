fn main() {
    let input = "123world";
    println!("source: {:?}, parsed: {:?}", input, expr(input));
}

#[derive(Debug, PartialEq)]
enum Expression<'a> {
    Ident(&'a str),
    NumLiteral(f64),
    Add(Box<Expression<'a>>, Box<Expression<'a>>),
}

trait CharsExt {
    fn peek(&self) -> Option<char>;
}

impl<'a> CharsExt for std::str::Chars<'a> {
    fn peek(&self) -> Option<char> {
        self.clone().next()
    }
}

fn add(input: &str) -> Option<(&str, Expression)> {
    let (input, lhs) = {
        let mut input = input;
        let mut prev = None;
        while let Some((next_input, lhs)) = add_term(input) {
            input = next_input;
            prev = Some(match prev {
                None => lhs,
                Some(p) => Expression::Add(Box::new(p), Box::new(lhs)),
            });
        }
        prev.map(|lhs| (input, lhs))
    }?;
    let (input, rhs) = expr(input)?;
    Some((input, Expression::Add(Box::new(lhs), Box::new(rhs))))
}

fn add_term(input: &str) -> Option<(&str, Expression)> {
    let (input, lhs) = term(input)?;
    let input = plus(whitespace(input))?;
    Some((input, lhs))
}

fn expr(input: &str) -> Option<(&str, Expression)> {
    if let Some(ret) = add(input) {
        return Some(ret);
    }

    if let Some(ret) = term(input) {
        return Some(ret);
    }

    None
}

fn ident(input: &str) -> Option<(&str, Expression)> {
    let mut chars = input.chars();
    if let Some('a'..='z' | 'A'..='Z') = chars.peek() {
        chars.next();
        while matches!(chars.peek(), Some('a'..='z' | 'A'..='Z' | '0'..='9')) {
            chars.next();
        }
        Some((
            chars.as_str(),
            Expression::Ident(&input[..input.len() - chars.as_str().len()]),
        ))
    } else {
        None
    }
}

fn lparen(input: &str) -> Option<&str> {
    let mut chars = input.chars();
    if let Some('(') = chars.peek() {
        chars.next();
        Some(chars.as_str())
    } else {
        None
    }
}

fn number(input: &str) -> Option<(&str, Expression)> {
    let mut chars = input.chars();
    if let Some('-' | '+' | '.' | '0'..='9') = chars.peek() {
        chars.next();
        while matches!(chars.clone().next(), Some('.' | '0'..='9')) {
            chars.next();
        }
        let v = input[..input.len() - chars.as_str().len()]
            .parse::<f64>()
            .expect("FIXME");
        Some((chars.as_str(), Expression::NumLiteral(v)))
    } else {
        None
    }
}

fn paren(input: &str) -> Option<(&str, Expression)> {
    let input = lparen(whitespace(input))?;
    let (input, expr) = expr(input)?;
    let input = rparen(whitespace(input))?;
    Some((input, expr))
}

fn plus(input: &str) -> Option<&str> {
    let mut chars = input.chars();
    if let Some('+') = chars.peek() {
        chars.next();
        Some(chars.as_str())
    } else {
        None
    }
}

fn rparen(input: &str) -> Option<&str> {
    let mut chars = input.chars();
    if let Some(')') = chars.peek() {
        chars.next();
        Some(chars.as_str())
    } else {
        None
    }
}

fn term(input: &str) -> Option<(&str, Expression)> {
    if let Some(ret) = paren(input) {
        return Some(ret);
    }

    if let Some(ret) = token(input) {
        return Some(ret);
    }

    None
}

fn token(input: &str) -> Option<(&str, Expression)> {
    let input = whitespace(input);

    if let Some((input, token)) = ident(input) {
        return Some((input, token));
    }

    if let Some((input, token)) = number(input) {
        return Some((input, token));
    }

    None
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
    fn test_add() {
        assert_eq!(
            add("1+2"),
            Some((
                "",
                Expression::Add(
                    Box::new(Expression::NumLiteral(1.0)),
                    Box::new(Expression::NumLiteral(2.0))
                )
            ))
        );
        assert_eq!(
            add("1 + 2 + 3"),
            Some((
                "",
                Expression::Add(
                    Box::new(Expression::Add(
                        Box::new(Expression::NumLiteral(1.0)),
                        Box::new(Expression::NumLiteral(2.0))
                    )),
                    Box::new(Expression::NumLiteral(3.0))
                )
            ))
        );
        assert_eq!(add("1+"), None);
    }

    #[test]
    fn test_expr() {
        assert_eq!(
            expr("123world"),
            Some(("world", Expression::NumLiteral(123.0)))
        );
        assert_eq!(
            expr("Hello world"),
            Some((" world", Expression::Ident("Hello")))
        );
        assert_eq!(expr("      world"), Some(("", Expression::Ident("world"))));
        assert_eq!(expr("(123)"), Some(("", Expression::NumLiteral(123.0))));
        assert_eq!(expr("(world)"), Some(("", Expression::Ident("world"))));
        assert_eq!(
            expr("(123+456)"),
            Some((
                "",
                Expression::Add(
                    Box::new(Expression::NumLiteral(123.0)),
                    Box::new(Expression::NumLiteral(456.0))
                )
            ))
        );
    }

    #[test]
    fn test_ident() {
        assert_eq!(ident("Adam"), Some(("", Expression::Ident("Adam"))));
        assert_eq!(ident("abc"), Some(("", Expression::Ident("abc"))));
        assert_eq!(ident("123abc"), None);
        assert_eq!(ident("abc123"), Some(("", Expression::Ident("abc123"))));
        assert_eq!(ident("abc123 "), Some((" ", Expression::Ident("abc123"))));
    }

    #[test]
    fn test_number() {
        assert_eq!(
            number("123.45 "),
            Some((" ", Expression::NumLiteral(123.45)))
        );
        assert_eq!(number("123"), Some(("", Expression::NumLiteral(123.0))));
        assert_eq!(number("+123.4"), Some(("", Expression::NumLiteral(123.4))));
        assert_eq!(number("-456.7"), Some(("", Expression::NumLiteral(-456.7))));
        assert_eq!(number(".0"), Some(("", Expression::NumLiteral(0.0))));
        // assert_eq!(number("..0"), Some(("", Expression::Number(_)))); // panic. OK ?????
        // assert_eq!(number("123.456.789"), Some(("", Expression::Number(_)))); // panic. OK ?????
        assert_eq!(
            number("+123.4abc "),
            Some(("abc ", Expression::NumLiteral(123.4)))
        );
    }

    #[test]
    fn test_whitespace() {
        assert_eq!(whitespace("   "), "");
        assert_eq!(whitespace(" abc "), "abc ");
    }
}
