# dhkex

This crate is a helper for DHKEX : Diffie-Hellman Key EXchange.

For more information about DHKEX, please see the following.

- RFC 2631 Diffie-Hellman Key Agreement Method <https://tools.ietf.org/html/rfc2631>
- Diffie–Hellman key exchange - Wikipedia <https://en.wikipedia.org/wiki/Diffie%E2%80%93Hellman_key_exchange>

## Example

```rust
// Wikipedia Example
// <https://en.wikipedia.org/wiki/Diffie%E2%80%93Hellman_key_exchange>
use dhkex::{Generator, Group, Modulus, PrivateKey, PublicKey, SharedSecret};

let g = Generator::from_bytes_be(&[5]);
let p = Modulus::from_bytes_be(&[23]);
let group = Group::new(g, p);

let ax = PrivateKey::from_bytes_be(&[6]);
let bx = PrivateKey::from_bytes_be(&[15]);
let a = group.create_key_pair_from_private_key(ax).unwrap();
let b = group.create_key_pair_from_private_key(bx).unwrap();
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
```

```rust
// OpenID Authentication 2.0 Example
// <https://openid.net/specs/openid-authentication-2_0.html>
// add `hex = "0.4.3"` to `Cargo.toml` dependencies
use dhkex::{Generator, Group, Modulus, PrivateKey, PublicKey, SharedSecret};
use hex::decode;

let g_bytes = hex::decode("02").unwrap();
let p_bytes = hex::decode(concat!(
    "DCF93A0B883972EC0E19989AC5A2CE310E1D37717E8D9571BB7623731866E61E",
    "F75A2E27898B057F9891C2E27A639C3F29B60814581CD3B2CA3986D268370557",
    "7D45C2E7E52DC81C7A171876E5CEA74B1448BFDFAF18828EFD2519F14E45E382",
    "6634AF1949E5B535CC829A483B8A76223E5D490A257F05BDFF16F2FB22C583AB"
)).unwrap();
let g = Generator::from_bytes_be(&g_bytes);
let p = Modulus::from_bytes_be(&p_bytes);
let group = Group::new(g, p);

let a = group.generate_key_pair();
let b = group.generate_key_pair();
assert_ne!(a, b);
assert_eq!(
    a.shared_secret(b.public_key()),
    b.shared_secret(a.public_key())
);

let a2 = group.create_key_pair_from_private_key(a.private_key().clone()).unwrap();
assert_eq!(a, a2);
```
