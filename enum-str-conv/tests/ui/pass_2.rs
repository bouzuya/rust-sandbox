fn main() {
    fn unknown_fn(s: String) -> String {
        format!("unknown: {s}")
    }

    // Debug and PartialEq are for testing
    #[derive(Debug, PartialEq, enum_str_conv::EnumStrConv)]
    #[enum_str_conv(error = String, unknown = unknown_fn)]
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
        <MyEnum as std::str::FromStr>::from_str("durian").unwrap_err(),
        "unknown: durian"
    );
}
