pub mod allocator;
mod r#box;
mod option;
mod result;
mod slice;
mod str;
mod tuple;
mod vec;

pub use self::str::{SMutStr, SStr};
pub use option::SOption;
pub use r#box::SBox;
pub use result::SResult;
pub use slice::{SMutSlice, SSlice};
pub use tuple::*;
pub use vec::SVec;

use std::mem::ManuallyDrop;

pub trait FfiSafeEquivalent: Sized {
    type Normal;

    /// Converts an FFI-safe type to back it's normal equivalent
    fn from_normal(normal: Self::Normal) -> Self;
    /// Constructs an FFI-safe equivalent of this type
    fn into_normal(self) -> Self::Normal;

    fn as_normal<R, F>(&self, f: F) -> R
    where
        F: FnOnce(&Self::Normal) -> R,
    {
        // SAFETY: we create a bitwise copy for the same object, convert it to the normal equivalent (e. g. SVec to Vec)
        // give immutable access to it and make sure it doesn't get dropped to avoid a double-free.
        let copy = ManuallyDrop::new(unsafe { std::ptr::read(self).into_normal() });

        f(&*copy)
    }
    fn as_normal_mut<R, F>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut Self::Normal) -> R,
    {
        // SAFETY: we create a bitwise copy for the same object, convert it to the normal equivalent (e. g. SVec to Vec)
        // and give mutable access to it (the original (self) is never going to be used)
        let mut copy = unsafe { std::ptr::read(self) }.into_normal();

        let r = f(&mut copy);

        // SAFETY: and now overwrite the original
        unsafe { std::ptr::write(self, Self::from_normal(copy)) }

        r
    }
}
