fn main() {
    #[derive(enum_str_conv::EnumStrConv)]
    #[enum_str_conv]
    enum MyEnum {
        #[enum_str_conv(str = "apple")]
        A,
    }
}
