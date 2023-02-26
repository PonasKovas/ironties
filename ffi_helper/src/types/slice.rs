use std::{fmt::Debug, marker::PhantomData, ptr::NonNull};

#[repr(C)]
pub struct SSlice<'a, T> {
    ptr: NonNull<T>,
    len: usize,
    _phantom: PhantomData<&'a [T]>,
}

#[repr(C)]
pub struct SMutSlice<'a, T> {
    ptr: NonNull<T>,
    len: usize,
    _phantom: PhantomData<&'a mut [T]>,
}

impl<'a, T> SSlice<'a, T> {
    pub const fn from_slice(slice: &'a [T]) -> Self {
        Self {
            ptr: unsafe { NonNull::new_unchecked(slice.as_ptr() as *mut _) },
            len: slice.len(),
            _phantom: PhantomData,
        }
    }
    pub const fn into_slice(self) -> &'a [T] {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
    pub const fn as_slice<'b>(&'b self) -> &'b [T]
    where
        'a: 'b,
    {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl<'a, T: PartialEq> PartialEq for SSlice<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<'a, T: Debug> Debug for SSlice<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_slice().fmt(f)
    }
}

impl<'a, T> SMutSlice<'a, T> {
    pub fn from_slice(slice: &'a mut [T]) -> Self {
        Self {
            ptr: unsafe { NonNull::new_unchecked(slice.as_ptr() as *mut _) },
            len: slice.len(),
            _phantom: PhantomData,
        }
    }
    pub fn into_slice(self) -> &'a mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
    pub fn as_slice<'b>(&'b self) -> &'b mut [T]
    where
        'a: 'b,
    {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}

impl<'a, T: PartialEq> PartialEq for SMutSlice<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<'a, T: Debug> Debug for SMutSlice<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_slice().fmt(f)
    }
}
