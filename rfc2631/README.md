# RFC 2631 : Diffie-Hellman Key Agreement Method

<https://datatracker.ietf.org/doc/html/rfc2631>

```rust
// <https://en.wikipedia.org/wiki/Diffie%E2%80%93Hellman_key_exchange>
#[test]
fn wikipedia_example() -> anyhow::Result<()> {
    let g = Generator::from_bytes_be(&[5]);
    let p = Modulus::from_bytes_be(&[23]);
    let group = Group::new(g, p);

    let ax = PrivateKey::from_bytes_be(&[6]);
    let bx = PrivateKey::from_bytes_be(&[15]);
    let a = group.create_key_pair_from_private_key(ax)?;
    let b = group.create_key_pair_from_private_key(bx)?;
    assert_eq!(a.private_key(), &PrivateKey::from_bytes_be(&[6]));
    assert_eq!(a.public_key(), &PublicKey::from_bytes_be(&[8]));
    assert_eq!(b.private_key(), &PrivateKey::from_bytes_be(&[15]));
    assert_eq!(b.public_key(), &PublicKey::from_bytes_be(&[19]));

    assert_eq!(
        a.shared_secret(b.public_key()),
        SharedSecret::from_bytes_be(&[2])
    );
    assert_eq!(
        b.shared_secret(a.public_key()),
        SharedSecret::from_bytes_be(&[2])
    );

    Ok(())
}
```
