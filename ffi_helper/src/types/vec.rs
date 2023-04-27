use super::allocator::SGlobal;
use ffi_helper::TypeInfo;
use std::alloc::Allocator;
use std::fmt::Debug;
use std::mem::{forget, ManuallyDrop};

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

impl<T: Debug, A: Allocator> Debug for SVec<T, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_vec(move |v| v.fmt(f))
    }
}

impl<T: PartialEq, A: Allocator> PartialEq for SVec<T, A> {
    fn eq(&self, other: &Self) -> bool {
        self.as_vec(move |v1| other.as_vec(move |v2| v1.eq(v2)))
    }
}

impl<T: Clone, A: Allocator + Clone> Clone for SVec<T, A> {
    fn clone(&self) -> Self {
        SVec::from_vec(self.as_vec(move |v| v.clone()))
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
