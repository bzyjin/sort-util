#![no_std]

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
        core::slice::from_raw_parts_mut(self, end.offset_from(self) as usize)
    }
}

/// Flag for indicating the result of a sorting operation.
#[derive(PartialEq, Eq)]
pub enum Sorted {
    /// Sorted successfully.
    Done,
    /// Could not sort.
    Fail,
}

impl Sorted {
    /// Propogate a `Done` value or return the result of calling `f()` with lazy evaluation.
    /// Associative and commutative.
    pub fn or(self, mut f: impl FnMut() -> Self) -> Self {
        if self == Self::Done { self } else { f() }
    }

    /// Propogate a `Fail` value or return the result of calling `f()` with lazy evaluation.
    /// Associative and commutative.
    pub fn and(self, mut f: impl FnMut() -> Self) -> Self {
        if self == Self::Fail { self } else { f() }
    }
}
