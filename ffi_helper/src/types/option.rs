use crate::TypeInfo;

#[repr(u8)]
#[derive(TypeInfo, Debug, PartialEq, Clone)]
pub enum SOption<T> {
    Some(T),
    None,
}

impl<T> SOption<T> {
    pub fn from_option(value: Option<T>) -> Self {
        match value {
            Some(v) => Self::Some(v),
            None => Self::None,
        }
    }
    pub fn into_option(self) -> Option<T> {
        match self {
            Self::Some(v) => Some(v),
            Self::None => None,
        }
    }
}

impl<T> From<Option<T>> for SOption<T> {
    fn from(value: Option<T>) -> Self {
        Self::from_option(value)
    }
}

impl<T> From<SOption<T>> for Option<T> {
    fn from(value: SOption<T>) -> Self {
        value.into_option()
    }
}
