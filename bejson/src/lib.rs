// <https://www.json.org/json-en.html>
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::{char, one_of},
    combinator::{all_consuming, map, opt},
    multi::{fold_many0, many1},
    sequence::{delimited, tuple},
    IResult,
};
use std::iter;

type Member = (JsonString, JsonValue);

#[derive(Debug, Eq, PartialEq)]
pub enum JsonValue {
    CommandString(JsonCommandString),
    Object(Vec<Member>),
    Array(Vec<JsonValue>),
    String(JsonString),
    Number(String),
    True,
    False,
    Null,
}

impl std::str::FromStr for JsonValue {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        json(s).map(|(_, v)| v).map_err(|_| "parse error")
    }
}

impl std::fmt::Display for JsonValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonValue::CommandString(s) => write!(f, "{}", s),
            JsonValue::Object(o) => write!(
                f,
                "{{{}}}",
                o.iter()
                    .map(|(k, v)| format!("{}:{}", k, v))
                    .collect::<Vec<String>>()
                    .join(",")
            ),
            JsonValue::Array(a) => write!(
                f,
                "[{}]",
                a.iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            ),
            JsonValue::String(s) => write!(f, "{}", s),
            JsonValue::Number(n) => write!(f, "{}", n),
            JsonValue::True => write!(f, "{}", "true"),
            JsonValue::False => write!(f, "{}", "false"),
            JsonValue::Null => write!(f, "{}", "null"),
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct JsonCommandString(String);

impl std::fmt::Display for JsonCommandString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"$`{}`"#, self.0)
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct JsonString(String);

impl std::fmt::Display for JsonString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#""{}""#, self.0)
    }
}

fn json(s: &str) -> IResult<&str, JsonValue> {
    all_consuming(json_element)(s)
}

fn json_value(s: &str) -> IResult<&str, JsonValue> {
    alt((
        json_command_string,
        json_object,
        json_array,
        json_string,
        json_number,
        map(tag("true"), |_| JsonValue::True),
        map(tag("false"), |_| JsonValue::False),
        map(tag("null"), |_| JsonValue::Null),
    ))(s)
}

fn json_command_string(s: &str) -> IResult<&str, JsonValue> {
    map(
        tuple((
            char('$'),
            delimited(char('`'), json_command_string_characters, char('`')),
        )),
        |(_, s)| JsonValue::CommandString(JsonCommandString(s)),
    )(s)
}

fn json_command_string_characters(s: &str) -> IResult<&str, String> {
    fold_many0(json_command_string_character, String::new(), |mut t, x| {
        t.push_str(&x);
        t
    })(s)
}

fn json_command_string_character(s: &str) -> IResult<&str, String> {
    alt((
        map(
            take_while_m_n(1, 1, |c: char| {
                ('\u{0020}'..='\u{10FFFF}').contains(&c) && c != '`' && c != '\\'
            }),
            |c: &str| c.to_string(),
        ),
        map(
            tuple((char('\\'), json_command_string_escape)),
            |(bs, escape)| iter::once(bs).chain(escape.chars()).collect::<String>(),
        ),
    ))(s)
}

fn json_command_string_escape(s: &str) -> IResult<&str, String> {
    alt((
        map(one_of(r#"\"/bfnrt`"#), |c| c.to_string()),
        map(
            tuple((char('u'), json_hex, json_hex, json_hex, json_hex)),
            |(u, h1, h2, h3, h4)| vec![u, h1, h2, h3, h4].iter().collect::<String>(),
        ),
    ))(s)
}

fn json_object(s: &str) -> IResult<&str, JsonValue> {
    map(
        alt((
            map(tuple((char('{'), json_ws, char('}'))), |_| vec![]),
            delimited(char('{'), json_members, char('}')),
        )),
        |o| JsonValue::Object(o),
    )(s)
}

fn json_members(s: &str) -> IResult<&str, Vec<Member>> {
    alt((
        map(
            tuple((json_member, char(','), json_members)),
            |(m, _, ms)| iter::once(m).chain(ms).collect::<Vec<Member>>(),
        ),
        map(json_member, |m| vec![m]),
    ))(s)
}

fn json_member(s: &str) -> IResult<&str, Member> {
    map(
        tuple((json_ws, json_string, json_ws, char(':'), json_element)),
        |(_, s, _, _, e)| {
            if let JsonValue::String(s) = s {
                (s, e)
            } else {
                unreachable!()
            }
        },
    )(s)
}

fn json_array(s: &str) -> IResult<&str, JsonValue> {
    map(
        alt((
            map(tuple((char('['), json_ws, char(']'))), |_| vec![]),
            delimited(char('['), json_elements, char(']')),
        )),
        |a| JsonValue::Array(a),
    )(s)
}

fn json_elements(s: &str) -> IResult<&str, Vec<JsonValue>> {
    alt((
        map(
            tuple((json_element, char(','), json_elements)),
            |(v, _, vs)| iter::once(v).chain(vs).collect::<Vec<JsonValue>>(),
        ),
        map(json_element, |v| vec![v]),
    ))(s)
}

fn json_element(s: &str) -> IResult<&str, JsonValue> {
    map(tuple((json_ws, json_value, json_ws)), |(_, v, _)| v)(s)
}

fn json_string(s: &str) -> IResult<&str, JsonValue> {
    map(delimited(char('"'), json_characters, char('"')), |s| {
        JsonValue::String(JsonString(s))
    })(s)
}

fn json_characters(s: &str) -> IResult<&str, String> {
    fold_many0(json_character, String::new(), |mut t, x| {
        t.push_str(&x);
        t
    })(s)
}

fn json_character(s: &str) -> IResult<&str, String> {
    alt((
        map(
            take_while_m_n(1, 1, |c: char| {
                ('\u{0020}'..='\u{10FFFF}').contains(&c) && c != '"' && c != '\\'
            }),
            |c: &str| c.to_string(),
        ),
        map(tuple((char('\\'), json_escape)), |(bs, escape)| {
            iter::once(bs).chain(escape.chars()).collect::<String>()
        }),
    ))(s)
}

fn json_escape(s: &str) -> IResult<&str, String> {
    alt((
        map(one_of(r#""\/bfnrt"#), |c| c.to_string()),
        map(
            tuple((char('u'), json_hex, json_hex, json_hex, json_hex)),
            |(u, h1, h2, h3, h4)| vec![u, h1, h2, h3, h4].iter().collect::<String>(),
        ),
    ))(s)
}

fn json_hex(s: &str) -> IResult<&str, char> {
    alt((json_digit, one_of("ABCDEF"), one_of("abcdef")))(s)
}

fn json_number(s: &str) -> IResult<&str, JsonValue> {
    map(
        tuple((json_integer, json_fraction, json_exponent)),
        |(i, f, e)| JsonValue::Number(format!("{}{}{}", i, f, e)),
    )(s)
}

fn json_integer(s: &str) -> IResult<&str, String> {
    map(
        alt((
            map(
                tuple((char('-'), json_onenine, json_digits)),
                |(minus, onenine, digits)| {
                    iter::once(minus)
                        .chain(iter::once(onenine))
                        .chain(digits.into_iter())
                        .collect::<Vec<char>>()
                },
            ),
            map(tuple((char('-'), json_digit)), |(minus, digit)| {
                iter::once(minus)
                    .chain(iter::once(digit))
                    .collect::<Vec<char>>()
            }),
            map(tuple((json_onenine, json_digits)), |(onenine, digits)| {
                iter::once(onenine)
                    .chain(digits.into_iter())
                    .collect::<Vec<char>>()
            }),
            map(json_digit, |c| vec![c]),
        )),
        |v| v.iter().collect::<String>(),
    )(s)
}

fn json_digits(s: &str) -> IResult<&str, Vec<char>> {
    many1(json_digit)(s)
}

fn json_digit(s: &str) -> IResult<&str, char> {
    alt((char('0'), json_onenine))(s)
}

fn json_onenine(s: &str) -> IResult<&str, char> {
    one_of("123456789")(s)
}

fn json_fraction(s: &str) -> IResult<&str, String> {
    map(
        opt(map(tuple((char('.'), json_digits)), |(dot, digits)| {
            iter::once(dot)
                .chain(digits.into_iter())
                .collect::<String>()
        })),
        |s| s.unwrap_or("".to_string()),
    )(s)
}

fn json_exponent(s: &str) -> IResult<&str, String> {
    map(
        opt(map(
            tuple((one_of("Ee"), json_sign, json_digits)),
            |(e, sign, digits)| {
                iter::once(e)
                    .chain(
                        sign.map(|s| s.to_string())
                            .unwrap_or("".to_string())
                            .chars()
                            .into_iter(),
                    )
                    .chain(digits.into_iter())
                    .collect::<String>()
            },
        )),
        |s| s.unwrap_or("".to_string()),
    )(s)
}

fn json_sign(s: &str) -> IResult<&str, Option<char>> {
    opt(one_of("+-"))(s)
}

fn json_ws(s: &str) -> IResult<&str, String> {
    fold_many0(
        one_of("\u{0020}\u{000A}\u{000D}\u{0009}"),
        String::new(),
        |mut s, c| {
            s.push(c);
            s
        },
    )(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn m(k: &str, v: JsonValue) -> Member {
        (JsonString(k.to_string()), v)
    }

    fn vcs(s: &str) -> JsonValue {
        JsonValue::CommandString(JsonCommandString(s.to_string()))
    }

    fn vo(o: Vec<Member>) -> JsonValue {
        JsonValue::Object(o)
    }

    fn va(a: Vec<JsonValue>) -> JsonValue {
        JsonValue::Array(a)
    }

    fn vs(s: &str) -> JsonValue {
        JsonValue::String(JsonString(s.to_string()))
    }

    fn vn(n: &str) -> JsonValue {
        JsonValue::Number(n.to_string())
    }

    fn vt() -> JsonValue {
        JsonValue::True
    }

    fn vf() -> JsonValue {
        JsonValue::False
    }

    fn vnull() -> JsonValue {
        JsonValue::Null
    }

    #[test]
    fn json_test() {
        // json
        //   element
        let f = json_value;
        assert_eq!(
            f(r#"{"abc": 123, "def": [123.456, true, false, null]}"#),
            Ok((
                "",
                vo(vec![
                    m("abc", vn("123")),
                    m("def", va(vec![vn("123.456"), vt(), vf(), vnull()]))
                ])
            ))
        );
    }

    #[test]
    fn json_value_test() {
        // value
        //   command_string
        //   object
        //   array
        //   string
        //   number
        //   "true"
        //   "false"
        //   "null"
        let f = json_value;
        assert_eq!(f(r#"$``"#), Ok(("", vcs(""))));
        assert_eq!(f(r#"{}"#), Ok(("", vo(vec![]))));
        assert_eq!(f(r#"[]"#), Ok(("", va(vec![]))));
        assert_eq!(f(r#""abc""#), Ok(("", vs("abc"))));
        assert_eq!(f(r#"123"#), Ok(("", vn("123"))));
        assert_eq!(f(r#"123.456"#), Ok(("", vn("123.456"))));
        assert_eq!(f(r#"true"#), Ok(("", vt())));
        assert_eq!(f(r#"false"#), Ok(("", vf())));
        assert_eq!(f(r#"null"#), Ok(("", vnull())));
    }

    #[test]
    fn json_command_string_test() {
        // command_string
        //   "$`" command_string_characters '`'
        let f = json_command_string;
        assert_eq!(
            f(r#"$`echo "\`hello\`"`"#),
            Ok(("", vcs(r#"echo "\`hello\`""#)))
        );
    }

    #[test]
    fn json_command_string_characters_test() {
        // command_string_characters
        //   ""
        //   command_string_character command_string_characters
        let f = json_command_string_characters;
        assert_eq!(f(""), Ok(("", "".to_string())));
        assert_eq!(f("abc"), Ok(("", "abc".to_string())));
    }

    #[test]
    fn json_command_string_character_test() {
        // command_string_character
        //   '0020' . '10FFFF' - '`' - '\'
        //   '\' command_string_escape
        let f = json_command_string_character;
        let s = |s: &str| s.to_string();
        assert_eq!(f("\u{0020}"), Ok(("", s("\u{0020}"))));
        assert_eq!(f("\u{10FFFF}"), Ok(("", s("\u{10FFFF}"))));
        assert_eq!(f(r#"`"#).is_err(), true);
        assert_eq!(f(r#"\`"#), Ok(("", s(r#"\`"#))));
        assert_eq!(f(r#"""#), Ok(("", s(r#"""#))));
        assert_eq!(f(r#"\""#), Ok(("", s(r#"\""#))));
        assert_eq!(f(r#"\"#).is_err(), true);
        assert_eq!(f(r#"\\"#), Ok(("", s(r#"\\"#))));
        assert_eq!(f(r#"\/"#), Ok(("", s(r#"\/"#))));
        assert_eq!(f(r#"\b"#), Ok(("", s(r#"\b"#))));
        assert_eq!(f(r#"\f"#), Ok(("", s(r#"\f"#))));
        assert_eq!(f(r#"\n"#), Ok(("", s(r#"\n"#))));
        assert_eq!(f(r#"\r"#), Ok(("", s(r#"\r"#))));
        assert_eq!(f(r#"\t"#), Ok(("", s(r#"\t"#))));
        assert_eq!(f(r#"\u0020"#), Ok(("", s(r#"\u0020"#))));
        assert_eq!(f(r#"\u002"#).is_err(), true);
        assert_eq!(f(r#"\u005A"#), Ok(("", s(r#"\u005A"#))));
        assert_eq!(f(r#"\u10FFFF"#), Ok(("FF", s(r#"\u10FF"#))));
    }

    #[test]
    fn json_command_string_escape_test() {
        // command_string_escape
        //   '`'
        //   '"'
        //   '\'
        //   '/'
        //   'b'
        //   'f'
        //   'n'
        //   'r'
        //   't'
        //   'u' hex hex hex hex
        let f = json_command_string_escape;
        let s = |s: &str| s.to_string();
        assert_eq!(f(r#"`"#), Ok(("", s(r#"`"#))));
        assert_eq!(f(r#"""#), Ok(("", s(r#"""#))));
        assert_eq!(f(r#"\"#), Ok(("", s(r#"\"#))));
        assert_eq!(f(r#"/"#), Ok(("", s(r#"/"#))));
        assert_eq!(f(r#"b"#), Ok(("", s(r#"b"#))));
        assert_eq!(f(r#"f"#), Ok(("", s(r#"f"#))));
        assert_eq!(f(r#"n"#), Ok(("", s(r#"n"#))));
        assert_eq!(f(r#"r"#), Ok(("", s(r#"r"#))));
        assert_eq!(f(r#"t"#), Ok(("", s(r#"t"#))));
        assert_eq!(f(r#"u0000"#), Ok(("", s(r#"u0000"#))));
        assert_eq!(f(r#"uffff"#), Ok(("", s(r#"uffff"#))));
    }

    #[test]
    fn json_object_test() {
        // object
        //   '{' ws '}'
        //   '{' members '}'
        let f = json_object;
        assert_eq!(f(r#"{ }"#), Ok(("", vo(vec![]))));
        assert_eq!(
            f(r#"{ "abc" : 123 , "def" : 456 }"#),
            Ok(("", vo(vec![m("abc", vn("123")), m("def", vn("456"))])))
        );
    }

    #[test]
    fn json_members_test() {
        // members
        //   member
        //   member ',' members
        let f = json_members;
        assert_eq!(f(r#" "abc" : null"#), Ok(("", vec![m("abc", vnull())])));
        assert_eq!(
            f(r#" "abc" : null, "def" : null"#),
            Ok(("", vec![m("abc", vnull()), m("def", vnull())]))
        );
    }

    #[test]
    fn json_member_test() {
        // member
        //   ws string ws ':' element
        let f = json_member;
        assert_eq!(f(r#" "abc" : null"#), Ok(("", m("abc", vnull()))));
    }

    #[test]
    fn json_array_test() {
        // array
        //   '[' ws ']'
        //   '[' elements ']'
        let f = json_array;
        assert_eq!(f(r#"[ ]"#), Ok(("", va(vec![]))));
        assert_eq!(f(r#"[ "a" , 1 ]"#), Ok(("", va(vec![vs("a"), vn("1")]))));
    }

    #[test]
    fn json_elements_test() {
        // elements
        //   element
        //   element ',' elements
        let f = json_elements;
        assert_eq!(f(r#""a""#), Ok(("", vec![vs("a")])));
        assert_eq!(f(r#""a", 1"#), Ok(("", vec![vs("a"), vn("1")])));
    }

    #[test]
    fn json_element_test() {
        // element
        //   ws value ws
        let f = json_element;
        assert_eq!(f(r#" "abc" "#), Ok(("", vs("abc"))));
        assert_eq!(f(r#" 123 "#), Ok(("", vn("123"))));
    }

    #[test]
    fn json_string_test() {
        // string
        //   '"' characters '"'
        let f = json_string;
        assert_eq!(f(r#""#).is_err(), true);
        assert_eq!(f(r#""a""#), Ok(("", vs("a"))));
        assert_eq!(f(r#""ab""#), Ok(("", vs("ab"))));
        assert_eq!(f(r#""a\"b""#), Ok(("", vs(r#"a\"b"#))));
        assert_eq!(f("\"\u{0020}\""), Ok(("", vs("\u{0020}"))));
        assert_eq!(f("\"\u{10FFFF}\""), Ok(("", vs("\u{10FFFF}"))));
        assert_eq!(f(r#"""#).is_err(), true);
        assert_eq!(f(r#""""#), Ok(("", vs(r#""#))));
        assert_eq!(f(r#""\"""#), Ok(("", vs(r#"\""#))));
        assert_eq!(f(r#""\\""#), Ok(("", vs(r#"\\"#))));
        assert_eq!(f(r#""\/""#), Ok(("", vs(r#"\/"#))));
        assert_eq!(f(r#""\b""#), Ok(("", vs(r#"\b"#))));
        assert_eq!(f(r#""\f""#), Ok(("", vs(r#"\f"#))));
        assert_eq!(f(r#""\n""#), Ok(("", vs(r#"\n"#))));
        assert_eq!(f(r#""\r""#), Ok(("", vs(r#"\r"#))));
        assert_eq!(f(r#""\t""#), Ok(("", vs(r#"\t"#))));
        assert_eq!(f(r#""\u0020""#), Ok(("", vs(r#"\u0020"#))));
        assert_eq!(f(r#""\u002""#).is_err(), true);
        assert_eq!(f(r#""\u005A""#), Ok(("", vs(r#"\u005A"#))));
        assert_eq!(f(r#""\u10FFFF""#), Ok(("", vs(r#"\u10FFFF"#))));
    }

    #[test]
    fn json_characters_test() {
        // characters
        //   ""
        //   character characters
        let f = json_characters;
        let s = |s: &str| s.to_string();
        assert_eq!(f(""), Ok(("", s(""))));
        assert_eq!(f("\u{0020}"), Ok(("", s("\u{0020}"))));
        assert_eq!(f("\u{10FFFF}"), Ok(("", s("\u{10FFFF}"))));
        assert_eq!(f(r#"""#), Ok(("\"", s(""))));
        assert_eq!(f(r#"\""#), Ok(("", s(r#"\""#))));
        assert_eq!(f(r#"\"#), Ok(("\\", s(""))));
        assert_eq!(f(r#"\\"#), Ok(("", s(r#"\\"#))));
        assert_eq!(f(r#"\/"#), Ok(("", s(r#"\/"#))));
        assert_eq!(f(r#"\b"#), Ok(("", s(r#"\b"#))));
        assert_eq!(f(r#"\f"#), Ok(("", s(r#"\f"#))));
        assert_eq!(f(r#"\n"#), Ok(("", s(r#"\n"#))));
        assert_eq!(f(r#"\r"#), Ok(("", s(r#"\r"#))));
        assert_eq!(f(r#"\t"#), Ok(("", s(r#"\t"#))));
        assert_eq!(f(r#"\u0020"#), Ok(("", s(r#"\u0020"#))));
        assert_eq!(f(r#"\u002"#), Ok((r#"\u002"#, s(""))));
        assert_eq!(f(r#"\u005A"#), Ok(("", s(r#"\u005A"#))));
        assert_eq!(f(r#"\u10FFFF"#), Ok(("", s(r#"\u10FFFF"#))));
    }

    #[test]
    fn json_character_test() {
        // character
        //   '0020' . '10FFFF' - '"' - '\'
        //   '\' escape
        let f = json_character;
        let s = |s: &str| s.to_string();
        assert_eq!(f("").is_err(), true);
        assert_eq!(f("\u{0020}"), Ok(("", s("\u{0020}"))));
        assert_eq!(f("\u{10FFFF}"), Ok(("", s("\u{10FFFF}"))));
        assert_eq!(f(r#"""#).is_err(), true);
        assert_eq!(f(r#"\""#), Ok(("", s(r#"\""#))));
        assert_eq!(f(r#"\"#).is_err(), true);
        assert_eq!(f(r#"\\"#), Ok(("", s(r#"\\"#))));
        assert_eq!(f(r#"\/"#), Ok(("", s(r#"\/"#))));
        assert_eq!(f(r#"\b"#), Ok(("", s(r#"\b"#))));
        assert_eq!(f(r#"\f"#), Ok(("", s(r#"\f"#))));
        assert_eq!(f(r#"\n"#), Ok(("", s(r#"\n"#))));
        assert_eq!(f(r#"\r"#), Ok(("", s(r#"\r"#))));
        assert_eq!(f(r#"\t"#), Ok(("", s(r#"\t"#))));
        assert_eq!(f(r#"\u0020"#), Ok(("", s(r#"\u0020"#))));
        assert_eq!(f(r#"\u002"#).is_err(), true);
        assert_eq!(f(r#"\u005A"#), Ok(("", s(r#"\u005A"#))));
        assert_eq!(f(r#"\u10FFFF"#), Ok(("FF", s(r#"\u10FF"#))));
    }

    #[test]
    fn json_escape_test() {
        // escape
        //   '"'
        //   '\'
        //   '/'
        //   'b'
        //   'f'
        //   'n'
        //   'r'
        //   't'
        //   'u' hex hex hex hex
        let f = json_escape;
        let s = |s: &str| s.to_string();
        assert_eq!(f(r#"""#), Ok(("", s(r#"""#))));
        assert_eq!(f(r#"\"#), Ok(("", s(r#"\"#))));
        assert_eq!(f(r#"/"#), Ok(("", s(r#"/"#))));
        assert_eq!(f(r#"b"#), Ok(("", s(r#"b"#))));
        assert_eq!(f(r#"f"#), Ok(("", s(r#"f"#))));
        assert_eq!(f(r#"n"#), Ok(("", s(r#"n"#))));
        assert_eq!(f(r#"r"#), Ok(("", s(r#"r"#))));
        assert_eq!(f(r#"t"#), Ok(("", s(r#"t"#))));
        assert_eq!(f(r#"u0000"#), Ok(("", s(r#"u0000"#))));
        assert_eq!(f(r#"uffff"#), Ok(("", s(r#"uffff"#))));
    }

    #[test]
    fn json_hex_test() {
        // hex
        //   digit
        //   'A' . 'F'
        //   'a' . 'f'
        let f = json_hex;
        assert_eq!(f("0"), Ok(("", '0')));
        assert_eq!(f("1"), Ok(("", '1')));
        assert_eq!(f("9"), Ok(("", '9')));
        assert_eq!(f("A"), Ok(("", 'A')));
        assert_eq!(f("F"), Ok(("", 'F')));
        assert_eq!(f("G").is_err(), true);
        assert_eq!(f("a"), Ok(("", 'a')));
        assert_eq!(f("f"), Ok(("", 'f')));
        assert_eq!(f("g").is_err(), true);
    }

    #[test]
    fn json_number_test() {
        // number
        //   integer fraction exponent
        let f = json_number;
        assert_eq!(f("").is_err(), true);
        assert_eq!(f("0"), Ok(("", vn("0"))));
        assert_eq!(f("1"), Ok(("", vn("1"))));
        assert_eq!(f("10"), Ok(("", vn("10"))));
        assert_eq!(f("12"), Ok(("", vn("12"))));
        assert_eq!(f("-0"), Ok(("", vn("-0"))));
        assert_eq!(f("-1"), Ok(("", vn("-1"))));
        assert_eq!(f("-10"), Ok(("", vn("-10"))));
        assert_eq!(f("-12"), Ok(("", vn("-12"))));
        assert_eq!(f("0.0"), Ok(("", vn("0.0"))));
        assert_eq!(f("0.1"), Ok(("", vn("0.1"))));
        assert_eq!(f("0.01"), Ok(("", vn("0.01"))));
        assert_eq!(f("0.1e5"), Ok(("", vn("0.1e5"))));
        assert_eq!(f("0.1e+5"), Ok(("", vn("0.1e+5"))));
        assert_eq!(f("0.1e-5"), Ok(("", vn("0.1e-5"))));
        assert_eq!(f("1e-5"), Ok(("", vn("1e-5"))));
    }

    #[test]
    fn json_integer_test() {
        // integer
        //   digit
        //   onenine digits
        //   '-' digit
        //   '-' onenine digits
        let f = json_integer;
        assert_eq!(f("").is_err(), true);
        assert_eq!(f("0"), Ok(("", "0".to_string())));
        assert_eq!(f("1"), Ok(("", "1".to_string())));
        assert_eq!(f("10"), Ok(("", "10".to_string())));
        assert_eq!(f("12"), Ok(("", "12".to_string())));
        assert_eq!(f("-0"), Ok(("", "-0".to_string())));
        assert_eq!(f("-1"), Ok(("", "-1".to_string())));
        assert_eq!(f("-10"), Ok(("", "-10".to_string())));
        assert_eq!(f("-12"), Ok(("", "-12".to_string())));
    }

    #[test]
    fn json_digits_test() {
        // digits
        //   digit
        //   digit digits
        let f = json_digits;
        assert_eq!(f("").is_err(), true);
        assert_eq!(f("0"), Ok(("", vec!['0'])));
        assert_eq!(f("1"), Ok(("", vec!['1'])));
        assert_eq!(f("01"), Ok(("", vec!['0', '1'])));
    }

    #[test]
    fn json_digit_test() {
        // digit
        //   '0'
        //   onenine
        let f = json_digit;
        assert_eq!(f("").is_err(), true);
        assert_eq!(f("0"), Ok(("", '0')));
        assert_eq!(f("1"), Ok(("", '1')));
        assert_eq!(f("01"), Ok(("1", '0')));
    }

    #[test]
    fn json_onenine_test() {
        // onenine
        //   '1' . '9'
        let f = json_onenine;
        assert_eq!(f("").is_err(), true);
        assert_eq!(f("0").is_err(), true);
        assert_eq!(f("1"), Ok(("", '1')));
        assert_eq!(f("12"), Ok(("2", '1')));
    }

    #[test]
    fn json_fraction_test() {
        // fraction
        //   ""
        //   '.' digits
        let f = json_fraction;
        assert_eq!(f(""), Ok(("", "".to_string())));
        assert_eq!(f("a"), Ok(("a", "".to_string())));
        assert_eq!(f("."), Ok((".", "".to_string())));
        assert_eq!(f(".123"), Ok(("", ".123".to_string())));
    }

    #[test]
    fn json_exponent_test() {
        // exponent
        //   ""
        //   'E' sign digits
        //   'e' sign digits
        let f = json_exponent;
        assert_eq!(f(""), Ok(("", "".to_string())));
        assert_eq!(f("E123"), Ok(("", "E123".to_string())));
        assert_eq!(f("e123"), Ok(("", "e123".to_string())));
        assert_eq!(f("a"), Ok(("a", "".to_string())));
        assert_eq!(f("E"), Ok(("E", "".to_string())));
        assert_eq!(f("e"), Ok(("e", "".to_string())));
    }

    #[test]
    fn json_sign_test() {
        // sign
        //   ""
        //   '+'
        //   '-'
        let f = json_sign;
        assert_eq!(f(""), Ok(("", None)));
        assert_eq!(f("+"), Ok(("", Some('+'))));
        assert_eq!(f("-"), Ok(("", Some('-'))));
        assert_eq!(f("a"), Ok(("a", None)));
    }

    #[test]
    fn json_ws_test() {
        // ws
        //   ""
        //   '0020' ws
        //   '000A' ws
        //   '000D' ws
        //   '0009' ws
        let f = json_ws;
        let s = |s: &str| s.to_string();
        assert_eq!(f(""), Ok(("", s(""))));
        assert_eq!(f("\u{0020}"), Ok(("", s("\u{0020}"))));
        assert_eq!(f("\u{000A}"), Ok(("", s("\u{000A}"))));
        assert_eq!(f("\u{000D}"), Ok(("", s("\u{000D}"))));
        assert_eq!(f("\u{0009}"), Ok(("", s("\u{0009}"))));
        assert_eq!(f("\u{0020}\u{000A}"), Ok(("", s("\u{0020}\u{000A}"))));
    }
}
