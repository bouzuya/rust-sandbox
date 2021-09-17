use num_bigint::{RandBigInt, ToBigUint};

use crate::{Generator, KeyPair, Modulus, PrivateKey, PublicKey};

#[derive(Debug, Eq, PartialEq)]
pub struct Group {
    pub(crate) g: Generator,
    pub(crate) p: Modulus,
    // q: BigUint,
}

impl Group {
    pub fn new(g: Generator, p: Modulus) -> Self {
        Self { g, p }
    }

    pub fn generator(&self) -> &Generator {
        &self.g
    }

    pub fn modulus(&self) -> &Modulus {
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
