#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MemberResponseId(i64);

impl From<i64> for MemberResponseId {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<MemberResponseId> for i64 {
    fn from(id: MemberResponseId) -> Self {
        id.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn i64_conversion_test() {
        let test = |i: i64| assert_eq!(i64::from(MemberResponseId::from(i)), i);
        test(-1);
        test(0);
        test(1);
    }
}
