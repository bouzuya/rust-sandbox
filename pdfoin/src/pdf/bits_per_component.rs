#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum BitsPerComponent {
    One = 1,
    Two = 2,
    Four = 4,
    Eight = 8,
    Sixteen = 16,
}

impl BitsPerComponent {
    pub fn from_u8(n: u8) -> Option<BitsPerComponent> {
        match n {
            1 => Some(BitsPerComponent::One),
            2 => Some(BitsPerComponent::Two),
            4 => Some(BitsPerComponent::Four),
            8 => Some(BitsPerComponent::Eight),
            16 => Some(BitsPerComponent::Sixteen),
            _ => None,
        }
    }

    pub fn to_u8(&self) -> u8 {
        *self as u8
    }

    pub(crate) fn to_lopdf_object_integer(&self) -> ::lopdf::Object {
        ::lopdf::Object::Integer(i64::from(self.to_u8()))
    }
}
