fn main() {
    #[derive(enum_str_conv::EnumStrConv)]
    // No error type or unknown fn is provided
    enum MyEnum {
        #[enum_str_conv(str = "apple")]
        A,
    }
}
