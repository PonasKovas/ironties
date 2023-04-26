use crate::TypeInfo;
use std::alloc::Allocator;
use std::borrow::{Borrow, BorrowMut};
use std::fmt::{Debug, Display, Pointer};
use std::mem::{forget, ManuallyDrop};
use std::ops::{Deref, DerefMut};

use super::allocator::SGlobal;

/// FFI-safe version of [`Box<T>`]
#[repr(C)]
#[derive(TypeInfo)]
pub struct SBox<T: Sized, A: Allocator = SGlobal> {
    ptr: *mut T,
    allocator: ManuallyDrop<A>,
}

impl<T: Sized, A: Allocator> SBox<T, A> {
    pub fn from_box(value: Box<T, A>) -> Self {
        let (ptr, allocator) = Box::into_raw_with_allocator(value);

        Self {
            ptr,
            allocator: ManuallyDrop::new(allocator),
        }
    }
    pub fn into_box(self) -> Box<T, A> {
        let r = unsafe {
            Box::from_raw_in(
                self.ptr,
                ManuallyDrop::into_inner(std::ptr::read(&self.allocator)),
            )
        };

        forget(self);

        r
    }
}

impl<T: Sized> SBox<T, SGlobal> {
    pub fn convert(value: Box<T>) -> Self {
        Self {
            ptr: Box::into_raw(value),
            allocator: ManuallyDrop::new(SGlobal::new()),
        }
    }
    pub fn new(value: T) -> Self {
        SBox::from_box(Box::new_in(value, SGlobal::new()))
    }
}

impl<T: Sized, A: Allocator> AsMut<T> for SBox<T, A> {
    fn as_mut(&mut self) -> &mut T {
        unsafe { self.ptr.as_mut().unwrap_unchecked() }
    }
}

impl<T: Sized, A: Allocator> AsRef<T> for SBox<T, A> {
    fn as_ref(&self) -> &T {
        unsafe { self.ptr.as_ref().unwrap_unchecked() }
    }
}

impl<T: Sized, A: Allocator> Borrow<T> for SBox<T, A> {
    fn borrow(&self) -> &T {
        self.as_ref()
    }
}

impl<T: Sized, A: Allocator> BorrowMut<T> for SBox<T, A> {
    fn borrow_mut(&mut self) -> &mut T {
        self.as_mut()
    }
}

impl<T: Clone + Sized, A: Clone + Allocator> Clone for SBox<T, A> {
    fn clone(&self) -> Self {
        SBox::from_box(Box::new_in(
            self.as_ref().clone(),
            ManuallyDrop::into_inner(self.allocator.clone()),
        ))
    }
}

impl<T: Debug + Sized, A: Allocator> Debug for SBox<T, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        T::fmt(self.as_ref(), f)
    }
}

impl<T: Default + Sized, A: Default + Allocator> Default for SBox<T, A> {
    fn default() -> Self {
        Self::from_box(Box::new_in(T::default(), A::default()))
    }
}

impl<T: Sized, A: Allocator> Deref for SBox<T, A> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T: Sized, A: Allocator> DerefMut for SBox<T, A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<T: Display + Sized, A: Allocator> Display for SBox<T, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl<T: Sized, A: Allocator> Drop for SBox<T, A> {
    fn drop(&mut self) {
        unsafe {
            Box::from_raw_in(
                self.ptr,
                ManuallyDrop::into_inner(std::ptr::read(&self.allocator)),
            )
        };
    }
}

impl<T: Eq + Sized, A: Allocator> Eq for SBox<T, A> {
    fn assert_receiver_is_total_eq(&self) {
        T::assert_receiver_is_total_eq(self.as_ref())
    }
}

impl<T: Sized> From<T> for SBox<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: Ord + Sized, A: Allocator> Ord for SBox<T, A> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        T::cmp(self.as_ref(), other.as_ref())
    }
}

impl<T: PartialEq + Sized, A: Allocator> PartialEq for SBox<T, A> {
    fn eq(&self, other: &Self) -> bool {
        T::eq(self.as_ref(), other.as_ref())
    }
    fn ne(&self, other: &Self) -> bool {
        T::ne(self.as_ref(), other.as_ref())
    }
}

impl<T: PartialOrd + Sized, A: Allocator> PartialOrd for SBox<T, A> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        T::partial_cmp(self.as_ref(), other.as_ref())
    }
    fn lt(&self, other: &Self) -> bool {
        T::lt(self.as_ref(), other.as_ref())
    }
    fn le(&self, other: &Self) -> bool {
        T::le(self.as_ref(), other.as_ref())
    }
    fn gt(&self, other: &Self) -> bool {
        T::gt(self.as_ref(), other.as_ref())
    }
    fn ge(&self, other: &Self) -> bool {
        T::ge(self.as_ref(), other.as_ref())
    }
}

impl<T: Sized, A: Allocator> Pointer for SBox<T, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Pointer::fmt(&self.ptr, f)
    }
}

impl<T: Sized, A: Allocator> Unpin for SBox<T, A> {}
