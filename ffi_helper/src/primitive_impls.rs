use crate::{
    layout::{DefinedType, DefinedTypes, FullLayout, Layout, TypeType},
    types::{SBox, SStr},
    TypeUid, _TypeInfoImpl,
};

#[rustfmt::skip]
macro_rules! id {
    ($($name:tt)+) => {
        TypeUid {
            rustpath: SStr::from_str(stringify!($($name)+)),
            file: SStr::from_str(file!()),
            line: line!(),
            column: column!(),
        }
    };
}

macro_rules! impl_primitives {
    ( $( $name:ty = $layout:ident),* ) => {
    	$(
    		unsafe impl $crate::_TypeInfoImpl for $name {
                const _UID: TypeUid = id!($name);

                fn _layout_impl(defined_types: DefinedTypes) -> FullLayout {
                    FullLayout {
                        layout: Layout::$layout,
                        defined_types,
                    }
                }
	        }
    	)*
    };
}
impl_primitives!(
    u8 = U8,
    u16 = U16,
    u32 = U32,
    u64 = U64,
    usize = USize,
    i8 = I8,
    i16 = I16,
    i32 = I32,
    i64 = I64,
    isize = ISize,
    f32 = F32,
    f64 = F64,
    bool = Bool,
    char = Char,
    () = Void
);

unsafe impl<T: _TypeInfoImpl> _TypeInfoImpl for *const T {
    const _UID: TypeUid = id!(*const T);

    fn _layout_impl(defined_types: DefinedTypes) -> FullLayout {
        let FullLayout {
            layout,
            defined_types,
        } = T::_layout_impl(defined_types);

        FullLayout {
            layout: Layout::ConstPtr(SBox::from_box(Box::new(layout))),
            defined_types,
        }
    }
}

unsafe impl<T: _TypeInfoImpl> _TypeInfoImpl for *mut T {
    const _UID: TypeUid = id!(*mut T);

    fn _layout_impl(defined_types: DefinedTypes) -> FullLayout {
        let FullLayout {
            layout,
            defined_types,
        } = T::_layout_impl(defined_types);

        FullLayout {
            layout: Layout::MutPtr(SBox::from_box(Box::new(layout))),
            defined_types,
        }
    }
}

unsafe impl<'a, T: _TypeInfoImpl> _TypeInfoImpl for &'a T {
    const _UID: TypeUid = id!(&T);

    fn _layout_impl(defined_types: DefinedTypes) -> FullLayout {
        let FullLayout {
            layout,
            defined_types,
        } = T::_layout_impl(defined_types);

        FullLayout {
            layout: Layout::Ref {
                referent: SBox::from_box(Box::new(layout)),
            },
            defined_types,
        }
    }
}

unsafe impl<'a, T: _TypeInfoImpl> _TypeInfoImpl for &'a mut T {
    const _UID: TypeUid = id!(&mut T);

    fn _layout_impl(defined_types: DefinedTypes) -> FullLayout {
        let FullLayout {
            layout,
            defined_types,
        } = T::_layout_impl(defined_types);

        FullLayout {
            layout: Layout::MutRef {
                referent: SBox::from_box(Box::new(layout)),
            },
            defined_types,
        }
    }
}

unsafe impl<const N: usize, T: _TypeInfoImpl> _TypeInfoImpl for [T; N] {
    const _UID: TypeUid = id!([T; N]);

    fn _layout_impl(defined_types: DefinedTypes) -> FullLayout {
        let FullLayout {
            layout,
            defined_types,
        } = T::_layout_impl(defined_types);

        FullLayout {
            layout: Layout::Array {
                len: N,
                layout: SBox::from_box(Box::new(layout)),
            },
            defined_types,
        }
    }
}

unsafe impl<T: ?Sized> _TypeInfoImpl for std::marker::PhantomData<T> {
    const _UID: TypeUid = id!(PhantomData);

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
                        name: SStr::from_str("::std::marker::PhantomData"),
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
