# blink

```console
$ # See: `.cargo/config.toml` and `rust-toolchain.toml`
$ cargo build --release
$ avrdude -F -patmega32u4 -cavr109 -P/dev/tty.usbmodem144101 -b57600 -D -Uflash:w:target/atmega32u4/release/blink.elf:e
```

## `.cargo/config.toml`

<https://doc.rust-lang.org/cargo/reference/config.html>

### `env.AVR_CPU_FREQUENCY_HZ`

README.md in [crates:avr-config]

> This crate aims to provide the boilerplate for getting the CPU frequency as an integer at runtime, as well as establishes a convention that $AVR_CPU_FREQUENCY_HZ is used to pass the target frequency to all AVR crates, if they opt-in to it.

### `unstable.build-std = ["core"]`

<https://book.avr-rust.com/003.2-note-about-rust-build-std-flag.html>

> AVR-Rust is not distributed with a pre-built libcore crate. Instead, it is compiled on-demand when a crate uses it via the Rust -Z build-std flag.
>
> There are many hundreds of variants of AVR microcontroller, and it is not feasible to distribute runtime libraries for all of them within a regular Rust distribution.
>
> Due to this, any time a crate is built for AVR, -Z build-std=core should be passed to cargo.

<https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#build-std>

## TODOs

- ☑ [crates:ruduino] の trait を利用する
- ☑ `.cargo/config.toml` について調べる
- data-layout が怪しい
- target の json のファイル名が怪しい
- embedded-hal との差を調べる

[crates:ruduino]: https://crates.io/crates/ruduino
[crates:avr-config]: https://crates.io/crates/avr-config
