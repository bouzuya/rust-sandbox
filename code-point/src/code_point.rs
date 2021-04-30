use std::{convert::TryFrom, fmt::Display};

#[derive(Debug, Eq, PartialEq)]
pub struct CodePoint(u32);

impl Display for CodePoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "U+{:04X}", self.0)
    }
}

impl From<char> for CodePoint {
    fn from(c: char) -> Self {
        Self(c as u32)
    }
}

impl From<CodePoint> for char {
    fn from(code_point: CodePoint) -> Self {
        std::char::from_u32(code_point.0).expect("invalid CodePoint")
    }
}

impl TryFrom<&str> for CodePoint {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if !(3..=8).contains(&value.chars().count()) {
            return Err("code-point string length must be between 3 and 8");
        }
        if !value.starts_with("U+") {
            return Err("code-point string must start with U+");
        }
        let code_point = value.trim_start_matches("U+");
        code_point
            .chars()
            .map(|c| c.to_digit(16))
            .fold(Some(0_u32), |acc, i| {
                acc.and_then(|a| i.and_then(|i| Some(a * 16 + i)))
            })
            .map(|code_point| Self(code_point))
            .ok_or("code-point string must be in U+0000 - U+10FFFF")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(char::from(CodePoint::from('A')), 'A');
        assert_eq!(format!("{}", CodePoint::from('A')), "U+0041");
        assert_eq!(CodePoint::try_from("U+0041").unwrap(), CodePoint::from('A'));

        assert_eq!(CodePoint::try_from("U+2610").unwrap(), CodePoint::from('☐'));
        assert_eq!(CodePoint::try_from("U+2611").unwrap(), CodePoint::from('☑'));
        assert_eq!(CodePoint::try_from("U+2612").unwrap(), CodePoint::from('☒'));
    }
}
