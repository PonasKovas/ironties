use super::{FfiSafeEquivalent, SMutSlice, SSlice};
use crate::TypeInfo;
use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut, Index, IndexMut},
};

/// FFI-safe equivalent of [`&'a str`][str]
#[repr(C)]
#[derive(TypeInfo, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SStr<'a> {
    inner: SSlice<'a, u8>,
}

/// FFI-safe equivalent of [`&'a mut str`][str]
#[repr(C)]
#[derive(TypeInfo)]
pub struct SMutStr<'a> {
    inner: SMutSlice<'a, u8>,
}

impl<'a> FfiSafeEquivalent for SStr<'a> {
    type Normal = &'a str;

    fn from_normal(normal: Self::Normal) -> Self {
        Self {
            inner: SSlice::from_normal(normal.as_bytes()),
        }
    }
    fn into_normal(self) -> Self::Normal {
        unsafe { std::str::from_utf8_unchecked(self.inner.into_normal()) }
    }
}

impl<'a> SStr<'a> {
    pub fn to_str<'b>(&'b self) -> &'b str {
        unsafe { std::str::from_utf8_unchecked(self.inner.into_normal()) }
    }
}

impl<'a> FfiSafeEquivalent for SMutStr<'a> {
    type Normal = &'a mut str;

    fn from_normal(normal: Self::Normal) -> Self {
        Self {
            inner: SMutSlice::from_normal(unsafe { normal.as_bytes_mut() }),
        }
    }
    fn into_normal(self) -> Self::Normal {
        unsafe { std::str::from_utf8_unchecked_mut(self.inner.into_normal()) }
    }
}

impl<'a> SMutStr<'a> {
    pub fn to_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(self.inner.to_slice()) }
    }
    pub fn to_str_mut(&mut self) -> &mut str {
        unsafe { std::str::from_utf8_unchecked_mut(self.inner.to_slice_mut()) }
    }
}

impl<'a> Debug for SStr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.to_str(), f)
    }
}

impl<'a> Debug for SMutStr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.to_str(), f)
    }
}

impl<'a> Default for SStr<'a> {
    fn default() -> Self {
        Self::from_normal("")
    }
}

impl<'a> Default for SMutStr<'a> {
    fn default() -> Self {
        Self::from_normal(<&mut str>::default())
    }
}

impl<'a> Deref for SStr<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.to_str()
    }
}

impl<'a> Deref for SMutStr<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.to_str()
    }
}

impl<'a> DerefMut for SMutStr<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.to_str_mut()
    }
}

impl<'a> Display for SStr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.to_str(), f)
    }
}

impl<'a> Display for SMutStr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.to_str(), f)
    }
}

impl<'a> From<&'a str> for SStr<'a> {
    fn from(value: &'a str) -> Self {
        Self::from_normal(value)
    }
}

impl<'a> From<&'a mut str> for SStr<'a> {
    fn from(value: &'a mut str) -> Self {
        Self::from_normal(value)
    }
}

impl<'a> From<SMutStr<'a>> for SStr<'a> {
    fn from(value: SMutStr<'a>) -> Self {
        Self::from_normal(value.into_normal())
    }
}

impl<'a> From<&'a mut str> for SMutStr<'a> {
    fn from(value: &'a mut str) -> Self {
        Self::from_normal(value)
    }
}

impl<'a, I: std::slice::SliceIndex<str>> Index<I> for SStr<'a> {
    type Output = <I as std::slice::SliceIndex<str>>::Output;

    fn index(&self, index: I) -> &Self::Output {
        self.to_str().index(index)
    }
}

impl<'a, I: std::slice::SliceIndex<str>> Index<I> for SMutStr<'a> {
    type Output = <I as std::slice::SliceIndex<str>>::Output;

    fn index(&self, index: I) -> &Self::Output {
        self.to_str().index(index)
    }
}

impl<'a, I: std::slice::SliceIndex<str>> IndexMut<I> for SMutStr<'a> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.to_str_mut().index_mut(index)
    }
}
