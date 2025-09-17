fn main() {
    #[derive(enum_str_conv::EnumStrConv)]
    // No unknown fn is provided
    #[enum_str_conv(error = ())]
    enum MyEnum {
        #[enum_str_conv(str = "apple")]
        A,
    }
}
