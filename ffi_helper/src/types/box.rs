use crate::TypeInfo;
use std::borrow::{Borrow, BorrowMut};
use std::error::Error;
use std::fmt::{Debug, Display, Pointer};
use std::future::Future;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, Read, Seek, Write};
use std::iter::FusedIterator;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;

/// FFI-safe version of [`Box<T>`]
///
/// # Differences between [`Box<T>`]
///
/// - `T` must be `Sized` for [`SBox<T>`]
/// - [`SBox<T>`] is represented by 2 words in memory, instead of 1.
#[repr(C)]
#[derive(TypeInfo)]
pub struct SBox<T> {
    ptr: *mut T,
    drop: unsafe extern "C" fn(ptr: *mut T),
}

impl<T> SBox<T> {
    /// Converts a [`Box<T>`] to [`SBox<T>`].
    ///
    /// This operation is zero-cost, but can't be reversed (that is, you can't get a [`Box<T>`] back from an [`SBox<T>`])
    pub fn from_box(value: Box<T>) -> Self {
        unsafe extern "C" fn drop<T>(ptr: *mut T) {
            let _ = Box::from_raw(ptr);
        }
        Self {
            ptr: Box::into_raw(value),
            drop: drop::<T>,
        }
    }
    /// Consumes and leaks the [`SBox<T>`], returning a reference `&'a mut T` to the underlying data.
    pub fn leak<'a>(s: Self) -> &'a mut T {
        unsafe { s.ptr.as_mut().unwrap_unchecked() }
    }
    /// Constructs a new [`SBox<T>`]
    pub fn new(value: T) -> Self {
        SBox::from_box(Box::new(value))
    }
}

impl<T> AsMut<T> for SBox<T> {
    fn as_mut(&mut self) -> &mut T {
        unsafe { self.ptr.as_mut().unwrap_unchecked() }
    }
}

impl<T> AsRef<T> for SBox<T> {
    fn as_ref(&self) -> &T {
        unsafe { self.ptr.as_ref().unwrap_unchecked() }
    }
}

impl<T> Borrow<T> for SBox<T> {
    fn borrow(&self) -> &T {
        self.as_ref()
    }
}

impl<T> BorrowMut<T> for SBox<T> {
    fn borrow_mut(&mut self) -> &mut T {
        self.as_mut()
    }
}

impl<T: BufRead> BufRead for SBox<T> {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        T::fill_buf(self)
    }
    fn consume(&mut self, amt: usize) {
        T::consume(self, amt)
    }
    fn read_until(&mut self, byte: u8, buf: &mut Vec<u8>) -> std::io::Result<usize> {
        T::read_until(self, byte, buf)
    }
    fn read_line(&mut self, buf: &mut String) -> std::io::Result<usize> {
        T::read_line(self, buf)
    }
}

impl<T: Clone> Clone for SBox<T> {
    fn clone(&self) -> Self {
        SBox::new(self.as_ref().clone())
    }
}

impl<T: Debug> Debug for SBox<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        T::fmt(self.as_ref(), f)
    }
}

impl<T: Default> Default for SBox<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> Deref for SBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> DerefMut for SBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<T: Display> Display for SBox<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl<T: DoubleEndedIterator> DoubleEndedIterator for SBox<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        T::next_back(self)
    }
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        T::nth_back(self, n)
    }
}

impl<T> Drop for SBox<T> {
    fn drop(&mut self) {
        unsafe { (self.drop)(self.ptr) }
    }
}

impl<T: Eq> Eq for SBox<T> {
    fn assert_receiver_is_total_eq(&self) {
        T::assert_receiver_is_total_eq(self.as_ref())
    }
}

impl<T: Error> Error for SBox<T> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        T::source(self)
    }
    fn description(&self) -> &str {
        #[allow(deprecated)]
        T::description(self)
    }
    fn cause(&self) -> Option<&dyn Error> {
        #[allow(deprecated)]
        T::cause(self)
    }
}

impl<T: ExactSizeIterator> ExactSizeIterator for SBox<T> {
    fn len(&self) -> usize {
        T::len(self.as_ref())
    }
}

impl<T> From<T> for SBox<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: FusedIterator> FusedIterator for SBox<T> {}

impl<T: Future + Unpin> Future for SBox<T> {
    type Output = T::Output;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        T::poll(Pin::new(&mut **self), cx)
    }
}

impl<T: Hash> Hash for SBox<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        T::hash(self.as_ref(), state)
    }
}

impl<T: Hasher> Hasher for SBox<T> {
    fn finish(&self) -> u64 {
        T::finish(self.as_ref())
    }
    fn write(&mut self, bytes: &[u8]) {
        T::write(self.as_mut(), bytes)
    }
    fn write_u8(&mut self, i: u8) {
        T::write_u8(self.as_mut(), i)
    }
    fn write_u16(&mut self, i: u16) {
        T::write_u16(self.as_mut(), i)
    }
    fn write_u32(&mut self, i: u32) {
        T::write_u32(self.as_mut(), i)
    }
    fn write_u64(&mut self, i: u64) {
        T::write_u64(self.as_mut(), i)
    }
    fn write_u128(&mut self, i: u128) {
        T::write_u128(self.as_mut(), i)
    }
    fn write_usize(&mut self, i: usize) {
        T::write_usize(self.as_mut(), i)
    }
    fn write_i8(&mut self, i: i8) {
        T::write_i8(self.as_mut(), i)
    }
    fn write_i16(&mut self, i: i16) {
        T::write_i16(self.as_mut(), i)
    }
    fn write_i32(&mut self, i: i32) {
        T::write_i32(self.as_mut(), i)
    }
    fn write_i64(&mut self, i: i64) {
        T::write_i64(self.as_mut(), i)
    }
    fn write_i128(&mut self, i: i128) {
        T::write_i128(self.as_mut(), i)
    }
    fn write_isize(&mut self, i: isize) {
        T::write_isize(self.as_mut(), i)
    }
}

impl<T: Iterator> Iterator for SBox<T> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        T::next(self.as_mut())
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        T::size_hint(self.as_ref())
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        T::nth(self.as_mut(), n)
    }
}

impl<T: Ord> Ord for SBox<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        T::cmp(self.as_ref(), other.as_ref())
    }
}

impl<T: PartialEq> PartialEq for SBox<T> {
    fn eq(&self, other: &Self) -> bool {
        T::eq(self.as_ref(), other.as_ref())
    }
    fn ne(&self, other: &Self) -> bool {
        T::ne(self.as_ref(), other.as_ref())
    }
}

impl<T: PartialOrd> PartialOrd for SBox<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        T::partial_cmp(self.as_ref(), other.as_ref())
    }
    fn lt(&self, other: &Self) -> bool {
        T::lt(self.as_ref(), other.as_ref())
    }
    fn le(&self, other: &Self) -> bool {
        T::le(self.as_ref(), other.as_ref())
    }
    fn gt(&self, other: &Self) -> bool {
        T::gt(self.as_ref(), other.as_ref())
    }
    fn ge(&self, other: &Self) -> bool {
        T::ge(self.as_ref(), other.as_ref())
    }
}

impl<T> Pointer for SBox<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Pointer::fmt(&self.ptr, f)
    }
}

impl<T: Read> Read for SBox<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        T::read(self.as_mut(), buf)
    }
    fn read_vectored(&mut self, bufs: &mut [std::io::IoSliceMut<'_>]) -> std::io::Result<usize> {
        T::read_vectored(self.as_mut(), bufs)
    }
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> std::io::Result<usize> {
        T::read_to_end(self.as_mut(), buf)
    }
    fn read_to_string(&mut self, buf: &mut String) -> std::io::Result<usize> {
        T::read_to_string(self.as_mut(), buf)
    }
    fn read_exact(&mut self, buf: &mut [u8]) -> std::io::Result<()> {
        T::read_exact(self.as_mut(), buf)
    }
    fn by_ref(&mut self) -> &mut Self {
        self
    }
}

impl<T: Seek> Seek for SBox<T> {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        T::seek(self.as_mut(), pos)
    }
    fn stream_position(&mut self) -> std::io::Result<u64> {
        T::stream_position(self.as_mut())
    }
}

impl<T> Unpin for SBox<T> {}

impl<T: Write> Write for SBox<T> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        T::write(self.as_mut(), buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        T::flush(self.as_mut())
    }
    fn write_vectored(&mut self, bufs: &[std::io::IoSlice<'_>]) -> std::io::Result<usize> {
        T::write_vectored(self.as_mut(), bufs)
    }
    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        T::write_all(self.as_mut(), buf)
    }
    fn write_fmt(&mut self, fmt: std::fmt::Arguments<'_>) -> std::io::Result<()> {
        T::write_fmt(self.as_mut(), fmt)
    }
}
