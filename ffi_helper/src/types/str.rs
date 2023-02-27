use std::fmt::Debug;

use super::SSlice;

#[repr(C)]
#[derive(Clone)]
pub struct SStr<'a> {
    inner: SSlice<'a, u8>,
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
    pub const fn as_str<'b>(&'b self) -> &'b str
    where
        'a: 'b,
    {
        unsafe { std::str::from_utf8_unchecked(self.inner.as_slice()) }
    }
}

impl<'a> PartialEq for SStr<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl<'a> Debug for SStr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
