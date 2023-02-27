use crate::TypeInfo;

#[derive(TypeInfo)]
#[repr(u8)]
enum Test {
    Zero,
    First(u32) = 1,
    Second = 3,
    Third { a: *mut u8, b: &'static Test },
    Fourth = 100,
    Fifth,
    Sixth = 50,
    Seventh,
    Eighth,
    Ninth,
    Tenth,
}

#[test]
fn test() {
    panic!("\n\nLAYOUT: {:?}\n\n", <Test>::layout());
}
