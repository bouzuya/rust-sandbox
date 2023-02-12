#[allow(non_camel_case_types)]
pub struct PORTD;

impl ruduino::Register for PORTD {
    type T = u8;
    const ADDRESS: *mut u8 = avrd::atmega32u4::PORTD;
}

#[allow(non_camel_case_types)]
pub struct DDRD;

impl ruduino::Register for DDRD {
    type T = u8;
    const ADDRESS: *mut u8 = avrd::atmega32u4::DDRD;
}

#[allow(non_camel_case_types)]
pub struct PIND;

impl ruduino::Register for PIND {
    type T = u8;
    const ADDRESS: *mut u8 = avrd::atmega32u4::PIND;
}

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

    define_pin!(PORTD, DDRD, PIND, D0, 0);
    define_pin!(PORTD, DDRD, PIND, D1, 1);
    define_pin!(PORTD, DDRD, PIND, D2, 2);
    define_pin!(PORTD, DDRD, PIND, D3, 3);
    define_pin!(PORTD, DDRD, PIND, D4, 4);
    define_pin!(PORTD, DDRD, PIND, D5, 5);
    define_pin!(PORTD, DDRD, PIND, D6, 6);
    define_pin!(PORTD, DDRD, PIND, D7, 7);
    define_pin!(PORTC, DDRC, PINC, C6, 6);
    define_pin!(PORTC, DDRC, PINC, C7, 7);
}
