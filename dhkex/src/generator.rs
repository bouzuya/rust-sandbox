use num_bigint::BigUint;

#[derive(Clone, Eq, PartialEq)]
pub struct Generator(BigUint);

impl Generator {
    pub fn from_bytes_be(bytes: &[u8]) -> Self {
        Self(BigUint::from_bytes_be(bytes))
    }

    pub fn to_bytes_be(&self) -> Vec<u8> {
        self.0.to_bytes_be()
    }

    pub(crate) fn as_big_uint(&self) -> &BigUint {
        &self.0
    }
}

impl std::fmt::Debug for Generator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Generator")
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
        assert_eq!(Generator::from_bytes_be(bytes).to_bytes_be(), bytes);

        let bytes = &[0x01, 0x02];
        assert_eq!(Generator::from_bytes_be(bytes).to_bytes_be(), bytes);
    }

    #[test]
    fn big_uint_conversion_test() {
        let big_uint = 0.to_biguint().unwrap();
        // no from_big_uint
        let g = Generator::from_bytes_be(&big_uint.to_bytes_be());
        assert_eq!(g.as_big_uint(), &big_uint);
    }

    #[test]
    fn debug_test() {
        let g = Generator::from_bytes_be(&[0x01, 0x02]);
        assert_eq!(format!("{:?}", g), "Generator([1, 2])");
    }
}
