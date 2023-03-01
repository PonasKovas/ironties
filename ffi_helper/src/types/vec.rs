use ffi_helper::TypeInfo;
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
struct SVecVTable {}

/// DO NOT ATTEMPT TO DO ANYTHING!!!
unsafe fn as_vec<T>(svec: &SVec<T>) -> ManuallyDrop<Vec<T>> {
    ManuallyDrop::new(Vec::from_raw_parts(svec.ptr, svec.len, svec.capacity))
}

impl<T> SVec<T> {
    pub fn from_vec(value: Vec<T>) -> Self {
        static VTABLE: SVecVTable = SVecVTable {};

        let value = ManuallyDrop::new(value);

        Self {
            ptr: value.as_ptr() as *mut _,
            len: value.len(),
            capacity: value.capacity(),
            vtable: &VTABLE,
        }
    }
}

impl<T: Debug> Debug for SVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (*unsafe { as_vec(self) }).fmt(f)
    }
}

impl<T: PartialEq> PartialEq for SVec<T> {
    fn eq(&self, other: &Self) -> bool {
        unsafe { (*as_vec(self)).eq(&*as_vec(other)) }
    }
}

impl<T: Clone> Clone for SVec<T> {
    fn clone(&self) -> Self {
        SVec::from_vec((*unsafe { as_vec(self) }).clone())
    }
}
