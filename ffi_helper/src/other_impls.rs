use crate::{
    layout::{DefinedType, DefinedTypes, FullLayout, Layout, TypeType},
    types::{FfiSafeEquivalent, SStr, SVec},
    TypeUid, _TypeInfoImpl, id,
};

trait All {}
impl<T: ?Sized> All for T {}

macro_rules! impl_opaque {
    ($path:path where $($generic:ident [$($bound:tt),*]),* ) => {
        unsafe impl<$($generic : $($bound +)* All),*> _TypeInfoImpl for $path {
            const _UID: TypeUid = id!($path);

            fn _layout_impl(defined_types: DefinedTypes) -> FullLayout {
                let mut layouts = Vec::new();

                $(
                    let FullLayout {
                        layout,
                        mut defined_types,
                    } = $generic::_layout_impl(defined_types);
                    layouts.push(layout);
                )*

                match defined_types.iter().position(|t| t.0 == Self::_UID) {
                    Some(pos) => FullLayout {
                        layout: Layout::DefinedType { id: pos },
                        defined_types,
                    },
                    None => {
                        defined_types.push((
                            Self::_UID,
                            DefinedType {
                                name: SStr::from_normal(stringify!($path)),
                                ty: TypeType::StructUnnamed {
                                    fields: SVec::from_vec(layouts),
                                },
                            },
                        ));

                        FullLayout {
                            layout: Layout::DefinedType {
                                id: defined_types.len() - 1,
                            },
                            defined_types,
                        }
                    }
                }
            }
        }
    };
}

impl_opaque! {::std::mem::ManuallyDrop<T> where T [_TypeInfoImpl]}
impl_opaque! {::std::ptr::NonNull<T> where T [_TypeInfoImpl]}

unsafe impl<T: ?Sized> _TypeInfoImpl for std::marker::PhantomData<T> {
    const _UID: TypeUid = id!(::std::marker::PhantomData);

    fn _layout_impl(mut defined_types: DefinedTypes) -> FullLayout {
        match defined_types.iter().position(|t| t.0 == Self::_UID) {
            Some(pos) => FullLayout {
                layout: Layout::DefinedType { id: pos },
                defined_types,
            },
            None => {
                defined_types.push((
                    Self::_UID,
                    DefinedType {
                        name: SStr::from_normal("::std::marker::PhantomData"),
                        ty: TypeType::StructUnit,
                    },
                ));
                let my_type_id = defined_types.len() - 1;

                FullLayout {
                    layout: Layout::DefinedType { id: my_type_id },
                    defined_types,
                }
            }
        }
    }
}
