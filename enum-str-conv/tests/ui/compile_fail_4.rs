fn main() {
    fn unknown_fn(_s: String) {
        unreachable!()
    }

    #[derive(enum_str_conv::EnumStrConv)]
    // No error type is provided
    #[enum_str_conv(unknown = unknown_fn)]
    enum MyEnum {
        #[enum_str_conv(str = "apple")]
        A,
    }
}
