#[test]
fn test_decode() -> Result<(), base64::Error> {
    use base64::decode;

    // <https://datatracker.ietf.org/doc/html/rfc4648#section-9>
    assert_eq!(
        decode("FPucA9l+")?,
        vec![0x14, 0xfb, 0x9c, 0x03, 0xd9, 0x7e]
    );
    assert_eq!(decode("FPucA9k=")?, vec![0x14, 0xfb, 0x9c, 0x03, 0xd9]);
    assert_eq!(decode("FPucAw==")?, vec![0x14, 0xfb, 0x9c, 0x03]);

    // <https://datatracker.ietf.org/doc/html/rfc4648#section-10>
    assert_eq!(decode("")?, vec![]);
    assert_eq!(decode("Zg==")?, "f".as_bytes().to_vec());
    assert_eq!(decode("Zm8=")?, "fo".as_bytes().to_vec());
    assert_eq!(decode("Zm9v")?, "foo".as_bytes().to_vec());
    assert_eq!(decode("Zm9vYg==")?, "foob".as_bytes().to_vec());
    assert_eq!(decode("Zm9vYmE=")?, "fooba".as_bytes().to_vec());
    assert_eq!(decode("Zm9vYmFy")?, "foobar".as_bytes().to_vec());

    assert_eq!(decode("1").unwrap_err().to_string(), "invalid length: 1");
    assert_eq!(decode("12").unwrap_err().to_string(), "invalid length: 2");
    assert_eq!(decode("123").unwrap_err().to_string(), "invalid length: 3");

    assert_eq!(
        decode("123!").unwrap_err().to_string(),
        "invalid character: !"
    );

    fn assert_fn<T: std::error::Error + Send + Sync>() {}
    assert_fn::<base64::Error>();

    Ok(())
}

#[test]
fn test_encode() {
    use base64::encode;

    // <https://datatracker.ietf.org/doc/html/rfc4648#section-9>
    assert_eq!(encode([0x14, 0xfb, 0x9c, 0x03, 0xd9, 0x7e]), "FPucA9l+");
    assert_eq!(encode([0x14, 0xfb, 0x9c, 0x03, 0xd9]), "FPucA9k=");
    assert_eq!(encode([0x14, 0xfb, 0x9c, 0x03]), "FPucAw==");

    // <https://datatracker.ietf.org/doc/html/rfc4648#section-10>
    assert_eq!(encode(""), "");
    assert_eq!(encode("f"), "Zg==");
    assert_eq!(encode("fo"), "Zm8=");
    assert_eq!(encode("foo"), "Zm9v");
    assert_eq!(encode("foob"), "Zm9vYg==");
    assert_eq!(encode("fooba"), "Zm9vYmE=");
    assert_eq!(encode("foobar"), "Zm9vYmFy");
}
