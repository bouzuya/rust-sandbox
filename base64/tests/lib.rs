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
