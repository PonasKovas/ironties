use crate::types::STuple2;

use super::SResult;
use std::fmt::Debug;
use std::mem::ManuallyDrop;
use std::{alloc::Allocator, ptr::NonNull};

#[repr(C)]
pub struct SBox<T: ?Sized, A: Allocator = SGlobal> {
    ptr: *mut T,
    allocator: A,
}

impl<T: ?Sized> SBox<T, SGlobal> {
    pub fn from_box(value: Box<T, std::alloc::Global>) -> Self {
        Self {
            ptr: Box::into_raw(value),
            allocator: SGlobal::new(),
        }
    }
}

impl<T: ?Sized, A: Allocator> SBox<T, A> {
    pub fn from_box_with_alloc(value: Box<T, A>) -> Self {
        let (ptr, allocator) = Box::into_raw_with_allocator(value);

        Self { ptr, allocator }
    }
    pub fn into_box(value: Self) -> Box<T, A> {
        let Self { ptr, allocator } = value;

        unsafe { Box::from_raw_in(ptr, allocator) }
    }
}

impl<T: ?Sized + Debug, A: Allocator> Debug for SBox<T, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let b = ManuallyDrop::new(SBox::into_box(unsafe { std::ptr::read(self) }));

        (*b).fmt(f)
    }
}

impl<T: ?Sized + PartialEq, A: Allocator> PartialEq for SBox<T, A> {
    fn eq(&self, other: &Self) -> bool {
        let b1 = ManuallyDrop::new(SBox::into_box(unsafe { std::ptr::read(self) }));
        let b2 = ManuallyDrop::new(SBox::into_box(unsafe { std::ptr::read(other) }));

        b1.eq(&b2)
    }
}

impl<T: ?Sized + Clone, A: Allocator + Clone> Clone for SBox<T, A> {
    fn clone(&self) -> Self {
        let b = ManuallyDrop::new(SBox::into_box(unsafe { std::ptr::read(self) }));

        SBox::from_box_with_alloc((*b).clone())
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SGlobal {
    allocate: extern "C" fn(layout: SLayout) -> SResult<STuple2<*mut (), usize>, SAllocError>,
    deallocate: unsafe extern "C" fn(ptr: NonNull<u8>, layout: SLayout),
}

impl SGlobal {
    pub const fn new() -> Self {
        extern "C" fn allocate(layout: SLayout) -> SResult<STuple2<*mut (), usize>, SAllocError> {
            match std::alloc::Global.allocate(layout.into_layout()) {
                Ok(ptr) => SResult::Ok(STuple2(ptr.as_ptr() as *mut (), ptr.len())),
                Err(_) => SResult::Err(SAllocError::new()),
            }
        }
        unsafe extern "C" fn deallocate(ptr: NonNull<u8>, layout: SLayout) {
            std::alloc::Global.deallocate(ptr, layout.into_layout())
        }
        Self {
            allocate,
            deallocate,
        }
    }
}

unsafe impl Allocator for SGlobal {
    fn allocate(
        &self,
        layout: std::alloc::Layout,
    ) -> Result<NonNull<[u8]>, std::alloc::AllocError> {
        (self.allocate)(SLayout::from_layout(layout))
            .into_result()
            .map(|STuple2(ptr, len)| unsafe {
                NonNull::new(std::ptr::slice_from_raw_parts_mut(ptr as *mut _, len))
                    .unwrap_unchecked()
            })
            .map_err(|_| std::alloc::AllocError)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: std::alloc::Layout) {
        (self.deallocate)(ptr, SLayout::from_layout(layout))
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SAllocError {
    _private: [u8; 0],
}

impl SAllocError {
    pub const fn new() -> Self {
        Self { _private: [] }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SLayout {
    size: usize,
    align: usize,
}

impl SLayout {
    pub const fn from_layout(layout: std::alloc::Layout) -> Self {
        Self {
            size: layout.size(),
            align: layout.align(),
        }
    }
    pub const fn into_layout(self) -> std::alloc::Layout {
        unsafe { std::alloc::Layout::from_size_align_unchecked(self.size, self.align) }
    }
}
