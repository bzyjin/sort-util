#![feature(maybe_uninit_slice, ptr_sub_ptr)]

pub mod buffer;
pub mod op;

/// An extension trait to serve as syntax sugar for "immutable decomposition" operations.
pub trait Raw<T> {
    type Output;
    fn raw(self) -> Self::Output;
}

/// An extension trait to serve as syntax sugar for "mutable decomposition" operations.
pub trait RawMut<T> {
    type Output;
    fn raw_mut(self) -> Self::Output;
}

impl<T> Raw<T> for &[T] {
    type Output = (*const T, usize);

    /// Return this slice as its fat pointer components.
    fn raw(self) -> Self::Output {
        (self.as_ptr(), self.len())
    }
}

impl<T> RawMut<T> for &mut [T] {
    type Output = (*mut T, usize);

    /// Return this mutable slice as its fat pointer components.
    fn raw_mut(self) -> Self::Output {
        (self.as_mut_ptr(), self.len())
    }
}

/// An extension trait to serve as syntax sugar for constructing slices.
pub trait GenerateSlice<T> {
    unsafe fn crop<'a>(self, range: core::ops::Range<usize>) -> &'a mut [T];
    unsafe fn to<'a>(self, end: Self) -> &'a mut [T];
}

impl<T> GenerateSlice<T> for *mut T {
    /// Return a slice of the elements on a `range` starting at `self`.
    unsafe fn crop<'a>(self, range: core::ops::Range<usize>) -> &'a mut [T] {
        core::slice::from_raw_parts_mut(self.add(range.start), range.len())
    }

    /// Return a slice of the elements starting at `self` and ending before `end`.
    unsafe fn to<'a>(self, end: Self) -> &'a mut [T] {
        core::slice::from_raw_parts_mut(self, end.sub_ptr(self))
    }
}
