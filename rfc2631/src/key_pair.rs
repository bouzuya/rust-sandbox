use crate::{Group, PrivateKey, PublicKey, SharedSecret};

pub struct KeyPair<'a> {
    pub(crate) group: &'a Group,
    pub(crate) x: PrivateKey,
    pub(crate) y: PublicKey,
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
