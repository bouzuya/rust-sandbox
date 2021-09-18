use crate::{Group, PrivateKey, PublicKey, SharedSecret};

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
        SharedSecret::from_big_uint(
            y.as_big_uint()
                .modpow(self.x.as_big_uint(), self.group.modulus().as_big_uint()),
        )
    }
}
