use crate::TypeInfo;
use std::{
    fmt::Debug,
    hash::Hash,
    io::{BufRead, Read, Write},
    marker::PhantomData,
    ops::{Deref, DerefMut, Index, IndexMut},
};

use super::FfiSafeEquivalent;

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

impl<'a, T> FfiSafeEquivalent for SSlice<'a, T> {
    type Normal = &'a [T];

    fn from_normal(normal: Self::Normal) -> Self {
        Self::new(normal)
    }
    fn into_normal(self) -> Self::Normal {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl<'a, T> SSlice<'a, T> {
    pub const fn new(normal: &'a [T]) -> Self {
        Self {
            ptr: normal.as_ptr(),
            len: normal.len(),
            _phantom: PhantomData,
        }
    }
    pub fn to_slice<'b>(&'b self) -> &'b [T] {
        // SAFETY: since 'a strictly outlives 'b, it is safe to assume that
        // the slice is valid for the lifetime of 'b
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl<'a, T> FfiSafeEquivalent for SMutSlice<'a, T> {
    type Normal = &'a mut [T];

    fn from_normal(normal: Self::Normal) -> Self {
        Self {
            ptr: normal.as_mut_ptr(),
            len: normal.len(),
            _phantom: PhantomData,
        }
    }
    fn into_normal(self) -> Self::Normal {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

impl<'a, T> SMutSlice<'a, T> {
    pub fn to_slice<'b>(&'b self) -> &'b [T] {
        // SAFETY: since 'a strictly outlives 'b, it is safe to assume that
        // the slice is valid for the lifetime of 'b
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
    pub fn to_slice_mut<'b>(&'b mut self) -> &'b mut [T] {
        // SAFETY: since 'a strictly outlives 'b, it is safe to assume that
        // the slice is valid for the lifetime of 'b
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
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
        self.as_normal_mut(move |s| s.consume(amt))
    }
}

impl<'a, T: Debug> Debug for SMutSlice<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_normal(move |s| s.fmt(f))
    }
}

impl<'a, T: Debug> Debug for SSlice<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.into_normal().fmt(f)
    }
}

impl<'a, T> Default for SSlice<'a, T> {
    fn default() -> Self {
        Self::from_normal(&[])
    }
}

impl<'a, T> Default for SMutSlice<'a, T> {
    fn default() -> Self {
        Self::from_normal(&mut [])
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
        Self::from_normal(value)
    }
}

impl<'a, T> From<SSlice<'a, T>> for &'a [T] {
    fn from(value: SSlice<'a, T>) -> Self {
        value.into_normal()
    }
}

impl<'a, T> From<&'a mut [T]> for SSlice<'a, T> {
    fn from(value: &'a mut [T]) -> Self {
        Self::from_normal(value)
    }
}

impl<'a, T> From<SMutSlice<'a, T>> for &'a [T] {
    fn from(value: SMutSlice<'a, T>) -> Self {
        value.into_normal()
    }
}

impl<'a, T> From<&'a mut [T]> for SMutSlice<'a, T> {
    fn from(value: &'a mut [T]) -> Self {
        Self::from_normal(value)
    }
}

impl<'a, T> From<SMutSlice<'a, T>> for &'a mut [T] {
    fn from(value: SMutSlice<'a, T>) -> Self {
        value.into_normal()
    }
}

impl<'a, T: Hash> Hash for SSlice<'a, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.into_normal().hash(state)
    }
}

impl<'a, T: Hash> Hash for SMutSlice<'a, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_normal(move |s| s.hash(state))
    }
}

impl<'a, T, I: std::slice::SliceIndex<[T]>> Index<I> for SSlice<'a, T> {
    type Output = <I as std::slice::SliceIndex<[T]>>::Output;

    fn index(&self, index: I) -> &Self::Output {
        self.into_normal().index(index)
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
        self.into_normal().iter()
    }
}

impl<'a, T> IntoIterator for SMutSlice<'a, T> {
    type Item = &'a mut T;

    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_normal().iter_mut()
    }
}

impl<'a, T: Ord> Ord for SSlice<'a, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.into_normal().cmp(other.into_normal())
    }
}

impl<'a, T: Ord> Ord for SMutSlice<'a, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_normal(move |s1| other.as_normal(move |s2| s1.cmp(s2)))
    }
}

impl<'a, T: PartialEq> PartialEq for SSlice<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.into_normal() == other.into_normal()
    }
}

impl<'a, T: PartialEq> PartialEq for SMutSlice<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.as_normal(move |s1| other.as_normal(move |s2| s1 == s2))
    }
}

impl<'a, T: PartialOrd> PartialOrd for SSlice<'a, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.into_normal().partial_cmp(other.into_normal())
    }
}

impl<'a, T: PartialOrd> PartialOrd for SMutSlice<'a, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_normal(move |s1| other.as_normal(move |s2| s1.partial_cmp(s2)))
    }
}

impl<'a> Read for SSlice<'a, u8> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.as_normal_mut(move |s| s.read(buf))
    }
    fn read_vectored(&mut self, bufs: &mut [std::io::IoSliceMut<'_>]) -> std::io::Result<usize> {
        self.as_normal_mut(move |s| s.read_vectored(bufs))
    }
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> std::io::Result<usize> {
        self.as_normal_mut(move |s| s.read_to_end(buf))
    }
    fn read_exact(&mut self, buf: &mut [u8]) -> std::io::Result<()> {
        self.as_normal_mut(move |s| s.read_exact(buf))
    }
}

impl<'a> Read for SMutSlice<'a, u8> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.as_normal_mut(move |s| (&**s).read(buf))
    }
    fn read_vectored(&mut self, bufs: &mut [std::io::IoSliceMut<'_>]) -> std::io::Result<usize> {
        self.as_normal_mut(move |s| (&**s).read_vectored(bufs))
    }
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> std::io::Result<usize> {
        self.as_normal_mut(move |s| (&**s).read_to_end(buf))
    }
    fn read_exact(&mut self, buf: &mut [u8]) -> std::io::Result<()> {
        self.as_normal_mut(move |s| (&**s).read_exact(buf))
    }
}

impl<'a> Write for SMutSlice<'a, u8> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.as_normal_mut(move |s| s.write(buf))
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.as_normal_mut(move |s| s.flush())
    }
    fn write_vectored(&mut self, bufs: &[std::io::IoSlice<'_>]) -> std::io::Result<usize> {
        self.as_normal_mut(move |s| s.write_vectored(bufs))
    }
    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.as_normal_mut(move |s| s.write_all(buf))
    }
}

unsafe impl<'a, T> Send for SSlice<'a, T> {}
unsafe impl<'a, T> Sync for SSlice<'a, T> {}
unsafe impl<'a, T> Send for SMutSlice<'a, T> {}
unsafe impl<'a, T> Sync for SMutSlice<'a, T> {}
