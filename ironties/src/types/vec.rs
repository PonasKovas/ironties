use super::allocator::SGlobal;
use super::FfiSafeEquivalent;
use ironties::TypeInfo;
use std::alloc::Allocator;
use std::borrow::{Borrow, BorrowMut};
use std::fmt::Debug;
use std::hash::Hash;
use std::mem::{forget, ManuallyDrop};
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::slice;

/// FFI-safe equivalent of [`Vec<T>`]
#[repr(C)]
#[derive(TypeInfo)]
pub struct SVec<T, A: Allocator = SGlobal> {
    ptr: *const T,
    len: usize,
    capacity: usize,
    // ManuallyDrop to avoid a double-free, because on drop (see Drop impl) the whole thing
    // will be converted to a Box and that dropped, which takes care of dropping the allocator.
    allocator: ManuallyDrop<A>,
}

impl<T, A: Allocator> FfiSafeEquivalent for SVec<T, A> {
    type Normal = Vec<T, A>;

    fn from_normal(normal: Self::Normal) -> Self {
        let (ptr, len, capacity, allocator) = normal.into_raw_parts_with_alloc();
        Self {
            ptr,
            len,
            capacity,
            allocator: ManuallyDrop::new(allocator),
        }
    }
    fn into_normal(self) -> Self::Normal {
        // SAFETY: we construct a Vec for the same object as our SVec, and then forget the original SVec to avoid a double-free.
        // We have to make a bitwise copy of the allocator here, because we need it by value, and SVec can't be
        // destructured (because it implements Drop). This is basically just a simple move, but explicit and manual, to satisfy the compiler.
        let copy = unsafe {
            Vec::from_raw_parts_in(
                self.ptr as *mut T,
                self.len,
                self.capacity,
                std::ptr::read(&*self.allocator),
            )
        };

        forget(self);

        copy
    }
}

impl<T> SVec<T, SGlobal> {
    pub fn from_vec(value: Vec<T, std::alloc::Global>) -> Self {
        let (ptr, len, capacity, _allocator) = value.into_raw_parts_with_alloc();
        Self {
            ptr,
            len,
            capacity,
            allocator: ManuallyDrop::new(SGlobal::new()),
        }
    }
    pub fn new() -> Self {
        Self::from_normal(Vec::new_in(SGlobal::new()))
    }
}

impl<T, A: Allocator> Drop for SVec<T, A> {
    fn drop(&mut self) {
        // SAFETY: We make a Vec for the same object as our SVec and drop it.
        unsafe { std::ptr::read(self) }.into_normal();
    }
}

impl<T, A: Allocator> AsMut<[T]> for SVec<T, A> {
    fn as_mut(&mut self) -> &mut [T] {
        // SAFETY: Converting the pointer to a mutable reference, which is safe, since
        // we know that it must point to valid data as long as the SVec lives.
        unsafe { slice::from_raw_parts_mut(self.ptr as *mut T, self.len) }
    }
}

impl<T, A: Allocator> AsRef<[T]> for SVec<T, A> {
    fn as_ref(&self) -> &[T] {
        // SAFETY: Converting the pointer to an immutable reference, which is safe, because
        // we know that it must point to valid data as long as the SVec lives.
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl<T, A: Allocator> Borrow<[T]> for SVec<T, A> {
    fn borrow(&self) -> &[T] {
        self.as_ref()
    }
}

impl<T, A: Allocator> BorrowMut<[T]> for SVec<T, A> {
    fn borrow_mut(&mut self) -> &mut [T] {
        self.as_mut()
    }
}

impl<T: Clone, A: Allocator + Clone> Clone for SVec<T, A> {
    fn clone(&self) -> Self {
        SVec::from_normal(self.as_normal(move |v| v.clone()))
    }
}

impl<T: Debug, A: Allocator> Debug for SVec<T, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_normal(move |v| v.fmt(f))
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

impl<T: Eq, A: Allocator> Eq for SVec<T, A> {}

impl<T, A: Allocator> From<Vec<T, A>> for SVec<T, A> {
    fn from(value: Vec<T, A>) -> Self {
        Self::from_normal(value)
    }
}

impl<T, A: Allocator> From<SVec<T, A>> for Vec<T, A> {
    fn from(value: SVec<T, A>) -> Self {
        value.into_normal()
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
        self.into_normal().into_iter()
    }
}

impl<T: Ord, A: Allocator> Ord for SVec<T, A> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_normal(move |v1| other.as_normal(move |v2| v1.cmp(v2)))
    }
}

impl<T: PartialEq, A: Allocator> PartialEq for SVec<T, A> {
    fn eq(&self, other: &Self) -> bool {
        self.as_normal(move |v1| other.as_normal(move |v2| v1.eq(v2)))
    }
}

impl<T: PartialOrd, A: Allocator> PartialOrd for SVec<T, A> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_normal(move |v1| other.as_normal(move |v2| v1.partial_cmp(v2)))
    }
}
