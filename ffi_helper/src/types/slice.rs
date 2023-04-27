use crate::TypeInfo;
use std::{
    fmt::Debug,
    hash::Hash,
    io::{BufRead, Read, Write},
    marker::PhantomData,
    ops::{Deref, DerefMut, Index, IndexMut},
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
    pub fn to_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
    pub fn as_slice_mut<R, F>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut &'a [T]) -> R,
    {
        let mut copy = self.into_slice();

        let r = f(&mut copy);

        *self = SSlice::from_slice(copy);

        r
    }
}

impl<'a, T> SMutSlice<'a, T> {
    /// Converts a [`&'a mut [T]`][slice] to [`SMutSlice<'a, T>`][SMutSlice]
    pub fn from_slice(slice: &'a mut [T]) -> Self {
        Self {
            ptr: slice.as_mut_ptr(),
            len: slice.len(),
            _phantom: PhantomData,
        }
    }
    /// Converts a [`SMutSlice<'a, T>`][SMutSlice] to [`&'a mut [T]`][slice]
    pub fn into_slice(self) -> &'a mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
    pub fn to_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
    pub fn to_slice_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
    pub fn as_slice<R, F>(&self, f: F) -> R
    where
        F: FnOnce(&&'a mut [T]) -> R,
    {
        let copy = unsafe { std::ptr::read(self) }.into_slice();

        f(&copy)
    }
    pub fn as_slice_mut<R, F>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut &'a mut [T]) -> R,
    {
        let mut copy = unsafe { std::ptr::read(self) }.into_slice();

        let r = f(&mut copy);

        unsafe { std::ptr::write(self, SMutSlice::from_slice(copy)) }

        r
    }
}

// The reason Copy and Clone have to be implemented manually instead of being derived
// Is because deriving them automatically introduces unnecessary generic bounds on T.
impl<'a, T> Clone for SSlice<'a, T> {
    fn clone(&self) -> Self {
        Self { ..*self }
    }
}
impl<'a, T> Copy for SSlice<'a, T> {}

impl<'a> BufRead for SSlice<'a, u8> {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        Ok(self.to_slice())
    }
    fn consume(&mut self, amt: usize) {
        self.as_slice_mut(move |s| s.consume(amt))
    }
}

impl<'a, T: Debug> Debug for SMutSlice<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_slice(move |s| s.fmt(f))
    }
}

impl<'a, T: Debug> Debug for SSlice<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.into_slice().fmt(f)
    }
}

impl<'a, T> Default for SSlice<'a, T> {
    fn default() -> Self {
        Self::from_slice(&[])
    }
}

impl<'a, T> Default for SMutSlice<'a, T> {
    fn default() -> Self {
        Self::from_slice(&mut [])
    }
}

impl<'a, T> Deref for SSlice<'a, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.to_slice()
    }
}

impl<'a, T> Deref for SMutSlice<'a, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.to_slice()
    }
}

impl<'a, T> DerefMut for SMutSlice<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.to_slice_mut()
    }
}

impl<'a, T: Eq> Eq for SSlice<'a, T> {}
impl<'a, T: Eq> Eq for SMutSlice<'a, T> {}

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
        value.into_slice()
    }
}

impl<'a, T> From<&'a mut [T]> for SMutSlice<'a, T> {
    fn from(value: &'a mut [T]) -> Self {
        Self::from_slice(value)
    }
}

impl<'a, T> From<SMutSlice<'a, T>> for &'a mut [T] {
    fn from(value: SMutSlice<'a, T>) -> Self {
        value.into_slice()
    }
}

impl<'a, T: Hash> Hash for SSlice<'a, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.into_slice().hash(state)
    }
}

impl<'a, T: Hash> Hash for SMutSlice<'a, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_slice(move |s| s.hash(state))
    }
}

impl<'a, T, I: std::slice::SliceIndex<[T]>> Index<I> for SSlice<'a, T> {
    type Output = <I as std::slice::SliceIndex<[T]>>::Output;

    fn index(&self, index: I) -> &Self::Output {
        self.into_slice().index(index)
    }
}

impl<'a, T, I: std::slice::SliceIndex<[T]>> Index<I> for SMutSlice<'a, T> {
    type Output = <I as std::slice::SliceIndex<[T]>>::Output;

    fn index(&self, index: I) -> &Self::Output {
        self.to_slice().index(index)
    }
}

impl<'a, T, I: std::slice::SliceIndex<[T]>> IndexMut<I> for SMutSlice<'a, T> {
    fn index_mut(&mut self, index: I) -> &mut <I as std::slice::SliceIndex<[T]>>::Output {
        self.to_slice_mut().index_mut(index)
    }
}

impl<'a, T> IntoIterator for SSlice<'a, T> {
    type Item = &'a T;

    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_slice().iter()
    }
}

impl<'a, T> IntoIterator for SMutSlice<'a, T> {
    type Item = &'a mut T;

    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_slice().iter_mut()
    }
}

impl<'a, T: Ord> Ord for SSlice<'a, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.into_slice().cmp(other.into_slice())
    }
}

impl<'a, T: Ord> Ord for SMutSlice<'a, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_slice(move |s1| other.as_slice(move |s2| s1.cmp(s2)))
    }
}

impl<'a, T: PartialEq> PartialEq for SSlice<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.into_slice() == other.into_slice()
    }
}

impl<'a, T: PartialEq> PartialEq for SMutSlice<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice(move |s1| other.as_slice(move |s2| s1 == s2))
    }
}

impl<'a, T: PartialOrd> PartialOrd for SSlice<'a, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.into_slice().partial_cmp(other.into_slice())
    }
}

impl<'a, T: PartialOrd> PartialOrd for SMutSlice<'a, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_slice(move |s1| other.as_slice(move |s2| s1.partial_cmp(s2)))
    }
}

impl<'a> Read for SSlice<'a, u8> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.as_slice_mut(move |s| s.read(buf))
    }
    fn read_vectored(&mut self, bufs: &mut [std::io::IoSliceMut<'_>]) -> std::io::Result<usize> {
        self.as_slice_mut(move |s| s.read_vectored(bufs))
    }
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> std::io::Result<usize> {
        self.as_slice_mut(move |s| s.read_to_end(buf))
    }
    fn read_exact(&mut self, buf: &mut [u8]) -> std::io::Result<()> {
        self.as_slice_mut(move |s| s.read_exact(buf))
    }
}

impl<'a> Read for SMutSlice<'a, u8> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.as_slice_mut(move |s| (&**s).read(buf))
    }
    fn read_vectored(&mut self, bufs: &mut [std::io::IoSliceMut<'_>]) -> std::io::Result<usize> {
        self.as_slice_mut(move |s| (&**s).read_vectored(bufs))
    }
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> std::io::Result<usize> {
        self.as_slice_mut(move |s| (&**s).read_to_end(buf))
    }
    fn read_exact(&mut self, buf: &mut [u8]) -> std::io::Result<()> {
        self.as_slice_mut(move |s| (&**s).read_exact(buf))
    }
}

impl<'a> Write for SMutSlice<'a, u8> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.as_slice_mut(move |s| s.write(buf))
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.as_slice_mut(move |s| s.flush())
    }
    fn write_vectored(&mut self, bufs: &[std::io::IoSlice<'_>]) -> std::io::Result<usize> {
        self.as_slice_mut(move |s| s.write_vectored(bufs))
    }
    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.as_slice_mut(move |s| s.write_all(buf))
    }
}
