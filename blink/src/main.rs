#![no_std]
#![no_main]

pub mod atmega32u4;

extern crate avr_std_stub;

#[no_mangle]
pub extern "C" fn main() {
    use atmega32u4::port::C7;
    use ruduino::Pin;

    C7::set_output();

    loop {
        C7::set_high();
        avr_delay::delay_ms(1000);
        C7::set_low();
        avr_delay::delay_ms(1000);
    }
}
