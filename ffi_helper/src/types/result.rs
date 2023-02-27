#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum SResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> SResult<T, E> {
    pub fn from_result(value: Result<T, E>) -> Self {
        match value {
            Ok(v) => Self::Ok(v),
            Err(e) => Self::Err(e),
        }
    }
    pub fn into_result(self) -> Result<T, E> {
        match self {
            SResult::Ok(v) => Ok(v),
            SResult::Err(e) => Err(e),
        }
    }
}

impl<T, E> From<Result<T, E>> for SResult<T, E> {
    fn from(value: Result<T, E>) -> Self {
        Self::from_result(value)
    }
}

impl<T, E> From<SResult<T, E>> for Result<T, E> {
    fn from(value: SResult<T, E>) -> Self {
        value.into_result()
    }
}
