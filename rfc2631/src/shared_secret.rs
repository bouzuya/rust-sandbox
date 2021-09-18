use num_bigint::BigUint;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SharedSecret(BigUint);

impl SharedSecret {
    pub fn from_bytes_be(bytes: &[u8]) -> Self {
        Self(BigUint::from_bytes_be(bytes))
    }

    pub(crate) fn from_big_uint(big_uint: BigUint) -> Self {
        Self(big_uint)
    }

    pub fn to_bytes_be(&self) -> Vec<u8> {
        self.0.to_bytes_be()
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::ToBigUint;

    use super::*;

    #[test]
    fn bytes_be_conversion_test() {
        let bytes = &[0x00];
        assert_eq!(SharedSecret::from_bytes_be(bytes).to_bytes_be(), bytes);

        let bytes = &[0x01, 0x02];
        assert_eq!(SharedSecret::from_bytes_be(bytes).to_bytes_be(), bytes);
    }

    #[test]
    fn big_uint_conversion_test() {
        let big_uint = 0.to_biguint().unwrap();
        // no as_big_uint
        assert_eq!(
            SharedSecret::from_big_uint(big_uint.clone()),
            SharedSecret::from_bytes_be(&big_uint.to_bytes_be())
        );
    }
}
