use crate::{
    layout::{DefinedTypes, FullLayout, Layout},
    types::SBox,
    TypeUid, _TypeInfoImpl, id,
};

mod functions;

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
            layout: Layout::ConstPtr(SBox::new(layout)),
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
            layout: Layout::MutPtr(SBox::new(layout)),
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
                referent: SBox::new(layout),
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
                referent: SBox::new(layout),
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
                layout: SBox::new(layout),
            },
            defined_types,
        }
    }
}
