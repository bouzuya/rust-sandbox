// <https://datatracker.ietf.org/doc/html/rfc2631>
use num_bigint::{BigUint, RandBigInt, ToBigUint};

pub struct T {
    g: BigUint,
    p: BigUint,
    // q: BigUint,
    x: BigUint,
    y: BigUint,
}

impl T {
    pub fn new() -> Self {
        let g = BigUint::parse_bytes(b"2", 16).unwrap();
        let p = BigUint::parse_bytes(
            concat!(
                "DCF93A0B883972EC0E19989AC5A2CE310E1D37717E8D9571BB7623731866E61E",
                "F75A2E27898B057F9891C2E27A639C3F29B60814581CD3B2CA3986D268370557",
                "7D45C2E7E52DC81C7A171876E5CEA74B1448BFDFAF18828EFD2519F14E45E382",
                "6634AF1949E5B535CC829A483B8A76223E5D490A257F05BDFF16F2FB22C583AB"
            )
            .as_bytes(),
            16,
        )
        .unwrap();

        let mut rng = rand::thread_rng();
        let low = 1.to_biguint().unwrap();
        let x = rng.gen_biguint_range(&low, &p);

        let y = g.modpow(&x, &p);
        Self { g, p, x, y }
    }

    // generator
    pub fn g(&self) -> &BigUint {
        &self.g
    }

    // large prime (modulus)
    pub fn p(&self) -> &BigUint {
        &self.p
    }

    // // large prime
    // pub fn q(&self) -> &BigUint {
    //     &self.q
    // }

    // private key
    pub fn x(&self) -> &BigUint {
        &self.x
    }

    // public key
    pub fn y(&self) -> &BigUint {
        &self.y
    }

    // shared secret number
    pub fn zz(&self, t: &T) -> BigUint {
        t.y().modpow(&self.x, &self.p)
    }
}
