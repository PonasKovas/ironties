use std::fmt::Debug;
use std::{alloc::Allocator, mem::ManuallyDrop};

use super::r#box::SGlobal;

#[repr(C)]
pub struct SVec<T, A: Allocator = SGlobal> {
    ptr: *mut T,
    len: usize,
    capacity: usize,
    allocator: A,
}

impl<T> SVec<T, SGlobal> {
    pub fn from_std(value: Vec<T, std::alloc::Global>) -> Self {
        let (ptr, len, capacity, _allocator) = value.into_raw_parts_with_alloc();

        Self {
            ptr,
            len,
            capacity,
            allocator: SGlobal::new(),
        }
    }
}

impl<T, A: Allocator> SVec<T, A> {
    pub fn from_vec(value: Vec<T, A>) -> Self {
        let (ptr, len, capacity, allocator) = value.into_raw_parts_with_alloc();

        Self {
            ptr,
            len,
            capacity,
            allocator,
        }
    }
}

impl<T, A: Allocator> SVec<T, A> {
    pub fn from_vec_with_alloc(value: Vec<T, A>) -> Self {
        let (ptr, len, capacity, allocator) = value.into_raw_parts_with_alloc();

        Self {
            ptr,
            len,
            capacity,
            allocator,
        }
    }
    pub fn into_vec(self) -> Vec<T, A> {
        let Self {
            ptr,
            len,
            capacity,
            allocator,
        } = self;

        unsafe { Vec::from_raw_parts_in(ptr, len, capacity, allocator) }
    }
}

impl<T: Debug, A: Allocator> Debug for SVec<T, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let b = ManuallyDrop::new(unsafe { std::ptr::read(self) }.into_vec());

        (*b).fmt(f)
    }
}

impl<T: PartialEq, A: Allocator> PartialEq for SVec<T, A> {
    fn eq(&self, other: &Self) -> bool {
        let b1 = ManuallyDrop::new(unsafe { std::ptr::read(self) }.into_vec());
        let b2 = ManuallyDrop::new(unsafe { std::ptr::read(other) }.into_vec());

        b1.eq(&b2)
    }
}
