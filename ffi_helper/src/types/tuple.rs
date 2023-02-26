#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(C)]
pub struct STuple2<T1, T2>(pub T1, pub T2);
