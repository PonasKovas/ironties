use std::mem::ManuallyDrop;

use crate::TypeInfo;

/// FFI-safe equivalent of [`Result<T>`]
#[repr(u8)]
#[derive(TypeInfo, Debug, PartialEq)]
pub enum SResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> SResult<T, E> {
    pub fn from_result(value: Result<T, E>) -> Self {
        match value {
            Ok(v) => Self::Ok(v),
            Err(e) => Self::Err(e),
        }
    }
    pub fn into_result(self) -> Result<T, E> {
        match self {
            SResult::Ok(v) => Ok(v),
            SResult::Err(e) => Err(e),
        }
    }
    pub fn as_result<R, F>(&self, f: F) -> R
    where
        F: FnOnce(&Result<T, E>) -> R,
    {
        let copy = ManuallyDrop::new(unsafe { std::ptr::read(self) }.into_result());

        f(&*copy)
    }
    pub fn as_result_mut<R, F>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut Result<T, E>) -> R,
    {
        let mut copy = unsafe { std::ptr::read(self) }.into_result();

        let r = f(&mut copy);

        unsafe { std::ptr::write(self, SResult::from_result(copy)) }

        r
    }
}

impl<T, E> From<Result<T, E>> for SResult<T, E> {
    fn from(value: Result<T, E>) -> Self {
        Self::from_result(value)
    }
}

impl<T, E> From<SResult<T, E>> for Result<T, E> {
    fn from(value: SResult<T, E>) -> Self {
        value.into_result()
    }
}
