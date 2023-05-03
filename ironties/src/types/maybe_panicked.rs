use std::panic::{catch_unwind, resume_unwind, UnwindSafe};

use super::SUnit;

#[repr(C)]
pub enum MaybePanicked<T = SUnit> {
    Ok(T),
    Panicked,
}

impl<T> MaybePanicked<T> {
    pub fn new<F: FnOnce() -> T + UnwindSafe>(f: F) -> Self {
        match catch_unwind(f) {
            Ok(v) => Self::Ok(v),
            Err(_) => Self::Panicked,
        }
    }
    pub fn unwrap(self) -> T {
        match self {
            Self::Ok(v) => v,
            Self::Panicked => resume_unwind(Box::new(())),
        }
    }
}
