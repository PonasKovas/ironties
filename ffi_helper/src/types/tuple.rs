use crate::TypeInfo;

/// FFI-safe equivalent of `(T1, T2)`
#[derive(TypeInfo, Debug, PartialEq, Clone, Copy)]
#[repr(C)]
pub struct STuple2<T1, T2>(pub T1, pub T2);
