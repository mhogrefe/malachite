// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::vecs::exhaustive::{
    ExhaustiveOrderedUniqueCollections, LexFixedLengthOrderedUniqueCollections,
    LexOrderedUniqueCollections, ShortlexOrderedUniqueCollections,
};
#[cfg(not(feature = "test_build"))]
use alloc::collections::BTreeSet;
use core::hash::Hash;
#[cfg(not(feature = "test_build"))]
use hashbrown::HashSet;
#[cfg(feature = "test_build")]
use std::collections::{BTreeSet, HashSet};

/// Generates [`HashSet`]s of a given size with elements from a single iterator.
///
/// The [`HashSet`]s are ordered lexicographically with respect to the order of the element
/// iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// If $k$ is 0, the output length is 1.
///
/// If $k$ is nonzero and the input iterator is infinite, the output length is also infinite.
///
/// If $k$ is nonzero and the input iterator length is $n$, the output length is $\binom{n}{k}$.
///
/// If $k$ is 0, the output consists of one empty [`HashSet`].
///
/// If `xs` is empty, the output is also empty, unless $k$ is 0.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::lex_hash_sets_fixed_length;
/// use maplit::hashset;
///
/// let xss = lex_hash_sets_fixed_length(4, 1..=6).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         hashset! {1, 2, 3, 4},
///         hashset! {1, 2, 3, 5},
///         hashset! {1, 2, 3, 6},
///         hashset! {1, 2, 4, 5},
///         hashset! {1, 2, 4, 6},
///         hashset! {1, 2, 5, 6},
///         hashset! {1, 3, 4, 5},
///         hashset! {1, 3, 4, 6},
///         hashset! {1, 3, 5, 6},
///         hashset! {1, 4, 5, 6},
///         hashset! {2, 3, 4, 5},
///         hashset! {2, 3, 4, 6},
///         hashset! {2, 3, 5, 6},
///         hashset! {2, 4, 5, 6},
///         hashset! {3, 4, 5, 6}
///     ]
/// );
/// ```
#[inline]
pub fn lex_hash_sets_fixed_length<I: Iterator>(
    k: u64,
    xs: I,
) -> LexFixedLengthOrderedUniqueCollections<I, HashSet<I::Item>>
where
    I::Item: Clone + Eq + Hash,
{
    LexFixedLengthOrderedUniqueCollections::new(k, xs)
}

/// Generates [`HashSet`]s with elements from a single iterator.
///
/// The [`HashSet`]s are generated in order of increasing length, and within each length they are
/// ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, [`HashSet`]s of length 2 and above will never
/// be generated.
///
/// If the input iterator is infinite, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is $2^n$.
///
/// If `xs` is empty, the output consists of a single empty [`HashSet`].
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::shortlex_hash_sets;
/// use maplit::hashset;
///
/// let xss = shortlex_hash_sets(1..=4).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         hashset! {},
///         hashset! {1},
///         hashset! {2},
///         hashset! {3},
///         hashset! {4},
///         hashset! {1, 2},
///         hashset! {1, 3},
///         hashset! {1, 4},
///         hashset! {2, 3},
///         hashset! {2, 4},
///         hashset! {3, 4},
///         hashset! {1, 2, 3},
///         hashset! {1, 2, 4},
///         hashset! {1, 3, 4},
///         hashset! {2, 3, 4},
///         hashset! {1, 2, 3, 4}
///     ]
/// );
/// ```
#[inline]
pub fn shortlex_hash_sets<I: Clone + Iterator>(
    xs: I,
) -> ShortlexOrderedUniqueCollections<I, HashSet<I::Item>>
where
    I::Item: Clone + Eq + Hash,
{
    shortlex_hash_sets_length_inclusive_range(0, u64::MAX, xs)
}

/// Generates [`HashSet`]s with a mininum length, with elements from a single iterator.
///
/// The [`HashSet`]s are generated in order of increasing length, and within each length they are
/// ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, [`HashSet`]s of length `\max(2, \ell + 1)` and
/// above will never be generated.
///
/// If the input iterator is infinite, the output length is also infinite.
///
/// If the input iterator length is $n$ and the `min_length` is $\ell$, the output length is
/// $$
/// \sum_{i=\ell}^n \binom{n}{i}.
/// $$
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::shortlex_hash_sets_min_length;
/// use maplit::hashset;
///
/// let xss = shortlex_hash_sets_min_length(2, 1..=4).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         hashset! {1, 2},
///         hashset! {1, 3},
///         hashset! {1, 4},
///         hashset! {2, 3},
///         hashset! {2, 4},
///         hashset! {3, 4},
///         hashset! {1, 2, 3},
///         hashset! {1, 2, 4},
///         hashset! {1, 3, 4},
///         hashset! {2, 3, 4},
///         hashset! {1, 2, 3, 4}
///     ]
/// );
/// ```
#[inline]
pub fn shortlex_hash_sets_min_length<I: Clone + Iterator>(
    min_length: u64,
    xs: I,
) -> ShortlexOrderedUniqueCollections<I, HashSet<I::Item>>
where
    I::Item: Clone + Eq + Hash,
{
    shortlex_hash_sets_length_inclusive_range(min_length, u64::MAX, xs)
}

/// Generates [`HashSet`]s, with lengths in a range $[a, b)$, with elements from a single iterator.
///
/// The [`HashSet`]s are generated in order of increasing length, and within each length they are
/// ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, [`HashSet`]s of length `\max(2, a + 1)` and
/// above will never be generated.
///
/// If $a \leq b$, the output is empty.
///
/// If $a = 0$ and $b = 1$, the output consists of a single empty [`HashSet`].
///
/// If the input iterator is infinite and $0 < a < b$, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is
/// $$
/// \sum_{i=a}^b - 1 \binom{n}{i}.
/// $$
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::shortlex_hash_sets_length_range;
/// use maplit::hashset;
///
/// let xss = shortlex_hash_sets_length_range(2, 4, 1..=4).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         hashset! {1, 2},
///         hashset! {1, 3},
///         hashset! {1, 4},
///         hashset! {2, 3},
///         hashset! {2, 4},
///         hashset! {3, 4},
///         hashset! {1, 2, 3},
///         hashset! {1, 2, 4},
///         hashset! {1, 3, 4},
///         hashset! {2, 3, 4},
///     ]
/// );
/// ```
#[inline]
pub fn shortlex_hash_sets_length_range<I: Clone + Iterator>(
    mut a: u64,
    mut b: u64,
    xs: I,
) -> ShortlexOrderedUniqueCollections<I, HashSet<I::Item>>
where
    I::Item: Clone + Eq + Hash,
{
    if b == 0 {
        // Transform an empty (x, 0) range into (2, 1), which is also empty but doesn't cause
        // overflow
        a = 2;
        b = 1;
    }
    shortlex_hash_sets_length_inclusive_range(a, b - 1, xs)
}

/// Generates [`HashSet`]s, with lengths in a range $[a, b]$, with elements from a single iterator.
///
/// The [`HashSet`]s are generated in order of increasing length, and within each length they are
/// ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, [`HashSet`]s of length `\max(2, a + 1)` and
/// above will never be generated.
///
/// If $a < b$, the output is empty.
///
/// If $a = b = 0$, the output consists of a single empty [`HashSet`].
///
/// If the input iterator is infinite and $0 < a \leq b$, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is
/// $$
/// \sum_{i=a}^b \binom{n}{i}.
/// $$
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::shortlex_hash_sets_length_inclusive_range;
/// use maplit::hashset;
///
/// let xss = shortlex_hash_sets_length_inclusive_range(2, 3, 1..=4).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         hashset! {1, 2},
///         hashset! {1, 3},
///         hashset! {1, 4},
///         hashset! {2, 3},
///         hashset! {2, 4},
///         hashset! {3, 4},
///         hashset! {1, 2, 3},
///         hashset! {1, 2, 4},
///         hashset! {1, 3, 4},
///         hashset! {2, 3, 4},
///     ]
/// );
/// ```
#[inline]
pub fn shortlex_hash_sets_length_inclusive_range<I: Clone + Iterator>(
    a: u64,
    b: u64,
    xs: I,
) -> ShortlexOrderedUniqueCollections<I, HashSet<I::Item>>
where
    I::Item: Clone + Eq + Hash,
{
    ShortlexOrderedUniqueCollections::new(a, b, xs)
}

/// Generates [`HashSet`]s with elements from a single iterator.
///
/// The [`HashSet`]s are ordered lexicographically with respect to the order of the element
/// iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, only prefixes of the iterator will be
/// generated.
///
/// If the input iterator is infinite, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is $2^n$.
///
/// If `xs` is empty, the output consists of a single empty [`HashSet`].
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::lex_hash_sets;
/// use maplit::hashset;
///
/// let xss = lex_hash_sets(1..=4).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         hashset! {},
///         hashset! {1},
///         hashset! {1, 2},
///         hashset! {1, 2, 3},
///         hashset! {1, 2, 3, 4},
///         hashset! {1, 2, 4},
///         hashset! {1, 3},
///         hashset! {1, 3, 4},
///         hashset! {1, 4},
///         hashset! {2},
///         hashset! {2, 3},
///         hashset! {2, 3, 4},
///         hashset! {2, 4},
///         hashset! {3},
///         hashset! {3, 4},
///         hashset! {4}
///     ]
/// );
/// ```
#[inline]
pub fn lex_hash_sets<I: Clone + Iterator>(xs: I) -> LexOrderedUniqueCollections<I, HashSet<I::Item>>
where
    I::Item: Clone + Eq + Hash,
{
    lex_hash_sets_length_inclusive_range(0, u64::MAX, xs)
}

/// Generates [`HashSet`]s with a mininum length, with elements from a single iterator.
///
/// The [`HashSet`]s are ordered lexicographically with respect to the order of the element
/// iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, only prefixes of the iterator will be
/// generated.
///
/// If the input iterator is infinite, the output length is also infinite.
///
/// If the input iterator length is $n$ and the `min_length` is $\ell$, the output length is
/// $$
/// \sum_{i=\ell}^n \binom{n}{i}.
/// $$
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::lex_hash_sets_min_length;
/// use maplit::hashset;
///
/// let xss = lex_hash_sets_min_length(2, 1..=4).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         hashset! {1, 2},
///         hashset! {1, 2, 3},
///         hashset! {1, 2, 3, 4},
///         hashset! {1, 2, 4},
///         hashset! {1, 3},
///         hashset! {1, 3, 4},
///         hashset! {1, 4},
///         hashset! {2, 3},
///         hashset! {2, 3, 4},
///         hashset! {2, 4},
///         hashset! {3, 4},
///     ]
/// );
/// ```
#[inline]
pub fn lex_hash_sets_min_length<I: Clone + Iterator>(
    min_length: u64,
    xs: I,
) -> LexOrderedUniqueCollections<I, HashSet<I::Item>>
where
    I::Item: Clone + Eq + Hash,
{
    lex_hash_sets_length_inclusive_range(min_length, u64::MAX, xs)
}

/// Generates [`HashSet`]s, with lengths in a range $[a, b)$, with elements from a single iterator.
///
/// The [`HashSet`]s are ordered lexicographically with respect to the order of the element
/// iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, only prefixes of the iterator will be
/// generated.
///
/// If $a \leq b$, the output is empty.
///
/// If $a = 0$ and $b = 1$, the output consists of a single empty [`HashSet`].
///
/// If the input iterator is infinite and $0 < a < b$, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is
/// $$
/// \sum_{i=a}^b - 1 \binom{n}{i}.
/// $$
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::lex_hash_sets_length_range;
/// use maplit::hashset;
///
/// let xss = lex_hash_sets_length_range(2, 4, 1..=4).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         hashset! {1, 2},
///         hashset! {1, 2, 3},
///         hashset! {1, 2, 4},
///         hashset! {1, 3},
///         hashset! {1, 3, 4},
///         hashset! {1, 4},
///         hashset! {2, 3},
///         hashset! {2, 3, 4},
///         hashset! {2, 4},
///         hashset! {3, 4},
///     ]
/// );
/// ```
#[inline]
pub fn lex_hash_sets_length_range<I: Clone + Iterator>(
    mut a: u64,
    mut b: u64,
    xs: I,
) -> LexOrderedUniqueCollections<I, HashSet<I::Item>>
where
    I::Item: Clone + Eq + Hash,
{
    if b == 0 {
        // Transform an empty (x, 0) range into (2, 1), which is also empty but doesn't cause
        // overflow
        a = 2;
        b = 1;
    }
    lex_hash_sets_length_inclusive_range(a, b - 1, xs)
}

/// Generates [`HashSet`]s, with lengths in a range $[a, b]$, with elements from a single iterator.
///
/// The [`HashSet`]s are ordered lexicographically with respect to the order of the element
/// iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, only prefixes of the iterator will be
/// generated.
///
/// If $a < b$, the output is empty.
///
/// If $a = b = 0$, the output consists of a single empty [`HashSet`].
///
/// If the input iterator is infinite and $0 < a \leq b$, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is
/// $$
/// \sum_{i=a}^b \binom{n}{i}.
/// $$
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::lex_hash_sets_length_inclusive_range;
/// use maplit::hashset;
///
/// let xss = lex_hash_sets_length_inclusive_range(2, 3, 1..=4).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         hashset! {1, 2},
///         hashset! {1, 2, 3},
///         hashset! {1, 2, 4},
///         hashset! {1, 3},
///         hashset! {1, 3, 4},
///         hashset! {1, 4},
///         hashset! {2, 3},
///         hashset! {2, 3, 4},
///         hashset! {2, 4},
///         hashset! {3, 4},
///     ]
/// );
/// ```
#[inline]
pub fn lex_hash_sets_length_inclusive_range<I: Clone + Iterator>(
    a: u64,
    b: u64,
    xs: I,
) -> LexOrderedUniqueCollections<I, HashSet<I::Item>>
where
    I::Item: Clone + Eq + Hash,
{
    LexOrderedUniqueCollections::new(a, b, xs)
}

/// Generates [`HashSet`]s of a given size with elements from a single iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// If $k$ is 0, the output length is 1.
///
/// If $k$ is nonzero and the input iterator is infinite, the output length is also infinite.
///
/// If $k$ is nonzero and the input iterator length is $n$, the output length is $\binom{n}{k}$.
///
/// If $k$ is 0, the output consists of one empty [`HashSet`].
///
/// If `xs` is empty, the output is also empty, unless $k$ is 0.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::exhaustive_hash_sets_fixed_length;
/// use maplit::hashset;
///
/// let xss = exhaustive_hash_sets_fixed_length(4, 1..=6).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         hashset! {1, 2, 3, 4},
///         hashset! {1, 2, 3, 5},
///         hashset! {1, 2, 4, 5},
///         hashset! {1, 3, 4, 5},
///         hashset! {2, 3, 4, 5},
///         hashset! {1, 2, 3, 6},
///         hashset! {1, 2, 4, 6},
///         hashset! {1, 3, 4, 6},
///         hashset! {2, 3, 4, 6},
///         hashset! {1, 2, 5, 6},
///         hashset! {1, 3, 5, 6},
///         hashset! {2, 3, 5, 6},
///         hashset! {1, 4, 5, 6},
///         hashset! {2, 4, 5, 6},
///         hashset! {3, 4, 5, 6}
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_hash_sets_fixed_length<I: Clone + Iterator>(
    k: u64,
    xs: I,
) -> ExhaustiveOrderedUniqueCollections<I, HashSet<I::Item>>
where
    I::Item: Clone + Eq + Hash,
{
    exhaustive_hash_sets_length_inclusive_range(k, k, xs)
}

/// Generates [`HashSet`]s with elements from a single iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, only prefixes of the iterator will be
/// generated.
///
/// If the input iterator is infinite, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is $2^n$.
///
/// If `xs` is empty, the output consists of a single empty [`HashSet`].
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::exhaustive_hash_sets;
/// use maplit::hashset;
///
/// let xss = exhaustive_hash_sets(1..=4).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         hashset! {},
///         hashset! {1},
///         hashset! {2},
///         hashset! {1, 2},
///         hashset! {3},
///         hashset! {1, 3},
///         hashset! {2, 3},
///         hashset! {1, 2, 3},
///         hashset! {4},
///         hashset! {1, 4},
///         hashset! {2, 4},
///         hashset! {1, 2, 4},
///         hashset! {3, 4},
///         hashset! {1, 3, 4},
///         hashset! {2, 3, 4},
///         hashset! {1, 2, 3, 4}
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_hash_sets<I: Clone + Iterator>(
    xs: I,
) -> ExhaustiveOrderedUniqueCollections<I, HashSet<I::Item>>
where
    I::Item: Clone + Eq + Hash,
{
    exhaustive_hash_sets_length_inclusive_range(0, u64::MAX, xs)
}

/// Generates [`HashSet`]s with a mininum length, with elements from a single iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, only prefixes of the iterator will be
/// generated.
///
/// If the input iterator is infinite, the output length is also infinite.
///
/// If the input iterator length is $n$ and the `min_length` is $\ell$, the output length is
/// $$
/// \sum_{i=\ell}^n \binom{n}{i}.
/// $$
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::exhaustive_hash_sets_min_length;
/// use maplit::hashset;
///
/// let xss = exhaustive_hash_sets_min_length(2, 1..=4).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         hashset! {1, 2},
///         hashset! {1, 3},
///         hashset! {2, 3},
///         hashset! {1, 2, 3},
///         hashset! {1, 4},
///         hashset! {2, 4},
///         hashset! {1, 2, 4},
///         hashset! {3, 4},
///         hashset! {1, 3, 4},
///         hashset! {2, 3, 4},
///         hashset! {1, 2, 3, 4}
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_hash_sets_min_length<I: Clone + Iterator>(
    min_length: u64,
    xs: I,
) -> ExhaustiveOrderedUniqueCollections<I, HashSet<I::Item>>
where
    I::Item: Clone + Eq + Hash,
{
    exhaustive_hash_sets_length_inclusive_range(min_length, u64::MAX, xs)
}

/// Generates [`HashSet`]s, with lengths in a range $[a, b)$, with elements from a single iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, only prefixes of the iterator will be
/// generated.
///
/// If $a \leq b$, the output is empty.
///
/// If $a = 0$ and $b = 1$, the output consists of a single empty [`HashSet`].
///
/// If the input iterator is infinite and $0 < a < b$, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is
/// $$
/// \sum_{i=a}^b - 1 \binom{n}{i}.
/// $$
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::exhaustive_hash_sets_length_range;
/// use maplit::hashset;
///
/// let xss = exhaustive_hash_sets_length_range(2, 4, 1..=4).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         hashset! {1, 2},
///         hashset! {1, 3},
///         hashset! {2, 3},
///         hashset! {1, 2, 3},
///         hashset! {1, 4},
///         hashset! {2, 4},
///         hashset! {1, 2, 4},
///         hashset! {3, 4},
///         hashset! {1, 3, 4},
///         hashset! {2, 3, 4}
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_hash_sets_length_range<I: Clone + Iterator>(
    a: u64,
    b: u64,
    xs: I,
) -> ExhaustiveOrderedUniqueCollections<I, HashSet<I::Item>>
where
    I::Item: Clone + Eq + Hash,
{
    if a >= b {
        ExhaustiveOrderedUniqueCollections::None
    } else {
        exhaustive_hash_sets_length_inclusive_range(a, b - 1, xs)
    }
}

/// Generates [`HashSet`]s, with lengths in a range $[a, b]$, with elements from a single iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, only prefixes of the iterator will be
/// generated.
///
/// If $a < b$, the output is empty.
///
/// If $a = b = 0$, the output consists of a single empty [`HashSet`].
///
/// If the input iterator is infinite and $0 < a \leq b$, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is
/// $$
/// \sum_{i=a}^b \binom{n}{i}.
/// $$
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::exhaustive_hash_sets_length_inclusive_range;
/// use maplit::hashset;
///
/// let xss = exhaustive_hash_sets_length_inclusive_range(2, 3, 1..=4).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         hashset! {1, 2},
///         hashset! {1, 3},
///         hashset! {2, 3},
///         hashset! {1, 2, 3},
///         hashset! {1, 4},
///         hashset! {2, 4},
///         hashset! {1, 2, 4},
///         hashset! {3, 4},
///         hashset! {1, 3, 4},
///         hashset! {2, 3, 4}
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_hash_sets_length_inclusive_range<I: Clone + Iterator>(
    a: u64,
    b: u64,
    xs: I,
) -> ExhaustiveOrderedUniqueCollections<I, HashSet<I::Item>>
where
    I::Item: Clone + Eq + Hash,
{
    ExhaustiveOrderedUniqueCollections::new(a, b, xs)
}

/// Generates [`BTreeSet`]s of a given size with elements from a single iterator.
///
/// The [`BTreeSet`]s are ordered lexicographically with respect to the order of the element
/// iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// If $k$ is 0, the output length is 1.
///
/// If $k$ is nonzero and the input iterator is infinite, the output length is also infinite.
///
/// If $k$ is nonzero and the input iterator length is $n$, the output length is $\binom{n}{k}$.
///
/// If $k$ is 0, the output consists of one empty [`BTreeSet`].
///
/// If `xs` is empty, the output is also empty, unless $k$ is 0.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::lex_b_tree_sets_fixed_length;
/// use maplit::btreeset;
///
/// let xss = lex_b_tree_sets_fixed_length(4, 1..=6).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         btreeset! {1, 2, 3, 4},
///         btreeset! {1, 2, 3, 5},
///         btreeset! {1, 2, 3, 6},
///         btreeset! {1, 2, 4, 5},
///         btreeset! {1, 2, 4, 6},
///         btreeset! {1, 2, 5, 6},
///         btreeset! {1, 3, 4, 5},
///         btreeset! {1, 3, 4, 6},
///         btreeset! {1, 3, 5, 6},
///         btreeset! {1, 4, 5, 6},
///         btreeset! {2, 3, 4, 5},
///         btreeset! {2, 3, 4, 6},
///         btreeset! {2, 3, 5, 6},
///         btreeset! {2, 4, 5, 6},
///         btreeset! {3, 4, 5, 6}
///     ]
/// );
/// ```
#[inline]
pub fn lex_b_tree_sets_fixed_length<I: Iterator>(
    k: u64,
    xs: I,
) -> LexFixedLengthOrderedUniqueCollections<I, BTreeSet<I::Item>>
where
    I::Item: Clone + Ord,
{
    LexFixedLengthOrderedUniqueCollections::new(k, xs)
}

/// Generates [`BTreeSet`]s with elements from a single iterator.
///
/// The [`BTreeSet`]s are generated in order of increasing length, and within each length they are
/// ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, [`BTreeSet`]s of length 2 and above will never
/// be generated.
///
/// If the input iterator is infinite, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is $2^n$.
///
/// If `xs` is empty, the output consists of a single empty [`HashSet`].
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::shortlex_b_tree_sets;
/// use maplit::btreeset;
///
/// let xss = shortlex_b_tree_sets(1..=4).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         btreeset! {},
///         btreeset! {1},
///         btreeset! {2},
///         btreeset! {3},
///         btreeset! {4},
///         btreeset! {1, 2},
///         btreeset! {1, 3},
///         btreeset! {1, 4},
///         btreeset! {2, 3},
///         btreeset! {2, 4},
///         btreeset! {3, 4},
///         btreeset! {1, 2, 3},
///         btreeset! {1, 2, 4},
///         btreeset! {1, 3, 4},
///         btreeset! {2, 3, 4},
///         btreeset! {1, 2, 3, 4}
///     ]
/// );
/// ```
#[inline]
pub fn shortlex_b_tree_sets<I: Clone + Iterator>(
    xs: I,
) -> ShortlexOrderedUniqueCollections<I, BTreeSet<I::Item>>
where
    I::Item: Clone + Ord,
{
    shortlex_b_tree_sets_length_inclusive_range(0, u64::MAX, xs)
}

/// Generates [`BTreeSet`]s with a mininum length, with elements from a single iterator.
///
/// The [`BTreeSet`]s are generated in order of increasing length, and within each length they are
/// ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, [`BTreeSet`]s of length `\max(2, \ell + 1)`
/// and above will never be generated.
///
/// If the input iterator is infinite, the output length is also infinite.
///
/// If the input iterator length is $n$ and the `min_length` is $\ell$, the output length is
/// $$
/// \sum_{i=\ell}^n \binom{n}{i}.
/// $$
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::shortlex_b_tree_sets_min_length;
/// use maplit::btreeset;
///
/// let xss = shortlex_b_tree_sets_min_length(2, 1..=4).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         btreeset! {1, 2},
///         btreeset! {1, 3},
///         btreeset! {1, 4},
///         btreeset! {2, 3},
///         btreeset! {2, 4},
///         btreeset! {3, 4},
///         btreeset! {1, 2, 3},
///         btreeset! {1, 2, 4},
///         btreeset! {1, 3, 4},
///         btreeset! {2, 3, 4},
///         btreeset! {1, 2, 3, 4}
///     ]
/// );
/// ```
#[inline]
pub fn shortlex_b_tree_sets_min_length<I: Clone + Iterator>(
    min_length: u64,
    xs: I,
) -> ShortlexOrderedUniqueCollections<I, BTreeSet<I::Item>>
where
    I::Item: Clone + Ord,
{
    shortlex_b_tree_sets_length_inclusive_range(min_length, u64::MAX, xs)
}

/// Generates [`BTreeSet`]s, with lengths in a range $[a, b)$, with elements from a single iterator.
///
/// The [`BTreeSet`]s are generated in order of increasing length, and within each length they are
/// ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, [`BTreeSet`]s of length `\max(2, a + 1)` and
/// above will never be generated.
///
/// If $a \leq b$, the output is empty.
///
/// If $a = 0$ and $b = 1$, the output consists of a single empty [`BTreeSet`].
///
/// If the input iterator is infinite and $0 < a < b$, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is
/// $$
/// \sum_{i=a}^b - 1 \binom{n}{i}.
/// $$
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::shortlex_b_tree_sets_length_range;
/// use maplit::btreeset;
///
/// let xss = shortlex_b_tree_sets_length_range(2, 4, 1..=4).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         btreeset! {1, 2},
///         btreeset! {1, 3},
///         btreeset! {1, 4},
///         btreeset! {2, 3},
///         btreeset! {2, 4},
///         btreeset! {3, 4},
///         btreeset! {1, 2, 3},
///         btreeset! {1, 2, 4},
///         btreeset! {1, 3, 4},
///         btreeset! {2, 3, 4},
///     ]
/// );
/// ```
#[inline]
pub fn shortlex_b_tree_sets_length_range<I: Clone + Iterator>(
    mut a: u64,
    mut b: u64,
    xs: I,
) -> ShortlexOrderedUniqueCollections<I, BTreeSet<I::Item>>
where
    I::Item: Clone + Ord,
{
    if b == 0 {
        // Transform an empty (x, 0) range into (2, 1), which is also empty but doesn't cause
        // overflow
        a = 2;
        b = 1;
    }
    shortlex_b_tree_sets_length_inclusive_range(a, b - 1, xs)
}

/// Generates [`BTreeSet`]s, with lengths in a range $[a, b]$, with elements from a single iterator.
///
/// The [`BTreeSet`]s are generated in order of increasing length, and within each length they are
/// ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, [`BTreeSet`]s of length `\max(2, a + 1)` and
/// above will never be generated.
///
/// If $a < b$, the output is empty.
///
/// If $a = b = 0$, the output consists of a single empty [`BTreeSet`].
///
/// If the input iterator is infinite and $0 < a \leq b$, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is
/// $$
/// \sum_{i=a}^b \binom{n}{i}.
/// $$
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::shortlex_b_tree_sets_length_inclusive_range;
/// use maplit::btreeset;
///
/// let xss = shortlex_b_tree_sets_length_inclusive_range(2, 3, 1..=4).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         btreeset! {1, 2},
///         btreeset! {1, 3},
///         btreeset! {1, 4},
///         btreeset! {2, 3},
///         btreeset! {2, 4},
///         btreeset! {3, 4},
///         btreeset! {1, 2, 3},
///         btreeset! {1, 2, 4},
///         btreeset! {1, 3, 4},
///         btreeset! {2, 3, 4},
///     ]
/// );
/// ```
#[inline]
pub fn shortlex_b_tree_sets_length_inclusive_range<I: Clone + Iterator>(
    a: u64,
    b: u64,
    xs: I,
) -> ShortlexOrderedUniqueCollections<I, BTreeSet<I::Item>>
where
    I::Item: Clone + Ord,
{
    ShortlexOrderedUniqueCollections::new(a, b, xs)
}

/// Generates [`BTreeSet`]s with elements from a single iterator.
///
/// The [`BTreeSet`]s are ordered lexicographically with respect to the order of the element
/// iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, only prefixes of the iterator will be
/// generated.
///
/// If the input iterator is infinite, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is $2^n$.
///
/// If `xs` is empty, the output consists of a single empty [`BTreeSet`].
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::lex_b_tree_sets;
/// use maplit::btreeset;
///
/// let xss = lex_b_tree_sets(1..=4).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         btreeset! {},
///         btreeset! {1},
///         btreeset! {1, 2},
///         btreeset! {1, 2, 3},
///         btreeset! {1, 2, 3, 4},
///         btreeset! {1, 2, 4},
///         btreeset! {1, 3},
///         btreeset! {1, 3, 4},
///         btreeset! {1, 4},
///         btreeset! {2},
///         btreeset! {2, 3},
///         btreeset! {2, 3, 4},
///         btreeset! {2, 4},
///         btreeset! {3},
///         btreeset! {3, 4},
///         btreeset! {4}
///     ]
/// );
/// ```
#[inline]
pub fn lex_b_tree_sets<I: Clone + Iterator>(
    xs: I,
) -> LexOrderedUniqueCollections<I, BTreeSet<I::Item>>
where
    I::Item: Clone + Ord,
{
    lex_b_tree_sets_length_inclusive_range(0, u64::MAX, xs)
}

/// Generates [`BTreeSet`]s with a mininum length, with elements from a single iterator.
///
/// The [`BTreeSet`]s are ordered lexicographically with respect to the order of the element
/// iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, only prefixes of the iterator will be
/// generated.
///
/// If the input iterator is infinite, the output length is also infinite.
///
/// If the input iterator length is $n$ and the `min_length` is $\ell$, the output length is
/// $$
/// \sum_{i=\ell}^n \binom{n}{i}.
/// $$
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::lex_b_tree_sets_min_length;
/// use maplit::btreeset;
///
/// let xss = lex_b_tree_sets_min_length(2, 1..=4).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         btreeset! {1, 2},
///         btreeset! {1, 2, 3},
///         btreeset! {1, 2, 3, 4},
///         btreeset! {1, 2, 4},
///         btreeset! {1, 3},
///         btreeset! {1, 3, 4},
///         btreeset! {1, 4},
///         btreeset! {2, 3},
///         btreeset! {2, 3, 4},
///         btreeset! {2, 4},
///         btreeset! {3, 4},
///     ]
/// );
/// ```
#[inline]
pub fn lex_b_tree_sets_min_length<I: Clone + Iterator>(
    min_length: u64,
    xs: I,
) -> LexOrderedUniqueCollections<I, BTreeSet<I::Item>>
where
    I::Item: Clone + Ord,
{
    lex_b_tree_sets_length_inclusive_range(min_length, u64::MAX, xs)
}

/// Generates [`BTreeSet`]s, with lengths in a range $[a, b)$, with elements from a single iterator.
///
/// The [`BTreeSet`]s are ordered lexicographically with respect to the order of the element
/// iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, only prefixes of the iterator will be
/// generated.
///
/// If $a \leq b$, the output is empty.
///
/// If $a = 0$ and $b = 1$, the output consists of a single empty [`BTreeSet`].
///
/// If the input iterator is infinite and $0 < a < b$, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is
/// $$
/// \sum_{i=a}^b - 1 \binom{n}{i}.
/// $$
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::lex_b_tree_sets_length_range;
/// use maplit::btreeset;
///
/// let xss = lex_b_tree_sets_length_range(2, 4, 1..=4).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         btreeset! {1, 2},
///         btreeset! {1, 2, 3},
///         btreeset! {1, 2, 4},
///         btreeset! {1, 3},
///         btreeset! {1, 3, 4},
///         btreeset! {1, 4},
///         btreeset! {2, 3},
///         btreeset! {2, 3, 4},
///         btreeset! {2, 4},
///         btreeset! {3, 4},
///     ]
/// );
/// ```
#[inline]
pub fn lex_b_tree_sets_length_range<I: Clone + Iterator>(
    mut a: u64,
    mut b: u64,
    xs: I,
) -> LexOrderedUniqueCollections<I, BTreeSet<I::Item>>
where
    I::Item: Clone + Ord,
{
    if b == 0 {
        // Transform an empty (x, 0) range into (2, 1), which is also empty but doesn't cause
        // overflow
        a = 2;
        b = 1;
    }
    lex_b_tree_sets_length_inclusive_range(a, b - 1, xs)
}

/// Generates [`BTreeSet`]s, with lengths in a range $[a, b]$, with elements from a single iterator.
///
/// The [`BTreeSet`]s are ordered lexicographically with respect to the order of the element
/// iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, only prefixes of the iterator will be
/// generated.
///
/// If $a < b$, the output is empty.
///
/// If $a = b = 0$, the output consists of a single empty [`BTreeSet`].
///
/// If the input iterator is infinite and $0 < a \leq b$, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is
/// $$
/// \sum_{i=a}^b \binom{n}{i}.
/// $$
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::lex_b_tree_sets_length_inclusive_range;
/// use maplit::btreeset;
///
/// let xss = lex_b_tree_sets_length_inclusive_range(2, 3, 1..=4).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         btreeset! {1, 2},
///         btreeset! {1, 2, 3},
///         btreeset! {1, 2, 4},
///         btreeset! {1, 3},
///         btreeset! {1, 3, 4},
///         btreeset! {1, 4},
///         btreeset! {2, 3},
///         btreeset! {2, 3, 4},
///         btreeset! {2, 4},
///         btreeset! {3, 4},
///     ]
/// );
/// ```
#[inline]
pub fn lex_b_tree_sets_length_inclusive_range<I: Clone + Iterator>(
    a: u64,
    b: u64,
    xs: I,
) -> LexOrderedUniqueCollections<I, BTreeSet<I::Item>>
where
    I::Item: Clone + Ord,
{
    LexOrderedUniqueCollections::new(a, b, xs)
}

/// Generates [`BTreeSet`]s of a given size with elements from a single iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// If $k$ is 0, the output length is 1.
///
/// If $k$ is nonzero and the input iterator is infinite, the output length is also infinite.
///
/// If $k$ is nonzero and the input iterator length is $n$, the output length is $\binom{n}{k}$.
///
/// If $k$ is 0, the output consists of one empty [`BTreeSet`].
///
/// If `xs` is empty, the output is also empty, unless $k$ is 0.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::exhaustive_b_tree_sets_fixed_length;
/// use maplit::btreeset;
///
/// let xss = exhaustive_b_tree_sets_fixed_length(4, 1..=6).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         btreeset! {1, 2, 3, 4},
///         btreeset! {1, 2, 3, 5},
///         btreeset! {1, 2, 4, 5},
///         btreeset! {1, 3, 4, 5},
///         btreeset! {2, 3, 4, 5},
///         btreeset! {1, 2, 3, 6},
///         btreeset! {1, 2, 4, 6},
///         btreeset! {1, 3, 4, 6},
///         btreeset! {2, 3, 4, 6},
///         btreeset! {1, 2, 5, 6},
///         btreeset! {1, 3, 5, 6},
///         btreeset! {2, 3, 5, 6},
///         btreeset! {1, 4, 5, 6},
///         btreeset! {2, 4, 5, 6},
///         btreeset! {3, 4, 5, 6}
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_b_tree_sets_fixed_length<I: Clone + Iterator>(
    k: u64,
    xs: I,
) -> ExhaustiveOrderedUniqueCollections<I, BTreeSet<I::Item>>
where
    I::Item: Clone + Ord,
{
    exhaustive_b_tree_sets_length_inclusive_range(k, k, xs)
}

/// Generates [`BTreeSet`]s with elements from a single iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, only prefixes of the iterator will be
/// generated.
///
/// If the input iterator is infinite, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is $2^n$.
///
/// If `xs` is empty, the output consists of a single empty [`BTreeSet`].
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::exhaustive_b_tree_sets;
/// use maplit::btreeset;
///
/// let xss = exhaustive_b_tree_sets(1..=4).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         btreeset! {},
///         btreeset! {1},
///         btreeset! {2},
///         btreeset! {1, 2},
///         btreeset! {3},
///         btreeset! {1, 3},
///         btreeset! {2, 3},
///         btreeset! {1, 2, 3},
///         btreeset! {4},
///         btreeset! {1, 4},
///         btreeset! {2, 4},
///         btreeset! {1, 2, 4},
///         btreeset! {3, 4},
///         btreeset! {1, 3, 4},
///         btreeset! {2, 3, 4},
///         btreeset! {1, 2, 3, 4}
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_b_tree_sets<I: Clone + Iterator>(
    xs: I,
) -> ExhaustiveOrderedUniqueCollections<I, BTreeSet<I::Item>>
where
    I::Item: Clone + Ord,
{
    exhaustive_b_tree_sets_length_inclusive_range(0, u64::MAX, xs)
}

/// Generates [`BTreeSet`]s with a mininum length, with elements from a single iterator.
///
/// The [`BTreeSet`]s are ordered lexicographically with respect to the order of the element
/// iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, only prefixes of the iterator will be
/// generated.
///
/// If the input iterator is infinite, the output length is also infinite.
///
/// If the input iterator length is $n$ and the `min_length` is $\ell$, the output length is
/// $$
/// \sum_{i=\ell}^n \binom{n}{i}.
/// $$
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::exhaustive_b_tree_sets_min_length;
/// use maplit::btreeset;
///
/// let xss = exhaustive_b_tree_sets_min_length(2, 1..=4).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         btreeset! {1, 2},
///         btreeset! {1, 3},
///         btreeset! {2, 3},
///         btreeset! {1, 2, 3},
///         btreeset! {1, 4},
///         btreeset! {2, 4},
///         btreeset! {1, 2, 4},
///         btreeset! {3, 4},
///         btreeset! {1, 3, 4},
///         btreeset! {2, 3, 4},
///         btreeset! {1, 2, 3, 4}
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_b_tree_sets_min_length<I: Clone + Iterator>(
    min_length: u64,
    xs: I,
) -> ExhaustiveOrderedUniqueCollections<I, BTreeSet<I::Item>>
where
    I::Item: Clone + Ord,
{
    exhaustive_b_tree_sets_length_inclusive_range(min_length, u64::MAX, xs)
}

/// Generates [`BTreeSet`]s, with lengths in a range $[a, b)$, with elements from a single iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, only prefixes of the iterator will be
/// generated.
///
/// If $a \leq b$, the output is empty.
///
/// If $a = 0$ and $b = 1$, the output consists of a single empty [`BTreeSet`].
///
/// If the input iterator is infinite and $0 < a < b$, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is
/// $$
/// \sum_{i=a}^b - 1 \binom{n}{i}.
/// $$
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::exhaustive_b_tree_sets_length_range;
/// use maplit::btreeset;
///
/// let xss = exhaustive_b_tree_sets_length_range(2, 4, 1..=4).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         btreeset! {1, 2},
///         btreeset! {1, 3},
///         btreeset! {2, 3},
///         btreeset! {1, 2, 3},
///         btreeset! {1, 4},
///         btreeset! {2, 4},
///         btreeset! {1, 2, 4},
///         btreeset! {3, 4},
///         btreeset! {1, 3, 4},
///         btreeset! {2, 3, 4}
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_b_tree_sets_length_range<I: Clone + Iterator>(
    a: u64,
    b: u64,
    xs: I,
) -> ExhaustiveOrderedUniqueCollections<I, BTreeSet<I::Item>>
where
    I::Item: Clone + Ord,
{
    if a >= b {
        ExhaustiveOrderedUniqueCollections::None
    } else {
        exhaustive_b_tree_sets_length_inclusive_range(a, b - 1, xs)
    }
}

/// Generates [`BTreeSet`]s, with lengths in a range $[a, b]$, with elements from a single iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, only prefixes of the iterator will be
/// generated.
///
/// If $a < b$, the output is empty.
///
/// If $a = b = 0$, the output consists of a single empty [`BTreeSet`].
///
/// If the input iterator is infinite and $0 < a \leq b$, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is
/// $$
/// \sum_{i=a}^b \binom{n}{i}.
/// $$
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::sets::exhaustive::exhaustive_b_tree_sets_length_inclusive_range;
/// use maplit::btreeset;
///
/// let xss = exhaustive_b_tree_sets_length_inclusive_range(2, 3, 1..=4).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         btreeset! {1, 2},
///         btreeset! {1, 3},
///         btreeset! {2, 3},
///         btreeset! {1, 2, 3},
///         btreeset! {1, 4},
///         btreeset! {2, 4},
///         btreeset! {1, 2, 4},
///         btreeset! {3, 4},
///         btreeset! {1, 3, 4},
///         btreeset! {2, 3, 4}
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_b_tree_sets_length_inclusive_range<I: Clone + Iterator>(
    a: u64,
    b: u64,
    xs: I,
) -> ExhaustiveOrderedUniqueCollections<I, BTreeSet<I::Item>>
where
    I::Item: Clone + Ord,
{
    ExhaustiveOrderedUniqueCollections::new(a, b, xs)
}
