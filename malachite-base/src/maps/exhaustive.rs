// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::conversion::traits::ExactFrom;
use crate::num::iterators::{RulerSequence, ruler_sequence};
use crate::tuples::exhaustive::{
    ExhaustiveDependentPairs, ExhaustiveDependentPairsYsGenerator, exhaustive_dependent_pairs,
};
use crate::vecs::exhaustive::{
    ExhaustiveFixedLengthVecs1Input, ExhaustiveOrderedUniqueCollections,
    ExhaustiveVecsFixedLengthWithDistinctCount, exhaustive_vecs_fixed_length_from_single,
    exhaustive_vecs_fixed_length_with_distinct_count_inclusive_range,
};
#[cfg(not(feature = "std"))]
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::hash::Hash;
use core::marker::PhantomData;
#[cfg(not(feature = "std"))]
use hashbrown::HashMap;
#[cfg(feature = "std")]
use std::collections::{BTreeMap, HashMap};

// Given a set of keys, generates the value assignments for it: one value per key, in key order.
#[derive(Clone, Debug)]
struct ExhaustiveMapsGenerator<K: Clone, J: Clone + Iterator>
where
    J::Item: Clone,
{
    values: J,
    phantom: PhantomData<*const K>,
}

impl<K: Clone, J: Clone + Iterator>
    ExhaustiveDependentPairsYsGenerator<Vec<K>, Vec<J::Item>, ExhaustiveFixedLengthVecs1Input<J>>
    for ExhaustiveMapsGenerator<K, J>
where
    J::Item: Clone,
{
    #[inline]
    fn get_ys(&self, ks: &Vec<K>) -> ExhaustiveFixedLengthVecs1Input<J> {
        exhaustive_vecs_fixed_length_from_single(u64::exact_from(ks.len()), self.values.clone())
    }
}

/// Generates all maps with keys from one iterator and values from another.
#[derive(Clone)]
pub struct ExhaustiveMaps<I: Iterator, J: Clone + Iterator, C: FromIterator<(I::Item, J::Item)>>
where
    I::Item: Clone,
    J::Item: Clone,
{
    xs: ExhaustiveDependentPairs<
        Vec<I::Item>,
        Vec<J::Item>,
        RulerSequence<usize>,
        ExhaustiveMapsGenerator<I::Item, J>,
        ExhaustiveOrderedUniqueCollections<I, Vec<I::Item>>,
        ExhaustiveFixedLengthVecs1Input<J>,
    >,
    phantom: PhantomData<*const C>,
}

impl<I: Iterator, J: Clone + Iterator, C: FromIterator<(I::Item, J::Item)>> Iterator
    for ExhaustiveMaps<I, J, C>
where
    I::Item: Clone,
    J::Item: Clone,
{
    type Item = C;

    #[inline]
    fn next(&mut self) -> Option<C> {
        self.xs
            .next()
            .map(|(ks, vs)| ks.into_iter().zip(vs).collect())
    }
}

fn exhaustive_maps_helper<I: Iterator, J: Clone + Iterator, C: FromIterator<(I::Item, J::Item)>>(
    a: u64,
    b: u64,
    keys: I,
    values: J,
) -> ExhaustiveMaps<I, J, C>
where
    I::Item: Clone,
    J::Item: Clone,
{
    ExhaustiveMaps {
        xs: exhaustive_dependent_pairs(
            ruler_sequence(),
            ExhaustiveOrderedUniqueCollections::new(a, b, keys),
            ExhaustiveMapsGenerator {
                values,
                phantom: PhantomData,
            },
        ),
        phantom: PhantomData,
    }
}

// Given a set of keys, generates the value assignments whose number of distinct values lies in a
// range.
#[derive(Clone, Debug)]
struct ExhaustiveMapsWithUniqueValueCountGenerator<K: Clone, J: Clone + Iterator>
where
    J::Item: Clone,
{
    values: J,
    a: u64,
    b: u64,
    phantom: PhantomData<*const K>,
}

impl<K: Clone, J: Clone + Iterator>
    ExhaustiveDependentPairsYsGenerator<
        Vec<K>,
        Vec<J::Item>,
        ExhaustiveVecsFixedLengthWithDistinctCount<J>,
    > for ExhaustiveMapsWithUniqueValueCountGenerator<K, J>
where
    J::Item: Clone,
{
    #[inline]
    fn get_ys(&self, ks: &Vec<K>) -> ExhaustiveVecsFixedLengthWithDistinctCount<J> {
        exhaustive_vecs_fixed_length_with_distinct_count_inclusive_range(
            u64::exact_from(ks.len()),
            self.a,
            self.b,
            self.values.clone(),
        )
    }
}

/// Generates all maps with keys from one iterator and values from another, where the number of
/// distinct values is restricted.
#[derive(Clone)]
pub struct ExhaustiveMapsWithUniqueValueCount<
    I: Iterator,
    J: Clone + Iterator,
    C: FromIterator<(I::Item, J::Item)>,
> where
    I::Item: Clone,
    J::Item: Clone,
{
    xs: ExhaustiveDependentPairs<
        Vec<I::Item>,
        Vec<J::Item>,
        RulerSequence<usize>,
        ExhaustiveMapsWithUniqueValueCountGenerator<I::Item, J>,
        ExhaustiveOrderedUniqueCollections<I, Vec<I::Item>>,
        ExhaustiveVecsFixedLengthWithDistinctCount<J>,
    >,
    phantom: PhantomData<*const C>,
}

impl<I: Iterator, J: Clone + Iterator, C: FromIterator<(I::Item, J::Item)>> Iterator
    for ExhaustiveMapsWithUniqueValueCount<I, J, C>
where
    I::Item: Clone,
    J::Item: Clone,
{
    type Item = C;

    #[inline]
    fn next(&mut self) -> Option<C> {
        self.xs
            .next()
            .map(|(ks, vs)| ks.into_iter().zip(vs).collect())
    }
}

fn exhaustive_maps_with_unique_value_count_helper<
    I: Iterator,
    J: Clone + Iterator,
    C: FromIterator<(I::Item, J::Item)>,
>(
    size_a: u64,
    size_b: u64,
    value_count_a: u64,
    value_count_b: u64,
    keys: I,
    values: J,
) -> ExhaustiveMapsWithUniqueValueCount<I, J, C>
where
    I::Item: Clone,
    J::Item: Clone,
{
    ExhaustiveMapsWithUniqueValueCount {
        xs: exhaustive_dependent_pairs(
            ruler_sequence(),
            ExhaustiveOrderedUniqueCollections::new(size_a, size_b, keys),
            ExhaustiveMapsWithUniqueValueCountGenerator {
                values,
                a: value_count_a,
                b: value_count_b,
                phantom: PhantomData,
            },
        ),
        phantom: PhantomData,
    }
}

/// Generates all [`BTreeMap`]s with keys from one iterator and values from another.
///
/// The key iterator should not repeat any elements, but this is not enforced; a repeated key makes
/// the same map reachable more than once.
///
/// If either input iterator is infinite, the output length is also infinite.
///
/// If the key iterator length is $n$ and the value iterator length is $m$, the output length is
/// $$
/// \sum_{k=0}^{n}} \binom{n}{k} m^k.
/// $$
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\ell + T^\prime(i) + T^{\prime\prime}(i))$
///
/// $M(i) = O(\ell + M^\prime(i) + M^{\prime\prime}(i))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of `keys`, $T^{\prime\prime}$ and
/// $M^{\prime\prime}$ are the time and memory functions of `values`, and $\ell$ is the number of
/// entries in the $i$th output.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::maps::exhaustive::exhaustive_b_tree_maps;
/// use maplit::btreemap;
///
/// let xss = exhaustive_b_tree_maps('a'..='b', 0..2u8).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         btreemap! {},
///         btreemap! {'a' => 0},
///         btreemap! {'a' => 1},
///         btreemap! {'a' => 0, 'b' => 0},
///         btreemap! {'b' => 0},
///         btreemap! {'a' => 0, 'b' => 1},
///         btreemap! {'b' => 1},
///         btreemap! {'a' => 1, 'b' => 0},
///         btreemap! {'a' => 1, 'b' => 1}
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_b_tree_maps<I: Iterator, J: Clone + Iterator>(
    keys: I,
    values: J,
) -> ExhaustiveMaps<I, J, BTreeMap<I::Item, J::Item>>
where
    I::Item: Clone + Ord,
    J::Item: Clone,
{
    exhaustive_maps_helper(0, u64::MAX, keys, values)
}

/// Generates all [`BTreeMap`]s of a given size, with keys from one iterator and values from
/// another.
///
/// The key iterator should not repeat any elements, but this is not enforced; a repeated key makes
/// the same map reachable more than once.
///
/// If either input iterator is infinite, the output length is also infinite.
///
/// If the key iterator length is $n$ and the value iterator length is $m$, the output length is
/// $$
/// \sum_{k=k_0}^{k_0}} \binom{n}{k} m^k.
/// $$
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\ell + T^\prime(i) + T^{\prime\prime}(i))$
///
/// $M(i) = O(\ell + M^\prime(i) + M^{\prime\prime}(i))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of `keys`, $T^{\prime\prime}$ and
/// $M^{\prime\prime}$ are the time and memory functions of `values`, and $\ell$ is the number of
/// entries in the $i$th output.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::maps::exhaustive::exhaustive_b_tree_maps_fixed_size;
/// use maplit::btreemap;
///
/// let xss = exhaustive_b_tree_maps_fixed_size(2, 'a'..='c', 0..2u8).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         btreemap! {'a' => 0, 'b' => 0},
///         btreemap! {'a' => 0, 'c' => 0},
///         btreemap! {'a' => 0, 'b' => 1},
///         btreemap! {'b' => 0, 'c' => 0},
///         btreemap! {'a' => 1, 'b' => 0},
///         btreemap! {'a' => 0, 'c' => 1},
///         btreemap! {'a' => 1, 'b' => 1},
///         btreemap! {'b' => 0, 'c' => 1},
///         btreemap! {'a' => 1, 'c' => 0},
///         btreemap! {'b' => 1, 'c' => 0},
///         btreemap! {'a' => 1, 'c' => 1},
///         btreemap! {'b' => 1, 'c' => 1}
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_b_tree_maps_fixed_size<I: Iterator, J: Clone + Iterator>(
    size: u64,
    keys: I,
    values: J,
) -> ExhaustiveMaps<I, J, BTreeMap<I::Item, J::Item>>
where
    I::Item: Clone + Ord,
    J::Item: Clone,
{
    exhaustive_maps_helper(size, size, keys, values)
}

/// Generates all [`BTreeMap`]s with a minimum size, with keys from one iterator and values from
/// another.
///
/// The key iterator should not repeat any elements, but this is not enforced; a repeated key makes
/// the same map reachable more than once.
///
/// If either input iterator is infinite, the output length is also infinite.
///
/// If the key iterator length is $n$ and the value iterator length is $m$, the output length is
/// $$
/// \sum_{k=\\ell}^{n}} \binom{n}{k} m^k.
/// $$
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\ell + T^\prime(i) + T^{\prime\prime}(i))$
///
/// $M(i) = O(\ell + M^\prime(i) + M^{\prime\prime}(i))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of `keys`, $T^{\prime\prime}$ and
/// $M^{\prime\prime}$ are the time and memory functions of `values`, and $\ell$ is the number of
/// entries in the $i$th output.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::maps::exhaustive::exhaustive_b_tree_maps_min_size;
/// use maplit::btreemap;
///
/// let xss = exhaustive_b_tree_maps_min_size(1, 'a'..='b', 0..2u8).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         btreemap! {'a' => 0},
///         btreemap! {'b' => 0},
///         btreemap! {'a' => 1},
///         btreemap! {'a' => 0, 'b' => 0},
///         btreemap! {'b' => 1},
///         btreemap! {'a' => 0, 'b' => 1},
///         btreemap! {'a' => 1, 'b' => 0},
///         btreemap! {'a' => 1, 'b' => 1}
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_b_tree_maps_min_size<I: Iterator, J: Clone + Iterator>(
    min_size: u64,
    keys: I,
    values: J,
) -> ExhaustiveMaps<I, J, BTreeMap<I::Item, J::Item>>
where
    I::Item: Clone + Ord,
    J::Item: Clone,
{
    exhaustive_maps_helper(min_size, u64::MAX, keys, values)
}

/// Generates all [`BTreeMap`]s with sizes in the half-open interval $[a, b)$, with keys from one
/// iterator and values from another.
///
/// The key iterator should not repeat any elements, but this is not enforced; a repeated key makes
/// the same map reachable more than once.
///
/// If either input iterator is infinite, the output length is also infinite.
///
/// If the key iterator length is $n$ and the value iterator length is $m$, the output length is
/// $$
/// \sum_{k=a}^{b-1}} \binom{n}{k} m^k.
/// $$
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\ell + T^\prime(i) + T^{\prime\prime}(i))$
///
/// $M(i) = O(\ell + M^\prime(i) + M^{\prime\prime}(i))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of `keys`, $T^{\prime\prime}$ and
/// $M^{\prime\prime}$ are the time and memory functions of `values`, and $\ell$ is the number of
/// entries in the $i$th output.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::maps::exhaustive::exhaustive_b_tree_maps_size_range;
/// use maplit::btreemap;
///
/// let xss = exhaustive_b_tree_maps_size_range(1, 3, 'a'..='b', 0..2u8).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         btreemap! {'a' => 0},
///         btreemap! {'b' => 0},
///         btreemap! {'a' => 1},
///         btreemap! {'a' => 0, 'b' => 0},
///         btreemap! {'b' => 1},
///         btreemap! {'a' => 0, 'b' => 1},
///         btreemap! {'a' => 1, 'b' => 0},
///         btreemap! {'a' => 1, 'b' => 1}
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_b_tree_maps_size_range<I: Iterator, J: Clone + Iterator>(
    a: u64,
    b: u64,
    keys: I,
    values: J,
) -> ExhaustiveMaps<I, J, BTreeMap<I::Item, J::Item>>
where
    I::Item: Clone + Ord,
    J::Item: Clone,
{
    if b == 0 {
        exhaustive_maps_helper(1, 0, keys, values)
    } else {
        exhaustive_maps_helper(a, b - 1, keys, values)
    }
}

/// Generates all [`BTreeMap`]s with sizes in the closed interval $[a, b]$, with keys from one
/// iterator and values from another.
///
/// The key iterator should not repeat any elements, but this is not enforced; a repeated key makes
/// the same map reachable more than once.
///
/// If either input iterator is infinite, the output length is also infinite.
///
/// If the key iterator length is $n$ and the value iterator length is $m$, the output length is
/// $$
/// \sum_{k=a}^{b}} \binom{n}{k} m^k.
/// $$
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\ell + T^\prime(i) + T^{\prime\prime}(i))$
///
/// $M(i) = O(\ell + M^\prime(i) + M^{\prime\prime}(i))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of `keys`, $T^{\prime\prime}$ and
/// $M^{\prime\prime}$ are the time and memory functions of `values`, and $\ell$ is the number of
/// entries in the $i$th output.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::maps::exhaustive::exhaustive_b_tree_maps_size_inclusive_range;
/// use maplit::btreemap;
///
/// let xss = exhaustive_b_tree_maps_size_inclusive_range(1, 2, 'a'..='b', 0..2u8).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         btreemap! {'a' => 0},
///         btreemap! {'b' => 0},
///         btreemap! {'a' => 1},
///         btreemap! {'a' => 0, 'b' => 0},
///         btreemap! {'b' => 1},
///         btreemap! {'a' => 0, 'b' => 1},
///         btreemap! {'a' => 1, 'b' => 0},
///         btreemap! {'a' => 1, 'b' => 1}
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_b_tree_maps_size_inclusive_range<I: Iterator, J: Clone + Iterator>(
    a: u64,
    b: u64,
    keys: I,
    values: J,
) -> ExhaustiveMaps<I, J, BTreeMap<I::Item, J::Item>>
where
    I::Item: Clone + Ord,
    J::Item: Clone,
{
    exhaustive_maps_helper(a, b, keys, values)
}

/// Generates all [`BTreeMap`]s with a given number of distinct values, with keys from one iterator
/// and values from another.
///
/// The key iterator should not repeat any elements, but this is not enforced; a repeated key makes
/// the same map reachable more than once.
///
/// A map with $k$ entries has between 1 and $k$ distinct values, or 0 if $k$ is 0, so the requested
/// interval is intersected with that range; a map whose size leaves the intersection empty is not
/// generated at all.
///
/// If either input iterator is infinite, the output length is also infinite.
///
/// If the key iterator length is $n$ and the value iterator length is $m$, the output length is
/// $$
/// \sum_{k=0}^{n}} \binom{n}{k} \sum_{i=d_0}^{d_0}} S(k, i) \frac{m!}{(m-i)!},
/// $$
/// where $S$ is a Stirling number of the second kind.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\ell + T^\prime(i) + T^{\prime\prime}(i))$
///
/// $M(i) = O(\ell + M^\prime(i) + M^{\prime\prime}(i))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of `keys`, $T^{\prime\prime}$ and
/// $M^{\prime\prime}$ are the time and memory functions of `values`, and $\ell$ is the number of
/// entries in the $i$th output.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::maps::exhaustive::exhaustive_b_tree_maps_fixed_unique_value_count;
/// use maplit::btreemap;
///
/// let xss = exhaustive_b_tree_maps_fixed_unique_value_count(1, 'a'..='b', 0..3u8).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         btreemap! {'a' => 0},
///         btreemap! {'b' => 0},
///         btreemap! {'a' => 1},
///         btreemap! {'a' => 0, 'b' => 0},
///         btreemap! {'a' => 2},
///         btreemap! {'b' => 1},
///         btreemap! {'b' => 2},
///         btreemap! {'a' => 1, 'b' => 1},
///         btreemap! {'a' => 2, 'b' => 2}
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_b_tree_maps_fixed_unique_value_count<I: Iterator, J: Clone + Iterator>(
    unique_value_count: u64,
    keys: I,
    values: J,
) -> ExhaustiveMapsWithUniqueValueCount<I, J, BTreeMap<I::Item, J::Item>>
where
    I::Item: Clone + Ord,
    J::Item: Clone,
{
    exhaustive_maps_with_unique_value_count_helper(
        0,
        u64::MAX,
        unique_value_count,
        unique_value_count,
        keys,
        values,
    )
}

/// Generates all [`BTreeMap`]s whose number of distinct values is in the half-open interval $[a,
/// b)$, with keys from one iterator and values from another.
///
/// The key iterator should not repeat any elements, but this is not enforced; a repeated key makes
/// the same map reachable more than once.
///
/// A map with $k$ entries has between 1 and $k$ distinct values, or 0 if $k$ is 0, so the requested
/// interval is intersected with that range; a map whose size leaves the intersection empty is not
/// generated at all.
///
/// If either input iterator is infinite, the output length is also infinite.
///
/// If the key iterator length is $n$ and the value iterator length is $m$, the output length is
/// $$
/// \sum_{k=0}^{n}} \binom{n}{k} \sum_{i=a}^{b-1}} S(k, i) \frac{m!}{(m-i)!},
/// $$
/// where $S$ is a Stirling number of the second kind.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\ell + T^\prime(i) + T^{\prime\prime}(i))$
///
/// $M(i) = O(\ell + M^\prime(i) + M^{\prime\prime}(i))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of `keys`, $T^{\prime\prime}$ and
/// $M^{\prime\prime}$ are the time and memory functions of `values`, and $\ell$ is the number of
/// entries in the $i$th output.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::maps::exhaustive::exhaustive_b_tree_maps_unique_value_count_range;
/// use maplit::btreemap;
///
/// let xss = exhaustive_b_tree_maps_unique_value_count_range(2, 3, 'a'..='c', 0..3u8)
///     .take(12)
///     .collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         btreemap! {'a' => 0, 'b' => 1},
///         btreemap! {'a' => 0, 'c' => 1},
///         btreemap! {'a' => 1, 'b' => 0},
///         btreemap! {'b' => 0, 'c' => 1},
///         btreemap! {'a' => 0, 'b' => 2},
///         btreemap! {'a' => 1, 'c' => 0},
///         btreemap! {'a' => 2, 'b' => 0},
///         btreemap! {'a' => 0, 'b' => 0, 'c' => 1},
///         btreemap! {'a' => 1, 'b' => 2},
///         btreemap! {'a' => 0, 'c' => 2},
///         btreemap! {'a' => 2, 'b' => 1},
///         btreemap! {'b' => 1, 'c' => 0}
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_b_tree_maps_unique_value_count_range<I: Iterator, J: Clone + Iterator>(
    a: u64,
    b: u64,
    keys: I,
    values: J,
) -> ExhaustiveMapsWithUniqueValueCount<I, J, BTreeMap<I::Item, J::Item>>
where
    I::Item: Clone + Ord,
    J::Item: Clone,
{
    if b == 0 {
        exhaustive_maps_with_unique_value_count_helper(1, 0, 1, 0, keys, values)
    } else {
        exhaustive_maps_with_unique_value_count_helper(0, u64::MAX, a, b - 1, keys, values)
    }
}

/// Generates all [`BTreeMap`]s whose number of distinct values is in the closed interval $[a, b]$,
/// with keys from one iterator and values from another.
///
/// The key iterator should not repeat any elements, but this is not enforced; a repeated key makes
/// the same map reachable more than once.
///
/// A map with $k$ entries has between 1 and $k$ distinct values, or 0 if $k$ is 0, so the requested
/// interval is intersected with that range; a map whose size leaves the intersection empty is not
/// generated at all.
///
/// If either input iterator is infinite, the output length is also infinite.
///
/// If the key iterator length is $n$ and the value iterator length is $m$, the output length is
/// $$
/// \sum_{k=0}^{n}} \binom{n}{k} \sum_{i=a}^{b}} S(k, i) \frac{m!}{(m-i)!},
/// $$
/// where $S$ is a Stirling number of the second kind.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\ell + T^\prime(i) + T^{\prime\prime}(i))$
///
/// $M(i) = O(\ell + M^\prime(i) + M^{\prime\prime}(i))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of `keys`, $T^{\prime\prime}$ and
/// $M^{\prime\prime}$ are the time and memory functions of `values`, and $\ell$ is the number of
/// entries in the $i$th output.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::maps::exhaustive::exhaustive_b_tree_maps_unique_value_count_inclusive_range;
/// use maplit::btreemap;
///
/// let xss = exhaustive_b_tree_maps_unique_value_count_inclusive_range(1, 1, 'a'..='b', 0..3u8)
///     .collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         btreemap! {'a' => 0},
///         btreemap! {'b' => 0},
///         btreemap! {'a' => 1},
///         btreemap! {'a' => 0, 'b' => 0},
///         btreemap! {'a' => 2},
///         btreemap! {'b' => 1},
///         btreemap! {'b' => 2},
///         btreemap! {'a' => 1, 'b' => 1},
///         btreemap! {'a' => 2, 'b' => 2}
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_b_tree_maps_unique_value_count_inclusive_range<I: Iterator, J: Clone + Iterator>(
    a: u64,
    b: u64,
    keys: I,
    values: J,
) -> ExhaustiveMapsWithUniqueValueCount<I, J, BTreeMap<I::Item, J::Item>>
where
    I::Item: Clone + Ord,
    J::Item: Clone,
{
    exhaustive_maps_with_unique_value_count_helper(0, u64::MAX, a, b, keys, values)
}

/// Generates all [`BTreeMap`]s whose size is in the closed interval $[a_s, b_s]$ and whose number
/// of distinct values is in the closed interval $[a_v, b_v]$, with keys from one iterator and
/// values from another.
///
/// The key iterator should not repeat any elements, but this is not enforced; a repeated key makes
/// the same map reachable more than once.
///
/// A map with $k$ entries has between 1 and $k$ distinct values, or 0 if $k$ is 0, so the requested
/// interval is intersected with that range; a map whose size leaves the intersection empty is not
/// generated at all.
///
/// If either input iterator is infinite, the output length is also infinite.
///
/// If the key iterator length is $n$ and the value iterator length is $m$, the output length is
/// $$
/// \sum_{k=a_s}^{b_s}} \binom{n}{k} \sum_{i=a_v}^{b_v}} S(k, i) \frac{m!}{(m-i)!},
/// $$
/// where $S$ is a Stirling number of the second kind.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\ell + T^\prime(i) + T^{\prime\prime}(i))$
///
/// $M(i) = O(\ell + M^\prime(i) + M^{\prime\prime}(i))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of `keys`, $T^{\prime\prime}$ and
/// $M^{\prime\prime}$ are the time and memory functions of `values`, and $\ell$ is the number of
/// entries in the $i$th output.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::maps::exhaustive::*;
/// use maplit::btreemap;
///
/// let xss = exhaustive_b_tree_maps_size_and_unique_value_count_inclusive_range(
///     2,
///     3,
///     1,
///     1,
///     'a'..='c',
///     0..3u8,
/// )
/// .collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         btreemap! {'a' => 0, 'b' => 0},
///         btreemap! {'a' => 0, 'c' => 0},
///         btreemap! {'a' => 1, 'b' => 1},
///         btreemap! {'b' => 0, 'c' => 0},
///         btreemap! {'a' => 2, 'b' => 2},
///         btreemap! {'a' => 1, 'c' => 1},
///         btreemap! {'a' => 2, 'c' => 2},
///         btreemap! {'a' => 0, 'b' => 0, 'c' => 0},
///         btreemap! {'b' => 1, 'c' => 1},
///         btreemap! {'a' => 1, 'b' => 1, 'c' => 1},
///         btreemap! {'b' => 2, 'c' => 2},
///         btreemap! {'a' => 2, 'b' => 2, 'c' => 2}
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_b_tree_maps_size_and_unique_value_count_inclusive_range<
    I: Iterator,
    J: Clone + Iterator,
>(
    size_a: u64,
    size_b: u64,
    unique_value_count_a: u64,
    unique_value_count_b: u64,
    keys: I,
    values: J,
) -> ExhaustiveMapsWithUniqueValueCount<I, J, BTreeMap<I::Item, J::Item>>
where
    I::Item: Clone + Ord,
    J::Item: Clone,
{
    exhaustive_maps_with_unique_value_count_helper(
        size_a,
        size_b,
        unique_value_count_a,
        unique_value_count_b,
        keys,
        values,
    )
}

/// Generates all [`HashMap`]s with keys from one iterator and values from another.
///
/// The key iterator should not repeat any elements, but this is not enforced; a repeated key makes
/// the same map reachable more than once.
///
/// If either input iterator is infinite, the output length is also infinite.
///
/// If the key iterator length is $n$ and the value iterator length is $m$, the output length is
/// $$
/// \sum_{k=0}^{n}} \binom{n}{k} m^k.
/// $$
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\ell + T^\prime(i) + T^{\prime\prime}(i))$
///
/// $M(i) = O(\ell + M^\prime(i) + M^{\prime\prime}(i))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of `keys`, $T^{\prime\prime}$ and
/// $M^{\prime\prime}$ are the time and memory functions of `values`, and $\ell$ is the number of
/// entries in the $i$th output.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::maps::exhaustive::exhaustive_hash_maps;
/// use maplit::hashmap;
///
/// let xss = exhaustive_hash_maps('a'..='b', 0..2u8).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         hashmap! {},
///         hashmap! {'a' => 0},
///         hashmap! {'a' => 1},
///         hashmap! {'a' => 0, 'b' => 0},
///         hashmap! {'b' => 0},
///         hashmap! {'a' => 0, 'b' => 1},
///         hashmap! {'b' => 1},
///         hashmap! {'a' => 1, 'b' => 0},
///         hashmap! {'a' => 1, 'b' => 1}
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_hash_maps<I: Iterator, J: Clone + Iterator>(
    keys: I,
    values: J,
) -> ExhaustiveMaps<I, J, HashMap<I::Item, J::Item>>
where
    I::Item: Clone + Eq + Hash,
    J::Item: Clone,
{
    exhaustive_maps_helper(0, u64::MAX, keys, values)
}

/// Generates all [`HashMap`]s of a given size, with keys from one iterator and values from another.
///
/// The key iterator should not repeat any elements, but this is not enforced; a repeated key makes
/// the same map reachable more than once.
///
/// If either input iterator is infinite, the output length is also infinite.
///
/// If the key iterator length is $n$ and the value iterator length is $m$, the output length is
/// $$
/// \sum_{k=k_0}^{k_0}} \binom{n}{k} m^k.
/// $$
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\ell + T^\prime(i) + T^{\prime\prime}(i))$
///
/// $M(i) = O(\ell + M^\prime(i) + M^{\prime\prime}(i))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of `keys`, $T^{\prime\prime}$ and
/// $M^{\prime\prime}$ are the time and memory functions of `values`, and $\ell$ is the number of
/// entries in the $i$th output.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::maps::exhaustive::exhaustive_hash_maps_fixed_size;
/// use maplit::hashmap;
///
/// let xss = exhaustive_hash_maps_fixed_size(2, 'a'..='c', 0..2u8).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         hashmap! {'a' => 0, 'b' => 0},
///         hashmap! {'a' => 0, 'c' => 0},
///         hashmap! {'a' => 0, 'b' => 1},
///         hashmap! {'b' => 0, 'c' => 0},
///         hashmap! {'a' => 1, 'b' => 0},
///         hashmap! {'a' => 0, 'c' => 1},
///         hashmap! {'a' => 1, 'b' => 1},
///         hashmap! {'b' => 0, 'c' => 1},
///         hashmap! {'a' => 1, 'c' => 0},
///         hashmap! {'b' => 1, 'c' => 0},
///         hashmap! {'a' => 1, 'c' => 1},
///         hashmap! {'b' => 1, 'c' => 1}
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_hash_maps_fixed_size<I: Iterator, J: Clone + Iterator>(
    size: u64,
    keys: I,
    values: J,
) -> ExhaustiveMaps<I, J, HashMap<I::Item, J::Item>>
where
    I::Item: Clone + Eq + Hash,
    J::Item: Clone,
{
    exhaustive_maps_helper(size, size, keys, values)
}

/// Generates all [`HashMap`]s with a minimum size, with keys from one iterator and values from
/// another.
///
/// The key iterator should not repeat any elements, but this is not enforced; a repeated key makes
/// the same map reachable more than once.
///
/// If either input iterator is infinite, the output length is also infinite.
///
/// If the key iterator length is $n$ and the value iterator length is $m$, the output length is
/// $$
/// \sum_{k=\\ell}^{n}} \binom{n}{k} m^k.
/// $$
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\ell + T^\prime(i) + T^{\prime\prime}(i))$
///
/// $M(i) = O(\ell + M^\prime(i) + M^{\prime\prime}(i))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of `keys`, $T^{\prime\prime}$ and
/// $M^{\prime\prime}$ are the time and memory functions of `values`, and $\ell$ is the number of
/// entries in the $i$th output.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::maps::exhaustive::exhaustive_hash_maps_min_size;
/// use maplit::hashmap;
///
/// let xss = exhaustive_hash_maps_min_size(1, 'a'..='b', 0..2u8).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         hashmap! {'a' => 0},
///         hashmap! {'b' => 0},
///         hashmap! {'a' => 1},
///         hashmap! {'a' => 0, 'b' => 0},
///         hashmap! {'b' => 1},
///         hashmap! {'a' => 0, 'b' => 1},
///         hashmap! {'a' => 1, 'b' => 0},
///         hashmap! {'a' => 1, 'b' => 1}
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_hash_maps_min_size<I: Iterator, J: Clone + Iterator>(
    min_size: u64,
    keys: I,
    values: J,
) -> ExhaustiveMaps<I, J, HashMap<I::Item, J::Item>>
where
    I::Item: Clone + Eq + Hash,
    J::Item: Clone,
{
    exhaustive_maps_helper(min_size, u64::MAX, keys, values)
}

/// Generates all [`HashMap`]s with sizes in the half-open interval $[a, b)$, with keys from one
/// iterator and values from another.
///
/// The key iterator should not repeat any elements, but this is not enforced; a repeated key makes
/// the same map reachable more than once.
///
/// If either input iterator is infinite, the output length is also infinite.
///
/// If the key iterator length is $n$ and the value iterator length is $m$, the output length is
/// $$
/// \sum_{k=a}^{b-1}} \binom{n}{k} m^k.
/// $$
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\ell + T^\prime(i) + T^{\prime\prime}(i))$
///
/// $M(i) = O(\ell + M^\prime(i) + M^{\prime\prime}(i))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of `keys`, $T^{\prime\prime}$ and
/// $M^{\prime\prime}$ are the time and memory functions of `values`, and $\ell$ is the number of
/// entries in the $i$th output.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::maps::exhaustive::exhaustive_hash_maps_size_range;
/// use maplit::hashmap;
///
/// let xss = exhaustive_hash_maps_size_range(1, 3, 'a'..='b', 0..2u8).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         hashmap! {'a' => 0},
///         hashmap! {'b' => 0},
///         hashmap! {'a' => 1},
///         hashmap! {'a' => 0, 'b' => 0},
///         hashmap! {'b' => 1},
///         hashmap! {'a' => 0, 'b' => 1},
///         hashmap! {'a' => 1, 'b' => 0},
///         hashmap! {'a' => 1, 'b' => 1}
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_hash_maps_size_range<I: Iterator, J: Clone + Iterator>(
    a: u64,
    b: u64,
    keys: I,
    values: J,
) -> ExhaustiveMaps<I, J, HashMap<I::Item, J::Item>>
where
    I::Item: Clone + Eq + Hash,
    J::Item: Clone,
{
    if b == 0 {
        exhaustive_maps_helper(1, 0, keys, values)
    } else {
        exhaustive_maps_helper(a, b - 1, keys, values)
    }
}

/// Generates all [`HashMap`]s with sizes in the closed interval $[a, b]$, with keys from one
/// iterator and values from another.
///
/// The key iterator should not repeat any elements, but this is not enforced; a repeated key makes
/// the same map reachable more than once.
///
/// If either input iterator is infinite, the output length is also infinite.
///
/// If the key iterator length is $n$ and the value iterator length is $m$, the output length is
/// $$
/// \sum_{k=a}^{b}} \binom{n}{k} m^k.
/// $$
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\ell + T^\prime(i) + T^{\prime\prime}(i))$
///
/// $M(i) = O(\ell + M^\prime(i) + M^{\prime\prime}(i))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of `keys`, $T^{\prime\prime}$ and
/// $M^{\prime\prime}$ are the time and memory functions of `values`, and $\ell$ is the number of
/// entries in the $i$th output.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::maps::exhaustive::exhaustive_hash_maps_size_inclusive_range;
/// use maplit::hashmap;
///
/// let xss = exhaustive_hash_maps_size_inclusive_range(1, 2, 'a'..='b', 0..2u8).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         hashmap! {'a' => 0},
///         hashmap! {'b' => 0},
///         hashmap! {'a' => 1},
///         hashmap! {'a' => 0, 'b' => 0},
///         hashmap! {'b' => 1},
///         hashmap! {'a' => 0, 'b' => 1},
///         hashmap! {'a' => 1, 'b' => 0},
///         hashmap! {'a' => 1, 'b' => 1}
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_hash_maps_size_inclusive_range<I: Iterator, J: Clone + Iterator>(
    a: u64,
    b: u64,
    keys: I,
    values: J,
) -> ExhaustiveMaps<I, J, HashMap<I::Item, J::Item>>
where
    I::Item: Clone + Eq + Hash,
    J::Item: Clone,
{
    exhaustive_maps_helper(a, b, keys, values)
}

/// Generates all [`HashMap`]s with a given number of distinct values, with keys from one iterator
/// and values from another.
///
/// The key iterator should not repeat any elements, but this is not enforced; a repeated key makes
/// the same map reachable more than once.
///
/// A map with $k$ entries has between 1 and $k$ distinct values, or 0 if $k$ is 0, so the requested
/// interval is intersected with that range; a map whose size leaves the intersection empty is not
/// generated at all.
///
/// If either input iterator is infinite, the output length is also infinite.
///
/// If the key iterator length is $n$ and the value iterator length is $m$, the output length is
/// $$
/// \sum_{k=0}^{n}} \binom{n}{k} \sum_{i=d_0}^{d_0}} S(k, i) \frac{m!}{(m-i)!},
/// $$
/// where $S$ is a Stirling number of the second kind.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\ell + T^\prime(i) + T^{\prime\prime}(i))$
///
/// $M(i) = O(\ell + M^\prime(i) + M^{\prime\prime}(i))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of `keys`, $T^{\prime\prime}$ and
/// $M^{\prime\prime}$ are the time and memory functions of `values`, and $\ell$ is the number of
/// entries in the $i$th output.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::maps::exhaustive::exhaustive_hash_maps_fixed_unique_value_count;
/// use maplit::hashmap;
///
/// let xss = exhaustive_hash_maps_fixed_unique_value_count(1, 'a'..='b', 0..3u8).collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         hashmap! {'a' => 0},
///         hashmap! {'b' => 0},
///         hashmap! {'a' => 1},
///         hashmap! {'a' => 0, 'b' => 0},
///         hashmap! {'a' => 2},
///         hashmap! {'b' => 1},
///         hashmap! {'b' => 2},
///         hashmap! {'a' => 1, 'b' => 1},
///         hashmap! {'a' => 2, 'b' => 2}
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_hash_maps_fixed_unique_value_count<I: Iterator, J: Clone + Iterator>(
    unique_value_count: u64,
    keys: I,
    values: J,
) -> ExhaustiveMapsWithUniqueValueCount<I, J, HashMap<I::Item, J::Item>>
where
    I::Item: Clone + Eq + Hash,
    J::Item: Clone,
{
    exhaustive_maps_with_unique_value_count_helper(
        0,
        u64::MAX,
        unique_value_count,
        unique_value_count,
        keys,
        values,
    )
}

/// Generates all [`HashMap`]s whose number of distinct values is in the half-open interval $[a,
/// b)$, with keys from one iterator and values from another.
///
/// The key iterator should not repeat any elements, but this is not enforced; a repeated key makes
/// the same map reachable more than once.
///
/// A map with $k$ entries has between 1 and $k$ distinct values, or 0 if $k$ is 0, so the requested
/// interval is intersected with that range; a map whose size leaves the intersection empty is not
/// generated at all.
///
/// If either input iterator is infinite, the output length is also infinite.
///
/// If the key iterator length is $n$ and the value iterator length is $m$, the output length is
/// $$
/// \sum_{k=0}^{n}} \binom{n}{k} \sum_{i=a}^{b-1}} S(k, i) \frac{m!}{(m-i)!},
/// $$
/// where $S$ is a Stirling number of the second kind.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\ell + T^\prime(i) + T^{\prime\prime}(i))$
///
/// $M(i) = O(\ell + M^\prime(i) + M^{\prime\prime}(i))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of `keys`, $T^{\prime\prime}$ and
/// $M^{\prime\prime}$ are the time and memory functions of `values`, and $\ell$ is the number of
/// entries in the $i$th output.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::maps::exhaustive::exhaustive_hash_maps_unique_value_count_range;
/// use maplit::hashmap;
///
/// let xss = exhaustive_hash_maps_unique_value_count_range(2, 3, 'a'..='c', 0..3u8)
///     .take(12)
///     .collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         hashmap! {'a' => 0, 'b' => 1},
///         hashmap! {'a' => 0, 'c' => 1},
///         hashmap! {'a' => 1, 'b' => 0},
///         hashmap! {'b' => 0, 'c' => 1},
///         hashmap! {'a' => 0, 'b' => 2},
///         hashmap! {'a' => 1, 'c' => 0},
///         hashmap! {'a' => 2, 'b' => 0},
///         hashmap! {'a' => 0, 'b' => 0, 'c' => 1},
///         hashmap! {'a' => 1, 'b' => 2},
///         hashmap! {'a' => 0, 'c' => 2},
///         hashmap! {'a' => 2, 'b' => 1},
///         hashmap! {'b' => 1, 'c' => 0}
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_hash_maps_unique_value_count_range<I: Iterator, J: Clone + Iterator>(
    a: u64,
    b: u64,
    keys: I,
    values: J,
) -> ExhaustiveMapsWithUniqueValueCount<I, J, HashMap<I::Item, J::Item>>
where
    I::Item: Clone + Eq + Hash,
    J::Item: Clone,
{
    if b == 0 {
        exhaustive_maps_with_unique_value_count_helper(1, 0, 1, 0, keys, values)
    } else {
        exhaustive_maps_with_unique_value_count_helper(0, u64::MAX, a, b - 1, keys, values)
    }
}

/// Generates all [`HashMap`]s whose number of distinct values is in the closed interval $[a, b]$,
/// with keys from one iterator and values from another.
///
/// The key iterator should not repeat any elements, but this is not enforced; a repeated key makes
/// the same map reachable more than once.
///
/// A map with $k$ entries has between 1 and $k$ distinct values, or 0 if $k$ is 0, so the requested
/// interval is intersected with that range; a map whose size leaves the intersection empty is not
/// generated at all.
///
/// If either input iterator is infinite, the output length is also infinite.
///
/// If the key iterator length is $n$ and the value iterator length is $m$, the output length is
/// $$
/// \sum_{k=0}^{n}} \binom{n}{k} \sum_{i=a}^{b}} S(k, i) \frac{m!}{(m-i)!},
/// $$
/// where $S$ is a Stirling number of the second kind.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\ell + T^\prime(i) + T^{\prime\prime}(i))$
///
/// $M(i) = O(\ell + M^\prime(i) + M^{\prime\prime}(i))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of `keys`, $T^{\prime\prime}$ and
/// $M^{\prime\prime}$ are the time and memory functions of `values`, and $\ell$ is the number of
/// entries in the $i$th output.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::maps::exhaustive::exhaustive_hash_maps_unique_value_count_inclusive_range;
/// use maplit::hashmap;
///
/// let xss = exhaustive_hash_maps_unique_value_count_inclusive_range(1, 1, 'a'..='b', 0..3u8)
///     .collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         hashmap! {'a' => 0},
///         hashmap! {'b' => 0},
///         hashmap! {'a' => 1},
///         hashmap! {'a' => 0, 'b' => 0},
///         hashmap! {'a' => 2},
///         hashmap! {'b' => 1},
///         hashmap! {'b' => 2},
///         hashmap! {'a' => 1, 'b' => 1},
///         hashmap! {'a' => 2, 'b' => 2}
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_hash_maps_unique_value_count_inclusive_range<I: Iterator, J: Clone + Iterator>(
    a: u64,
    b: u64,
    keys: I,
    values: J,
) -> ExhaustiveMapsWithUniqueValueCount<I, J, HashMap<I::Item, J::Item>>
where
    I::Item: Clone + Eq + Hash,
    J::Item: Clone,
{
    exhaustive_maps_with_unique_value_count_helper(0, u64::MAX, a, b, keys, values)
}

/// Generates all [`HashMap`]s whose size is in the closed interval $[a_s, b_s]$ and whose number of
/// distinct values is in the closed interval $[a_v, b_v]$, with keys from one iterator and values
/// from another.
///
/// The key iterator should not repeat any elements, but this is not enforced; a repeated key makes
/// the same map reachable more than once.
///
/// A map with $k$ entries has between 1 and $k$ distinct values, or 0 if $k$ is 0, so the requested
/// interval is intersected with that range; a map whose size leaves the intersection empty is not
/// generated at all.
///
/// If either input iterator is infinite, the output length is also infinite.
///
/// If the key iterator length is $n$ and the value iterator length is $m$, the output length is
/// $$
/// \sum_{k=a_s}^{b_s}} \binom{n}{k} \sum_{i=a_v}^{b_v}} S(k, i) \frac{m!}{(m-i)!},
/// $$
/// where $S$ is a Stirling number of the second kind.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\ell + T^\prime(i) + T^{\prime\prime}(i))$
///
/// $M(i) = O(\ell + M^\prime(i) + M^{\prime\prime}(i))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of `keys`, $T^{\prime\prime}$ and
/// $M^{\prime\prime}$ are the time and memory functions of `values`, and $\ell$ is the number of
/// entries in the $i$th output.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::maps::exhaustive::*;
/// use maplit::hashmap;
///
/// let xss = exhaustive_hash_maps_size_and_unique_value_count_inclusive_range(
///     2,
///     3,
///     1,
///     1,
///     'a'..='c',
///     0..3u8,
/// )
/// .collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         hashmap! {'a' => 0, 'b' => 0},
///         hashmap! {'a' => 0, 'c' => 0},
///         hashmap! {'a' => 1, 'b' => 1},
///         hashmap! {'b' => 0, 'c' => 0},
///         hashmap! {'a' => 2, 'b' => 2},
///         hashmap! {'a' => 1, 'c' => 1},
///         hashmap! {'a' => 2, 'c' => 2},
///         hashmap! {'a' => 0, 'b' => 0, 'c' => 0},
///         hashmap! {'b' => 1, 'c' => 1},
///         hashmap! {'a' => 1, 'b' => 1, 'c' => 1},
///         hashmap! {'b' => 2, 'c' => 2},
///         hashmap! {'a' => 2, 'b' => 2, 'c' => 2}
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_hash_maps_size_and_unique_value_count_inclusive_range<
    I: Iterator,
    J: Clone + Iterator,
>(
    size_a: u64,
    size_b: u64,
    unique_value_count_a: u64,
    unique_value_count_b: u64,
    keys: I,
    values: J,
) -> ExhaustiveMapsWithUniqueValueCount<I, J, HashMap<I::Item, J::Item>>
where
    I::Item: Clone + Eq + Hash,
    J::Item: Clone,
{
    exhaustive_maps_with_unique_value_count_helper(
        size_a,
        size_b,
        unique_value_count_a,
        unique_value_count_b,
        keys,
        values,
    )
}
