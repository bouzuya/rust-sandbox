use num_bigint::{RandBigInt, ToBigUint};
use thiserror::Error;

use crate::{Generator, KeyPair, Modulus, PrivateKey, PublicKey};

#[derive(Debug, Error)]
pub enum CreateKeyPairError {
    #[error("out of range")]
    OutOfRange,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Group {
    g: Generator,
    p: Modulus,
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
        let x = PrivateKey::from_big_uint(rng.gen_biguint_range(&low, self.p.as_big_uint()));
        let y = PublicKey::from_big_uint(
            self.g
                .as_big_uint()
                .modpow(x.as_big_uint(), self.p.as_big_uint()),
        );
        KeyPair::internal_new(self, x, y)
    }

    pub fn create_key_pair_from_private_key(
        &self,
        private_key: PrivateKey,
    ) -> Result<KeyPair, CreateKeyPairError> {
        let low = 1.to_biguint().unwrap();
        let range = &low..self.p.as_big_uint();
        if !range.contains(&private_key.as_big_uint()) {
            return Err(CreateKeyPairError::OutOfRange);
        }

        let x = private_key;
        let y = PublicKey::from_big_uint(
            self.g
                .as_big_uint()
                .modpow(x.as_big_uint(), self.p.as_big_uint()),
        );
        Ok(KeyPair::internal_new(self, x, y))
    }
}
