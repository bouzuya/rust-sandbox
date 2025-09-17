# enum-str-conv

`enum-str-conv` is a Rust procedural macro to automatically implement string conversions (`FromStr`/`ToString`) for enums.

## Overview

This crate allows you to easily implement conversions between enums and strings by adding `#[derive(enum_str_conv::EnumStrConv)]` to your enum and specifying `#[enum_str_conv(str = "...")]` for each variant.

You can also customize the error type and how unknown values are handled.

## Usage

```rust
#[derive(Debug)]
enum MyError {
	Unknown(String),
}
impl std::fmt::Display for MyError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			MyError::Unknown(s) => write!(f, "unknown variant: {0}", s),
		}
	}
}
impl std::error::Error for MyError {}

#[derive(Debug, PartialEq, enum_str_conv::EnumStrConv)]
#[enum_str_conv(error = MyError, unknown = MyError::Unknown)]
enum MyEnum {
	#[enum_str_conv(str = "apple")]
	A,
	#[enum_str_conv(str = "banana")]
	B,
	#[enum_str_conv(str = "cherry")]
	C,
}

use std::str::FromStr;
assert_eq!(MyEnum::from_str("apple").unwrap(), MyEnum::A);
assert_eq!(MyEnum::A.to_string(), "apple");
assert_eq!(MyEnum::from_str("durian").unwrap_err().to_string(), "unknown variant: durian");
```

```rust
// You can also use a non-Error type (e.g., String) as the error type:
fn unknown_fn(s: String) -> String {
	format!("unknown: {s}")
}

#[derive(Debug, PartialEq, enum_str_conv::EnumStrConv)]
#[enum_str_conv(error = String, unknown = unknown_fn)]
enum MyEnum2 {
	#[enum_str_conv(str = "apple")]
	A,
	#[enum_str_conv(str = "banana")]
	B,
	#[enum_str_conv(str = "cherry")]
	C,
}

use std::str::FromStr;
assert_eq!(MyEnum2::from_str("apple").unwrap(), MyEnum2::A);
assert_eq!(MyEnum2::A.to_string(), "apple");
assert_eq!(MyEnum2::from_str("durian").unwrap_err(), "unknown: durian");
```

## Attributes

Attribute arguments:

- On the enum (container):
	- `#[enum_str_conv(error = ...)]`: Specifies the error type.
	- `#[enum_str_conv(unknown = ...)]`: Specifies the handler for unknown values.
- On each variant (field):
	- `#[enum_str_conv(str = "...")]`: Specifies the string for the variant.


## Prior Art

This crate was inspired by the following prior art:

- [strum](https://crates.io/crates/strum): A feature-rich crate that provides custom derive macros for enums, such as `EnumString` and `ToString`, but comes with many dependencies.
- [parse-display](https://crates.io/crates/parse-display): A simple derive macro for parsing and displaying enums using format strings, but does not allow specifying a custom error type.

## License

This project is licensed under either of

- MIT license
- Apache License, Version 2.0

at your option.
