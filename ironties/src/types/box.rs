use super::allocator::SGlobal;
use super::FfiSafeEquivalent;
use crate::TypeInfo;
use std::alloc::Allocator;
use std::borrow::{Borrow, BorrowMut};
use std::fmt::{Debug, Display, Pointer};
use std::hash::Hash;
use std::mem::{forget, ManuallyDrop};
use std::ops::{Deref, DerefMut};

/// FFI-safe equivalent of [`Box<T>`]
#[repr(C)]
#[derive(TypeInfo)]
pub struct SBox<T, A: Allocator = SGlobal> {
    ptr: *const T,
    // ManuallyDrop to avoid a double-free, because on drop (see Drop impl) the whole thing
    // will be converted to a Box and that dropped, which takes care of dropping the allocator.
    allocator: ManuallyDrop<A>,
}

impl<T, A: Allocator> FfiSafeEquivalent for SBox<T, A> {
    type Normal = Box<T, A>;

    fn from_normal(normal: Self::Normal) -> Self {
        let (ptr, allocator) = Box::into_raw_with_allocator(normal);

        Self {
            ptr,
            allocator: ManuallyDrop::new(allocator),
        }
    }
    fn into_normal(self) -> Self::Normal {
        // SAFETY: we construct a Box for the same object as our SBox, and then forget the original SBox to avoid a double-free.
        // We have to make a bitwise copy of the allocator here, because we need it by value, and SBox can't be
        // destructured (because it implements Drop). This is basically just a simple move, but explicit and manual, to satisfy the compiler.
        let copy =
            unsafe { Box::from_raw_in(self.ptr as *mut T, std::ptr::read(&*self.allocator)) };

        forget(self);

        copy
    }
}

impl<T> SBox<T, SGlobal> {
    /// Convenience method for converting a `Box<T, Global>` to `SBox` while simultaneously
    /// converting the allocator to `SGlobal`, which is required for the type to be FFI-safe and
    /// have a stable ABI
    pub fn from_box(value: Box<T, std::alloc::Global>) -> Self {
        let ptr = Box::into_raw(value);

        Self {
            ptr,
            allocator: ManuallyDrop::new(SGlobal::new()),
        }
    }
    /// Constructs a new `SBox<T, SGlobal>`
    pub fn new(value: T) -> Self {
        SBox::from_normal(Box::new_in(value, SGlobal::new()))
    }
}

impl<T, A: Allocator> Drop for SBox<T, A> {
    fn drop(&mut self) {
        // SAFETY: We make a Box for the same object as our SBox and drop it.
        unsafe { std::ptr::read(self) }.into_normal();
    }
}

impl<T, A: Allocator> AsMut<T> for SBox<T, A> {
    fn as_mut(&mut self) -> &mut T {
        // SAFETY: Converting the pointer to a mutable reference, which is safe, since
        // we know that it must point to valid data as long as the SBox lives.
        unsafe { (self.ptr as *mut T).as_mut().unwrap_unchecked() }
    }
}

impl<T, A: Allocator> AsRef<T> for SBox<T, A> {
    fn as_ref(&self) -> &T {
        // SAFETY: Converting the pointer to an immutable reference, which is safe, because
        // we know that it must point to valid data as long as the SBox lives.
        unsafe { self.ptr.as_ref().unwrap_unchecked() }
    }
}

impl<T, A: Allocator> Borrow<T> for SBox<T, A> {
    fn borrow(&self) -> &T {
        self.as_ref()
    }
}

impl<T, A: Allocator> BorrowMut<T> for SBox<T, A> {
    fn borrow_mut(&mut self) -> &mut T {
        self.as_mut()
    }
}

impl<T: Clone, A: Clone + Allocator> Clone for SBox<T, A> {
    fn clone(&self) -> Self {
        SBox::from_normal(Box::new_in(
            self.as_ref().clone(),
            ManuallyDrop::into_inner(self.allocator.clone()),
        ))
    }
}

impl<T: Debug, A: Allocator> Debug for SBox<T, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        T::fmt(self.as_ref(), f)
    }
}

impl<T: Default> Default for SBox<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T, A: Allocator> Deref for SBox<T, A> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T, A: Allocator> DerefMut for SBox<T, A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<T: Display, A: Allocator> Display for SBox<T, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl<T: Eq, A: Allocator> Eq for SBox<T, A> {
    fn assert_receiver_is_total_eq(&self) {
        T::assert_receiver_is_total_eq(self.as_ref())
    }
}

impl<T> From<T> for SBox<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T, A: Allocator> From<Box<T, A>> for SBox<T, A> {
    fn from(value: Box<T, A>) -> Self {
        Self::from_normal(value)
    }
}

impl<T: Hash, A: Allocator> Hash for SBox<T, A> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (**self).hash(state)
    }
}

impl<T: Ord, A: Allocator> Ord for SBox<T, A> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        T::cmp(self.as_ref(), other.as_ref())
    }
}

impl<T: PartialEq, A: Allocator> PartialEq for SBox<T, A> {
    fn eq(&self, other: &Self) -> bool {
        T::eq(self.as_ref(), other.as_ref())
    }
    // Possible optimizations
    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &Self) -> bool {
        T::ne(self.as_ref(), other.as_ref())
    }
}

impl<T: PartialOrd, A: Allocator> PartialOrd for SBox<T, A> {
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

impl<T, A: Allocator> Pointer for SBox<T, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Pointer::fmt(&self.ptr, f)
    }
}

impl<T, A: Allocator> Unpin for SBox<T, A> {}
