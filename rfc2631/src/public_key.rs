use num_bigint::BigUint;

#[derive(Debug, Eq, PartialEq)]
pub struct PublicKey(pub(crate) BigUint);

impl PublicKey {
    pub fn from_bytes_be(bytes: &[u8]) -> Self {
        Self(BigUint::from_bytes_be(bytes))
    }

    pub fn to_bytes_be(&self) -> Vec<u8> {
        self.0.to_bytes_be()
    }
}
