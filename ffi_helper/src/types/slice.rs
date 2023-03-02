use crate::TypeInfo;
use std::{
    fmt::Debug,
    io::{BufRead, Read, Write},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

/// FFI-safe equivalent of [`&'a [T]`][slice]
#[repr(C)]
#[derive(TypeInfo)]
pub struct SSlice<'a, T: 'a> {
    ptr: *const T,
    len: usize,
    _phantom: PhantomData<&'a [T]>,
}

/// FFI-safe equivalent of [`&'a mut [T]`][slice]
#[repr(C)]
#[derive(TypeInfo)]
pub struct SMutSlice<'a, T: 'a> {
    ptr: *mut T,
    len: usize,
    _phantom: PhantomData<&'a mut [T]>,
}

fn as_slice<'b, 'a: 'b, T: 'a, R, F>(s: &'b mut SSlice<'a, T>, f: F) -> R
where
    F: FnOnce(&mut &'a [T]) -> R,
{
    let mut slice: &'a [T] = s.into_slice();
    let r = f(&mut slice);
    *s = SSlice::from_slice(slice);
    r
}

fn as_mut_slice<'b, 'a: 'b, T: 'a, R, F>(s: &'b mut SMutSlice<'a, T>, f: F) -> R
where
    F: FnOnce(&mut &'a mut [T]) -> R,
{
    // SAFETY:
    // This is safe 👍.
    let mut slice: &'a mut [T] = unsafe { std::ptr::read(s) }.into_mut_slice();
    let r = f(&mut slice);
    *s = SMutSlice::from_mut_slice(slice);
    r
}

impl<'a, T> SSlice<'a, T> {
    /// Converts a [`&'a [T]`][slice] to [`SSlice<'a, T>`][SSlice]
    pub const fn from_slice(slice: &'a [T]) -> Self {
        Self {
            ptr: slice.as_ptr(),
            len: slice.len(),
            _phantom: PhantomData,
        }
    }
    /// Converts a [`SSlice<'a, T>`][SSlice] to [`&'a [T]`][slice]
    pub const fn into_slice(self) -> &'a [T] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl<'a, T> SMutSlice<'a, T> {
    /// Converts a [`&'a mut [T]`][slice] to [`SMutSlice<'a, T>`][SMutSlice]
    pub fn from_mut_slice(slice: &'a mut [T]) -> Self {
        Self {
            ptr: slice.as_mut_ptr(),
            len: slice.len(),
            _phantom: PhantomData,
        }
    }
    /// Converts a [`SMutSlice<'a, T>`][SMutSlice] to [`&'a mut [T]`][slice]
    pub fn into_mut_slice(self) -> &'a mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
    /// Converts to a [`&[T]`][slice]
    pub fn as_slice<'b>(&'b self) -> &'b [T]
    where
        'a: 'b,
    {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
    /// Converts to a [`&mut [T]`][slice]
    pub fn as_mut_slice<'b>(&'b mut self) -> &'b mut [T]
    where
        'a: 'b,
    {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

impl<'a, T> Clone for SSlice<'a, T> {
    fn clone(&self) -> Self {
        Self { ..*self }
    }
}

impl<'a, T> Copy for SSlice<'a, T> {}

impl<'a, T> Deref for SSlice<'a, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.into_slice()
    }
}

impl<'a, T: PartialEq> PartialEq for SSlice<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.into_slice() == other.into_slice()
    }
}

impl<'a, T: Eq> Eq for SSlice<'a, T> {}

impl<'a, T: Debug> Debug for SSlice<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.into_slice().fmt(f)
    }
}

impl<'a, T> AsRef<[T]> for SSlice<'a, T> {
    fn as_ref(&self) -> &[T] {
        self.into_slice()
    }
}

impl<'a, T> AsRef<SSlice<'a, T>> for SSlice<'a, T> {
    fn as_ref(&self) -> &SSlice<'a, T> {
        self
    }
}

impl<'a> BufRead for SSlice<'a, u8> {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        Ok(self.into_slice())
    }
    fn consume(&mut self, amt: usize) {
        as_slice(self, move |s| s.consume(amt))
    }
}

impl<'a> Read for SSlice<'a, u8> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        as_slice(self, move |s| s.read(buf))
    }
    fn read_vectored(&mut self, bufs: &mut [std::io::IoSliceMut<'_>]) -> std::io::Result<usize> {
        as_slice(self, move |s| s.read_vectored(bufs))
    }
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> std::io::Result<usize> {
        as_slice(self, move |s| s.read_to_end(buf))
    }
    fn read_exact(&mut self, buf: &mut [u8]) -> std::io::Result<()> {
        as_slice(self, move |s| s.read_exact(buf))
    }
}

impl<'a, T> Default for SSlice<'a, T> {
    fn default() -> Self {
        Self::from_slice(&[])
    }
}

impl<'a, T> From<&'a [T]> for SSlice<'a, T> {
    fn from(value: &'a [T]) -> Self {
        Self::from_slice(value)
    }
}

impl<'a, T> From<SSlice<'a, T>> for &'a [T] {
    fn from(value: SSlice<'a, T>) -> Self {
        value.into_slice()
    }
}

impl<'a, T> From<&'a mut [T]> for SSlice<'a, T> {
    fn from(value: &'a mut [T]) -> Self {
        Self::from_slice(value)
    }
}

impl<'a, T> From<SMutSlice<'a, T>> for &'a [T] {
    fn from(value: SMutSlice<'a, T>) -> Self {
        value.into_mut_slice()
    }
}

impl<'a, T> Deref for SMutSlice<'a, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<'a, T> DerefMut for SMutSlice<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl<'a, T> AsRef<[T]> for SMutSlice<'a, T> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<'a, T> AsRef<SMutSlice<'a, T>> for SMutSlice<'a, T> {
    fn as_ref(&self) -> &SMutSlice<'a, T> {
        self
    }
}

impl<'a, T> AsMut<[T]> for SMutSlice<'a, T> {
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<'a, T> AsMut<SMutSlice<'a, T>> for SMutSlice<'a, T> {
    fn as_mut(&mut self) -> &mut SMutSlice<'a, T> {
        self
    }
}

impl<'a, T: PartialEq> PartialEq for SMutSlice<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<'a, T: Eq> Eq for SMutSlice<'a, T> {}

impl<'a, T: Debug> Debug for SMutSlice<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_slice().fmt(f)
    }
}

impl<'a, T> Default for SMutSlice<'a, T> {
    fn default() -> Self {
        Self::from_mut_slice(&mut [])
    }
}

impl<'a, T> From<&'a mut [T]> for SMutSlice<'a, T> {
    fn from(value: &'a mut [T]) -> Self {
        Self::from_mut_slice(value)
    }
}

impl<'a, T> From<SMutSlice<'a, T>> for &'a mut [T] {
    fn from(value: SMutSlice<'a, T>) -> Self {
        value.into_mut_slice()
    }
}

impl<'a> Write for SMutSlice<'a, u8> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        as_mut_slice(self, move |s| s.write(buf))
    }
    fn flush(&mut self) -> std::io::Result<()> {
        as_mut_slice(self, move |s| s.flush())
    }
    fn write_vectored(&mut self, bufs: &[std::io::IoSlice<'_>]) -> std::io::Result<usize> {
        as_mut_slice(self, move |s| s.write_vectored(bufs))
    }
    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        as_mut_slice(self, move |s| s.write_all(buf))
    }
}
