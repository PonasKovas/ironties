use std::{
    alloc::{Allocator, Global, Layout},
    ptr::{self, null_mut, NonNull},
};

use super::STuple2;
use ffi_helper::TypeInfo;

/// FFI-safe equivalent of [`std::alloc::Global`]
#[repr(C)]
#[derive(TypeInfo, Clone, Copy, PartialEq)]
pub struct SGlobal {
    vtable: &'static SGlobalVTable,
}

#[repr(C)]
#[derive(TypeInfo, Clone, Copy, PartialEq)]
struct SGlobalVTable {
    allocate: unsafe extern "C" fn(layout: SLayout) -> STuple2<*mut u8, usize>,
    deallocate: unsafe extern "C" fn(ptr: NonNull<u8>, layout: SLayout),
    allocate_zeroed: unsafe extern "C" fn(layout: SLayout) -> STuple2<*mut u8, usize>,
    grow: unsafe extern "C" fn(
        ptr: NonNull<u8>,
        old_layout: SLayout,
        new_layout: SLayout,
    ) -> STuple2<*mut u8, usize>,
    grow_zeroed: unsafe extern "C" fn(
        ptr: NonNull<u8>,
        old_layout: SLayout,
        new_layout: SLayout,
    ) -> STuple2<*mut u8, usize>,
    shrink: unsafe extern "C" fn(
        ptr: NonNull<u8>,
        old_layout: SLayout,
        new_layout: SLayout,
    ) -> STuple2<*mut u8, usize>,
}

#[repr(C)]
#[derive(TypeInfo, Clone, Copy, PartialEq)]
struct SLayout {
    size: usize,
    align: usize,
}

impl SGlobal {
    pub fn new() -> Self {
        unsafe extern "C" fn allocate(layout: SLayout) -> STuple2<*mut u8, usize> {
            match Global.allocate(Layout::from_size_align_unchecked(layout.size, layout.align)) {
                Ok(slice) => STuple2(slice.as_ptr() as *mut u8, slice.len()),
                Err(_) => STuple2(null_mut(), 0),
            }
        }
        unsafe extern "C" fn deallocate(ptr: NonNull<u8>, layout: SLayout) {
            Global.deallocate(
                ptr,
                Layout::from_size_align_unchecked(layout.size, layout.align),
            )
        }
        unsafe extern "C" fn allocate_zeroed(layout: SLayout) -> STuple2<*mut u8, usize> {
            match Global
                .allocate_zeroed(Layout::from_size_align_unchecked(layout.size, layout.align))
            {
                Ok(slice) => STuple2(slice.as_ptr() as *mut u8, slice.len()),
                Err(_) => STuple2(null_mut(), 0),
            }
        }
        unsafe extern "C" fn grow(
            ptr: NonNull<u8>,
            old_layout: SLayout,
            new_layout: SLayout,
        ) -> STuple2<*mut u8, usize> {
            match Global.grow(
                ptr,
                Layout::from_size_align_unchecked(old_layout.size, old_layout.align),
                Layout::from_size_align_unchecked(new_layout.size, new_layout.align),
            ) {
                Ok(slice) => STuple2(slice.as_ptr() as *mut u8, slice.len()),
                Err(_) => STuple2(null_mut(), 0),
            }
        }
        unsafe extern "C" fn grow_zeroed(
            ptr: NonNull<u8>,
            old_layout: SLayout,
            new_layout: SLayout,
        ) -> STuple2<*mut u8, usize> {
            match Global.grow_zeroed(
                ptr,
                Layout::from_size_align_unchecked(old_layout.size, old_layout.align),
                Layout::from_size_align_unchecked(new_layout.size, new_layout.align),
            ) {
                Ok(slice) => STuple2(slice.as_ptr() as *mut u8, slice.len()),
                Err(_) => STuple2(null_mut(), 0),
            }
        }
        unsafe extern "C" fn shrink(
            ptr: NonNull<u8>,
            old_layout: SLayout,
            new_layout: SLayout,
        ) -> STuple2<*mut u8, usize> {
            match Global.shrink(
                ptr,
                Layout::from_size_align_unchecked(old_layout.size, old_layout.align),
                Layout::from_size_align_unchecked(new_layout.size, new_layout.align),
            ) {
                Ok(slice) => STuple2(slice.as_ptr() as *mut u8, slice.len()),
                Err(_) => STuple2(null_mut(), 0),
            }
        }
        static VTABLE: SGlobalVTable = SGlobalVTable {
            allocate,
            deallocate,
            allocate_zeroed,
            grow,
            grow_zeroed,
            shrink,
        };
        Self { vtable: &VTABLE }
    }
}

unsafe impl Allocator for SGlobal {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, std::alloc::AllocError> {
        unsafe {
            let STuple2(ptr, len) = (self.vtable.allocate)(SLayout {
                size: layout.size(),
                align: layout.align(),
            });

            match NonNull::new(ptr::slice_from_raw_parts_mut(ptr, len)) {
                Some(ptr) => Ok(ptr),
                None => Err(std::alloc::AllocError),
            }
        }
    }
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        (self.vtable.deallocate)(
            ptr,
            SLayout {
                size: layout.size(),
                align: layout.align(),
            },
        )
    }
    fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<[u8]>, std::alloc::AllocError> {
        unsafe {
            let STuple2(ptr, len) = (self.vtable.allocate_zeroed)(SLayout {
                size: layout.size(),
                align: layout.align(),
            });

            match NonNull::new(ptr::slice_from_raw_parts_mut(ptr, len)) {
                Some(ptr) => Ok(ptr),
                None => Err(std::alloc::AllocError),
            }
        }
    }
    unsafe fn grow(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, std::alloc::AllocError> {
        unsafe {
            let STuple2(ptr, len) = (self.vtable.grow)(
                ptr,
                SLayout {
                    size: old_layout.size(),
                    align: old_layout.align(),
                },
                SLayout {
                    size: new_layout.size(),
                    align: new_layout.align(),
                },
            );

            match NonNull::new(ptr::slice_from_raw_parts_mut(ptr, len)) {
                Some(ptr) => Ok(ptr),
                None => Err(std::alloc::AllocError),
            }
        }
    }
    unsafe fn grow_zeroed(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, std::alloc::AllocError> {
        unsafe {
            let STuple2(ptr, len) = (self.vtable.grow_zeroed)(
                ptr,
                SLayout {
                    size: old_layout.size(),
                    align: old_layout.align(),
                },
                SLayout {
                    size: new_layout.size(),
                    align: new_layout.align(),
                },
            );

            match NonNull::new(ptr::slice_from_raw_parts_mut(ptr, len)) {
                Some(ptr) => Ok(ptr),
                None => Err(std::alloc::AllocError),
            }
        }
    }
    unsafe fn shrink(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, std::alloc::AllocError> {
        unsafe {
            let STuple2(ptr, len) = (self.vtable.shrink)(
                ptr,
                SLayout {
                    size: old_layout.size(),
                    align: old_layout.align(),
                },
                SLayout {
                    size: new_layout.size(),
                    align: new_layout.align(),
                },
            );

            match NonNull::new(ptr::slice_from_raw_parts_mut(ptr, len)) {
                Some(ptr) => Ok(ptr),
                None => Err(std::alloc::AllocError),
            }
        }
    }
}

impl Default for SGlobal {
    fn default() -> Self {
        Self::new()
    }
}
