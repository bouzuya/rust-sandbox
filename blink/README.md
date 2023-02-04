# blink

```console
$ # See: `.cargo/config.toml` and `rust-toolchain.toml`
$ cargo build --release
$ avrdude -F -patmega32u4 -cavr109 -P/dev/tty.usbmodem144101 -b57600 -D -Uflash:w:target/atmega32u4/release/blink.elf:e
```

## TODOs

- ☑ [crates:rduino] の trait を利用する
- data-layout が怪しい
- target の json のファイル名が怪しい
- `.cargo/config.toml` について調べる
- embedded-hal との差を調べる
