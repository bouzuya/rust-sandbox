# blink

```console
$ rustup override set nightly
$ cargo build --release
$ avrdude -F -patmega32u4 -cavr109 -P/dev/tty.usbmodem144101 -b57600 -D -Uflash:w:target/atmega32u4/release/blink.elf:e
```

## TODOs

- data-layout が怪しい
- target の json のファイル名が怪しい
- `.cargo/config.toml` について調べる
- [crates:rduino] の trait を利用する
- embedded-hal との差を調べる
