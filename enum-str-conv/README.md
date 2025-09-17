# enum-str-conv

A Rust library that provides derive macro to convert between enum and str (`Display`/`FromStr`).

## Usage

1. Add `#[derive(enum_str_conv::EnumStrConv)]` to your enum
2. Add `#[enum_str_conv(error = ErrorType, unknown = unknown_fn)]` to your enum
3. Add `#[enum_str_conv(str = "...")]` to your enum variants

This code generates the following code:

```rust
#[derive(enum_str_conv::EnumStrConv)]
#[enum_str_conv(error = MyError, unknown = MyError::Unknown)]
enum MyEnum {
	#[enum_str_conv(str = "apple")]
	A,
	#[enum_str_conv(str = "banana")]
	B,
	#[enum_str_conv(str = "cherry")]
	C,
}
```

```rust
#[automatically_derived]
impl ::std::fmt::Display for MyEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
			Self::A => write!(f, "apple"),
			Self::B => write!(f, "banana"),
			Self::C => write!(f, "cherry"),
        }
    }
}

#[automatically_derived]
impl ::std::str::FromStr for MyEnum {
    type Err = MyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
			"apple" => Ok(Self::A),
			"banana" => Ok(Self::B),
			"cherry" => Ok(Self::C),
            _ => Err(MyError::Unknown(s.to_owned())),
        }
    }
}
```

## Examples

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
