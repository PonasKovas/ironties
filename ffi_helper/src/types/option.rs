use crate::TypeInfo;
use std::mem::ManuallyDrop;

/// FFI-safe equivalent of [`Option<T>`]
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
    pub fn as_option<R, F>(&self, f: F) -> R
    where
        F: FnOnce(&Option<T>) -> R,
    {
        let copy = ManuallyDrop::new(unsafe { std::ptr::read(self) }.into_option());

        f(&*copy)
    }
    pub fn as_option_mut<R, F>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut Option<T>) -> R,
    {
        let mut copy = unsafe { std::ptr::read(self) }.into_option();

        let r = f(&mut copy);

        unsafe { std::ptr::write(self, SOption::from_option(copy)) }

        r
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
