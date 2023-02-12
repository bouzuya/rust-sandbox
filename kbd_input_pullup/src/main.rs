#![no_std]
#![no_main]

pub mod atmega32u4;

extern crate avr_std_stub;

#[no_mangle]
pub extern "C" fn main() {
    // PINOUT: <https://docs.arduino.cc/hardware/micro>
    //
    // pinMode(13, OUTPUT);
    // 13 -> PC7
    // pinMode(2, INPUT_PULLUP);
    // 2 -> PD1
    use atmega32u4::port::{C7 as led_pin, D1 as input_pin};
    use ruduino::Pin;

    led_pin::set_output();

    input_pin::set_input();
    input_pin::set_high(); // INPUT_PULLUP

    loop {
        if input_pin::is_low() {
            led_pin::set_high();
        } else {
            led_pin::set_low();
        }
    }
}
