# kbd_input_pullup

<https://www.arduino.cc/en/Tutorial/BuiltInExamples/InputPullupSerial> の Circit / Schematic を Arduino Micro でつくり、
`pinMode(_, INPUT_PULLUP)` を試す。

`INPUT_PULLUP` は ATmega32U4 の Data Sheet (<https://cdn.sparkfun.com/datasheets/Dev/Arduino/Boards/ATMega32U4.pdf>) の 10.2.3 Switching Between Input and Output によると、次のようにすると良い。

- DDxn を 0 (`ruduino::Pin::set_input`)
- PORTxn を 1 (`ruduiono::Pin::set_high`)
- PUD を 0 (おそらく既定値で 0)

`INPUT_PULLUP` は ATMega32U4 内部のプルアップ抵抗を使うものらしい (怪しい知識) 。有効にすると HIGH と LOW が反転するようだ (怪しい知識) 。外部に抵抗を用意しなくて済みそう (怪しい知識) なので良い。

参考

- <https://docs.arduino.cc/hardware/micro>
- <https://www.arduino.cc/en/Tutorial/BuiltInExamples/InputPullupSerial>
- <https://cdn.sparkfun.com/datasheets/Dev/Arduino/Boards/ATMega32U4.pdf>
- <https://mag.switch-science.com/2013/05/23/input_pullup/>
