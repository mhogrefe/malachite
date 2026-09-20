// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::conversion::traits::ExactFrom;
use crate::num::random::geometric::{
    GeometricRandomNaturalValues, geometric_random_unsigned_inclusive_range,
    geometric_random_unsigneds,
};
use crate::num::random::{
    RandomUnsignedInclusiveRange, RandomUnsignedRange, VariableRangeGenerator,
    random_unsigned_inclusive_range, random_unsigned_range,
};
use crate::random::Seed;
#[cfg(not(feature = "std"))]
use alloc::collections::BTreeMap;
use core::cmp::{max, min};
use core::hash::Hash;
#[cfg(not(feature = "std"))]
use hashbrown::{HashMap, HashSet};
#[cfg(feature = "std")]
use std::collections::{BTreeMap, HashMap, HashSet};

/// Generates random [`BTreeMap`]s of a fixed size.
///
/// This `struct` is created by [`random_b_tree_maps_fixed_size`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct RandomBTreeMapsFixedSize<I: Iterator, J: Iterator>
where
    I::Item: Ord,
{
    size: usize,
    keys: I,
    values: J,
}

impl<I: Iterator, J: Iterator> Iterator for RandomBTreeMapsFixedSize<I, J>
where
    I::Item: Ord,
{
    type Item = BTreeMap<I::Item, J::Item>;

    fn next(&mut self) -> Option<BTreeMap<I::Item, J::Item>> {
        let mut map = BTreeMap::new();
        while map.len() < self.size {
            // A repeated key costs a key draw but no value draw, so exactly one value is consumed
            // per entry, paired with its key in the order the keys are drawn rather than in the
            // map's own order. The [`HashMap`] versions must do this to be reproducible, and these
            // match them.
            let key = self.keys.next().unwrap();
            map.entry(key)
                .or_insert_with(|| self.values.next().unwrap());
        }
        Some(map)
    }
}

/// Generates random [`BTreeMap`]s of a given size, with keys from one iterator and values from
/// another.
///
/// The key iterator must generate at least `size` distinct elements; otherwise, this iterator will
/// hang. The value iterator has no such requirement, since values may repeat.
///
/// $$
/// P(\\{(k_i, v_i)\\}_ {i=0}^{n-1}) = n!\prod_ {i=0}^{n-1}P(k_i)P(v_i).
/// $$
///
/// The above formula assumes that the map is valid, \emph{i.e.} its keys are distinct. The $n!$
/// counts the orders in which the keys can be drawn; the values are drawn in the same order, so
/// each key order pairs with exactly one value order that produces the map.
///
/// If `size` is 0, the output consists of the empty map, repeated.
///
/// `keys` and `values` must be infinite.
///
/// # Expected complexity per iteration
/// $T(i) = O(n (T^\prime(i) + T^{\prime\prime}(i)))$
///
/// $M(i) = O(n (M^\prime(i) + M^{\prime\prime}(i)))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of `keys`, $T^{\prime\prime}$ and
/// $M^{\prime\prime}$ are those of `values`, and $n$ is `size`.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::chars::random::random_char_inclusive_range;
/// use malachite_base::maps::random::random_b_tree_maps_fixed_size;
/// use malachite_base::num::random::random_unsigned_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
/// use maplit::btreemap;
///
/// let xs = random_b_tree_maps_fixed_size(
///     2,
///     random_char_inclusive_range(EXAMPLE_SEED.fork("keys"), 'a', 'c'),
///     random_unsigned_inclusive_range::<u8>(EXAMPLE_SEED.fork("values"), 0, 2),
/// );
/// let values = xs.take(10).collect_vec();
/// assert_eq!(
///     values,
///     &[
///         btreemap! {'a' => 0, 'b' => 1},
///         btreemap! {'a' => 2, 'b' => 0},
///         btreemap! {'a' => 0, 'b' => 0},
///         btreemap! {'a' => 2, 'c' => 0},
///         btreemap! {'a' => 0, 'c' => 0},
///         btreemap! {'a' => 0, 'c' => 1},
///         btreemap! {'a' => 2, 'b' => 1},
///         btreemap! {'b' => 1, 'c' => 2},
///         btreemap! {'a' => 2, 'c' => 0},
///         btreemap! {'b' => 0, 'c' => 1}
///     ]
/// );
/// ```
#[inline]
pub fn random_b_tree_maps_fixed_size<I: Iterator, J: Iterator>(
    size: u64,
    keys: I,
    values: J,
) -> RandomBTreeMapsFixedSize<I, J>
where
    I::Item: Ord,
{
    RandomBTreeMapsFixedSize {
        size: usize::exact_from(size),
        keys,
        values,
    }
}

/// Generates random [`BTreeMap`]s with sizes from an iterator.
#[derive(Clone, Debug)]
pub struct RandomBTreeMaps<
    K: Ord,
    V,
    I: Iterator<Item = u64>,
    J: Iterator<Item = K>,
    L: Iterator<Item = V>,
> {
    sizes: I,
    keys: J,
    values: L,
}

impl<K: Ord, V, I: Iterator<Item = u64>, J: Iterator<Item = K>, L: Iterator<Item = V>> Iterator
    for RandomBTreeMaps<K, V, I, J, L>
{
    type Item = BTreeMap<K, V>;

    fn next(&mut self) -> Option<BTreeMap<K, V>> {
        let size = usize::exact_from(self.sizes.next().unwrap());
        let mut map = BTreeMap::new();
        while map.len() < size {
            // A repeated key costs a key draw but no value draw, so exactly one value is consumed
            // per entry, paired with its key in the order the keys are drawn rather than in the
            // map's own order. The [`HashMap`] versions must do this to be reproducible, and these
            // match them.
            let key = self.keys.next().unwrap();
            map.entry(key)
                .or_insert_with(|| self.values.next().unwrap());
        }
        Some(map)
    }
}

/// Generates random [`BTreeMap`]s with keys from one iterator, values from another, and sizes from
/// a third.
///
/// The key iterator must generate at least as many distinct elements as any size that is requested;
/// otherwise, this iterator will hang. The value iterator has no such requirement, since values may
/// repeat.
///
/// $$
/// P(\\{(k_i, v_i)\\}_ {i=0}^{n-1}) = n!P(n)\prod_ {i=0}^{n-1}P(k_i)P(v_i).
/// $$
///
/// The above formula assumes that the map is valid, \emph{i.e.} its keys are distinct. The $n!$
/// counts the orders in which the keys can be drawn; the values are drawn in the same order, so
/// each key order pairs with exactly one value order that produces the map.
///
/// `keys_gen` and `values_gen` must produce infinite iterators.
///
/// # Expected complexity per iteration
/// $T(i) = O(m (T^\prime(i) + T^{\prime\prime}(i)))$
///
/// $M(i) = O(m (M^\prime(i) + M^{\prime\prime}(i)))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of the iterators produced by `keys_gen`,
/// $T^{\prime\prime}$ and $M^{\prime\prime}$ are those of the iterators produced by `values_gen`,
/// and $m$ is the mean size.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::maps::random::random_b_tree_maps_from_size_iterator;
/// use malachite_base::num::random::geometric::geometric_random_unsigneds;
/// use malachite_base::num::random::random_primitive_ints;
/// use malachite_base::random::EXAMPLE_SEED;
/// use maplit::btreemap;
///
/// let xs = random_b_tree_maps_from_size_iterator(
///     EXAMPLE_SEED,
///     &|seed| geometric_random_unsigneds::<u64>(seed, 2, 1),
///     &random_primitive_ints::<u8>,
///     &random_primitive_ints::<u8>,
/// );
/// let values = xs.take(10).collect_vec();
/// assert_eq!(
///     values,
///     &[
///         btreemap! {},
///         btreemap! {},
///         btreemap! {51 => 64, 79 => 36, 80 => 32},
///         btreemap! {},
///         btreemap! {10 => 158},
///         btreemap! {59 => 39},
///         btreemap! {160 => 91, 246 => 8, 253 => 71},
///         btreemap! {245 => 18},
///         btreemap! {53 => 243, 85 => 220, 139 => 67, 214 => 153, 219 => 134},
///         btreemap! {233 => 107}
///     ]
/// );
/// ```
pub fn random_b_tree_maps_from_size_iterator<
    K: Ord,
    V,
    I: Iterator<Item = u64>,
    J: Iterator<Item = K>,
    L: Iterator<Item = V>,
>(
    seed: Seed,
    sizes_gen: &dyn Fn(Seed) -> I,
    keys_gen: &dyn Fn(Seed) -> J,
    values_gen: &dyn Fn(Seed) -> L,
) -> RandomBTreeMaps<K, V, I, J, L> {
    RandomBTreeMaps {
        sizes: sizes_gen(seed.fork("sizes")),
        keys: keys_gen(seed.fork("keys")),
        values: values_gen(seed.fork("values")),
    }
}

/// Generates random [`BTreeMap`]s with keys from one iterator and values from another.
///
/// The sizes of the maps are sampled from a geometric distribution with a specified mean $m$, equal
/// to `mean_size_numerator / mean_size_denominator`. $m$ must be greater than 0.
///
/// The key iterator must generate at least as many distinct elements as any size that is requested;
/// otherwise, this iterator will hang. The value iterator has no such requirement, since values may
/// repeat.
///
/// $$
/// P(\\{(k_i, v_i)\\}_ {i=0}^{n-1}) = n!P(n)\prod_ {i=0}^{n-1}P(k_i)P(v_i).
/// $$
///
/// The above formula assumes that the map is valid, \emph{i.e.} its keys are distinct. The $n!$
/// counts the orders in which the keys can be drawn; the values are drawn in the same order, so
/// each key order pairs with exactly one value order that produces the map.
///
/// `keys_gen` and `values_gen` must produce infinite iterators.
///
/// # Expected complexity per iteration
/// $T(i) = O(m (T^\prime(i) + T^{\prime\prime}(i)))$
///
/// $M(i) = O(m (M^\prime(i) + M^{\prime\prime}(i)))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of the iterators produced by `keys_gen`,
/// $T^{\prime\prime}$ and $M^{\prime\prime}$ are those of the iterators produced by `values_gen`,
/// and $m$ is the mean size.
///
/// # Panics
/// Panics if `mean_size_numerator` or `mean_size_denominator` are zero, or, if after being reduced
/// to lowest terms, their sum is greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::bools::random::random_bools;
/// use malachite_base::maps::random::random_b_tree_maps;
/// use malachite_base::num::random::random_unsigned_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
/// use maplit::btreemap;
///
/// let xs = random_b_tree_maps(
///     EXAMPLE_SEED,
///     &|seed| random_unsigned_inclusive_range::<u32>(seed, 1, 100),
///     &random_bools,
///     1,
///     1,
/// );
/// let values = xs.take(20).collect_vec();
/// assert_eq!(
///     values,
///     &[
///         btreemap! {},
///         btreemap! {},
///         btreemap! {33 => false, 78 => true, 80 => false, 82 => false},
///         btreemap! {},
///         btreemap! {40 => true, 49 => false},
///         btreemap! {33 => false, 64 => false},
///         btreemap! {88 => false},
///         btreemap! {43 => false, 100 => false},
///         btreemap! {},
///         btreemap! {},
///         btreemap! {},
///         btreemap! {},
///         btreemap! {70 => false},
///         btreemap! {},
///         btreemap! {6 => false, 74 => true},
///         btreemap! {94 => false},
///         btreemap! {},
///         btreemap! {79 => false},
///         btreemap! {},
///         btreemap! {18 => false, 34 => false}
///     ]
/// );
/// ```
#[inline]
pub fn random_b_tree_maps<I: Iterator, J: Iterator>(
    seed: Seed,
    keys_gen: &dyn Fn(Seed) -> I,
    values_gen: &dyn Fn(Seed) -> J,
    mean_size_numerator: u64,
    mean_size_denominator: u64,
) -> RandomBTreeMaps<I::Item, J::Item, GeometricRandomNaturalValues<u64>, I, J>
where
    I::Item: Ord,
{
    random_b_tree_maps_from_size_iterator(
        seed,
        &|seed_2| geometric_random_unsigneds(seed_2, mean_size_numerator, mean_size_denominator),
        keys_gen,
        values_gen,
    )
}

/// Generates random [`BTreeMap`]s with a minimum size, with keys from one iterator and values from
/// another.
///
/// The sizes of the maps are sampled from a geometric distribution with a specified mean $m$, equal
/// to `mean_size_numerator / mean_size_denominator`. $m$ must be greater than `min_size`.
///
/// The key iterator must generate at least as many distinct elements as any size that is requested;
/// otherwise, this iterator will hang. The value iterator has no such requirement, since values may
/// repeat.
///
/// $$
/// P(\\{(k_i, v_i)\\}_ {i=0}^{n-1}) = n!P(n)\prod_ {i=0}^{n-1}P(k_i)P(v_i).
/// $$
///
/// The above formula assumes that the map is valid, \emph{i.e.} its keys are distinct. The $n!$
/// counts the orders in which the keys can be drawn; the values are drawn in the same order, so
/// each key order pairs with exactly one value order that produces the map.
///
/// `keys_gen` and `values_gen` must produce infinite iterators.
///
/// # Expected complexity per iteration
/// $T(i) = O(m (T^\prime(i) + T^{\prime\prime}(i)))$
///
/// $M(i) = O(m (M^\prime(i) + M^{\prime\prime}(i)))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of the iterators produced by `keys_gen`,
/// $T^{\prime\prime}$ and $M^{\prime\prime}$ are those of the iterators produced by `values_gen`,
/// and $m$ is the mean size.
///
/// # Panics
/// Panics if `mean_size_numerator` or `mean_size_denominator` are zero, if their ratio is less than
/// or equal to `min_size`, or if they are too large and manipulating them leads to arithmetic
/// overflow.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::maps::random::random_b_tree_maps_min_size;
/// use malachite_base::num::random::{random_primitive_ints, random_unsigned_inclusive_range};
/// use malachite_base::random::EXAMPLE_SEED;
/// use maplit::btreemap;
///
/// let xs = random_b_tree_maps_min_size(
///     EXAMPLE_SEED,
///     1,
///     &random_primitive_ints::<u8>,
///     &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 2),
///     4,
///     1,
/// );
/// let values = xs.take(10).collect_vec();
/// assert_eq!(
///     values,
///     &[
///         btreemap! {79 => 0},
///         btreemap! {10 => 0, 51 => 2, 80 => 1},
///         btreemap! {59 => 0, 85 => 0, 160 => 0, 245 => 0, 246 => 2, 253 => 0},
///         btreemap! {53 => 0},
///         btreemap! {214 => 1, 219 => 2},
///         btreemap! {120 => 1, 139 => 1, 233 => 2},
///         btreemap! {33 => 2, 157 => 1, 158 => 0, 161 => 0, 202 => 0, 236 => 1},
///         btreemap! {19 => 0, 72 => 2, 153 => 1, 155 => 0, 194 => 2},
///         btreemap! {25 => 1, 68 => 0, 74 => 0, 80 => 2, 97 => 1, 119 => 0, 236 => 1, 252 => 0},
///         btreemap! {33 => 1, 241 => 2}
///     ]
/// );
/// ```
#[inline]
pub fn random_b_tree_maps_min_size<I: Iterator, J: Iterator>(
    seed: Seed,
    min_size: u64,
    keys_gen: &dyn Fn(Seed) -> I,
    values_gen: &dyn Fn(Seed) -> J,
    mean_size_numerator: u64,
    mean_size_denominator: u64,
) -> RandomBTreeMaps<I::Item, J::Item, GeometricRandomNaturalValues<u64>, I, J>
where
    I::Item: Ord,
{
    random_b_tree_maps_from_size_iterator(
        seed,
        &|seed_2| {
            geometric_random_unsigned_inclusive_range(
                seed_2,
                min_size,
                u64::MAX,
                mean_size_numerator,
                mean_size_denominator,
            )
        },
        keys_gen,
        values_gen,
    )
}

/// Generates random [`BTreeMap`]s with sizes in the half-open interval $[a, b)$, with keys from one
/// iterator and values from another.
///
/// The key iterator must generate at least as many distinct elements as any size that is requested;
/// otherwise, this iterator will hang. The value iterator has no such requirement, since values may
/// repeat.
///
/// $$
/// P(\\{(k_i, v_i)\\}_ {i=0}^{n-1}) = n!P(n)\prod_ {i=0}^{n-1}P(k_i)P(v_i).
/// $$
///
/// The above formula assumes that the map is valid, \emph{i.e.} its keys are distinct. The $n!$
/// counts the orders in which the keys can be drawn; the values are drawn in the same order, so
/// each key order pairs with exactly one value order that produces the map.
///
/// `keys_gen` and `values_gen` must produce infinite iterators.
///
/// # Expected complexity per iteration
/// $T(i) = O(m (T^\prime(i) + T^{\prime\prime}(i)))$
///
/// $M(i) = O(m (M^\prime(i) + M^{\prime\prime}(i)))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of the iterators produced by `keys_gen`,
/// $T^{\prime\prime}$ and $M^{\prime\prime}$ are those of the iterators produced by `values_gen`,
/// and $m$ is the mean size.
///
/// # Panics
/// Panics if $a \geq b$.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::chars::random::random_char_inclusive_range;
/// use malachite_base::maps::random::random_b_tree_maps_size_range;
/// use malachite_base::num::random::random_unsigned_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
/// use maplit::btreemap;
///
/// let xs = random_b_tree_maps_size_range(
///     EXAMPLE_SEED,
///     1,
///     3,
///     &|seed| random_char_inclusive_range(seed, 'a', 'c'),
///     &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 1),
/// );
/// let values = xs.take(20).collect_vec();
/// assert_eq!(
///     values,
///     &[
///         btreemap! {'a' => 0},
///         btreemap! {'b' => 0},
///         btreemap! {'a' => 1, 'b' => 0},
///         btreemap! {'a' => 1, 'b' => 0},
///         btreemap! {'a' => 0, 'c' => 0},
///         btreemap! {'a' => 0, 'c' => 0},
///         btreemap! {'a' => 0},
///         btreemap! {'c' => 0},
///         btreemap! {'a' => 0, 'b' => 1},
///         btreemap! {'b' => 0, 'c' => 0},
///         btreemap! {'a' => 0},
///         btreemap! {'a' => 0, 'c' => 0},
///         btreemap! {'b' => 0, 'c' => 0},
///         btreemap! {'b' => 0},
///         btreemap! {'a' => 0, 'b' => 1},
///         btreemap! {'c' => 0},
///         btreemap! {'b' => 1, 'c' => 1},
///         btreemap! {'b' => 1, 'c' => 1},
///         btreemap! {'a' => 0},
///         btreemap! {'c' => 0}
///     ]
/// );
/// ```
#[inline]
pub fn random_b_tree_maps_size_range<I: Iterator, J: Iterator>(
    seed: Seed,
    a: u64,
    b: u64,
    keys_gen: &dyn Fn(Seed) -> I,
    values_gen: &dyn Fn(Seed) -> J,
) -> RandomBTreeMaps<I::Item, J::Item, RandomUnsignedRange<u64>, I, J>
where
    I::Item: Ord,
{
    random_b_tree_maps_from_size_iterator(
        seed,
        &|seed_2| random_unsigned_range(seed_2, a, b),
        keys_gen,
        values_gen,
    )
}

/// Generates random [`BTreeMap`]s with sizes in the closed interval $[a, b]$, with keys from one
/// iterator and values from another.
///
/// The key iterator must generate at least as many distinct elements as any size that is requested;
/// otherwise, this iterator will hang. The value iterator has no such requirement, since values may
/// repeat.
///
/// $$
/// P(\\{(k_i, v_i)\\}_ {i=0}^{n-1}) = n!P(n)\prod_ {i=0}^{n-1}P(k_i)P(v_i).
/// $$
///
/// The above formula assumes that the map is valid, \emph{i.e.} its keys are distinct. The $n!$
/// counts the orders in which the keys can be drawn; the values are drawn in the same order, so
/// each key order pairs with exactly one value order that produces the map.
///
/// `keys_gen` and `values_gen` must produce infinite iterators.
///
/// # Expected complexity per iteration
/// $T(i) = O(m (T^\prime(i) + T^{\prime\prime}(i)))$
///
/// $M(i) = O(m (M^\prime(i) + M^{\prime\prime}(i)))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of the iterators produced by `keys_gen`,
/// $T^{\prime\prime}$ and $M^{\prime\prime}$ are those of the iterators produced by `values_gen`,
/// and $m$ is the mean size.
///
/// # Panics
/// Panics if $a > b$.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::chars::random::random_char_inclusive_range;
/// use malachite_base::maps::random::random_b_tree_maps_size_inclusive_range;
/// use malachite_base::num::random::random_unsigned_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
/// use maplit::btreemap;
///
/// let xs = random_b_tree_maps_size_inclusive_range(
///     EXAMPLE_SEED,
///     1,
///     2,
///     &|seed| random_char_inclusive_range(seed, 'a', 'c'),
///     &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 1),
/// );
/// let values = xs.take(20).collect_vec();
/// assert_eq!(
///     values,
///     &[
///         btreemap! {'a' => 0},
///         btreemap! {'b' => 0},
///         btreemap! {'a' => 1, 'b' => 0},
///         btreemap! {'a' => 1, 'b' => 0},
///         btreemap! {'a' => 0, 'c' => 0},
///         btreemap! {'a' => 0, 'c' => 0},
///         btreemap! {'a' => 0},
///         btreemap! {'c' => 0},
///         btreemap! {'a' => 0, 'b' => 1},
///         btreemap! {'b' => 0, 'c' => 0},
///         btreemap! {'a' => 0},
///         btreemap! {'a' => 0, 'c' => 0},
///         btreemap! {'b' => 0, 'c' => 0},
///         btreemap! {'b' => 0},
///         btreemap! {'a' => 0, 'b' => 1},
///         btreemap! {'c' => 0},
///         btreemap! {'b' => 1, 'c' => 1},
///         btreemap! {'b' => 1, 'c' => 1},
///         btreemap! {'a' => 0},
///         btreemap! {'c' => 0}
///     ]
/// );
/// ```
#[inline]
pub fn random_b_tree_maps_size_inclusive_range<I: Iterator, J: Iterator>(
    seed: Seed,
    a: u64,
    b: u64,
    keys_gen: &dyn Fn(Seed) -> I,
    values_gen: &dyn Fn(Seed) -> J,
) -> RandomBTreeMaps<I::Item, J::Item, RandomUnsignedInclusiveRange<u64>, I, J>
where
    I::Item: Ord,
{
    random_b_tree_maps_from_size_iterator(
        seed,
        &|seed_2| random_unsigned_inclusive_range(seed_2, a, b),
        keys_gen,
        values_gen,
    )
}

// Given the inclusive range $[a, b]$ of allowed distinct-value counts and the inclusive range
// $[\text{size\_a}, \text{size\_b}]$ of allowed sizes, returns the sub-range of sizes for which
// some allowed count is achievable. A map with $k$ entries has between 1 and $k$ distinct values,
// or 0 if $k$ is 0, so a size of 0 is compatible only with a count of 0, and a positive size $k$
// only with a positive count no greater than $k$.
fn compatible_size_range(size_a: u64, size_b: u64, a: u64, b: u64) -> (u64, u64) {
    assert!(a <= b, "a must be less than or equal to b. a: {a}, b: {b}");
    assert!(
        size_a <= size_b,
        "size_a must be less than or equal to size_b. size_a: {size_a}, size_b: {size_b}"
    );
    let (lo, hi) = if b == 0 {
        (size_a, 0)
    } else {
        (max(size_a, a), size_b)
    };
    assert!(
        lo <= hi,
        "no map size in [{size_a}, {size_b}] can have a number of distinct values in [{a}, {b}]"
    );
    (lo, hi)
}

/// Generates random [`BTreeMap`]s with a restricted number of distinct values.
#[derive(Clone, Debug)]
pub struct RandomBTreeMapsWithUniqueValueCount<
    K: Ord,
    V: Clone + Eq + Hash,
    I: Iterator<Item = u64>,
    J: Iterator<Item = K>,
    L: Iterator<Item = V>,
> {
    sizes: I,
    keys: J,
    values: L,
    range_generator: VariableRangeGenerator,
    a: u64,
    b: u64,
}

impl<
    K: Ord,
    V: Clone + Eq + Hash,
    I: Iterator<Item = u64>,
    J: Iterator<Item = K>,
    L: Iterator<Item = V>,
> Iterator for RandomBTreeMapsWithUniqueValueCount<K, V, I, J, L>
{
    type Item = BTreeMap<K, V>;

    fn next(&mut self) -> Option<BTreeMap<K, V>> {
        let size = self.sizes.next().unwrap();
        // A map with `size` entries has between 1 and `size` distinct values, or 0 if `size` is 0.
        // The size iterator is built so that this range always meets [`self.a`, `self.b`].
        let count = usize::exact_from(
            self.range_generator
                .next_in_inclusive_range::<u64>(max(self.a, min(size, 1)), min(self.b, size)),
        );
        let size = usize::exact_from(size);
        let mut vs = Vec::with_capacity(count);
        let mut seen = HashSet::with_capacity(count);
        while vs.len() < count {
            let value = self.values.next().unwrap();
            if seen.insert(value.clone()) {
                vs.push(value);
            }
        }
        // The first `count` keys drawn take the `count` distinct values, one each, so that every
        // one of them is used; the rest take a uniformly random one of them. The keys are drawn in
        // random order, so this does not favor any particular position in the map.
        let mut map = BTreeMap::new();
        let mut assigned = 0;
        while map.len() < size {
            let key = self.keys.next().unwrap();
            let range_generator = &mut self.range_generator;
            map.entry(key).or_insert_with(|| {
                let value = if assigned < count {
                    vs[assigned].clone()
                } else {
                    vs[range_generator.next_less_than::<usize>(count)].clone()
                };
                assigned += 1;
                value
            });
        }
        Some(map)
    }
}

fn random_b_tree_maps_with_unique_value_count_from_size_iterator<
    K: Ord,
    V: Clone + Eq + Hash,
    I: Iterator<Item = u64>,
    J: Iterator<Item = K>,
    L: Iterator<Item = V>,
>(
    seed: Seed,
    sizes_gen: &dyn Fn(Seed) -> I,
    keys_gen: &dyn Fn(Seed) -> J,
    values_gen: &dyn Fn(Seed) -> L,
    a: u64,
    b: u64,
) -> RandomBTreeMapsWithUniqueValueCount<K, V, I, J, L> {
    RandomBTreeMapsWithUniqueValueCount {
        sizes: sizes_gen(seed.fork("sizes")),
        keys: keys_gen(seed.fork("keys")),
        values: values_gen(seed.fork("values")),
        range_generator: VariableRangeGenerator::new(seed.fork("unique_value_counts")),
        a,
        b,
    }
}

/// Generates random [`BTreeMap`]s with a given number of distinct values, with keys from one
/// iterator and values from another.
///
/// The key iterator must generate at least as many distinct elements as any size that is requested,
/// and the value iterator at least as many distinct elements as any number of distinct values that
/// is requested; otherwise, this iterator will hang.
///
/// $$
/// P(m) = P(n)P(d)d!(n-d)!\frac{\prod_{j=0}^{d-1}c_j}{d^{n-d}}
///     \prod_{i=0}^{n-1}P(k_i)\prod_{j=0}^{d-1}P(w_j).
/// $$
///
/// Here the map $m$ has $n$ entries $(k_i, v_i)$ and $d$ distinct values $w_0, \ldots, w_{d-1}$,
/// the $j$th of which is the value of $c_j$ of the entries; $P(n)$ is the probability of drawing
/// the size $n$, and $P(d)$ that of drawing the distinct-value count $d$, which is uniform over the
/// counts that a size-$n$ map is allowed to have. The formula assumes that the map is valid,
/// \emph{i.e.} its keys are distinct and it has exactly $d$ distinct values. Maps with the same $n$
/// and $d$ are not equally likely: the $\prod_j c_j$ factor favors those whose values are spread
/// evenly over the keys.
///
/// `keys_gen` and `values_gen` must produce infinite iterators.
///
/// If `unique_value_count` is 0, the output consists of the empty map, repeated.
///
/// # Expected complexity per iteration
/// $T(i) = O(m (T^\prime(i) + T^{\prime\prime}(i)))$
///
/// $M(i) = O(m (M^\prime(i) + M^{\prime\prime}(i)))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of the iterators produced by `keys_gen`,
/// $T^{\prime\prime}$ and $M^{\prime\prime}$ are those of the iterators produced by `values_gen`,
/// and $m$ is the mean size.
///
/// # Panics
/// Panics if `mean_size_numerator` or `mean_size_denominator` are zero, if their ratio is less than
/// or equal to `unique_value_count`, or if they are too large and manipulating them leads to
/// arithmetic overflow.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::maps::random::random_b_tree_maps_fixed_unique_value_count;
/// use malachite_base::num::random::{random_primitive_ints, random_unsigned_inclusive_range};
/// use malachite_base::random::EXAMPLE_SEED;
/// use maplit::btreemap;
///
/// let xs = random_b_tree_maps_fixed_unique_value_count(
///     EXAMPLE_SEED,
///     1,
///     &random_primitive_ints::<u8>,
///     &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 2),
///     2,
///     1,
/// );
/// let values = xs.take(10).collect_vec();
/// assert_eq!(
///     values,
///     &[
///         btreemap! {79 => 0},
///         btreemap! {80 => 1},
///         btreemap! {10 => 2, 51 => 2, 59 => 2, 246 => 2, 253 => 2},
///         btreemap! {160 => 0},
///         btreemap! {53 => 0, 85 => 0, 245 => 0},
///         btreemap! {139 => 0, 214 => 0, 219 => 0},
///         btreemap! {120 => 2, 233 => 2},
///         btreemap! {33 => 0, 158 => 0, 236 => 0},
///         btreemap! {202 => 0},
///         btreemap! {157 => 0}
///     ]
/// );
/// ```
#[inline]
pub fn random_b_tree_maps_fixed_unique_value_count<I: Iterator, J: Iterator>(
    seed: Seed,
    unique_value_count: u64,
    keys_gen: &dyn Fn(Seed) -> I,
    values_gen: &dyn Fn(Seed) -> J,
    mean_size_numerator: u64,
    mean_size_denominator: u64,
) -> RandomBTreeMapsWithUniqueValueCount<I::Item, J::Item, GeometricRandomNaturalValues<u64>, I, J>
where
    I::Item: Ord,
    J::Item: Clone + Eq + Hash,
{
    random_b_tree_maps_unique_value_count_inclusive_range(
        seed,
        unique_value_count,
        unique_value_count,
        keys_gen,
        values_gen,
        mean_size_numerator,
        mean_size_denominator,
    )
}

/// Generates random [`BTreeMap`]s whose number of distinct values is in the half-open interval $[a,
/// b)$, with keys from one iterator and values from another.
///
/// The key iterator must generate at least as many distinct elements as any size that is requested,
/// and the value iterator at least as many distinct elements as any number of distinct values that
/// is requested; otherwise, this iterator will hang.
///
/// $$
/// P(m) = P(n)P(d)d!(n-d)!\frac{\prod_{j=0}^{d-1}c_j}{d^{n-d}}
///     \prod_{i=0}^{n-1}P(k_i)\prod_{j=0}^{d-1}P(w_j).
/// $$
///
/// Here the map $m$ has $n$ entries $(k_i, v_i)$ and $d$ distinct values $w_0, \ldots, w_{d-1}$,
/// the $j$th of which is the value of $c_j$ of the entries; $P(n)$ is the probability of drawing
/// the size $n$, and $P(d)$ that of drawing the distinct-value count $d$, which is uniform over the
/// counts that a size-$n$ map is allowed to have. The formula assumes that the map is valid,
/// \emph{i.e.} its keys are distinct and it has exactly $d$ distinct values. Maps with the same $n$
/// and $d$ are not equally likely: the $\prod_j c_j$ factor favors those whose values are spread
/// evenly over the keys.
///
/// `keys_gen` and `values_gen` must produce infinite iterators.
///
/// # Expected complexity per iteration
/// $T(i) = O(m (T^\prime(i) + T^{\prime\prime}(i)))$
///
/// $M(i) = O(m (M^\prime(i) + M^{\prime\prime}(i)))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of the iterators produced by `keys_gen`,
/// $T^{\prime\prime}$ and $M^{\prime\prime}$ are those of the iterators produced by `values_gen`,
/// and $m$ is the mean size.
///
/// # Panics
/// Panics if $a \geq b$, if `mean_size_numerator` or `mean_size_denominator` are zero, if their
/// ratio is less than or equal to $a$, or if they are too large and manipulating them leads to
/// arithmetic overflow.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::maps::random::random_b_tree_maps_unique_value_count_range;
/// use malachite_base::num::random::{random_primitive_ints, random_unsigned_inclusive_range};
/// use malachite_base::random::EXAMPLE_SEED;
/// use maplit::btreemap;
///
/// let xs = random_b_tree_maps_unique_value_count_range(
///     EXAMPLE_SEED,
///     0,
///     2,
///     &random_primitive_ints::<u8>,
///     &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 2),
///     2,
///     1,
/// );
/// let values = xs.take(10).collect_vec();
/// assert_eq!(
///     values,
///     &[
///         btreemap! {},
///         btreemap! {},
///         btreemap! {51 => 0, 79 => 0, 80 => 0},
///         btreemap! {},
///         btreemap! {10 => 1},
///         btreemap! {59 => 2},
///         btreemap! {160 => 0, 246 => 0, 253 => 0},
///         btreemap! {245 => 0},
///         btreemap! {53 => 0, 85 => 0, 139 => 0, 214 => 0, 219 => 0},
///         btreemap! {233 => 2}
///     ]
/// );
/// ```
#[inline]
pub fn random_b_tree_maps_unique_value_count_range<I: Iterator, J: Iterator>(
    seed: Seed,
    a: u64,
    b: u64,
    keys_gen: &dyn Fn(Seed) -> I,
    values_gen: &dyn Fn(Seed) -> J,
    mean_size_numerator: u64,
    mean_size_denominator: u64,
) -> RandomBTreeMapsWithUniqueValueCount<I::Item, J::Item, GeometricRandomNaturalValues<u64>, I, J>
where
    I::Item: Ord,
    J::Item: Clone + Eq + Hash,
{
    assert!(a < b, "a must be less than b. a: {a}, b: {b}");
    random_b_tree_maps_unique_value_count_inclusive_range(
        seed,
        a,
        b - 1,
        keys_gen,
        values_gen,
        mean_size_numerator,
        mean_size_denominator,
    )
}

/// Generates random [`BTreeMap`]s whose number of distinct values is in the closed interval $[a,
/// b]$, with keys from one iterator and values from another.
///
/// The key iterator must generate at least as many distinct elements as any size that is requested,
/// and the value iterator at least as many distinct elements as any number of distinct values that
/// is requested; otherwise, this iterator will hang.
///
/// $$
/// P(m) = P(n)P(d)d!(n-d)!\frac{\prod_{j=0}^{d-1}c_j}{d^{n-d}}
///     \prod_{i=0}^{n-1}P(k_i)\prod_{j=0}^{d-1}P(w_j).
/// $$
///
/// Here the map $m$ has $n$ entries $(k_i, v_i)$ and $d$ distinct values $w_0, \ldots, w_{d-1}$,
/// the $j$th of which is the value of $c_j$ of the entries; $P(n)$ is the probability of drawing
/// the size $n$, and $P(d)$ that of drawing the distinct-value count $d$, which is uniform over the
/// counts that a size-$n$ map is allowed to have. The formula assumes that the map is valid,
/// \emph{i.e.} its keys are distinct and it has exactly $d$ distinct values. Maps with the same $n$
/// and $d$ are not equally likely: the $\prod_j c_j$ factor favors those whose values are spread
/// evenly over the keys.
///
/// `keys_gen` and `values_gen` must produce infinite iterators.
///
/// If $b$ is 0, the output consists of the empty map, repeated.
///
/// # Expected complexity per iteration
/// $T(i) = O(m (T^\prime(i) + T^{\prime\prime}(i)))$
///
/// $M(i) = O(m (M^\prime(i) + M^{\prime\prime}(i)))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of the iterators produced by `keys_gen`,
/// $T^{\prime\prime}$ and $M^{\prime\prime}$ are those of the iterators produced by `values_gen`,
/// and $m$ is the mean size.
///
/// # Panics
/// Panics if $a > b$, if `mean_size_numerator` or `mean_size_denominator` are zero, if their ratio
/// is less than or equal to $a$, or if they are too large and manipulating them leads to arithmetic
/// overflow.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::maps::random::random_b_tree_maps_unique_value_count_inclusive_range;
/// use malachite_base::num::random::{random_primitive_ints, random_unsigned_inclusive_range};
/// use malachite_base::random::EXAMPLE_SEED;
/// use maplit::btreemap;
///
/// let xs = random_b_tree_maps_unique_value_count_inclusive_range(
///     EXAMPLE_SEED,
///     0,
///     1,
///     &random_primitive_ints::<u8>,
///     &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 2),
///     2,
///     1,
/// );
/// let values = xs.take(10).collect_vec();
/// assert_eq!(
///     values,
///     &[
///         btreemap! {},
///         btreemap! {},
///         btreemap! {51 => 0, 79 => 0, 80 => 0},
///         btreemap! {},
///         btreemap! {10 => 1},
///         btreemap! {59 => 2},
///         btreemap! {160 => 0, 246 => 0, 253 => 0},
///         btreemap! {245 => 0},
///         btreemap! {53 => 0, 85 => 0, 139 => 0, 214 => 0, 219 => 0},
///         btreemap! {233 => 2}
///     ]
/// );
/// ```
pub fn random_b_tree_maps_unique_value_count_inclusive_range<I: Iterator, J: Iterator>(
    seed: Seed,
    a: u64,
    b: u64,
    keys_gen: &dyn Fn(Seed) -> I,
    values_gen: &dyn Fn(Seed) -> J,
    mean_size_numerator: u64,
    mean_size_denominator: u64,
) -> RandomBTreeMapsWithUniqueValueCount<I::Item, J::Item, GeometricRandomNaturalValues<u64>, I, J>
where
    I::Item: Ord,
    J::Item: Clone + Eq + Hash,
{
    let (size_a, size_b) = compatible_size_range(0, u64::MAX, a, b);
    random_b_tree_maps_with_unique_value_count_from_size_iterator(
        seed,
        &|seed_2| {
            geometric_random_unsigned_inclusive_range(
                seed_2,
                size_a,
                size_b,
                mean_size_numerator,
                mean_size_denominator,
            )
        },
        keys_gen,
        values_gen,
        a,
        b,
    )
}

/// Generates random [`BTreeMap`]s whose size is in the closed interval $[\text{size\_a},
/// \text{size\_b}]$ and whose number of distinct values is in the closed interval
/// $[\text{unique\_value\_count\_a}, \text{unique\_value\_count\_b}]$, with keys from one iterator
/// and values from another.
///
/// A map with $k$ entries has between 1 and $k$ distinct values, or 0 if $k$ is 0, so the sizes are
/// drawn uniformly not from all of $[\text{size\_a}, \text{size\_b}]$ but from those of its members
/// that can have an allowed number of distinct values.
///
/// The key iterator must generate at least as many distinct elements as any size that is requested,
/// and the value iterator at least as many distinct elements as any number of distinct values that
/// is requested; otherwise, this iterator will hang.
///
/// $$
/// P(m) = P(n)P(d)d!(n-d)!\frac{\prod_{j=0}^{d-1}c_j}{d^{n-d}}
///     \prod_{i=0}^{n-1}P(k_i)\prod_{j=0}^{d-1}P(w_j).
/// $$
///
/// Here the map $m$ has $n$ entries $(k_i, v_i)$ and $d$ distinct values $w_0, \ldots, w_{d-1}$,
/// the $j$th of which is the value of $c_j$ of the entries; $P(n)$ is the probability of drawing
/// the size $n$, and $P(d)$ that of drawing the distinct-value count $d$, which is uniform over the
/// counts that a size-$n$ map is allowed to have. The formula assumes that the map is valid,
/// \emph{i.e.} its keys are distinct and it has exactly $d$ distinct values. Maps with the same $n$
/// and $d$ are not equally likely: the $\prod_j c_j$ factor favors those whose values are spread
/// evenly over the keys.
///
/// `keys_gen` and `values_gen` must produce infinite iterators.
///
/// # Expected complexity per iteration
/// $T(i) = O(m (T^\prime(i) + T^{\prime\prime}(i)))$
///
/// $M(i) = O(m (M^\prime(i) + M^{\prime\prime}(i)))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of the iterators produced by `keys_gen`,
/// $T^{\prime\prime}$ and $M^{\prime\prime}$ are those of the iterators produced by `values_gen`,
/// and $m$ is the mean size.
///
/// # Panics
/// Panics if $\text{size\_a} > \text{size\_b}$, if $\text{unique\_value\_count\_a} >
/// \text{unique\_value\_count\_b}$, or if no size in $[\text{size\_a}, \text{size\_b}]$ can have a
/// number of distinct values in $[\text{unique\_value\_count\_a}, \text{unique\_value\_count\_b}]$.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::chars::random::random_char_inclusive_range;
/// use malachite_base::maps::random::*;
/// use malachite_base::num::random::random_unsigned_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
/// use maplit::btreemap;
///
/// let xs = random_b_tree_maps_size_and_unique_value_count_inclusive_range(
///     EXAMPLE_SEED,
///     2,
///     3,
///     1,
///     2,
///     &|seed| random_char_inclusive_range(seed, 'a', 'c'),
///     &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 2),
/// );
/// let values = xs.take(20).collect_vec();
/// assert_eq!(
///     values,
///     &[
///         btreemap! {'a' => 0, 'b' => 1},
///         btreemap! {'a' => 2, 'b' => 2},
///         btreemap! {'a' => 0, 'b' => 0, 'c' => 0},
///         btreemap! {'a' => 2, 'b' => 2, 'c' => 0},
///         btreemap! {'a' => 0, 'b' => 0, 'c' => 0},
///         btreemap! {'a' => 0, 'b' => 0, 'c' => 0},
///         btreemap! {'a' => 0, 'b' => 0},
///         btreemap! {'b' => 0, 'c' => 0},
///         btreemap! {'a' => 1, 'b' => 1, 'c' => 2},
///         btreemap! {'a' => 1, 'b' => 2, 'c' => 1},
///         btreemap! {'b' => 2, 'c' => 1},
///         btreemap! {'a' => 1, 'b' => 0, 'c' => 1},
///         btreemap! {'a' => 0, 'b' => 0, 'c' => 1},
///         btreemap! {'a' => 2, 'c' => 0},
///         btreemap! {'a' => 0, 'b' => 0, 'c' => 0},
///         btreemap! {'b' => 0, 'c' => 2},
///         btreemap! {'a' => 1, 'b' => 1, 'c' => 1},
///         btreemap! {'a' => 1, 'b' => 1, 'c' => 1},
///         btreemap! {'b' => 0, 'c' => 2},
///         btreemap! {'a' => 1, 'c' => 1}
///     ]
/// );
/// ```
pub fn random_b_tree_maps_size_and_unique_value_count_inclusive_range<I: Iterator, J: Iterator>(
    seed: Seed,
    size_a: u64,
    size_b: u64,
    unique_value_count_a: u64,
    unique_value_count_b: u64,
    keys_gen: &dyn Fn(Seed) -> I,
    values_gen: &dyn Fn(Seed) -> J,
) -> RandomBTreeMapsWithUniqueValueCount<I::Item, J::Item, RandomUnsignedInclusiveRange<u64>, I, J>
where
    I::Item: Ord,
    J::Item: Clone + Eq + Hash,
{
    let (lo, hi) =
        compatible_size_range(size_a, size_b, unique_value_count_a, unique_value_count_b);
    random_b_tree_maps_with_unique_value_count_from_size_iterator(
        seed,
        &|seed_2| random_unsigned_inclusive_range(seed_2, lo, hi),
        keys_gen,
        values_gen,
        unique_value_count_a,
        unique_value_count_b,
    )
}

/// Generates random [`HashMap`]s of a fixed size.
///
/// This `struct` is created by [`random_hash_maps_fixed_size`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct RandomHashMapsFixedSize<I: Iterator, J: Iterator>
where
    I::Item: Eq + Hash,
{
    size: usize,
    keys: I,
    values: J,
}

impl<I: Iterator, J: Iterator> Iterator for RandomHashMapsFixedSize<I, J>
where
    I::Item: Eq + Hash,
{
    type Item = HashMap<I::Item, J::Item>;

    fn next(&mut self) -> Option<HashMap<I::Item, J::Item>> {
        let mut map = HashMap::new();
        while map.len() < self.size {
            // A repeated key costs a key draw but no value draw, so exactly one value is consumed
            // per entry, paired with its key in the order the keys are drawn. Pairing them by the
            // map's own iteration order instead would make the output depend on the hasher, and so
            // differ between runs.
            let key = self.keys.next().unwrap();
            map.entry(key)
                .or_insert_with(|| self.values.next().unwrap());
        }
        Some(map)
    }
}

/// Generates random [`HashMap`]s of a given size, with keys from one iterator and values from
/// another.
///
/// The key iterator must generate at least `size` distinct elements; otherwise, this iterator will
/// hang. The value iterator has no such requirement, since values may repeat.
///
/// $$
/// P(\\{(k_i, v_i)\\}_ {i=0}^{n-1}) = n!\prod_ {i=0}^{n-1}P(k_i)P(v_i).
/// $$
///
/// The above formula assumes that the map is valid, \emph{i.e.} its keys are distinct. The $n!$
/// counts the orders in which the keys can be drawn; the values are drawn in the same order, so
/// each key order pairs with exactly one value order that produces the map.
///
/// If `size` is 0, the output consists of the empty map, repeated.
///
/// `keys` and `values` must be infinite.
///
/// # Expected complexity per iteration
/// $T(i) = O(n (T^\prime(i) + T^{\prime\prime}(i)))$
///
/// $M(i) = O(n (M^\prime(i) + M^{\prime\prime}(i)))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of `keys`, $T^{\prime\prime}$ and
/// $M^{\prime\prime}$ are those of `values`, and $n$ is `size`.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::chars::random::random_char_inclusive_range;
/// use malachite_base::maps::random::random_hash_maps_fixed_size;
/// use malachite_base::num::random::random_unsigned_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
/// use maplit::hashmap;
///
/// let xs = random_hash_maps_fixed_size(
///     2,
///     random_char_inclusive_range(EXAMPLE_SEED.fork("keys"), 'a', 'c'),
///     random_unsigned_inclusive_range::<u8>(EXAMPLE_SEED.fork("values"), 0, 2),
/// );
/// let values = xs.take(10).collect_vec();
/// assert_eq!(
///     values,
///     &[
///         hashmap! {'a' => 0, 'b' => 1},
///         hashmap! {'a' => 2, 'b' => 0},
///         hashmap! {'a' => 0, 'b' => 0},
///         hashmap! {'a' => 2, 'c' => 0},
///         hashmap! {'a' => 0, 'c' => 0},
///         hashmap! {'a' => 0, 'c' => 1},
///         hashmap! {'a' => 2, 'b' => 1},
///         hashmap! {'b' => 1, 'c' => 2},
///         hashmap! {'a' => 2, 'c' => 0},
///         hashmap! {'b' => 0, 'c' => 1}
///     ]
/// );
/// ```
#[inline]
pub fn random_hash_maps_fixed_size<I: Iterator, J: Iterator>(
    size: u64,
    keys: I,
    values: J,
) -> RandomHashMapsFixedSize<I, J>
where
    I::Item: Eq + Hash,
{
    RandomHashMapsFixedSize {
        size: usize::exact_from(size),
        keys,
        values,
    }
}

/// Generates random [`HashMap`]s with sizes from an iterator.
#[derive(Clone, Debug)]
pub struct RandomHashMaps<
    K: Eq + Hash,
    V,
    I: Iterator<Item = u64>,
    J: Iterator<Item = K>,
    L: Iterator<Item = V>,
> {
    sizes: I,
    keys: J,
    values: L,
}

impl<K: Eq + Hash, V, I: Iterator<Item = u64>, J: Iterator<Item = K>, L: Iterator<Item = V>>
    Iterator for RandomHashMaps<K, V, I, J, L>
{
    type Item = HashMap<K, V>;

    fn next(&mut self) -> Option<HashMap<K, V>> {
        let size = usize::exact_from(self.sizes.next().unwrap());
        let mut map = HashMap::new();
        while map.len() < size {
            // A repeated key costs a key draw but no value draw, so exactly one value is consumed
            // per entry, paired with its key in the order the keys are drawn. Pairing them by the
            // map's own iteration order instead would make the output depend on the hasher, and so
            // differ between runs.
            let key = self.keys.next().unwrap();
            map.entry(key)
                .or_insert_with(|| self.values.next().unwrap());
        }
        Some(map)
    }
}

/// Generates random [`HashMap`]s with keys from one iterator, values from another, and sizes from a
/// third.
///
/// The key iterator must generate at least as many distinct elements as any size that is requested;
/// otherwise, this iterator will hang. The value iterator has no such requirement, since values may
/// repeat.
///
/// $$
/// P(\\{(k_i, v_i)\\}_ {i=0}^{n-1}) = n!P(n)\prod_ {i=0}^{n-1}P(k_i)P(v_i).
/// $$
///
/// The above formula assumes that the map is valid, \emph{i.e.} its keys are distinct. The $n!$
/// counts the orders in which the keys can be drawn; the values are drawn in the same order, so
/// each key order pairs with exactly one value order that produces the map.
///
/// `keys_gen` and `values_gen` must produce infinite iterators.
///
/// # Expected complexity per iteration
/// $T(i) = O(m (T^\prime(i) + T^{\prime\prime}(i)))$
///
/// $M(i) = O(m (M^\prime(i) + M^{\prime\prime}(i)))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of the iterators produced by `keys_gen`,
/// $T^{\prime\prime}$ and $M^{\prime\prime}$ are those of the iterators produced by `values_gen`,
/// and $m$ is the mean size.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::maps::random::random_hash_maps_from_size_iterator;
/// use malachite_base::num::random::geometric::geometric_random_unsigneds;
/// use malachite_base::num::random::random_primitive_ints;
/// use malachite_base::random::EXAMPLE_SEED;
/// use maplit::hashmap;
///
/// let xs = random_hash_maps_from_size_iterator(
///     EXAMPLE_SEED,
///     &|seed| geometric_random_unsigneds::<u64>(seed, 2, 1),
///     &random_primitive_ints::<u8>,
///     &random_primitive_ints::<u8>,
/// );
/// let values = xs.take(10).collect_vec();
/// assert_eq!(
///     values,
///     &[
///         hashmap! {},
///         hashmap! {},
///         hashmap! {51 => 64, 79 => 36, 80 => 32},
///         hashmap! {},
///         hashmap! {10 => 158},
///         hashmap! {59 => 39},
///         hashmap! {160 => 91, 246 => 8, 253 => 71},
///         hashmap! {245 => 18},
///         hashmap! {53 => 243, 85 => 220, 139 => 67, 214 => 153, 219 => 134},
///         hashmap! {233 => 107}
///     ]
/// );
/// ```
pub fn random_hash_maps_from_size_iterator<
    K: Eq + Hash,
    V,
    I: Iterator<Item = u64>,
    J: Iterator<Item = K>,
    L: Iterator<Item = V>,
>(
    seed: Seed,
    sizes_gen: &dyn Fn(Seed) -> I,
    keys_gen: &dyn Fn(Seed) -> J,
    values_gen: &dyn Fn(Seed) -> L,
) -> RandomHashMaps<K, V, I, J, L> {
    RandomHashMaps {
        sizes: sizes_gen(seed.fork("sizes")),
        keys: keys_gen(seed.fork("keys")),
        values: values_gen(seed.fork("values")),
    }
}

/// Generates random [`HashMap`]s with keys from one iterator and values from another.
///
/// The sizes of the maps are sampled from a geometric distribution with a specified mean $m$, equal
/// to `mean_size_numerator / mean_size_denominator`. $m$ must be greater than 0.
///
/// The key iterator must generate at least as many distinct elements as any size that is requested;
/// otherwise, this iterator will hang. The value iterator has no such requirement, since values may
/// repeat.
///
/// $$
/// P(\\{(k_i, v_i)\\}_ {i=0}^{n-1}) = n!P(n)\prod_ {i=0}^{n-1}P(k_i)P(v_i).
/// $$
///
/// The above formula assumes that the map is valid, \emph{i.e.} its keys are distinct. The $n!$
/// counts the orders in which the keys can be drawn; the values are drawn in the same order, so
/// each key order pairs with exactly one value order that produces the map.
///
/// `keys_gen` and `values_gen` must produce infinite iterators.
///
/// # Expected complexity per iteration
/// $T(i) = O(m (T^\prime(i) + T^{\prime\prime}(i)))$
///
/// $M(i) = O(m (M^\prime(i) + M^{\prime\prime}(i)))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of the iterators produced by `keys_gen`,
/// $T^{\prime\prime}$ and $M^{\prime\prime}$ are those of the iterators produced by `values_gen`,
/// and $m$ is the mean size.
///
/// # Panics
/// Panics if `mean_size_numerator` or `mean_size_denominator` are zero, or, if after being reduced
/// to lowest terms, their sum is greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::bools::random::random_bools;
/// use malachite_base::maps::random::random_hash_maps;
/// use malachite_base::num::random::random_unsigned_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
/// use maplit::hashmap;
///
/// let xs = random_hash_maps(
///     EXAMPLE_SEED,
///     &|seed| random_unsigned_inclusive_range::<u32>(seed, 1, 100),
///     &random_bools,
///     1,
///     1,
/// );
/// let values = xs.take(20).collect_vec();
/// assert_eq!(
///     values,
///     &[
///         hashmap! {},
///         hashmap! {},
///         hashmap! {33 => false, 78 => true, 80 => false, 82 => false},
///         hashmap! {},
///         hashmap! {40 => true, 49 => false},
///         hashmap! {33 => false, 64 => false},
///         hashmap! {88 => false},
///         hashmap! {43 => false, 100 => false},
///         hashmap! {},
///         hashmap! {},
///         hashmap! {},
///         hashmap! {},
///         hashmap! {70 => false},
///         hashmap! {},
///         hashmap! {6 => false, 74 => true},
///         hashmap! {94 => false},
///         hashmap! {},
///         hashmap! {79 => false},
///         hashmap! {},
///         hashmap! {18 => false, 34 => false}
///     ]
/// );
/// ```
#[inline]
pub fn random_hash_maps<I: Iterator, J: Iterator>(
    seed: Seed,
    keys_gen: &dyn Fn(Seed) -> I,
    values_gen: &dyn Fn(Seed) -> J,
    mean_size_numerator: u64,
    mean_size_denominator: u64,
) -> RandomHashMaps<I::Item, J::Item, GeometricRandomNaturalValues<u64>, I, J>
where
    I::Item: Eq + Hash,
{
    random_hash_maps_from_size_iterator(
        seed,
        &|seed_2| geometric_random_unsigneds(seed_2, mean_size_numerator, mean_size_denominator),
        keys_gen,
        values_gen,
    )
}

/// Generates random [`HashMap`]s with a minimum size, with keys from one iterator and values from
/// another.
///
/// The sizes of the maps are sampled from a geometric distribution with a specified mean $m$, equal
/// to `mean_size_numerator / mean_size_denominator`. $m$ must be greater than `min_size`.
///
/// The key iterator must generate at least as many distinct elements as any size that is requested;
/// otherwise, this iterator will hang. The value iterator has no such requirement, since values may
/// repeat.
///
/// $$
/// P(\\{(k_i, v_i)\\}_ {i=0}^{n-1}) = n!P(n)\prod_ {i=0}^{n-1}P(k_i)P(v_i).
/// $$
///
/// The above formula assumes that the map is valid, \emph{i.e.} its keys are distinct. The $n!$
/// counts the orders in which the keys can be drawn; the values are drawn in the same order, so
/// each key order pairs with exactly one value order that produces the map.
///
/// `keys_gen` and `values_gen` must produce infinite iterators.
///
/// # Expected complexity per iteration
/// $T(i) = O(m (T^\prime(i) + T^{\prime\prime}(i)))$
///
/// $M(i) = O(m (M^\prime(i) + M^{\prime\prime}(i)))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of the iterators produced by `keys_gen`,
/// $T^{\prime\prime}$ and $M^{\prime\prime}$ are those of the iterators produced by `values_gen`,
/// and $m$ is the mean size.
///
/// # Panics
/// Panics if `mean_size_numerator` or `mean_size_denominator` are zero, if their ratio is less than
/// or equal to `min_size`, or if they are too large and manipulating them leads to arithmetic
/// overflow.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::maps::random::random_hash_maps_min_size;
/// use malachite_base::num::random::{random_primitive_ints, random_unsigned_inclusive_range};
/// use malachite_base::random::EXAMPLE_SEED;
/// use maplit::hashmap;
///
/// let xs = random_hash_maps_min_size(
///     EXAMPLE_SEED,
///     1,
///     &random_primitive_ints::<u8>,
///     &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 2),
///     4,
///     1,
/// );
/// let values = xs.take(10).collect_vec();
/// assert_eq!(
///     values,
///     &[
///         hashmap! {79 => 0},
///         hashmap! {10 => 0, 51 => 2, 80 => 1},
///         hashmap! {59 => 0, 85 => 0, 160 => 0, 245 => 0, 246 => 2, 253 => 0},
///         hashmap! {53 => 0},
///         hashmap! {214 => 1, 219 => 2},
///         hashmap! {120 => 1, 139 => 1, 233 => 2},
///         hashmap! {33 => 2, 157 => 1, 158 => 0, 161 => 0, 202 => 0, 236 => 1},
///         hashmap! {19 => 0, 72 => 2, 153 => 1, 155 => 0, 194 => 2},
///         hashmap! {25 => 1, 68 => 0, 74 => 0, 80 => 2, 97 => 1, 119 => 0, 236 => 1, 252 => 0},
///         hashmap! {33 => 1, 241 => 2}
///     ]
/// );
/// ```
#[inline]
pub fn random_hash_maps_min_size<I: Iterator, J: Iterator>(
    seed: Seed,
    min_size: u64,
    keys_gen: &dyn Fn(Seed) -> I,
    values_gen: &dyn Fn(Seed) -> J,
    mean_size_numerator: u64,
    mean_size_denominator: u64,
) -> RandomHashMaps<I::Item, J::Item, GeometricRandomNaturalValues<u64>, I, J>
where
    I::Item: Eq + Hash,
{
    random_hash_maps_from_size_iterator(
        seed,
        &|seed_2| {
            geometric_random_unsigned_inclusive_range(
                seed_2,
                min_size,
                u64::MAX,
                mean_size_numerator,
                mean_size_denominator,
            )
        },
        keys_gen,
        values_gen,
    )
}

/// Generates random [`HashMap`]s with sizes in the half-open interval $[a, b)$, with keys from one
/// iterator and values from another.
///
/// The key iterator must generate at least as many distinct elements as any size that is requested;
/// otherwise, this iterator will hang. The value iterator has no such requirement, since values may
/// repeat.
///
/// $$
/// P(\\{(k_i, v_i)\\}_ {i=0}^{n-1}) = n!P(n)\prod_ {i=0}^{n-1}P(k_i)P(v_i).
/// $$
///
/// The above formula assumes that the map is valid, \emph{i.e.} its keys are distinct. The $n!$
/// counts the orders in which the keys can be drawn; the values are drawn in the same order, so
/// each key order pairs with exactly one value order that produces the map.
///
/// `keys_gen` and `values_gen` must produce infinite iterators.
///
/// # Expected complexity per iteration
/// $T(i) = O(m (T^\prime(i) + T^{\prime\prime}(i)))$
///
/// $M(i) = O(m (M^\prime(i) + M^{\prime\prime}(i)))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of the iterators produced by `keys_gen`,
/// $T^{\prime\prime}$ and $M^{\prime\prime}$ are those of the iterators produced by `values_gen`,
/// and $m$ is the mean size.
///
/// # Panics
/// Panics if $a \geq b$.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::chars::random::random_char_inclusive_range;
/// use malachite_base::maps::random::random_hash_maps_size_range;
/// use malachite_base::num::random::random_unsigned_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
/// use maplit::hashmap;
///
/// let xs = random_hash_maps_size_range(
///     EXAMPLE_SEED,
///     1,
///     3,
///     &|seed| random_char_inclusive_range(seed, 'a', 'c'),
///     &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 1),
/// );
/// let values = xs.take(20).collect_vec();
/// assert_eq!(
///     values,
///     &[
///         hashmap! {'a' => 0},
///         hashmap! {'b' => 0},
///         hashmap! {'a' => 1, 'b' => 0},
///         hashmap! {'a' => 1, 'b' => 0},
///         hashmap! {'a' => 0, 'c' => 0},
///         hashmap! {'a' => 0, 'c' => 0},
///         hashmap! {'a' => 0},
///         hashmap! {'c' => 0},
///         hashmap! {'a' => 0, 'b' => 1},
///         hashmap! {'b' => 0, 'c' => 0},
///         hashmap! {'a' => 0},
///         hashmap! {'a' => 0, 'c' => 0},
///         hashmap! {'b' => 0, 'c' => 0},
///         hashmap! {'b' => 0},
///         hashmap! {'a' => 0, 'b' => 1},
///         hashmap! {'c' => 0},
///         hashmap! {'b' => 1, 'c' => 1},
///         hashmap! {'b' => 1, 'c' => 1},
///         hashmap! {'a' => 0},
///         hashmap! {'c' => 0}
///     ]
/// );
/// ```
#[inline]
pub fn random_hash_maps_size_range<I: Iterator, J: Iterator>(
    seed: Seed,
    a: u64,
    b: u64,
    keys_gen: &dyn Fn(Seed) -> I,
    values_gen: &dyn Fn(Seed) -> J,
) -> RandomHashMaps<I::Item, J::Item, RandomUnsignedRange<u64>, I, J>
where
    I::Item: Eq + Hash,
{
    random_hash_maps_from_size_iterator(
        seed,
        &|seed_2| random_unsigned_range(seed_2, a, b),
        keys_gen,
        values_gen,
    )
}

/// Generates random [`HashMap`]s with sizes in the closed interval $[a, b]$, with keys from one
/// iterator and values from another.
///
/// The key iterator must generate at least as many distinct elements as any size that is requested;
/// otherwise, this iterator will hang. The value iterator has no such requirement, since values may
/// repeat.
///
/// $$
/// P(\\{(k_i, v_i)\\}_ {i=0}^{n-1}) = n!P(n)\prod_ {i=0}^{n-1}P(k_i)P(v_i).
/// $$
///
/// The above formula assumes that the map is valid, \emph{i.e.} its keys are distinct. The $n!$
/// counts the orders in which the keys can be drawn; the values are drawn in the same order, so
/// each key order pairs with exactly one value order that produces the map.
///
/// `keys_gen` and `values_gen` must produce infinite iterators.
///
/// # Expected complexity per iteration
/// $T(i) = O(m (T^\prime(i) + T^{\prime\prime}(i)))$
///
/// $M(i) = O(m (M^\prime(i) + M^{\prime\prime}(i)))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of the iterators produced by `keys_gen`,
/// $T^{\prime\prime}$ and $M^{\prime\prime}$ are those of the iterators produced by `values_gen`,
/// and $m$ is the mean size.
///
/// # Panics
/// Panics if $a > b$.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::chars::random::random_char_inclusive_range;
/// use malachite_base::maps::random::random_hash_maps_size_inclusive_range;
/// use malachite_base::num::random::random_unsigned_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
/// use maplit::hashmap;
///
/// let xs = random_hash_maps_size_inclusive_range(
///     EXAMPLE_SEED,
///     1,
///     2,
///     &|seed| random_char_inclusive_range(seed, 'a', 'c'),
///     &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 1),
/// );
/// let values = xs.take(20).collect_vec();
/// assert_eq!(
///     values,
///     &[
///         hashmap! {'a' => 0},
///         hashmap! {'b' => 0},
///         hashmap! {'a' => 1, 'b' => 0},
///         hashmap! {'a' => 1, 'b' => 0},
///         hashmap! {'a' => 0, 'c' => 0},
///         hashmap! {'a' => 0, 'c' => 0},
///         hashmap! {'a' => 0},
///         hashmap! {'c' => 0},
///         hashmap! {'a' => 0, 'b' => 1},
///         hashmap! {'b' => 0, 'c' => 0},
///         hashmap! {'a' => 0},
///         hashmap! {'a' => 0, 'c' => 0},
///         hashmap! {'b' => 0, 'c' => 0},
///         hashmap! {'b' => 0},
///         hashmap! {'a' => 0, 'b' => 1},
///         hashmap! {'c' => 0},
///         hashmap! {'b' => 1, 'c' => 1},
///         hashmap! {'b' => 1, 'c' => 1},
///         hashmap! {'a' => 0},
///         hashmap! {'c' => 0}
///     ]
/// );
/// ```
#[inline]
pub fn random_hash_maps_size_inclusive_range<I: Iterator, J: Iterator>(
    seed: Seed,
    a: u64,
    b: u64,
    keys_gen: &dyn Fn(Seed) -> I,
    values_gen: &dyn Fn(Seed) -> J,
) -> RandomHashMaps<I::Item, J::Item, RandomUnsignedInclusiveRange<u64>, I, J>
where
    I::Item: Eq + Hash,
{
    random_hash_maps_from_size_iterator(
        seed,
        &|seed_2| random_unsigned_inclusive_range(seed_2, a, b),
        keys_gen,
        values_gen,
    )
}
/// Generates random [`HashMap`]s with a restricted number of distinct values.
#[derive(Clone, Debug)]
pub struct RandomHashMapsWithUniqueValueCount<
    K: Eq + Hash,
    V: Clone + Eq + Hash,
    I: Iterator<Item = u64>,
    J: Iterator<Item = K>,
    L: Iterator<Item = V>,
> {
    sizes: I,
    keys: J,
    values: L,
    range_generator: VariableRangeGenerator,
    a: u64,
    b: u64,
}

impl<
    K: Eq + Hash,
    V: Clone + Eq + Hash,
    I: Iterator<Item = u64>,
    J: Iterator<Item = K>,
    L: Iterator<Item = V>,
> Iterator for RandomHashMapsWithUniqueValueCount<K, V, I, J, L>
{
    type Item = HashMap<K, V>;

    fn next(&mut self) -> Option<HashMap<K, V>> {
        let size = self.sizes.next().unwrap();
        // A map with `size` entries has between 1 and `size` distinct values, or 0 if `size` is 0.
        // The size iterator is built so that this range always meets [`self.a`, `self.b`].
        let count = usize::exact_from(
            self.range_generator
                .next_in_inclusive_range::<u64>(max(self.a, min(size, 1)), min(self.b, size)),
        );
        let size = usize::exact_from(size);
        let mut vs = Vec::with_capacity(count);
        let mut seen = HashSet::with_capacity(count);
        while vs.len() < count {
            let value = self.values.next().unwrap();
            if seen.insert(value.clone()) {
                vs.push(value);
            }
        }
        // The first `count` keys drawn take the `count` distinct values, one each, so that every
        // one of them is used; the rest take a uniformly random one of them. The keys are drawn in
        // random order, so this does not favor any particular position in the map.
        let mut map = HashMap::new();
        let mut assigned = 0;
        while map.len() < size {
            let key = self.keys.next().unwrap();
            let range_generator = &mut self.range_generator;
            map.entry(key).or_insert_with(|| {
                let value = if assigned < count {
                    vs[assigned].clone()
                } else {
                    vs[range_generator.next_less_than::<usize>(count)].clone()
                };
                assigned += 1;
                value
            });
        }
        Some(map)
    }
}

fn random_hash_maps_with_unique_value_count_from_size_iterator<
    K: Eq + Hash,
    V: Clone + Eq + Hash,
    I: Iterator<Item = u64>,
    J: Iterator<Item = K>,
    L: Iterator<Item = V>,
>(
    seed: Seed,
    sizes_gen: &dyn Fn(Seed) -> I,
    keys_gen: &dyn Fn(Seed) -> J,
    values_gen: &dyn Fn(Seed) -> L,
    a: u64,
    b: u64,
) -> RandomHashMapsWithUniqueValueCount<K, V, I, J, L> {
    RandomHashMapsWithUniqueValueCount {
        sizes: sizes_gen(seed.fork("sizes")),
        keys: keys_gen(seed.fork("keys")),
        values: values_gen(seed.fork("values")),
        range_generator: VariableRangeGenerator::new(seed.fork("unique_value_counts")),
        a,
        b,
    }
}

/// Generates random [`HashMap`]s with a given number of distinct values, with keys from one
/// iterator and values from another.
///
/// The key iterator must generate at least as many distinct elements as any size that is requested,
/// and the value iterator at least as many distinct elements as any number of distinct values that
/// is requested; otherwise, this iterator will hang.
///
/// $$
/// P(m) = P(n)P(d)d!(n-d)!\frac{\prod_{j=0}^{d-1}c_j}{d^{n-d}}
///     \prod_{i=0}^{n-1}P(k_i)\prod_{j=0}^{d-1}P(w_j).
/// $$
///
/// Here the map $m$ has $n$ entries $(k_i, v_i)$ and $d$ distinct values $w_0, \ldots, w_{d-1}$,
/// the $j$th of which is the value of $c_j$ of the entries; $P(n)$ is the probability of drawing
/// the size $n$, and $P(d)$ that of drawing the distinct-value count $d$, which is uniform over the
/// counts that a size-$n$ map is allowed to have. The formula assumes that the map is valid,
/// \emph{i.e.} its keys are distinct and it has exactly $d$ distinct values. Maps with the same $n$
/// and $d$ are not equally likely: the $\prod_j c_j$ factor favors those whose values are spread
/// evenly over the keys.
///
/// `keys_gen` and `values_gen` must produce infinite iterators.
///
/// If `unique_value_count` is 0, the output consists of the empty map, repeated.
///
/// # Expected complexity per iteration
/// $T(i) = O(m (T^\prime(i) + T^{\prime\prime}(i)))$
///
/// $M(i) = O(m (M^\prime(i) + M^{\prime\prime}(i)))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of the iterators produced by `keys_gen`,
/// $T^{\prime\prime}$ and $M^{\prime\prime}$ are those of the iterators produced by `values_gen`,
/// and $m$ is the mean size.
///
/// # Panics
/// Panics if `mean_size_numerator` or `mean_size_denominator` are zero, if their ratio is less than
/// or equal to `unique_value_count`, or if they are too large and manipulating them leads to
/// arithmetic overflow.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::maps::random::random_hash_maps_fixed_unique_value_count;
/// use malachite_base::num::random::{random_primitive_ints, random_unsigned_inclusive_range};
/// use malachite_base::random::EXAMPLE_SEED;
/// use maplit::hashmap;
///
/// let xs = random_hash_maps_fixed_unique_value_count(
///     EXAMPLE_SEED,
///     1,
///     &random_primitive_ints::<u8>,
///     &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 2),
///     2,
///     1,
/// );
/// let values = xs.take(10).collect_vec();
/// assert_eq!(
///     values,
///     &[
///         hashmap! {79 => 0},
///         hashmap! {80 => 1},
///         hashmap! {10 => 2, 51 => 2, 59 => 2, 246 => 2, 253 => 2},
///         hashmap! {160 => 0},
///         hashmap! {53 => 0, 85 => 0, 245 => 0},
///         hashmap! {139 => 0, 214 => 0, 219 => 0},
///         hashmap! {120 => 2, 233 => 2},
///         hashmap! {33 => 0, 158 => 0, 236 => 0},
///         hashmap! {202 => 0},
///         hashmap! {157 => 0}
///     ]
/// );
/// ```
#[inline]
pub fn random_hash_maps_fixed_unique_value_count<I: Iterator, J: Iterator>(
    seed: Seed,
    unique_value_count: u64,
    keys_gen: &dyn Fn(Seed) -> I,
    values_gen: &dyn Fn(Seed) -> J,
    mean_size_numerator: u64,
    mean_size_denominator: u64,
) -> RandomHashMapsWithUniqueValueCount<I::Item, J::Item, GeometricRandomNaturalValues<u64>, I, J>
where
    I::Item: Eq + Hash,
    J::Item: Clone + Eq + Hash,
{
    random_hash_maps_unique_value_count_inclusive_range(
        seed,
        unique_value_count,
        unique_value_count,
        keys_gen,
        values_gen,
        mean_size_numerator,
        mean_size_denominator,
    )
}

/// Generates random [`HashMap`]s whose number of distinct values is in the half-open interval $[a,
/// b)$, with keys from one iterator and values from another.
///
/// The key iterator must generate at least as many distinct elements as any size that is requested,
/// and the value iterator at least as many distinct elements as any number of distinct values that
/// is requested; otherwise, this iterator will hang.
///
/// $$
/// P(m) = P(n)P(d)d!(n-d)!\frac{\prod_{j=0}^{d-1}c_j}{d^{n-d}}
///     \prod_{i=0}^{n-1}P(k_i)\prod_{j=0}^{d-1}P(w_j).
/// $$
///
/// Here the map $m$ has $n$ entries $(k_i, v_i)$ and $d$ distinct values $w_0, \ldots, w_{d-1}$,
/// the $j$th of which is the value of $c_j$ of the entries; $P(n)$ is the probability of drawing
/// the size $n$, and $P(d)$ that of drawing the distinct-value count $d$, which is uniform over the
/// counts that a size-$n$ map is allowed to have. The formula assumes that the map is valid,
/// \emph{i.e.} its keys are distinct and it has exactly $d$ distinct values. Maps with the same $n$
/// and $d$ are not equally likely: the $\prod_j c_j$ factor favors those whose values are spread
/// evenly over the keys.
///
/// `keys_gen` and `values_gen` must produce infinite iterators.
///
/// # Expected complexity per iteration
/// $T(i) = O(m (T^\prime(i) + T^{\prime\prime}(i)))$
///
/// $M(i) = O(m (M^\prime(i) + M^{\prime\prime}(i)))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of the iterators produced by `keys_gen`,
/// $T^{\prime\prime}$ and $M^{\prime\prime}$ are those of the iterators produced by `values_gen`,
/// and $m$ is the mean size.
///
/// # Panics
/// Panics if $a \geq b$, if `mean_size_numerator` or `mean_size_denominator` are zero, if their
/// ratio is less than or equal to $a$, or if they are too large and manipulating them leads to
/// arithmetic overflow.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::maps::random::random_hash_maps_unique_value_count_range;
/// use malachite_base::num::random::{random_primitive_ints, random_unsigned_inclusive_range};
/// use malachite_base::random::EXAMPLE_SEED;
/// use maplit::hashmap;
///
/// let xs = random_hash_maps_unique_value_count_range(
///     EXAMPLE_SEED,
///     0,
///     2,
///     &random_primitive_ints::<u8>,
///     &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 2),
///     2,
///     1,
/// );
/// let values = xs.take(10).collect_vec();
/// assert_eq!(
///     values,
///     &[
///         hashmap! {},
///         hashmap! {},
///         hashmap! {51 => 0, 79 => 0, 80 => 0},
///         hashmap! {},
///         hashmap! {10 => 1},
///         hashmap! {59 => 2},
///         hashmap! {160 => 0, 246 => 0, 253 => 0},
///         hashmap! {245 => 0},
///         hashmap! {53 => 0, 85 => 0, 139 => 0, 214 => 0, 219 => 0},
///         hashmap! {233 => 2}
///     ]
/// );
/// ```
#[inline]
pub fn random_hash_maps_unique_value_count_range<I: Iterator, J: Iterator>(
    seed: Seed,
    a: u64,
    b: u64,
    keys_gen: &dyn Fn(Seed) -> I,
    values_gen: &dyn Fn(Seed) -> J,
    mean_size_numerator: u64,
    mean_size_denominator: u64,
) -> RandomHashMapsWithUniqueValueCount<I::Item, J::Item, GeometricRandomNaturalValues<u64>, I, J>
where
    I::Item: Eq + Hash,
    J::Item: Clone + Eq + Hash,
{
    assert!(a < b, "a must be less than b. a: {a}, b: {b}");
    random_hash_maps_unique_value_count_inclusive_range(
        seed,
        a,
        b - 1,
        keys_gen,
        values_gen,
        mean_size_numerator,
        mean_size_denominator,
    )
}

/// Generates random [`HashMap`]s whose number of distinct values is in the closed interval $[a,
/// b]$, with keys from one iterator and values from another.
///
/// The key iterator must generate at least as many distinct elements as any size that is requested,
/// and the value iterator at least as many distinct elements as any number of distinct values that
/// is requested; otherwise, this iterator will hang.
///
/// $$
/// P(m) = P(n)P(d)d!(n-d)!\frac{\prod_{j=0}^{d-1}c_j}{d^{n-d}}
///     \prod_{i=0}^{n-1}P(k_i)\prod_{j=0}^{d-1}P(w_j).
/// $$
///
/// Here the map $m$ has $n$ entries $(k_i, v_i)$ and $d$ distinct values $w_0, \ldots, w_{d-1}$,
/// the $j$th of which is the value of $c_j$ of the entries; $P(n)$ is the probability of drawing
/// the size $n$, and $P(d)$ that of drawing the distinct-value count $d$, which is uniform over the
/// counts that a size-$n$ map is allowed to have. The formula assumes that the map is valid,
/// \emph{i.e.} its keys are distinct and it has exactly $d$ distinct values. Maps with the same $n$
/// and $d$ are not equally likely: the $\prod_j c_j$ factor favors those whose values are spread
/// evenly over the keys.
///
/// `keys_gen` and `values_gen` must produce infinite iterators.
///
/// If $b$ is 0, the output consists of the empty map, repeated.
///
/// # Expected complexity per iteration
/// $T(i) = O(m (T^\prime(i) + T^{\prime\prime}(i)))$
///
/// $M(i) = O(m (M^\prime(i) + M^{\prime\prime}(i)))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of the iterators produced by `keys_gen`,
/// $T^{\prime\prime}$ and $M^{\prime\prime}$ are those of the iterators produced by `values_gen`,
/// and $m$ is the mean size.
///
/// # Panics
/// Panics if $a > b$, if `mean_size_numerator` or `mean_size_denominator` are zero, if their ratio
/// is less than or equal to $a$, or if they are too large and manipulating them leads to arithmetic
/// overflow.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::maps::random::random_hash_maps_unique_value_count_inclusive_range;
/// use malachite_base::num::random::{random_primitive_ints, random_unsigned_inclusive_range};
/// use malachite_base::random::EXAMPLE_SEED;
/// use maplit::hashmap;
///
/// let xs = random_hash_maps_unique_value_count_inclusive_range(
///     EXAMPLE_SEED,
///     0,
///     1,
///     &random_primitive_ints::<u8>,
///     &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 2),
///     2,
///     1,
/// );
/// let values = xs.take(10).collect_vec();
/// assert_eq!(
///     values,
///     &[
///         hashmap! {},
///         hashmap! {},
///         hashmap! {51 => 0, 79 => 0, 80 => 0},
///         hashmap! {},
///         hashmap! {10 => 1},
///         hashmap! {59 => 2},
///         hashmap! {160 => 0, 246 => 0, 253 => 0},
///         hashmap! {245 => 0},
///         hashmap! {53 => 0, 85 => 0, 139 => 0, 214 => 0, 219 => 0},
///         hashmap! {233 => 2}
///     ]
/// );
/// ```
pub fn random_hash_maps_unique_value_count_inclusive_range<I: Iterator, J: Iterator>(
    seed: Seed,
    a: u64,
    b: u64,
    keys_gen: &dyn Fn(Seed) -> I,
    values_gen: &dyn Fn(Seed) -> J,
    mean_size_numerator: u64,
    mean_size_denominator: u64,
) -> RandomHashMapsWithUniqueValueCount<I::Item, J::Item, GeometricRandomNaturalValues<u64>, I, J>
where
    I::Item: Eq + Hash,
    J::Item: Clone + Eq + Hash,
{
    let (size_a, size_b) = compatible_size_range(0, u64::MAX, a, b);
    random_hash_maps_with_unique_value_count_from_size_iterator(
        seed,
        &|seed_2| {
            geometric_random_unsigned_inclusive_range(
                seed_2,
                size_a,
                size_b,
                mean_size_numerator,
                mean_size_denominator,
            )
        },
        keys_gen,
        values_gen,
        a,
        b,
    )
}

/// Generates random [`HashMap`]s whose size is in the closed interval $[\text{size\_a},
/// \text{size\_b}]$ and whose number of distinct values is in the closed interval
/// $[\text{unique\_value\_count\_a}, \text{unique\_value\_count\_b}]$, with keys from one iterator
/// and values from another.
///
/// A map with $k$ entries has between 1 and $k$ distinct values, or 0 if $k$ is 0, so the sizes are
/// drawn uniformly not from all of $[\text{size\_a}, \text{size\_b}]$ but from those of its members
/// that can have an allowed number of distinct values.
///
/// The key iterator must generate at least as many distinct elements as any size that is requested,
/// and the value iterator at least as many distinct elements as any number of distinct values that
/// is requested; otherwise, this iterator will hang.
///
/// $$
/// P(m) = P(n)P(d)d!(n-d)!\frac{\prod_{j=0}^{d-1}c_j}{d^{n-d}}
///     \prod_{i=0}^{n-1}P(k_i)\prod_{j=0}^{d-1}P(w_j).
/// $$
///
/// Here the map $m$ has $n$ entries $(k_i, v_i)$ and $d$ distinct values $w_0, \ldots, w_{d-1}$,
/// the $j$th of which is the value of $c_j$ of the entries; $P(n)$ is the probability of drawing
/// the size $n$, and $P(d)$ that of drawing the distinct-value count $d$, which is uniform over the
/// counts that a size-$n$ map is allowed to have. The formula assumes that the map is valid,
/// \emph{i.e.} its keys are distinct and it has exactly $d$ distinct values. Maps with the same $n$
/// and $d$ are not equally likely: the $\prod_j c_j$ factor favors those whose values are spread
/// evenly over the keys.
///
/// `keys_gen` and `values_gen` must produce infinite iterators.
///
/// # Expected complexity per iteration
/// $T(i) = O(m (T^\prime(i) + T^{\prime\prime}(i)))$
///
/// $M(i) = O(m (M^\prime(i) + M^{\prime\prime}(i)))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of the iterators produced by `keys_gen`,
/// $T^{\prime\prime}$ and $M^{\prime\prime}$ are those of the iterators produced by `values_gen`,
/// and $m$ is the mean size.
///
/// # Panics
/// Panics if $\text{size\_a} > \text{size\_b}$, if $\text{unique\_value\_count\_a} >
/// \text{unique\_value\_count\_b}$, or if no size in $[\text{size\_a}, \text{size\_b}]$ can have a
/// number of distinct values in $[\text{unique\_value\_count\_a}, \text{unique\_value\_count\_b}]$.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::chars::random::random_char_inclusive_range;
/// use malachite_base::maps::random::random_hash_maps_size_and_unique_value_count_inclusive_range;
/// use malachite_base::num::random::random_unsigned_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
/// use maplit::hashmap;
///
/// let xs = random_hash_maps_size_and_unique_value_count_inclusive_range(
///     EXAMPLE_SEED,
///     2,
///     3,
///     1,
///     2,
///     &|seed| random_char_inclusive_range(seed, 'a', 'c'),
///     &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 2),
/// );
/// let values = xs.take(20).collect_vec();
/// assert_eq!(
///     values,
///     &[
///         hashmap! {'a' => 0, 'b' => 1},
///         hashmap! {'a' => 2, 'b' => 2},
///         hashmap! {'a' => 0, 'b' => 0, 'c' => 0},
///         hashmap! {'a' => 2, 'b' => 2, 'c' => 0},
///         hashmap! {'a' => 0, 'b' => 0, 'c' => 0},
///         hashmap! {'a' => 0, 'b' => 0, 'c' => 0},
///         hashmap! {'a' => 0, 'b' => 0},
///         hashmap! {'b' => 0, 'c' => 0},
///         hashmap! {'a' => 1, 'b' => 1, 'c' => 2},
///         hashmap! {'a' => 1, 'b' => 2, 'c' => 1},
///         hashmap! {'b' => 2, 'c' => 1},
///         hashmap! {'a' => 1, 'b' => 0, 'c' => 1},
///         hashmap! {'a' => 0, 'b' => 0, 'c' => 1},
///         hashmap! {'a' => 2, 'c' => 0},
///         hashmap! {'a' => 0, 'b' => 0, 'c' => 0},
///         hashmap! {'b' => 0, 'c' => 2},
///         hashmap! {'a' => 1, 'b' => 1, 'c' => 1},
///         hashmap! {'a' => 1, 'b' => 1, 'c' => 1},
///         hashmap! {'b' => 0, 'c' => 2},
///         hashmap! {'a' => 1, 'c' => 1}
///     ]
/// );
/// ```
pub fn random_hash_maps_size_and_unique_value_count_inclusive_range<I: Iterator, J: Iterator>(
    seed: Seed,
    size_a: u64,
    size_b: u64,
    unique_value_count_a: u64,
    unique_value_count_b: u64,
    keys_gen: &dyn Fn(Seed) -> I,
    values_gen: &dyn Fn(Seed) -> J,
) -> RandomHashMapsWithUniqueValueCount<I::Item, J::Item, RandomUnsignedInclusiveRange<u64>, I, J>
where
    I::Item: Eq + Hash,
    J::Item: Clone + Eq + Hash,
{
    let (lo, hi) =
        compatible_size_range(size_a, size_b, unique_value_count_a, unique_value_count_b);
    random_hash_maps_with_unique_value_count_from_size_iterator(
        seed,
        &|seed_2| random_unsigned_inclusive_range(seed_2, lo, hi),
        keys_gen,
        values_gen,
        unique_value_count_a,
        unique_value_count_b,
    )
}
