use crate::{
    layout::{DefinedTypes, FullLayout, Layout, Lifetime},
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

                fn _layout_impl(defined_types: DefinedTypes, lifetimes: Vec<Lifetime>) -> FullLayout {
                    FullLayout {
                        layout: Layout::$layout,
                        defined_types,
                        lifetimes
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
    char = Char
);

unsafe impl<T: _TypeInfoImpl> _TypeInfoImpl for *const T {
    const _UID: TypeUid = id!(*const T);

    fn _layout_impl(defined_types: DefinedTypes, lifetimes: Vec<Lifetime>) -> FullLayout {
        let FullLayout {
            layout,
            defined_types,
            lifetimes,
        } = T::_layout_impl(defined_types, lifetimes);

        FullLayout {
            layout: Layout::ConstPtr(SBox::from_box(Box::new(layout))),
            defined_types,
            lifetimes,
        }
    }
}

unsafe impl<T: _TypeInfoImpl> _TypeInfoImpl for *mut T {
    const _UID: TypeUid = id!(*mut T);

    fn _layout_impl(defined_types: DefinedTypes, lifetimes: Vec<Lifetime>) -> FullLayout {
        let FullLayout {
            layout,
            defined_types,
            lifetimes,
        } = T::_layout_impl(defined_types, lifetimes);

        FullLayout {
            layout: Layout::MutPtr(SBox::from_box(Box::new(layout))),
            defined_types,
            lifetimes,
        }
    }
}

unsafe impl<'a, T: _TypeInfoImpl> _TypeInfoImpl for &'a T {
    const _UID: TypeUid = id!(&T);

    fn _layout_impl(defined_types: DefinedTypes, lifetimes: Vec<Lifetime>) -> FullLayout {
        let FullLayout {
            layout,
            defined_types,
            mut lifetimes,
        } = T::_layout_impl(defined_types, lifetimes);

        lifetimes.push(Lifetime::Unbound);
        let lifetime_id = lifetimes.len() - 1;

        FullLayout {
            layout: Layout::Ref {
                referent: SBox::from_box(Box::new(layout)),
                lifetime: lifetime_id,
            },
            defined_types,
            lifetimes,
        }
    }
}

unsafe impl<'a, T: _TypeInfoImpl> _TypeInfoImpl for &'a mut T {
    const _UID: TypeUid = id!(&mut T);

    fn _layout_impl(defined_types: DefinedTypes, lifetimes: Vec<Lifetime>) -> FullLayout {
        let FullLayout {
            layout,
            defined_types,
            mut lifetimes,
        } = T::_layout_impl(defined_types, lifetimes);

        lifetimes.push(Lifetime::Unbound);
        let lifetime_id = lifetimes.len() - 1;

        FullLayout {
            layout: Layout::MutRef {
                referent: SBox::from_box(Box::new(layout)),
                lifetime: lifetime_id,
            },
            defined_types,
            lifetimes,
        }
    }
}

unsafe impl<const N: usize, T: _TypeInfoImpl> _TypeInfoImpl for [T; N] {
    const _UID: TypeUid = id!([T; N]);

    fn _layout_impl(defined_types: DefinedTypes, lifetimes: Vec<Lifetime>) -> FullLayout {
        let FullLayout {
            layout,
            defined_types,
            lifetimes,
        } = T::_layout_impl(defined_types, lifetimes);

        FullLayout {
            layout: Layout::Array {
                len: N,
                layout: SBox::from_box(Box::new(layout)),
            },
            defined_types,
            lifetimes,
        }
    }
}

unsafe impl<T: ?Sized> _TypeInfoImpl for std::marker::PhantomData<T> {
    const _UID: TypeUid = id!(PhantomData);

    fn _layout_impl(defined_types: DefinedTypes, lifetimes: Vec<Lifetime>) -> FullLayout {
        FullLayout {
            layout: Layout::Void,
            defined_types,
            lifetimes,
        }
    }
}
