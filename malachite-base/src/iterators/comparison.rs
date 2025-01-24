// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};

/// An iterator that generates the [`Ordering`]s of adjacent elements of a given iterator.
///
/// This `struct` is created by [`delta_directions`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct DeltaDirections<I: Iterator>
where
    I::Item: Ord,
{
    previous: Option<I::Item>,
    xs: I,
}

impl<I: Iterator> Iterator for DeltaDirections<I>
where
    I::Item: Ord,
{
    type Item = Ordering;

    fn next(&mut self) -> Option<Ordering> {
        if self.previous.is_none() {
            if let Some(x) = self.xs.next() {
                self.previous = Some(x);
            } else {
                return None;
            }
        }
        self.xs.next().and_then(|x| {
            let result = Some(x.cmp(self.previous.as_ref().unwrap()));
            self.previous = Some(x);
            result
        })
    }
}

/// Returns an iterator that generates the [`Ordering`]s of adjacent pairs of elements of a given
/// iterator.
///
/// To put it another way (at least for types where subtraction is defined), the returned iterator
/// produces the signs of the finite differences of the input iterator.
///
/// $f((x_k)_{k=0}^N) = (\\operatorname{cmp}(x_k, x\_{k-1}))\_{k=1}^N$, where $N$ may be $\infty$.
///
/// The output length is infinite if `xs` is infinite, or $\max(n - 1, 0)$ otherwise, where $n$ is
/// `xs.count()`.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::iterators::comparison::delta_directions;
/// use std::cmp::Ordering::*;
///
/// assert_eq!(
///     delta_directions([3, 1, 4, 1, 5, 9].into_iter()).collect_vec(),
///     &[Less, Greater, Less, Greater, Greater]
/// )
/// ```
#[inline]
pub const fn delta_directions<I: Iterator>(xs: I) -> DeltaDirections<I>
where
    I::Item: Ord,
{
    DeltaDirections { previous: None, xs }
}

/// Determines whether each element of an iterator is greater than the preceding one.
///
/// This function will hang if given an infinite strictly ascending iterator.
///
/// $$
/// f((x_k)\_{k=0}^N) = \\bigwedge\_{k=1}^N{x\_k > x\_{k-1}},
/// $$
/// where $N$ may be $\infty$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::comparison::is_strictly_ascending;
///
/// assert_eq!(is_strictly_ascending([1, 2, 3, 4].into_iter()), true);
/// assert_eq!(is_strictly_ascending([1, 2, 2, 4].into_iter()), false);
/// ```
#[inline]
pub fn is_strictly_ascending<I: Iterator>(xs: I) -> bool
where
    I::Item: Ord,
{
    delta_directions(xs).all(|x| x == Greater)
}

/// Determines whether each element of an iterator is less than the preceding one.
///
/// This function will hang if given an infinite strictly descending iterator.
///
/// $$
/// f((x_k)\_{k=0}^N) = \\bigwedge\_{k=1}^N{x\_k < x\_{k-1}},
/// $$
/// where $N$ may be $\infty$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::comparison::is_strictly_descending;
///
/// assert_eq!(is_strictly_descending([4, 3, 2, 1].into_iter()), true);
/// assert_eq!(is_strictly_descending([4, 2, 2, 1].into_iter()), false);
/// ```
#[inline]
pub fn is_strictly_descending<I: Iterator>(xs: I) -> bool
where
    I::Item: Ord,
{
    delta_directions(xs).all(|x| x == Less)
}

/// Determines whether each element of an iterator is greater than or equal to the preceding one.
///
/// This function will hang if given an infinite weakly ascending iterator.
///
/// $$
/// f((x_k)\_{k=0}^N) = \\bigwedge\_{k=1}^N{x\_k \geq x\_{k-1}},
/// $$
/// where $N$ may be $\infty$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::comparison::is_weakly_ascending;
///
/// assert_eq!(is_weakly_ascending([1, 2, 3, 4].into_iter()), true);
/// assert_eq!(is_weakly_ascending([1, 2, 2, 4].into_iter()), true);
/// assert_eq!(is_weakly_ascending([1, 3, 2, 4].into_iter()), false);
/// ```
#[inline]
pub fn is_weakly_ascending<I: Iterator>(xs: I) -> bool
where
    I::Item: Ord,
{
    delta_directions(xs).all(|x| x != Less)
}

/// Determines whether each element of an iterator is less than or equal to the preceding one.
///
/// This function will hang if given an infinite weakly descending iterator.
///
/// $$
/// f((x_k)\_{k=0}^N) = \\bigwedge\_{k=1}^N{x\_k \leq x\_{k-1}},
/// $$
/// where $N$ may be $\infty$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::comparison::is_weakly_descending;
///
/// assert_eq!(is_weakly_descending([4, 3, 2, 1].into_iter()), true);
/// assert_eq!(is_weakly_descending([4, 2, 2, 1].into_iter()), true);
/// assert_eq!(is_weakly_descending([4, 2, 3, 1].into_iter()), false);
/// ```
#[inline]
pub fn is_weakly_descending<I: Iterator>(xs: I) -> bool
where
    I::Item: Ord,
{
    delta_directions(xs).all(|x| x != Greater)
}

/// Determines whether the sequence strictly zigzags.
///
/// A strictly zigzagging sequence is one where every odd-indexed element is greater than its
/// even-indexed neighbors, or one where every odd-indexed element is less than its even-indexed
/// neighbors.
///
/// This function will hang if given an infinite strictly zigzagging iterator.
///
/// # Examples
/// ```
/// use malachite_base::iterators::comparison::is_strictly_zigzagging;
///
/// assert_eq!(is_strictly_zigzagging([1, 2, 3, 4].into_iter()), false);
/// assert_eq!(is_strictly_zigzagging([4, 3, 2, 1].into_iter()), false);
/// assert_eq!(is_strictly_zigzagging([1, 3, 2, 4].into_iter()), true);
/// assert_eq!(is_strictly_zigzagging([1, 2, 2, 4].into_iter()), false);
/// ```
pub fn is_strictly_zigzagging<I: Iterator>(xs: I) -> bool
where
    I::Item: Ord,
{
    let mut previous = None;
    for direction in delta_directions(xs) {
        if direction == Equal {
            return false;
        }
        if let Some(previous) = previous {
            if direction == previous {
                return false;
            }
        }
        previous = Some(direction);
    }
    true
}

/// Determines whether the sequence weakly zigzags.
///
/// A weakly zigzagging sequence is one where every odd-indexed element is greater than or equal to
/// its even-indexed neighbors, or one where every odd-indexed element is less than or equal to its
/// even-indexed neighbors.
///
/// This function will hang if given an infinite strictly zigzagging iterator.
///
/// # Examples
/// ```
/// use malachite_base::iterators::comparison::is_weakly_zigzagging;
///
/// assert_eq!(is_weakly_zigzagging([1, 2, 3, 4].into_iter()), false);
/// assert_eq!(is_weakly_zigzagging([4, 3, 2, 1].into_iter()), false);
/// assert_eq!(is_weakly_zigzagging([1, 3, 2, 4].into_iter()), true);
/// assert_eq!(is_weakly_zigzagging([1, 2, 2, 4].into_iter()), true);
/// ```
pub fn is_weakly_zigzagging<I: Iterator>(xs: I) -> bool
where
    I::Item: Ord,
{
    let mut previous = None;
    for direction in delta_directions(xs) {
        if let Some(ref mut previous) = &mut previous {
            if direction == *previous {
                return false;
            }
            *previous = previous.reverse();
        } else if direction != Equal {
            previous = Some(direction);
        }
    }
    true
}
