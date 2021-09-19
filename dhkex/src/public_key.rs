use num_bigint::BigUint;

#[derive(Clone, Eq, PartialEq)]
pub struct PublicKey(BigUint);

impl PublicKey {
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

impl std::fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("PublicKey")
            .field(&self.0.to_bytes_be())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::ToBigUint;

    use super::*;

    #[test]
    fn bytes_be_conversion_test() {
        let bytes = &[0x00];
        assert_eq!(PublicKey::from_bytes_be(bytes).to_bytes_be(), bytes);

        let bytes = &[0x01, 0x02];
        assert_eq!(PublicKey::from_bytes_be(bytes).to_bytes_be(), bytes);
    }

    #[test]
    fn big_uint_conversion_test() {
        let big_uint = 0.to_biguint().unwrap();
        assert_eq!(
            PublicKey::from_big_uint(big_uint.clone()).as_big_uint(),
            &big_uint
        );
    }

    #[test]
    fn debug_test() {
        let y = PublicKey::from_bytes_be(&[0x01, 0x02]);
        assert_eq!(format!("{:?}", y), "PublicKey([1, 2])");
    }
}
