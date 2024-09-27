#[macro_export]
macro_rules! impl_sort_interface {
    (
        def $pdef:ident => $ldef:expr,
        min $pmin:ident => $lmin:expr,
        max $pmax:ident => $lmax:expr,
        $sort:expr
    ) => {
        use core::cmp::Ordering;

        use sort_util::buffer::{AsSliceMut};

        #[inline(always)]
        fn default_buffer_for<T>($pdef: &[T]) -> impl AsSliceMut<T> {
            buffer::create($ldef)
        }

        #[inline(always)]
        fn min_buffer_for<T>($pmin: &[T]) -> impl AsSliceMut<T> {
            buffer::create($lmin)
        }

        #[inline(always)]
        fn max_buffer_for<T>($pmax: &[T]) -> impl AsSliceMut<T> {
            buffer::create($lmax)
        }

        /// Sort `v`.
        #[inline(always)]
        pub fn sort<T: Ord>(v: &mut [T]) {
            sort_by(v, &mut T::cmp)
        }

        /// Sort `v` with a comparison function `f`.
        #[inline(always)]
        pub fn sort_by<T>(v: &mut [T], f: impl FnMut(&T, &T) -> Ordering) {
            sort_with_by(v, default_buffer_for(v), f)
        }

        /// Sort `v` with `aux` as a buffer.
        #[inline(always)]
        pub fn sort_with<T: Ord>(v: &mut [T], aux: impl AsSliceMut<T>) {
            sort_with_by(v, aux, &mut T::cmp)
        }

        /// Sort `v` with `aux` as a buffer and `f` as a comparison function.
        #[inline(always)]
        pub fn sort_with_by<T>(
            v: &mut [T], mut aux: impl AsSliceMut<T>, mut f: impl FnMut(&T, &T) -> Ordering,
        ) {
            ($sort)(v, aux.as_slice_mut(), &mut |x, y| f(x, y) == Ordering::Less)
        }

        /// Sort `v` with minimum memory settings.
        #[inline(always)]
        pub fn sort_min<T: Ord>(v: &mut [T]) {
            sort_with(v, min_buffer_for(v))
        }

        /// Sort `v` with minimum memory settings and `f` as a comparison function.
        #[inline(always)]
        pub fn sort_min_by<T>(v: &mut [T], f: impl FnMut(&T, &T) -> Ordering) {
            sort_with_by(v, min_buffer_for(v), f)
        }

        /// Sort `v` with maximum memory settings.
        #[inline(always)]
        pub fn sort_max<T: Ord>(v: &mut [T]) {
            sort_with(v, max_buffer_for(v))
        }

        /// Sort `v` with maximum memory settings and `f` as a comparison function.
        #[inline(always)]
        pub fn sort_max_by<T>(v: &mut [T], f: impl FnMut(&T, &T) -> Ordering) {
            sort_with_by(v, max_buffer_for(v), f)
        }
    }
}
