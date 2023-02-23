use crate::cdefinitions::CDefinitions;

fn array_name<const N: usize, T: CType>() -> String {
    format!(
        "{}_array_{N}",
        T::_prefix().replace(' ', "_").replace('*', "ptr")
    )
}

macro_rules! impl_primitives {
    ( $( $r:ty = $c:ident),* ) => {
    	$(
    		unsafe impl $crate::CType for $r {
                fn _prefix() -> String {
                    stringify!($c).to_owned()
                }
                fn _definitions(defs: &mut CDefinitions) {
                    defs.includes.insert("stdint.h".to_owned());
                }
	        }
    	)*
    };
}
impl_primitives!(
    u8 = uint8_t,
    u16 = uint16_t,
    u32 = uint32_t,
    u64 = uint64_t,
    usize = uintptr_t,
    i8 = int8_t,
    i16 = int16_t,
    i32 = int32_t,
    i64 = int64_t,
    isize = intptr_t,
    f32 = float,
    f64 = double,
    char = uint32_t
);

use crate::CType;

unsafe impl CType for bool {
    fn _prefix() -> String {
        "bool".to_owned()
    }
    fn _definitions(defs: &mut CDefinitions) {
        defs.includes.insert("stdbool.h".to_owned());
    }
}

unsafe impl<T: CType> CType for *const T {
    fn _prefix() -> String {
        format!("{}*", T::_prefix())
    }
    fn _definitions(defs: &mut CDefinitions) {
        defs.extend_once::<T>();
    }
}

unsafe impl<T: CType> CType for *mut T {
    fn _prefix() -> String {
        format!("{}*", T::_prefix())
    }
    fn _definitions(defs: &mut CDefinitions) {
        defs.extend_once::<T>();
    }
}

unsafe impl<'a, T: CType> CType for &'a T {
    fn _prefix() -> String {
        format!("{}*", T::_prefix())
    }
    fn _definitions(defs: &mut CDefinitions) {
        defs.extend_once::<T>();
    }
}

unsafe impl<'a, T: CType> CType for &'a mut T {
    fn _prefix() -> String {
        format!("{}*", T::_prefix())
    }
    fn _definitions(defs: &mut CDefinitions) {
        defs.extend_once::<T>();
    }
}

unsafe impl<const N: usize, T: CType> CType for [T; N] {
    fn _prefix() -> String {
        array_name::<N, T>()
    }
    fn _definitions(defs: &mut CDefinitions) {
        defs.extend_once::<T>();
        let prefix = T::_prefix();
        let name = array_name::<N, T>();
        defs.types.insert(
            std::any::type_name::<T>().to_owned(),
            format!("typedef struct {{ {prefix} data[{N}]; }} {name};"),
        );
    }
}

unsafe impl<T: ?Sized> CType for std::marker::PhantomData<T> {
    fn _prefix() -> String {
        format!("void")
    }
    fn _definitions(_defs: &mut CDefinitions) {}
}
