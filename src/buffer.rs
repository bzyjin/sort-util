use core::mem::MaybeUninit;
use core::slice;

/// Return a buffer of length `len`.
pub fn create<T>(len: usize) -> impl AsSliceMut<T> {
    Vec::with_capacity(len)
}

/// A utility trait to allow various data structures to be used as an external buffer.
pub trait AsSliceMut<T> {
    fn as_slice_mut(&mut self) -> &mut [T];
}

impl<T> AsSliceMut<T> for Vec<T> {
    fn as_slice_mut(&mut self) -> &mut [T] {
        unsafe {
            slice::from_raw_parts_mut(self.as_mut_ptr(), self.capacity())
        }
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
