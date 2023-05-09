use super::id;
use crate::{
    layout::{DefinedTypes, FullLayout, Layout},
    types::{FfiSafeEquivalent, SBox, SStr, SVec},
    TypeUid, _TypeInfoImpl,
};

/// Reverse the order of a sequence of ident tokens.
macro_rules! rev_args {
    ($($swapped:ident)* |) => {
        impl_with_args!($($swapped)*);
    };
    ($($swapped:ident)* | $first_arg:ident $($arg:ident)*) => {
        rev_args!($first_arg $($swapped)* | $($arg)*);
    };
}

/// Impl `CallableOnce` given set of arguments.
macro_rules! impl_with_args {
    ($($arg:ident)*) => {
        unsafe impl<R: _TypeInfoImpl $(, $arg : _TypeInfoImpl)*> _TypeInfoImpl for unsafe extern fn($($arg),*) -> R {
            const _UID: TypeUid = id!(unsafe extern fn($($arg),*) -> R);

            fn _layout_impl(defined_types: DefinedTypes) -> FullLayout {
                let FullLayout {
                    layout: return_layout,
                    defined_types,
                } = R::_layout_impl(defined_types);

                #[allow(unused_mut)]
                let mut args = Vec::new();

                $(
                    let FullLayout {
                        layout,
                        defined_types,
                    } = $arg::_layout_impl(defined_types);
                    args.push(layout);
                )*


                FullLayout {
                    layout: Layout::FunctionPointer {
                        is_unsafe: true,
                        abi: SStr::from_normal("C"),
                        args: SVec::from_vec(args),
                        return_ty: SBox::new(return_layout),
                    },
                    defined_types,
                }
            }
        }

        unsafe impl<R: _TypeInfoImpl $(, $arg : _TypeInfoImpl)*> _TypeInfoImpl for unsafe fn($($arg),*) -> R {
            const _UID: TypeUid = id!(unsafe fn($($arg),*) -> R);

            fn _layout_impl(defined_types: DefinedTypes) -> FullLayout {
                let FullLayout {
                    layout: return_layout,
                    defined_types,
                } = R::_layout_impl(defined_types);

                #[allow(unused_mut)]
                let mut args = Vec::new();

                $(
                    let FullLayout {
                        layout,
                        defined_types,
                    } = $arg::_layout_impl(defined_types);
                    args.push(layout);
                )*


                FullLayout {
                    layout: Layout::FunctionPointer {
                        is_unsafe: true,
                        abi: SStr::from_normal("Rust"),
                        args: SVec::from_vec(args),
                        return_ty: SBox::new(return_layout),
                    },
                    defined_types,
                }
            }
        }

        unsafe impl<R: _TypeInfoImpl $(, $arg : _TypeInfoImpl)*> _TypeInfoImpl for extern fn($($arg),*) -> R {
            const _UID: TypeUid = id!(extern fn($($arg),*) -> R);

            fn _layout_impl(defined_types: DefinedTypes) -> FullLayout {
                let FullLayout {
                    layout: return_layout,
                    defined_types,
                } = R::_layout_impl(defined_types);

                #[allow(unused_mut)]
                let mut args = Vec::new();

                $(
                    let FullLayout {
                        layout,
                        defined_types,
                    } = $arg::_layout_impl(defined_types);
                    args.push(layout);
                )*


                FullLayout {
                    layout: Layout::FunctionPointer {
                        is_unsafe: true,
                        abi: SStr::from_normal("C"),
                        args: SVec::from_vec(args),
                        return_ty: SBox::new(return_layout),
                    },
                    defined_types,
                }
            }
        }

        unsafe impl<R: _TypeInfoImpl $(, $arg : _TypeInfoImpl)*> _TypeInfoImpl for fn($($arg),*) -> R {
            const _UID: TypeUid = id!(fn($($arg),*) -> R);

            fn _layout_impl(defined_types: DefinedTypes) -> FullLayout {
                let FullLayout {
                    layout: return_layout,
                    defined_types,
                } = R::_layout_impl(defined_types);

                #[allow(unused_mut)]
                let mut args = Vec::new();

                $(
                    let FullLayout {
                        layout,
                        defined_types,
                    } = $arg::_layout_impl(defined_types);
                    args.push(layout);
                )*


                FullLayout {
                    layout: Layout::FunctionPointer {
                        is_unsafe: false,
                        abi: SStr::from_normal("Rust"),
                        args: SVec::from_vec(args),
                        return_ty: SBox::new(return_layout),
                    },
                    defined_types,
                }
            }
        }
    };
}

/// Recursively impl `CallableOnce` for all param amounts.
macro_rules! impl_with_args_all {
    () => {
        impl_with_args!();
    };
    ($last_arg:ident $($arg:ident)*) => {
        impl_with_args_all!($($arg)*);
        rev_args!(| $last_arg $($arg)*);
    };
}

// Implement for everything with 30 params or less.
impl_with_args_all!(A30 A29 A28 A27 A26 A25 A24 A23 A22 A21 A20 A19 A18 A17 A16 A15 A14 A13 A12 A11 A10 A9 A8 A7 A6 A5 A4 A3 A2 A1);
