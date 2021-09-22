use chrono::{FixedOffset, Local, Offset};
use nom::{
    bytes::complete::take_while_m_n,
    character::{complete::one_of, streaming::char},
    combinator::{all_consuming, map, map_res},
    sequence::tuple,
    IResult,
};
use std::num::ParseIntError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimeZoneOffset(FixedOffset);

impl Default for TimeZoneOffset {
    fn default() -> Self {
        Self(Local::now().offset().fix())
    }
}

impl std::fmt::Display for TimeZoneOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

impl From<FixedOffset> for TimeZoneOffset {
    fn from(fixed_offset: FixedOffset) -> Self {
        Self(fixed_offset)
    }
}

impl From<TimeZoneOffset> for FixedOffset {
    fn from(time_zone_offset: TimeZoneOffset) -> Self {
        time_zone_offset.0
    }
}

fn from_digit2(s: &str) -> Result<i32, ParseIntError> {
    s.parse::<i32>()
}

fn hour_or_minutes(s: &str) -> IResult<&str, i32> {
    map_res(take_while_m_n(2, 2, is_ascii_digit), from_digit2)(s)
}

fn is_ascii_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn parse(s: &str) -> IResult<&str, FixedOffset> {
    map(
        tuple((one_of("+-"), hour_or_minutes, char(':'), hour_or_minutes)),
        |(sign, h, _, min)| match sign {
            '+' => FixedOffset::east((h * 60 + min) * 60),
            '-' => FixedOffset::west((h * 60 + min) * 60),
            _ => unreachable!(),
        },
    )(s)
}

impl std::str::FromStr for TimeZoneOffset {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, offset) = all_consuming(parse)(s).map_err(|_| ())?;
        Ok(Self(offset))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn string_convert_test() {
        let f = |s: &str| {
            assert_eq!(TimeZoneOffset::from_str(s).unwrap().to_string(), s);
        };
        f("+09:00");
        f("+09:30");
    }

    #[test]
    fn fixed_offset_convert_test() {
        let f = |o: FixedOffset| {
            assert_eq!(FixedOffset::from(TimeZoneOffset::from(o)), o);
        };
        f(FixedOffset::east(9 * 60 * 60));
        f(FixedOffset::east((9 * 60 + 30) * 60));
    }
}
