# base64

This project is a Rust implementation of Base64 encoding and decoding as specified in [RFC 4648](https://datatracker.ietf.org/doc/html/rfc4648).

## Usage

```rust
fn main() -> Result<(), base64::Error> {
    assert_eq!(base64::encode([0x14, 0xfb, 0x9c, 0x03]), "FPucAw==");
    assert_eq!(base64::encode("fooba"), "Zm9vYmE=");

    assert_eq!(base64::decode("FPucAw==")?, vec![0x14, 0xfb, 0x9c, 0x03]);
    assert_eq!(base64::decode("Zm9vYmE=")?, "fooba".as_bytes().to_vec());

    Ok(())
}
```
