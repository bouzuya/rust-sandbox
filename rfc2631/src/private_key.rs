use num_bigint::BigUint;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrivateKey(BigUint);

impl PrivateKey {
    pub fn from_bytes_be(bytes: &[u8]) -> Self {
        Self(BigUint::from_bytes_be(bytes))
    }

    pub(crate) fn from_big_uint(big_uint: BigUint) -> Self {
        Self(big_uint)
    }

    pub fn to_bytes_be(&self) -> Vec<u8> {
        self.0.to_bytes_be()
    }

    pub(crate) fn as_big_uint(&self) -> &BigUint {
        &self.0
    }
}
