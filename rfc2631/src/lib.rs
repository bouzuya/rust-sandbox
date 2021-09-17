// <https://datatracker.ietf.org/doc/html/rfc2631>
use num_bigint::{BigUint, RandBigInt, ToBigUint};

#[derive(Debug, Eq, PartialEq)]
pub struct Generator(BigUint);

#[derive(Debug, Eq, PartialEq)]
pub struct Modulus(BigUint);

#[derive(Debug, Eq, PartialEq)]
pub struct PublicKey(BigUint);

#[derive(Debug, Eq, PartialEq)]
pub struct PrivateKey(BigUint);

#[derive(Debug, Eq, PartialEq)]
pub struct SharedSecret(BigUint);

#[derive(Debug, Eq, PartialEq)]
pub struct Group {
    g: Generator,
    p: Modulus,
    // q: BigUint,
}

impl Group {
    pub fn new() -> Self {
        let g = Generator(BigUint::parse_bytes(b"2", 16).unwrap());

        let p = Modulus(
            BigUint::parse_bytes(
                concat!(
                    "DCF93A0B883972EC0E19989AC5A2CE310E1D37717E8D9571BB7623731866E61E",
                    "F75A2E27898B057F9891C2E27A639C3F29B60814581CD3B2CA3986D268370557",
                    "7D45C2E7E52DC81C7A171876E5CEA74B1448BFDFAF18828EFD2519F14E45E382",
                    "6634AF1949E5B535CC829A483B8A76223E5D490A257F05BDFF16F2FB22C583AB"
                )
                .as_bytes(),
                16,
            )
            .unwrap(),
        );

        Self { g, p }
    }

    // g: generator
    pub fn g(&self) -> &Generator {
        &self.g
    }

    // p: large prime (modulus)
    pub fn p(&self) -> &Modulus {
        &self.p
    }

    // // large prime
    // pub fn q(&self) -> &BigUint {
    //     &self.q
    // }

    pub fn generate_key_pair(&self) -> KeyPair {
        let mut rng = rand::thread_rng();
        let low = 1.to_biguint().unwrap();
        let x = PrivateKey(rng.gen_biguint_range(&low, &self.p.0));
        let y = PublicKey(self.g.0.modpow(&x.0, &self.p.0));
        KeyPair { group: self, x, y }
    }
}

pub struct KeyPair<'a> {
    group: &'a Group,
    x: PrivateKey,
    y: PublicKey,
}

impl KeyPair<'_> {
    pub fn private_key(&self) -> &PrivateKey {
        &self.x
    }

    pub fn public_key(&self) -> &PublicKey {
        &self.y
    }

    pub fn shared_secret(&self, y: &PublicKey) -> SharedSecret {
        SharedSecret(y.0.modpow(&self.x.0, &self.group.p.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let group = Group::new();
        let a = group.generate_key_pair();
        let b = group.generate_key_pair();
        assert_eq!(
            a.shared_secret(b.public_key()),
            b.shared_secret(a.public_key())
        );
    }
}