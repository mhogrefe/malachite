// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::random::geometric::GeometricRandomNaturalValues;
use crate::random::Seed;
use crate::vecs::random::{
    RandomFixedLengthVecsFromSingle, RandomVecs, random_vecs, random_vecs_fixed_length_from_single,
};
use crate::vector::Vector;
use alloc::vec::Vec;

/// Generates random [`Vector`]s from an iterator of [`Vec`]s.
///
/// This `struct` is created by [`random_vectors`] and [`random_vectors_with_dimension`]; see
/// their documentation for more.
#[derive(Clone, Debug)]
pub struct RandomVectors<I>(I);

impl<T, I: Iterator<Item = Vec<T>>> Iterator for RandomVectors<I> {
    type Item = Vector<T>;

    #[inline]
    fn next(&mut self) -> Option<Vector<T>> {
        self.0.next().map(|elements| Vector { elements })
    }
}

/// Generates random [`Vector`]s with elements from a given iterator.
///
/// The elements are drawn from the iterator that `xs_gen` builds from a seed. The dimensions are
/// sampled from a geometric distribution with mean `mean_length_numerator /
/// mean_length_denominator`, so the 0-dimensional vector is generated with the probability that
/// that distribution gives to 0.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(n + T^\prime(i))$
///
/// $M(i) = O(n + M^\prime(i))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of the element iterator, and $n$ is the
/// dimension of the $i$th output.
///
/// # Panics
/// Panics if `mean_length_numerator` or `mean_length_denominator` are zero, or if, after being
/// reduced to lowest terms, their sum is greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::random::random_primitive_ints;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::vector::random::random_vectors;
///
/// assert_eq!(
///     prefix_to_string(
///         random_vectors(EXAMPLE_SEED, &random_primitive_ints::<u8>, 2, 1),
///         5
///     ),
///     "[(85, 11, 136, 200, 235, 134), (203), (223, 38, 235, 217, 177, 162, 32, 166), (234), \
///     (30, 218, 90, 106, 9, 216, 204, 151, 213, 97, 253, 78, 91, 39), ...]"
/// );
/// ```
#[inline]
pub fn random_vectors<I: Iterator>(
    seed: Seed,
    xs_gen: &dyn Fn(Seed) -> I,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> RandomVectors<RandomVecs<I::Item, GeometricRandomNaturalValues<u64>, I>> {
    RandomVectors(random_vecs(
        seed,
        xs_gen,
        mean_length_numerator,
        mean_length_denominator,
    ))
}

/// Generates random [`Vector`]s of a given dimension with elements from a given iterator.
///
/// If `dimension` is 0, the output consists of the 0-dimensional vector, repeated.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(n + T^\prime(i))$
///
/// $M(i) = O(n + M^\prime(i))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of `xs`, and $n$ is `dimension`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::random::random_primitive_ints;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::vector::random::random_vectors_with_dimension;
///
/// assert_eq!(
///     prefix_to_string(
///         random_vectors_with_dimension(3, random_primitive_ints::<u8>(EXAMPLE_SEED)),
///         5
///     ),
///     "[(113, 239, 69), (108, 228, 210), (168, 161, 87), (32, 110, 83), (188, 34, 89), ...]"
/// );
/// ```
#[inline]
pub const fn random_vectors_with_dimension<I: Iterator>(
    dimension: u64,
    xs: I,
) -> RandomVectors<RandomFixedLengthVecsFromSingle<I>> {
    RandomVectors(random_vecs_fixed_length_from_single(dimension, xs))
}
