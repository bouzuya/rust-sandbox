fn main() {
    #[derive(Debug)]
    enum MyError {
        Unknown(String),
    }
    impl std::fmt::Display for MyError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                MyError::Unknown(s) => write!(f, "unknown variant: {0}", s),
            }
        }
    }
    impl std::error::Error for MyError {}

    // Debug and PartialEq are for testing
    #[derive(Debug, PartialEq, enum_str_conv::EnumStrConv)]
    #[enum_str_conv(error = MyError, unknown = MyError::Unknown)]
    enum MyEnum {
        #[enum_str_conv(str = "apple")]
        A,
        #[enum_str_conv(str = "banana")]
        B,
        #[enum_str_conv(str = "cherry")]
        C,
    }

    for (s, o) in [
        ("apple", MyEnum::A),
        ("banana", MyEnum::B),
        ("cherry", MyEnum::C),
    ] {
        assert_eq!(<MyEnum as std::str::FromStr>::from_str(s).unwrap(), o);
        assert_eq!(o.to_string(), s);
    }

    assert_eq!(
        <MyEnum as std::str::FromStr>::from_str("durian")
            .unwrap_err()
            .to_string(),
        "unknown variant: durian"
    );
}
