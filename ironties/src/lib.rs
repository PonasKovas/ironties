#![feature(allocator_api)]

extern crate self as ironties;

#[doc(hidden)]
pub mod layout;
mod other_impls;
mod primitive_impls;
pub mod types;

pub use ironties_derive::TypeInfo;

use layout::{DefinedType, DefinedTypes, FullLayout, Layout, TypeUid};
use types::SVec;

/// Implementation detail. Use the [`TypeInfo`] trait.
#[allow(clippy::missing_safety_doc)]
pub unsafe trait _TypeInfoImpl {
    #[doc(hidden)]
    const _UID: TypeUid;

    #[doc(hidden)]
    fn _layout_impl(defined_types: DefinedTypes) -> FullLayout;
}

/// An FFI-safe structure containing layout data of a type
///
/// Can be compared to make sure correct types are used in contexts where compile time checks are not available (FFI for example)
///
/// # Obtaining
///
/// Use the [`TypeInfo::layout`] method on any type which implements [`TypeInfo`] to get it's [`TypeLayout`]
///
/// # Limitations
///
/// As of yet, [`TypeLayout`] does not encode lifetime information
#[repr(C)]
#[derive(TypeInfo, Debug, PartialEq, Clone)]
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
            defined_types: SVec::from_vec(
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

#[rustfmt::skip]
macro_rules! id {
    ($($name:tt)+) => {
        TypeUid {
            rustpath: stringify!($($name)+),
            file: file!(),
            line: line!(),
            column: column!(),
        }
    };
}
pub(crate) use id;

#[cfg(test)]
mod tests {
    use crate::{TypeInfo, TypeLayout};

    #[test]
    fn ffi_safe() {
        #[deny(improper_ctypes_definitions)]
        extern "C" fn _f(_: TypeLayout) {}
    }

    #[test]
    fn self_test() {
        assert_eq!(TypeLayout::layout(), TypeLayout::layout());
    }
}
