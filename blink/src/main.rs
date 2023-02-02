#![no_std]
#![no_main]

extern crate avr_std_stub;
use avrd::current::*;
use core::ptr::write_volatile;

#[no_mangle]
pub extern "C" fn main() {
    unsafe { write_volatile(DDRC, 0b10000000) }

    loop {
        unsafe { write_volatile(PORTC, 0b10000000) }
        avr_delay::delay_ms(1000);
        unsafe { write_volatile(PORTC, 0b00000000) }
        avr_delay::delay_ms(1000);
    }
}
