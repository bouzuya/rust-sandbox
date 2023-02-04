#[allow(non_camel_case_types)]
pub struct PORTC;

impl ruduino::Register for PORTC {
    type T = u8;
    const ADDRESS: *mut u8 = avrd::atmega32u4::PORTC;
}

#[allow(non_camel_case_types)]
pub struct DDRC;

impl ruduino::Register for DDRC {
    type T = u8;
    const ADDRESS: *mut u8 = avrd::atmega32u4::DDRC;
}

#[allow(non_camel_case_types)]
pub struct PINC;

impl ruduino::Register for PINC {
    type T = u8;
    const ADDRESS: *mut u8 = avrd::atmega32u4::PINC;
}

pub mod port {
    use super::*;

    macro_rules! define_pin {
        ($port: ty, $ddr: ident, $pin: ident, $name: ident, $index: expr) => {
            pub struct $name;

            impl ruduino::Pin for $name {
                type PORT = $port;
                type DDR = $ddr;
                type PIN = $pin;
                const MASK: u8 = 1 << $index;
            }
        };
    }

    define_pin!(PORTC, DDRC, PINC, C0, 0);
    define_pin!(PORTC, DDRC, PINC, C1, 1);
    define_pin!(PORTC, DDRC, PINC, C2, 2);
    define_pin!(PORTC, DDRC, PINC, C3, 3);
    define_pin!(PORTC, DDRC, PINC, C4, 4);
    define_pin!(PORTC, DDRC, PINC, C5, 5);
    define_pin!(PORTC, DDRC, PINC, C6, 6);
    define_pin!(PORTC, DDRC, PINC, C7, 7);
}
