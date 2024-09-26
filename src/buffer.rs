use core::{fmt::Debug, mem::MaybeUninit, slice};

/// Return a buffer of length `len`.
pub fn create<T>(len: usize) -> SortBuffer<T> {
    SortBuffer::with_capacity(len)
}

/// A heap-allocated buffer comprising an underlying vector.
#[repr(transparent)]
pub struct SortBuffer<T> {
    inner: Vec<T>,
}

impl<T> SortBuffer<T> {
    /// Return a new buffer with specified `capacity`.
    pub fn with_capacity(capacity: usize) -> Self {
        Self { inner: Vec::with_capacity(capacity) }
    }

    /// Resize this buffer.
    pub fn resize(&mut self, capacity: usize) {
        if capacity >= self.len() {
            Vec::reserve_exact(&mut self.inner, capacity)
        } else {
            Vec::shrink_to(&mut self.inner, capacity)
        }
    }

    /// Return the length of this buffer.
    pub fn len(&self) -> usize {
        self.inner.capacity()
    }
}

/// A utility trait to allow various data structures to be used as an external buffer.
pub trait AsSliceMut<T> {
    fn as_slice_mut(&mut self) -> &mut [T];
}

impl<T> AsSliceMut<T> for SortBuffer<T> {
    fn as_slice_mut(&mut self) -> &mut [T] {
        unsafe {
            slice::from_raw_parts_mut(self.inner.as_mut_ptr(), self.inner.capacity())
        }
    }
}

impl<T> AsSliceMut<T> for Vec<T> {
    fn as_slice_mut(&mut self) -> &mut [T] {
        self
    }
}

impl<T> AsSliceMut<T> for &mut [T] {
    fn as_slice_mut(&mut self) -> &mut [T] {
        self
    }
}

impl<T> AsSliceMut<T> for &mut [MaybeUninit<T>] {
    fn as_slice_mut(&mut self) -> &mut [T] {
        unsafe {
            slice::from_raw_parts_mut(MaybeUninit::slice_as_mut_ptr(self), self.len())
        }
    }
}

impl<T, R: AsSliceMut<T>> AsSliceMut<T> for &mut R {
    fn as_slice_mut(&mut self) -> &mut [T] {
        (*self).as_slice_mut()
    }
}

impl<T: Debug> Debug for SortBuffer<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        unsafe {
            slice::from_raw_parts(self.inner.as_ptr(), self.inner.capacity()).fmt(f)
        }
    }
}
