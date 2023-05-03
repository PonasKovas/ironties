use crate::TypeInfo;
use std::fmt::Debug;

use super::FfiSafeEquivalent;

/// FFI-safe equivalent of [`()`][unit]
#[derive(TypeInfo, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
#[repr(C)]
pub struct SUnit {
    _private: [u8; 0],
}

impl SUnit {
    pub const fn new() -> Self {
        Self { _private: [] }
    }
}

impl Debug for SUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "()")
    }
}

impl FfiSafeEquivalent for SUnit {
    type Normal = ();

    fn from_normal(_: Self::Normal) -> Self {
        Self::new()
    }
    fn into_normal(self) -> Self::Normal {
        ()
    }
}

/// FFI-safe equivalent of `(T1, T2)`
#[derive(TypeInfo, Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct STuple2<T1, T2>(pub T1, pub T2);

impl<T1, T2> FfiSafeEquivalent for STuple2<T1, T2> {
    type Normal = (T1, T2);

    fn from_normal(normal: Self::Normal) -> Self {
        Self(normal.0, normal.1)
    }
    fn into_normal(self) -> Self::Normal {
        (self.0, self.1)
    }
}
