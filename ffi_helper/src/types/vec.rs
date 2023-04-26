use ffi_helper::TypeInfo;
use std::alloc::Layout;
use std::fmt::Debug;
use std::mem::ManuallyDrop;

#[repr(C)]
#[derive(TypeInfo)]
pub struct SVec<T> {
    ptr: *mut T,
    len: usize,
    capacity: usize,
    vtable: &'static SVecVTable,
}

#[repr(C)]
#[derive(TypeInfo)]
struct SVecVTable {
    dealloc: unsafe extern "C" fn(ptr: *mut u8, size: usize, align: usize),
}

fn as_vec<T, R, F>(svec: &SVec<T>, f: F) -> R
where
    F: FnOnce(&Vec<T>) -> R,
{
    // SAFETY:
    // Even though the original Vec<T> might have not used the same allocator,
    // this is safe, because only immutable access is ever given to the vec.
    let vec = unsafe { Vec::from_raw_parts(svec.ptr, svec.len, svec.capacity) };
    let r = f(&vec);

    std::mem::forget(vec);

    r
}

impl<T> SVec<T> {
    pub fn from_vec(value: Vec<T>) -> Self {
        unsafe extern "C" fn dealloc(ptr: *mut u8, size: usize, align: usize) {
            let layout = Layout::from_size_align_unchecked(size, align);

            std::alloc::dealloc(ptr as *mut u8, layout);
        }
        static VTABLE: SVecVTable = SVecVTable { dealloc };

        let mut value = ManuallyDrop::new(value);

        Self {
            ptr: value.as_mut_ptr(),
            len: value.len(),
            capacity: value.capacity(),
            vtable: &VTABLE,
        }
    }
}

impl<T: Debug> Debug for SVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        as_vec(self, move |v| v.fmt(f))
    }
}

impl<T: PartialEq> PartialEq for SVec<T> {
    fn eq(&self, other: &Self) -> bool {
        as_vec(self, move |v1| as_vec(other, move |v2| v1.eq(v2)))
    }
}

impl<T: Clone> Clone for SVec<T> {
    fn clone(&self) -> Self {
        SVec::from_vec(as_vec(self, move |v| v.clone()))
    }
}

impl<T> Drop for SVec<T> {
    fn drop(&mut self) {
        // Drop all elements
        unsafe { std::ptr::drop_in_place(std::ptr::slice_from_raw_parts_mut(self.ptr, self.len)) }
        // Deallocate if capacity not 0
        if self.capacity > 0 {
            let align = std::mem::align_of::<T>();
            let size = std::mem::size_of::<T>() * self.capacity;
            unsafe {
                (self.vtable.dealloc)(self.ptr as *mut u8, size, align);
            }
        }
    }
}
