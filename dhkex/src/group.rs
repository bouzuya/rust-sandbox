use num_bigint::{RandBigInt, ToBigUint};
use thiserror::Error;

use crate::{Generator, KeyPair, Modulus, PrivateKey, PublicKey};

#[derive(Debug, Error)]
pub enum CreateKeyPairError {
    #[error("out of range")]
    OutOfRange,
}

/// A finite cyclic group
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Group {
    /// generator. primitive root modulo p.
    g: Generator,
    /// prime
    p: Modulus,
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

    pub fn generate_key_pair(&self) -> KeyPair {
        let mut rng = rand::thread_rng();
        let low = &1.to_biguint().unwrap();
        let high = self.p.as_big_uint();
        let x = PrivateKey::from_big_uint(rng.gen_biguint_range(low, high));
        let y = self.create_public_key_from_private_key(&x);
        KeyPair::internal_new(self, x, y)
    }

    pub fn create_key_pair_from_private_key(
        &self,
        private_key: PrivateKey,
    ) -> Result<KeyPair, CreateKeyPairError> {
        let low = &1.to_biguint().unwrap();
        let high = self.p.as_big_uint();
        if !(low..high).contains(&private_key.as_big_uint()) {
            return Err(CreateKeyPairError::OutOfRange);
        }

        let x = private_key;
        let y = self.create_public_key_from_private_key(&x);
        Ok(KeyPair::internal_new(self, x, y))
    }

    fn create_public_key_from_private_key(&self, private_key: &PrivateKey) -> PublicKey {
        // y = (g ^ x) mod p
        let g = self.g.as_big_uint();
        let x = private_key.as_big_uint();
        let p = self.p.as_big_uint();
        PublicKey::from_big_uint(g.modpow(x, p))
    }
}
