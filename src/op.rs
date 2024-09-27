use core::{ptr, slice};

/// Return the ceiling of the binary logarithm of `x`.
pub const fn log2_ceil(x: usize) -> usize {
    x.ilog2() as usize + !x.is_power_of_two() as usize
}

/// Rotate `n` elements to the left by `offset` starting at `s`.
pub unsafe fn rotate<T>(s: *mut T, n: usize, offset: usize) {
    let slice = slice::from_raw_parts_mut(s, n);
    if offset * 2 > n {
        slice.rotate_right(n - offset);
    } else {
        slice.rotate_left(offset);
    }
}

/// Perform either a swap or a copy to move a `slice` to start at `dst`, and return the new slice.
pub unsafe fn move_slice<'a, T, const S: bool>(dst: *mut T, slice: &'a mut [T]) -> &'a mut [T] {
    use crate::RawMut;
    let (src, len) = slice.raw_mut();
    write::<_, S>(src, dst, len);
    slice::from_raw_parts_mut(dst, len)
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
pub enum BisectionResult {
    /// The element was found.
    Exists(usize),
    /// The element was not found.
    Absent(usize),
}

/// Return the first `i` in `0..n` such that `!less(s.add(i), &*val)`, or `n` if not found, along
/// with a flag that identifies whether or not an comparatively equal element is found.
/// Cost: O(log n) comparisons.
pub fn search_unique<T, F: FnMut(&T, &T) -> bool>(
    s: *const T, n: usize, val: *const T, less: &mut F,
) -> BisectionResult {
    use BisectionResult::*;
    let i = search(s, n, val, less);
    if i == n || unsafe { less(&*val, &*s.add(i)) } { Absent(i) } else { Exists(i) }
}

/// Return the first `i` in `0..n` such that `!less(s.add(i), &*val)`, or `n` if not found.
/// Cost: O(log n) comparisons.
pub fn search<T, F: FnMut(&T, &T) -> bool>(
    s: *const T, n: usize, val: *const T, less: &mut F,
) -> usize {
    unsafe { lower_bound(n, |i| less(&*s.add(i), &*core::mem::ManuallyDrop::new(val.read()))) }
}

/// Return the first `i` in `0..v.len()` such that `!less(v[i], &*val)`, or `v.len()` if not found.
/// Cost: O(log n) comparisons.
pub fn search_slice<T, F: FnMut(&T, &T) -> bool>(v: &[T], val: *const T, less: &mut F) -> usize {
    search(v.as_ptr(), v.len(), val, less)
}

/// Return the first `i` in `0..n` such that `!f(i)`, or `n` if not found.
/// Behaviour is undefined only if `0..n` is not partitioned by any point `m` such that:
/// - `f(j)` for all `j` in `0..m`
/// - `!f(j)` for all `j` in `m..n`
///
/// Cost: O(log n) calls to `f`.
pub fn lower_bound(n: usize, mut f: impl FnMut(usize) -> bool) -> usize {
    // Source: https://orlp.net/blog/bitwise-binary-search/
    (0..=(n | 1).ilog2() as usize).rev().fold(0, |i, pow| {
        let low = (i | 1 << pow) - 1;
        if low < n && f(low) { low + 1 } else { i }
    })
}
