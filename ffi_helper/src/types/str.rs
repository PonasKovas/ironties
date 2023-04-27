use super::{SMutSlice, SSlice};
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

impl<'a> SStr<'a> {
    pub const fn from_str(other: &'a str) -> Self {
        Self {
            inner: SSlice::from_slice(other.as_bytes()),
        }
    }
    pub const fn into_str(self) -> &'a str {
        unsafe { std::str::from_utf8_unchecked(self.inner.into_slice()) }
    }
    pub const fn to_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(self.inner.into_slice()) }
    }
    pub fn as_str_mut<R, F>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut &'a str) -> R,
    {
        let mut copy = self.into_str();

        let r = f(&mut copy);

        *self = SStr::from_str(copy);

        r
    }
}

impl<'a> SMutStr<'a> {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(other: &'a mut str) -> Self {
        Self {
            inner: SMutSlice::from_slice(unsafe { other.as_bytes_mut() }),
        }
    }
    pub fn into_str(self) -> &'a mut str {
        unsafe { std::str::from_utf8_unchecked_mut(self.inner.into_slice()) }
    }
    pub fn to_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(self.inner.to_slice()) }
    }
    pub fn to_str_mut(&mut self) -> &mut str {
        unsafe { std::str::from_utf8_unchecked_mut(self.inner.to_slice_mut()) }
    }
    pub fn as_str<R, F>(&self, f: F) -> R
    where
        F: FnOnce(&&'a mut str) -> R,
    {
        let copy = unsafe { std::ptr::read(self) }.into_str();

        f(&copy)
    }
    pub fn as_str_mut<R, F>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut &'a mut str) -> R,
    {
        let mut copy = unsafe { std::ptr::read(self) }.into_str();

        let r = f(&mut copy);

        unsafe { std::ptr::write(self, SMutStr::from_str(copy)) }

        r
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
        Self::from_str("")
    }
}

impl<'a> Default for SMutStr<'a> {
    fn default() -> Self {
        Self::from_str(<&mut str>::default())
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
        Self::from_str(value)
    }
}

impl<'a> From<&'a mut str> for SStr<'a> {
    fn from(value: &'a mut str) -> Self {
        Self::from_str(value)
    }
}

impl<'a> From<SMutStr<'a>> for SStr<'a> {
    fn from(value: SMutStr<'a>) -> Self {
        Self::from_str(value.into_str())
    }
}

impl<'a> From<&'a mut str> for SMutStr<'a> {
    fn from(value: &'a mut str) -> Self {
        Self::from_str(value)
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
