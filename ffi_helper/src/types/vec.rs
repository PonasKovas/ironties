use super::allocator::SGlobal;
use ffi_helper::TypeInfo;
use std::alloc::Allocator;
use std::borrow::{Borrow, BorrowMut};
use std::fmt::Debug;
use std::hash::Hash;
use std::mem::{forget, ManuallyDrop};
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::slice;

/// FFI-safe version of [`Vec<T>`]
#[repr(C)]
#[derive(TypeInfo)]
pub struct SVec<T, A: Allocator = SGlobal> {
    ptr: *mut T,
    len: usize,
    capacity: usize,
    allocator: ManuallyDrop<A>,
}

impl<T, A: Allocator> SVec<T, A> {
    pub fn from_vec(value: Vec<T, A>) -> Self {
        let (ptr, len, capacity, allocator) = value.into_raw_parts_with_alloc();
        Self {
            ptr,
            len,
            capacity,
            allocator: ManuallyDrop::new(allocator),
        }
    }
    pub fn into_vec(self) -> Vec<T, A> {
        let r = unsafe { raw_convert_to_vec(&self) };

        forget(self);

        r
    }
    pub fn convert<A2: Allocator + Into<A>>(value: Vec<T, A2>) -> Self {
        let (ptr, len, capacity, allocator) = value.into_raw_parts_with_alloc();
        Self {
            ptr,
            len,
            capacity,
            allocator: ManuallyDrop::new(allocator.into()),
        }
    }
    pub fn as_vec<R, F>(&self, f: F) -> R
    where
        F: FnOnce(&Vec<T, A>) -> R,
    {
        let vec = ManuallyDrop::new(unsafe { raw_convert_to_vec(self) });

        let r = f(&*vec);

        r
    }
    pub fn as_vec_mut<R, F>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut Vec<T, A>) -> R,
    {
        let mut vec = ManuallyDrop::new(unsafe { raw_convert_to_vec(self) });

        let r = f(&mut *vec);

        r
    }
}

// incorrect usage may cause double-frees
unsafe fn raw_convert_to_vec<T, A: Allocator>(svec: &SVec<T, A>) -> Vec<T, A> {
    Vec::from_raw_parts_in(
        svec.ptr,
        svec.len,
        svec.capacity,
        ManuallyDrop::into_inner(std::ptr::read(&svec.allocator)),
    )
}

impl<T> SVec<T, SGlobal> {
    pub fn new() -> Self {
        Self::from_vec(Vec::new_in(SGlobal::new()))
    }
}

impl<T, A: Allocator> AsMut<[T]> for SVec<T, A> {
    fn as_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

impl<T, A: Allocator> AsRef<[T]> for SVec<T, A> {
    fn as_ref(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl<T, A: Allocator> Borrow<[T]> for SVec<T, A> {
    fn borrow(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl<T, A: Allocator> BorrowMut<[T]> for SVec<T, A> {
    fn borrow_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

impl<T: Clone, A: Allocator + Clone> Clone for SVec<T, A> {
    fn clone(&self) -> Self {
        SVec::from_vec(self.as_vec(move |v| v.clone()))
    }
}

impl<T: Debug, A: Allocator> Debug for SVec<T, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_vec(move |v| v.fmt(f))
    }
}

impl<T> Default for SVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, A: Allocator> Deref for SVec<T, A> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T, A: Allocator> DerefMut for SVec<T, A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<T, A: Allocator> Drop for SVec<T, A> {
    fn drop(&mut self) {
        unsafe {
            Vec::from_raw_parts_in(
                self.ptr,
                self.len,
                self.capacity,
                ManuallyDrop::into_inner(std::ptr::read(&self.allocator)),
            )
        };
    }
}

impl<T: Eq, A: Allocator> Eq for SVec<T, A> {}

impl<T, A: Allocator> From<Vec<T, A>> for SVec<T, A> {
    fn from(value: Vec<T, A>) -> Self {
        Self::from_vec(value)
    }
}

impl<T, A: Allocator> From<SVec<T, A>> for Vec<T, A> {
    fn from(value: SVec<T, A>) -> Self {
        value.into_vec()
    }
}

impl<T: Hash, A: Allocator> Hash for SVec<T, A> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Hash::hash(&**self, state)
    }
}

impl<T, I: std::slice::SliceIndex<[T]>, A: Allocator> Index<I> for SVec<T, A> {
    type Output = <I as std::slice::SliceIndex<[T]>>::Output;

    fn index(&self, index: I) -> &Self::Output {
        Index::index(&**self, index)
    }
}

impl<T, I: std::slice::SliceIndex<[T]>, A: Allocator> IndexMut<I> for SVec<T, A> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        IndexMut::index_mut(&mut **self, index)
    }
}

impl<'a, T, A: Allocator> IntoIterator for &'a SVec<T, A> {
    type Item = &'a T;

    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, A: Allocator> IntoIterator for &'a mut SVec<T, A> {
    type Item = &'a mut T;

    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T, A: Allocator> IntoIterator for SVec<T, A> {
    type Item = T;

    type IntoIter = std::vec::IntoIter<T, A>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_vec().into_iter()
    }
}

impl<T: Ord, A: Allocator> Ord for SVec<T, A> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_vec(move |v1| other.as_vec(move |v2| v1.cmp(v2)))
    }
}

impl<T: PartialEq, A: Allocator> PartialEq for SVec<T, A> {
    fn eq(&self, other: &Self) -> bool {
        self.as_vec(move |v1| other.as_vec(move |v2| v1.eq(v2)))
    }
}

impl<T: PartialOrd, A: Allocator> PartialOrd for SVec<T, A> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_vec(move |v1| other.as_vec(move |v2| v1.partial_cmp(v2)))
    }
}
