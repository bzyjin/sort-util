use core::ptr;

use crate::GenerateSlice;

/// Return the ceiling of the binary logarithm of `x`.
pub const fn log2_ceil(x: usize) -> usize {
    x.ilog2() as usize + !x.is_power_of_two() as usize
}

/// Rotate `n` elements to the left by `offset` starting at `s`.
/// Cost: `O(n)` swaps.
pub unsafe fn rotate<T>(s: *mut T, n: usize, offset: usize) {
    s.crop(0..n).rotate_left(offset);
}

/// Shift the value at `s` to be `n` elements to the left.
/// Cost: `O(n)` writes.
pub unsafe fn insert_left<T>(s: *mut T, n: usize) {
    if n != 0 {
        let val = core::mem::ManuallyDrop::new(s.read());
        ptr::copy(s.sub(n), s.sub(n - 1), n);
        ptr::copy_nonoverlapping(&*val, s.sub(n), 1);
    }
}

/// Shift the value at `s` to be `n` elements to the right.
/// Cost: `O(n)` writes.
pub unsafe fn insert_right<T>(s: *mut T, n: usize) {
    if n != 0 {
        let val = core::mem::ManuallyDrop::new(s.read());
        ptr::copy(s.add(1), s, n);
        ptr::copy_nonoverlapping(&*val, s.add(n), 1);
    }
}

/// Perform either a swap or a copy to move a `slice` to start at `dst`, and return the new slice.
/// Cost: `O(n)` writes.
pub unsafe fn move_slice<'a, T, const S: bool>(dst: *mut T, slice: &'a mut [T]) -> &'a mut [T] {
    use crate::RawMut;
    let (src, len) = slice.raw_mut();
    write::<_, S>(src, dst, len);
    dst.crop(0..len)
}

/// Perform either a swap between `src` and `dst` or a copy from `src` to `dst` of `count` elements.
#[inline(always)]
pub unsafe fn write<T, const S: bool>(src: *mut T, dst: *mut T, count: usize) {
    if S {
        ptr::swap_nonoverlapping(src, dst, count)
    } else {
        ptr::copy_nonoverlapping(src, dst, count)
    }
}

/// Stores the result of searching for an element.
pub struct Found(pub bool, pub usize);

impl core::ops::Add<usize> for Found {
    type Output = Self;

    /// Add `rhs` to the index property.
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0, self.1 + rhs)
    }
}

/// Return the first `i` in `0..n` such that `!less(s.add(i), &*val)`, or `n` if not found, along
/// with a flag that identifies whether or not an comparatively equal element is found.
///
/// Cost: `O(log n)` comparisons.
pub fn search_unique<T, F: FnMut(&T, &T) -> bool>(
    s: *const T, n: usize, val: *const T, less: &mut F,
) -> Found {
    let i = search::binary(s, n, val, less);
    Found(i != n && unsafe { !less(&*val, &*s.add(i)) }, i)
}

/// Set of functions that, given:
/// - `s`, an origin pointer
/// - `n`, the number of contiguous elements to search through
/// - `val`, the element to compare by
/// - `less`, a comparison function
/// return the first `i` in `0..n` such that `!less(s.add(i), &*val)`, or `n` if not found, along
/// with a flag that identifies whether or not an comparatively equal element is found.
///
/// Cost: `O(log n)` comparisons.
pub mod search {
    use super::lower_bound;

    /// Return the first `i` in `0..n` such that `!less(s.add(i), &*val)`, or `n` if not found.
    /// Cost: `O(n)` comparisons.
    #[inline(always)]
    pub fn linear<T, F: FnMut(&T, &T) -> bool>(
        s: *const T, n: usize, val: *const T, less: &mut F,
    ) -> usize {
        unsafe {
            lower_bound::linear(n, |i| less(&*s.add(i), &*core::mem::ManuallyDrop::new(val.read())))
        }
    }

    /// Return the first `i` in `0..n` such that `!less(s.add(i), &*val)`, or `n` if not found.
    /// Behaviour is undefined only if `0..n` is not sorted non-descending by `less`.
    ///
    /// Cost: `O(log n)` comparisons.
    #[inline(always)]
    pub fn binary<T, F: FnMut(&T, &T) -> bool>(
        s: *const T, n: usize, val: *const T, less: &mut F,
    ) -> usize {
        unsafe {
            lower_bound::binary(n, |i| less(&*s.add(i), &*core::mem::ManuallyDrop::new(val.read())))
        }
    }
}

/// Set of functions that, given a maximum value `n` and a predicate `f`, return the first `i` in
/// `0..n` such that `!f(i)`, or `n` if not found.
pub mod lower_bound {
    /// Cost: `O(n)` calls to `f`.
    pub fn linear(n: usize, mut f: impl FnMut(usize) -> bool) -> usize {
        (0..n).find(|&i| !f(i)).unwrap_or(n)
    }

    /// Behaviour is undefined only if `0..n` is not partitioned by any point `m` such that:
    /// - `f(j)` for all `j` in `0..m`
    /// - `!f(j)` for all `j` in `m..n`
    ///
    /// Cost: `O(log n)` calls to `f`.
    pub fn binary(n: usize, mut f: impl FnMut(usize) -> bool) -> usize {
        // Source: https://orlp.net/blog/bitwise-binary-search/
        (0..=(n | 1).ilog2() as usize)
            .rev()
            .fold(0, |i, k| i | ((i | 1 << k <= n && f((i | 1 << k) - 1)) as usize) << k)
    }
}
