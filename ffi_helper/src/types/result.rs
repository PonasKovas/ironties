use super::FfiSafeEquivalent;
use crate::TypeInfo;

/// FFI-safe equivalent of [`Result<T>`]
#[repr(u8)]
#[derive(TypeInfo, Debug, PartialEq)]
pub enum SResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> FfiSafeEquivalent for SResult<T, E> {
    type Normal = Result<T, E>;

    fn from_normal(normal: Self::Normal) -> Self {
        match normal {
            Ok(v) => Self::Ok(v),
            Err(e) => Self::Err(e),
        }
    }
    fn into_normal(self) -> Self::Normal {
        match self {
            SResult::Ok(v) => Ok(v),
            SResult::Err(e) => Err(e),
        }
    }
}

impl<T, E> From<Result<T, E>> for SResult<T, E> {
    fn from(value: Result<T, E>) -> Self {
        Self::from_normal(value)
    }
}

impl<T, E> From<SResult<T, E>> for Result<T, E> {
    fn from(value: SResult<T, E>) -> Self {
        value.into_normal()
    }
}
