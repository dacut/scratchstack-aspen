/// The variant on an operation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Variant {
    None = 0,
    IfExists = 1,
    Negated = 2,
    IfExistsNegated = 3,
}

impl Variant {
    #[inline]
    pub(super) fn as_usize(self) -> usize {
        self as usize
    }

    #[inline]
    pub(super) fn if_exists(self) -> bool {
        matches!(self, Self::IfExists | Self::IfExistsNegated)
    }

    #[inline]
    pub(super) fn negated(self) -> bool {
        matches!(self, Self::Negated | Self::IfExistsNegated)
    }
}

impl From<u8> for Variant {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::IfExists,
            2 => Self::Negated,
            3 => Self::IfExistsNegated,
            _ => panic!("Invalid variant value: {}", value),
        }
    }
}
