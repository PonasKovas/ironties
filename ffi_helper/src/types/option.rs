use super::FfiSafeEquivalent;
use crate::TypeInfo;

/// FFI-safe equivalent of [`Option<T>`]
#[repr(u8)]
#[derive(TypeInfo, Debug, PartialEq, Clone)]
pub enum SOption<T> {
    Some(T),
    None,
}

impl<T> FfiSafeEquivalent for SOption<T> {
    type Normal = Option<T>;

    fn from_normal(normal: Self::Normal) -> Self {
        match normal {
            Some(v) => Self::Some(v),
            None => Self::None,
        }
    }
    fn into_normal(self) -> Self::Normal {
        match self {
            Self::Some(v) => Some(v),
            Self::None => None,
        }
    }
}

impl<T> From<Option<T>> for SOption<T> {
    fn from(value: Option<T>) -> Self {
        Self::from_normal(value)
    }
}

impl<T> From<SOption<T>> for Option<T> {
    fn from(value: SOption<T>) -> Self {
        value.into_normal()
    }
}
