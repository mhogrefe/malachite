use num::basic::traits::Zero;
use num::random::{random_unsigneds_less_than, RandomUnsignedsLessThan};
use random::Seed;

/// Sets all values in a slice to 0.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Examples
/// ```
/// use malachite_base::slices::slice_set_zero;
///
/// let mut xs = [1, 2, 3, 4, 5];
/// slice_set_zero::<u32>(&mut xs[1..4]);
/// assert_eq!(xs, [1, 0, 0, 0, 5]);
/// ```
///
/// This is mpn_zero from mpn/generic/zero.c, GMP 6.1.2. Note that this is needed less often in
/// Malachite than in GMP, since Malachite generally initializes new memory with zeros.
pub fn slice_set_zero<T: Zero>(xs: &mut [T]) {
    for x in xs.iter_mut() {
        *x = T::ZERO;
    }
}

/// Tests whether all values in a slice are equal to 0.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Examples
/// ```
/// use malachite_base::slices::slice_test_zero;
///
/// assert!(slice_test_zero::<u32>(&[0, 0, 0]));
/// assert!(!slice_test_zero::<u32>(&[0, 1, 0]));
/// ```
///
/// This is mpn_zero_p from gmp.h, GMP 6.1.2.
pub fn slice_test_zero<T: Eq + Zero>(xs: &[T]) -> bool {
    let zero = T::ZERO;
    xs.iter().all(|x| x == &zero)
}

/// Counts the number of zeros that a slice starts with.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Examples
/// ```
/// use malachite_base::slices::slice_leading_zeros;
///
/// assert_eq!(slice_leading_zeros::<u32>(&[1, 2, 3]), 0);
/// assert_eq!(slice_leading_zeros::<u32>(&[0, 0, 0, 1, 2, 3]), 3);
/// ```
pub fn slice_leading_zeros<T: Eq + Zero>(xs: &[T]) -> usize {
    let zero = T::ZERO;
    xs.iter().take_while(|&x| x == &zero).count()
}

/// Counts the number of zeros that a slice ends with.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Examples
/// ```
/// use malachite_base::slices::slice_trailing_zeros;
///
/// assert_eq!(slice_trailing_zeros::<u32>(&[1, 2, 3]), 0);
/// assert_eq!(slice_trailing_zeros::<u32>(&[1, 2, 3, 0, 0, 0]), 3);
/// ```
pub fn slice_trailing_zeros<T: Eq + Zero>(xs: &[T]) -> usize {
    let zero = T::ZERO;
    xs.iter().rev().take_while(|&x| x == &zero).count()
}

/// Given a slice `xs` and an starting index, copies the contents of `&xs[starting_index..]` to
/// `&xs[..xs.len() - starting_index]`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `amount` is greater than the length of `xs`.
///
/// # Examples
/// ```
/// use malachite_base::slices::slice_move_left;
///
/// let xs = &mut [1, 2, 3, 4, 5, 6];
/// slice_move_left::<u32>(xs, 2);
/// assert_eq!(xs, &[3, 4, 5, 6, 5, 6]);
/// ```
#[inline]
pub fn slice_move_left<T: Copy>(xs: &mut [T], starting_index: usize) {
    xs.copy_within(starting_index..xs.len(), 0);
}

/// Uniformly generates a random reference to a value from a nonempty slice.
#[derive(Clone, Debug)]
pub struct RandomValuesFromSlice<'a, T> {
    pub(crate) xs: &'a [T],
    pub(crate) indices: RandomUnsignedsLessThan<usize>,
}

impl<'a, T> Iterator for RandomValuesFromSlice<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<&'a T> {
        Some(&self.xs[self.indices.next().unwrap()])
    }
}

/// Uniformly generates a random reference to a value from a nonempty slice. The iterator cannot
/// outlive the slice. It may be more convenient for the iterator to own the data, in which case you
/// may use `random_values_from_vec` instead.
///
/// Length is infinite.
///
/// Time per iteration: worst case O(1)
///
/// Additional memory per iteration: worst case O(1)
///
/// # Panics
/// Panics if `xs` is empty.
///
/// # Examples
/// ```
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::slices::random_values_from_slice;
///
/// let xs = &[2, 3, 5, 7, 11];
/// assert_eq!(
///     random_values_from_slice(EXAMPLE_SEED, xs).cloned().take(10).collect::<Vec<_>>(),
///     &[3, 7, 3, 5, 11, 3, 5, 11, 2, 2]
/// );
/// ```
#[inline]
pub fn random_values_from_slice<T>(seed: Seed, xs: &[T]) -> RandomValuesFromSlice<T> {
    if xs.is_empty() {
        panic!("empty slice");
    }
    RandomValuesFromSlice {
        xs,
        indices: random_unsigneds_less_than(seed, xs.len()),
    }
}

/// This macro splits an immutable slice into adjacent immutable chunks. There are |`$xs_i`| + 1
/// chunks; the first |`$xs_i`| have length `$n`, and the remainder, which is assigned to
/// `$xs_last`, has length `$xs.len()` - `$n` * |`$xs_i`| (which may be longer than `$n`). If
/// `$xs.len()` < `$n` * |`$xs_i`|, the generated code panics at runtime.
#[macro_export]
macro_rules! split_into_chunks {
    ($xs: expr, $n: expr, [$($xs_i: ident),*], $xs_last: ident) => {
        let remainder = &$xs[..];
        let n = $n;
        $(
            let ($xs_i, remainder) = remainder.split_at(n);
        )*
        let $xs_last = remainder;
    }
}

/// This macro splits a mutable slice into adjacent mutable chunks. There are |`$xs_i`| + 1 chunks;
/// the first |`$xs_i`| have length `$n`, and the remainder, which is assigned to `$xs_last`, has
/// length `$xs.len()` - `$n` * |`$xs_i`| (which may be longer than `$n`). If
/// `$xs.len()` < `$n` * |`$xs_i`|, the generated code panics at runtime.
#[macro_export]
macro_rules! split_into_chunks_mut {
    ($xs: expr, $n: expr, [$($xs_i: ident),*], $xs_last: ident) => {
        let remainder = &mut $xs[..];
        let n = $n;
        $(
            let ($xs_i, remainder) = remainder.split_at_mut(n);
        )*
        let $xs_last = remainder;
    }
}
