use crate::TypeInfo;

#[derive(TypeInfo)]
struct Test<'a, 'b: 'a> {
    integer: &'a u64,
    c: &'b char,
    bool: &'static bool,
    complex: *const Test<'a, 'b>,
}

#[test]
fn test() {
    panic!("\n\nLAYOUT: {:?}\n\n", <Test>::layout());
}
