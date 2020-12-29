use num::basic::traits::Zero;
use num::conversion::traits::ExactFrom;
use num::random::{random_unsigneds_less_than, RandomUnsignedsLessThan};
use random::Seed;

/// Sets all values in a slice to 0.
///
/// # Worst-case complexity
/// $T(n) = O(n)$
///
/// $M(n) = O(1)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
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
/// This is `mpn_zero` from `mpn/generic/zero.c`, GMP 6.2.1. Note that this is needed less often in
/// Malachite than in GMP, since Malachite generally initializes new memory with zeros.
pub fn slice_set_zero<T: Zero>(xs: &mut [T]) {
    for x in xs.iter_mut() {
        *x = T::ZERO;
    }
}

/// Tests whether all values in a slice are equal to 0.
///
/// # Worst-case complexity
/// $T(n) = O(n)$
///
/// $M(n) = O(1)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
///
/// # Examples
/// ```
/// use malachite_base::slices::slice_test_zero;
///
/// assert!(slice_test_zero::<u32>(&[0, 0, 0]));
/// assert!(!slice_test_zero::<u32>(&[0, 1, 0]));
/// ```
///
/// This is mpn_zero_p from gmp-h.in, GMP 6.2.1.
pub fn slice_test_zero<T: Eq + Zero>(xs: &[T]) -> bool {
    let zero = T::ZERO;
    xs.iter().all(|x| x == &zero)
}

/// Counts the number of zeros that a slice starts with.
///
/// # Worst-case complexity
/// $T(n) = O(n)$
///
/// $M(n) = O(1)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
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
/// # Worst-case complexity
/// $T(n) = O(n)$
///
/// $M(n) = O(1)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
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

/// Given a slice `xs` and an starting index, copies the subslice starting from that index to the
/// beginning of the slice.
///
/// In other words, this function copies the contents of `&xs[starting_index..]` to
/// `&xs[..xs.len() - starting_index]`.
///
/// In other other words, if $k$ is `starting_index`, the sequence $[x_0, x_1, \ldots, x_{n-1}]$
/// becomes $[x_k, x_{k+1}, \ldots, x_{n-1}, x_{n-k}, x_{n-k+1}, \ldots, x_{n-1}]$.
///
/// If `starting_index` is zero or `xs.len()`, nothing happens.
///
/// # Worst-case complexity
/// $T(n) = O(n)$
///
/// $M(n) = O(1)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
///
/// # Panics
/// Panics if `starting_index` is greater than the length of `xs`.
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

/// This macro splits an immutable slice into adjacent immutable chunks.
///
/// An input slice $\mathbf{x}$, a chunk length $n$, and $k + 1$ output slice names
/// $\\mathbf{x}_0, \\mathbf{x}_1, \\ldots, \\mathbf{x}_k$ are given. The last output slice name,
/// $\mathbf{x}_k$, is specified via a separate argument called `xs_last`.
///
/// The first $k$ output slice names are assigned adjacent length-$n$ chunks from $\mathbf{x}$. If
/// $|\mathbf{x}| < kn$, the generated code panics.
///
/// The last slice, $\mathbf{x}_k$, which is assigned to `xs_last`, has length $|\mathbf{x}| - kn$.
/// This length may be greater than $n$.
///
/// # Worst-case complexity
/// $T(k) = O(k)$
///
/// $M(k) = O(1)$
///
/// where $T$ is time, $M$ is additional memory, and $k$ is the number of output slice names `xs_i`.
///
/// # Examples
/// ```
/// #[macro_use]
/// extern crate malachite_base;
///
/// fn main() {
///     let xs = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
///     split_into_chunks!(xs, 3, [xs_1, xs_2, xs_3], xs_4);
///     assert_eq!(xs_1, &[0, 1, 2]);
///     assert_eq!(xs_2, &[3, 4, 5]);
///     assert_eq!(xs_3, &[6, 7, 8]);
///     assert_eq!(xs_4, &[9, 10, 11, 12]);
/// }
/// ```
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

/// This macro splits a mutable slice into adjacent mutable chunks.
///
/// An input slice $\mathbf{x}$, a chunk length $n$, and $k + 1$ output slice names
/// $\\mathbf{x}_0, \\mathbf{x}_1, \\ldots, \\mathbf{x}_k$ are given. The last output slice name,
/// $\mathbf{x}_k$, is specified via a separate argument called `xs_last`.
///
/// The first $k$ output slice names are assigned adjacent length-$n$ chunks from $\mathbf{x}$. If
/// $|\mathbf{x}| < kn$, the generated code panics.
///
/// The last slice, $\mathbf{x}_k$, which is assigned to `xs_last`, has length $|\mathbf{x}| - kn$.
/// This length may be greater than $n$.
///
/// # Worst-case complexity
/// $T(k) = O(k)$
///
/// $M(k) = O(1)$
///
/// where $T$ is time, $M$ is additional memory, and $k$ is the number of output slice names `xs_i`.
///
/// # Examples
/// ```
/// #[macro_use]
/// extern crate malachite_base;
///
/// use malachite_base::slices::slice_set_zero;
///
/// fn main() {
///     let xs = &mut [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
///     split_into_chunks_mut!(xs, 3, [xs_1, xs_2, xs_3], xs_4);
///     assert_eq!(xs_1, &[0, 1, 2]);
///     assert_eq!(xs_2, &[3, 4, 5]);
///     assert_eq!(xs_3, &[6, 7, 8]);
///     assert_eq!(xs_4, &[9, 10, 11, 12]);
///
///     slice_set_zero(xs_2);
///     assert_eq!(xs, &[0, 1, 2, 0, 0, 0, 6, 7, 8, 9, 10, 11, 12]);
/// }
/// ```
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

/// Uniformly generates a random reference to a value from a nonempty slice.
#[derive(Clone, Debug)]
pub struct RandomValuesFromSlice<'a, T> {
    xs: &'a [T],
    indices: RandomUnsignedsLessThan<u64>,
}

impl<'a, T> Iterator for RandomValuesFromSlice<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<&'a T> {
        Some(&self.xs[usize::exact_from(self.indices.next().unwrap())])
    }
}

/// Uniformly generates a random reference to a value from a nonempty slice. The iterator cannot
/// outlive the slice. It may be more convenient for the iterator to own the data, in which case you
/// may use `random_values_from_vec` instead.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
///
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `xs` is empty.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::slices::random_values_from_slice;
///
/// let xs = &[2, 3, 5, 7, 11];
/// assert_eq!(
///     random_values_from_slice(EXAMPLE_SEED, xs).cloned().take(10).collect_vec(),
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
        indices: random_unsigneds_less_than(seed, u64::exact_from(xs.len())),
    }
}
