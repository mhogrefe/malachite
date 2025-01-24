// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 1991, 1993-1997, 1999-2016, 2009, 2020 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::DivisibleBy;
use crate::num::basic::traits::Zero;
#[cfg(feature = "random")]
use crate::num::conversion::traits::ExactFrom;
#[cfg(feature = "random")]
use crate::num::random::{random_unsigneds_less_than, RandomUnsignedsLessThan};
#[cfg(feature = "random")]
use crate::random::Seed;
use alloc::vec::Vec;
#[cfg(feature = "random")]
use rand::prelude::SliceRandom;
#[cfg(feature = "random")]
use rand_chacha::ChaCha20Rng;

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
/// This is equivalent to `mpn_zero` from `mpn/generic/zero.c`, GMP 6.2.1. Note that this is needed
/// less often in Malachite than in GMP, since Malachite generally initializes new memory with
/// zeros.
pub fn slice_set_zero<T: Zero>(xs: &mut [T]) {
    for x in &mut *xs {
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
/// This is equivalent to `mpn_zero_p` from `gmp-h.in`, GMP 6.2.1.
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

/// Given a slice and an starting index, copies the subslice starting from that index to the
/// beginning of the slice.
///
/// In other words, this function copies the contents of `&xs[starting_index..]` to `&xs[..xs.len()
/// - starting_index]`.
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

/// Splits an immutable slice into adjacent immutable chunks.
///
/// An input slice $\mathbf{x}$, a chunk length $n$, and $k + 1$ output slice names $\\mathbf{x}_0,
/// \\mathbf{x}_1, \\ldots, \\mathbf{x}_k$ are given. The last output slice name, $\mathbf{x}_k$, is
/// specified via a separate argument called `xs_last`.
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
/// use malachite_base::split_into_chunks;
///
/// let xs = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
/// split_into_chunks!(xs, 3, [xs_1, xs_2, xs_3], xs_4);
/// assert_eq!(xs_1, &[0, 1, 2]);
/// assert_eq!(xs_2, &[3, 4, 5]);
/// assert_eq!(xs_3, &[6, 7, 8]);
/// assert_eq!(xs_4, &[9, 10, 11, 12]);
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

/// Splits a mutable slice into adjacent mutable chunks.
///
/// An input slice $\mathbf{x}$, a chunk length $n$, and $k + 1$ output slice names $\\mathbf{x}_0,
/// \\mathbf{x}_1, \\ldots, \\mathbf{x}_k$ are given. The last output slice name, $\mathbf{x}_k$, is
/// specified via a separate argument called `xs_last`.
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
/// use malachite_base::slices::slice_set_zero;
/// use malachite_base::split_into_chunks_mut;
///
/// let xs = &mut [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
/// split_into_chunks_mut!(xs, 3, [xs_1, xs_2, xs_3], xs_4);
/// assert_eq!(xs_1, &[0, 1, 2]);
/// assert_eq!(xs_2, &[3, 4, 5]);
/// assert_eq!(xs_3, &[6, 7, 8]);
/// assert_eq!(xs_4, &[9, 10, 11, 12]);
///
/// slice_set_zero(xs_2);
/// assert_eq!(xs, &[0, 1, 2, 0, 0, 0, 6, 7, 8, 9, 10, 11, 12]);
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

#[cfg(feature = "random")]
/// Uniformly generates a random reference to a value from a nonempty slice.
///
/// This `struct` is created by [`random_values_from_slice`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct RandomValuesFromSlice<'a, T> {
    xs: &'a [T],
    indices: RandomUnsignedsLessThan<u64>,
}

#[cfg(feature = "random")]
impl<'a, T> Iterator for RandomValuesFromSlice<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<&'a T> {
        Some(&self.xs[usize::exact_from(self.indices.next().unwrap())])
    }
}

#[cfg(feature = "random")]
/// Uniformly generates a random reference to a value from a nonempty slice.
///
/// The iterator cannot outlive the slice. It may be more convenient for the iterator to own the
/// data, in which case you may use [`random_values_from_vec`](crate::vecs::random_values_from_vec)
/// instead.
///
/// The output length is infinite.
///
/// $P(x) = 1/n$, where $n$ is `xs.len()`.
///
/// # Expected complexity per iteration
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `xs` is empty.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::slices::random_values_from_slice;
///
/// let xs = &[2, 3, 5, 7, 11];
/// assert_eq!(
///     random_values_from_slice(EXAMPLE_SEED, xs)
///         .cloned()
///         .take(10)
///         .collect_vec(),
///     &[3, 7, 3, 5, 11, 3, 5, 11, 2, 2]
/// );
/// ```
#[inline]
pub fn random_values_from_slice<T>(seed: Seed, xs: &[T]) -> RandomValuesFromSlice<T> {
    assert!(!xs.is_empty(), "empty slice");
    RandomValuesFromSlice {
        xs,
        indices: random_unsigneds_less_than(seed, u64::exact_from(xs.len())),
    }
}

pub(crate) fn advance_indices(indices: &mut [usize]) -> bool {
    let n = indices.len();
    if n == 0 {
        return true;
    }
    // Find the index of the value right before the longest descending suffix.
    let mut pivot_index = n;
    let mut i = 0;
    let mut reached_end = true;
    while pivot_index > 0 {
        pivot_index -= 1;
        let next_i = indices[pivot_index];
        if next_i < i {
            reached_end = false;
            break;
        }
        i = next_i;
    }
    if reached_end {
        return true;
    }
    let pivot = indices[pivot_index];
    let mut swap_index = n - 1;
    while indices[swap_index] < pivot {
        swap_index -= 1;
    }
    indices.swap(pivot_index, swap_index);
    indices[pivot_index + 1..].reverse();
    false
}

/// Generates every permutation of a slice.
///
/// This `struct` is created by [`exhaustive_slice_permutations`]; see its documentation for more.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ExhaustiveSlicePermutations<'a, T> {
    xs: &'a [T],
    indices: Vec<usize>,
    done: bool,
}

impl<'a, T> Iterator for ExhaustiveSlicePermutations<'a, T> {
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Vec<&'a T>> {
        if self.done {
            None
        } else {
            let out = Some(self.indices.iter().map(|&i| &self.xs[i]).collect());
            self.done = advance_indices(&mut self.indices);
            out
        }
    }
}

/// Generates every permutation of a slice.
///
/// The permutations are [`Vec`]s of references into the slice. It may be more convenient for the
/// iterator to own the data, in which case you may use
/// [`exhaustive_vec_permutations`](crate::vecs::exhaustive_vec_permutations) instead.
///
/// The permutations are generated in lexicographic order with respect to the ordering in the slice.
///
/// The output length is $n!$, where $n$ is `xs.len()`.
///
/// # Expected complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::slices::exhaustive_slice_permutations;
///
/// let css: Vec<String> = exhaustive_slice_permutations(&['a', 'b', 'c', 'd'])
///     .map(|ds| ds.into_iter().copied().collect())
///     .collect();
/// assert_eq!(
///     css.iter().map(String::as_str).collect_vec().as_slice(),
///     [
///         "abcd", "abdc", "acbd", "acdb", "adbc", "adcb", "bacd", "badc", "bcad", "bcda", "bdac",
///         "bdca", "cabd", "cadb", "cbad", "cbda", "cdab", "cdba", "dabc", "dacb", "dbac", "dbca",
///         "dcab", "dcba"
///     ]
/// );
/// ```
pub fn exhaustive_slice_permutations<T>(xs: &[T]) -> ExhaustiveSlicePermutations<T> {
    ExhaustiveSlicePermutations {
        xs,
        indices: (0..xs.len()).collect(),
        done: false,
    }
}

#[cfg(feature = "random")]
/// Uniformly generates a random permutation of references to a slice.
///
/// This `struct` is created by [`random_slice_permutations`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct RandomSlicePermutations<'a, T> {
    xs: &'a [T],
    indices: Vec<usize>,
    rng: ChaCha20Rng,
}

#[cfg(feature = "random")]
impl<'a, T> Iterator for RandomSlicePermutations<'a, T> {
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Vec<&'a T>> {
        self.indices.shuffle(&mut self.rng);
        Some(self.indices.iter().map(|&i| &self.xs[i]).collect())
    }
}

#[cfg(feature = "random")]
/// Uniformly generates a random permutation of references to a slice.
///
/// The iterator cannot outlive the slice. It may be more convenient for the iterator to own the
/// data, in which case you may use
/// [`random_vec_permutations`](crate::vecs::random_vec_permutations) instead.
///
/// The output length is infinite.
///
/// $P(p) = 1/n!$, where $n$ is `xs.len()`.
///
/// # Expected complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::slices::random_slice_permutations;
///
/// let css: Vec<String> = random_slice_permutations(EXAMPLE_SEED, &['a', 'b', 'c', 'd'])
///     .take(20)
///     .map(|ds| ds.into_iter().copied().collect())
///     .collect();
/// assert_eq!(
///     css.iter().map(String::as_str).collect_vec().as_slice(),
///     [
///         "cadb", "cbad", "cadb", "badc", "acdb", "cbad", "dabc", "dbca", "cdba", "cdab", "bacd",
///         "cabd", "adbc", "cdab", "dcab", "abcd", "abcd", "dacb", "bcad", "adcb"
///     ]
/// );
/// ```
pub fn random_slice_permutations<T>(seed: Seed, xs: &[T]) -> RandomSlicePermutations<T> {
    RandomSlicePermutations {
        xs,
        indices: (0..xs.len()).collect(),
        rng: seed.get_rng(),
    }
}

/// Given a slice with nonzero length $\ell$, returns the smallest $n$ such that the slice consists
/// of $n/\ell$ copies of a length-$\ell$ subslice.
///
/// Typically $\ell = n$.
///
/// # Worst-case complexity
/// $T(n) = O(n^{1+\varepsilon})$ for all $\varepsilon > 0$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
///
/// # Panics
/// Panics if `xs` is empty.
///
/// # Examples
/// ```
/// use malachite_base::slices::min_repeating_len;
///
/// assert_eq!(min_repeating_len(&[1, 2, 1, 2, 1, 2]), 2);
/// assert_eq!(min_repeating_len(&[1, 2, 1, 2, 1, 3]), 6);
/// assert_eq!(min_repeating_len(&[5, 5, 5]), 1);
/// ```
pub fn min_repeating_len<T: Eq>(xs: &[T]) -> usize {
    let len = xs.len();
    assert_ne!(len, 0);
    for start_i in 1..=len >> 1 {
        if !len.divisible_by(start_i) {
            continue;
        }
        let (xs_lo, xs_hi) = xs.split_at(start_i);
        if Iterator::eq(xs_lo.iter().cycle().take(len - start_i), xs_hi.iter()) {
            return start_i;
        }
    }
    len
}
