pub mod allocator;
mod r#box;
mod option;
mod result;
mod slice;
mod str;
mod tuple;
mod vec;

pub use self::str::SStr;
pub use option::SOption;
pub use r#box::SBox;
pub use result::SResult;
pub use slice::{SMutSlice, SSlice};
pub use tuple::*;
pub use vec::SVec;
