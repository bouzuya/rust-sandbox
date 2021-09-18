use crate::{Group, PrivateKey, PublicKey, SharedSecret};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyPair<'a> {
    group: &'a Group,
    x: PrivateKey,
    y: PublicKey,
}

impl KeyPair<'_> {
    pub(crate) fn internal_new<'a>(group: &'a Group, x: PrivateKey, y: PublicKey) -> KeyPair<'a> {
        KeyPair::<'a> { group, x, y }
    }

    pub fn private_key(&self) -> &PrivateKey {
        &self.x
    }

    pub fn public_key(&self) -> &PublicKey {
        &self.y
    }

    pub fn shared_secret(&self, y: &PublicKey) -> SharedSecret {
        // ZZ = g ^ (xb * xa) mod p
        // ZZ = (yb ^ xa) mod p  = (ya ^ xb) mod p
        let y = y.as_big_uint();
        let x = self.x.as_big_uint();
        let p = self.group.modulus().as_big_uint();
        SharedSecret::from_big_uint(y.modpow(x, p))
    }
}
