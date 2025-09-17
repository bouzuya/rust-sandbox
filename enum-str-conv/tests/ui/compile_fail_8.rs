fn main() {
    fn unknown_fn(_s: String) {
        unreachable!()
    }

    #[derive(enum_str_conv::EnumStrConv)]
    #[enum_str_conv(error = (), unknown = unknown_fn)]
    enum MyEnum {
        #[enum_str_conv(error = (), str = "apple")]
        A,
    }
}
