#![feature(allocator_api)]

extern crate self as ffi_helper;

#[doc(hidden)]
pub mod layout;
mod primitive_impls;
pub mod types;

pub use ffi_helper_derive::TypeInfo;

use layout::{DefinedType, DefinedTypes, FullLayout, Layout, TypeUid};
use types::SVec;

/// Implementation detail. Use the [`TypeInfo`] trait.
pub unsafe trait _TypeInfoImpl {
    #[doc(hidden)]
    const _UID: TypeUid;

    #[doc(hidden)]
    fn _layout_impl(defined_types: DefinedTypes) -> FullLayout;
}

/// An FFI-safe structure containing layout data of a type
///
/// Can be compared to make sure correct types are used (in FFI for example)
///
/// # Obtaining
///
/// Use the [`TypeInfo::layout`] method on any type which implements [`TypeInfo`] to get it's [`TypeLayout`]
///
/// # Limitations
///
/// As of now, [`TypeLayout`] does not encode lifetime data (that would require specialization, which Rust doesn't yet have)
#[repr(C)]
#[derive(Debug, PartialEq, Clone)]
pub struct TypeLayout {
    defined_types: SVec<DefinedType>,
    layout: Layout,
}

/// Allows to construct a [`TypeLayout`] of the type
pub trait TypeInfo: _TypeInfoImpl {
    fn layout() -> TypeLayout {
        let layout_impl = <Self as _TypeInfoImpl>::_layout_impl(Vec::new());

        TypeLayout {
            layout: layout_impl.layout,
            defined_types: SVec::from_std(
                layout_impl
                    .defined_types
                    .into_iter()
                    .map(|(_uid, ty)| ty)
                    .collect(),
            ),
        }
    }
}

impl<T: _TypeInfoImpl> TypeInfo for T {}

#[cfg(test)]
mod tests {
    use crate::TypeLayout;

    #[test]
    fn ffi_safe() {
        #[deny(improper_ctypes_definitions)]
        extern "C" fn _f(_: TypeLayout) {}
    }
}
