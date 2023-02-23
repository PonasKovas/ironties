extern crate self as ffi_helper;

mod cdefinitions;
mod primitive_impls;

#[allow(non_camel_case_types)]
pub unsafe trait CType {
    #[doc(hidden)]
    fn _prefix() -> String;
    #[doc(hidden)]
    fn _definitions(defs: &mut cdefinitions::CDefinitions);
}

use ffi_helper_derive::CType;
#[derive(CType)]
struct Test {
    integer: u64,
    boolean: bool,
    complex: *const Test,
}

#[test]
fn test() {
    type T = Test; //*mut [*const bool; 2];
    let mut defs = cdefinitions::CDefinitions::new();
    T::_definitions(&mut defs);
    panic!(
        "\n\nVAR: {{{} some_var}}\n\nDEFINITIONS: {:?}\n\n",
        <T>::_prefix(),
        defs.render()
    );
}
