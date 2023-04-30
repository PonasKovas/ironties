use crate::TypeInfo;

use super::FfiSafeEquivalent;

/// FFI-safe equivalent of `(T1, T2)`
#[derive(TypeInfo, Debug, PartialEq, Clone, Copy)]
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
