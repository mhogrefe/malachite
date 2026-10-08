// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer::random::{
    RandomIntegers, StripedRandomIntegers, random_integers, striped_random_integers,
};
use crate::integer_vector::IntegerVector;
use alloc::vec::Vec;
use malachite_base::num::random::geometric::{
    GeometricRandomNaturalValues, GeometricRandomSigneds,
};
use malachite_base::random::Seed;
use malachite_base::vecs::random::{
    RandomFixedLengthVecsFromSingle, RandomVecs, random_vecs, random_vecs_fixed_length_from_single,
};

/// Generates random [`IntegerVector`]s from an iterator of [`Vec`]s of [`Integer`]s.
///
/// This `struct` is created by [`random_integer_vectors_from_iterator`],
/// [`random_integer_vectors_with_dimension_from_iterator`], [`random_integer_vectors`],
/// [`random_integer_vectors_with_dimension`], [`striped_random_integer_vectors`], and
/// [`striped_random_integer_vectors_with_dimension`]; see their documentation for more.
#[derive(Clone, Debug)]
pub struct RandomIntegerVectors<I: Iterator<Item = Vec<Integer>>>(I);

impl<I: Iterator<Item = Vec<Integer>>> Iterator for RandomIntegerVectors<I> {
    type Item = IntegerVector;

    #[inline]
    fn next(&mut self) -> Option<IntegerVector> {
        self.0.next().map(|elements| IntegerVector { elements })
    }
}

/// The elements that the unstriped [`IntegerVector`] generators draw on.
pub type RandomVectorElements = RandomIntegers<GeometricRandomSigneds<i64>>;

/// The elements that the striped [`IntegerVector`] generators draw on.
pub type StripedRandomVectorElements = StripedRandomIntegers<GeometricRandomSigneds<i64>>;

/// Generates random [`IntegerVector`]s whose elements come from an iterator.
///
/// `xs_gen` is given a seed derived from `seed` and produces the iterator of elements; each vector
/// takes its elements, in order, from that iterator. This allows any distribution of elements, for
/// example one with a bounded number of bits. The dimensions are sampled from a geometric
/// distribution with mean `mean_length_numerator / mean_length_denominator`, so the 0-dimensional
/// vector is generated with the probability that that distribution gives to 0.
///
/// The iterator produced by `xs_gen` must be infinite, and so is the output.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(n T^\prime(i))$
///
/// $M(i) = O(n M^\prime(i))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of the iterator produced by `xs_gen`, and $n$ is
/// the dimension of the $i$th output.
///
/// # Panics
/// Panics if `mean_length_numerator` or `mean_length_denominator` are zero, or if, after being
/// reduced to lowest terms, their sum is greater than or equal to $2^{64}$.
///
/// # Examples
/// The elements here are [`Integer`]s from $-8$ to $8$.
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer::random::uniform_random_integer_inclusive_range;
/// use malachite_nz::integer_vector::random::random_integer_vectors_from_iterator;
///
/// assert_eq!(
///     prefix_to_string(
///         random_integer_vectors_from_iterator(
///             EXAMPLE_SEED,
///             &|seed| uniform_random_integer_inclusive_range(
///                 seed,
///                 Integer::from(-8),
///                 Integer::from(8)
///             ),
///             2,
///             1
///         ),
///         5
///     ),
///     "[(-2, 2, 5, -3, 2, 1), (8), (1, -5, 6, 7, -8, -5, 7, -4), (7), (6, -5, 8, 3, 4, -3, -1, \
///     1, -7, 2, -6, 6, -2, 8), ...]"
/// );
/// ```
#[inline]
pub fn random_integer_vectors_from_iterator<I: Iterator<Item = Integer>>(
    seed: Seed,
    xs_gen: &dyn Fn(Seed) -> I,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> RandomIntegerVectors<RandomVecs<Integer, GeometricRandomNaturalValues<u64>, I>> {
    RandomIntegerVectors(random_vecs(
        seed,
        xs_gen,
        mean_length_numerator,
        mean_length_denominator,
    ))
}

/// Generates random [`IntegerVector`]s of a given dimension whose elements come from an iterator.
///
/// Each vector takes its `dimension` elements, in order, from `xs`. This allows any distribution of
/// elements, for example one with a bounded number of bits. If `dimension` is 0, the output
/// consists of the 0-dimensional vector, repeated.
///
/// `xs` must be infinite, and so is the output.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(n T^\prime(i))$
///
/// $M(i) = O(n M^\prime(i))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of `xs`, and $n$ is `dimension`.
///
/// # Examples
/// The elements here are [`Integer`]s from $-8$ to $8$.
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer::random::uniform_random_integer_inclusive_range;
/// use malachite_nz::integer_vector::random::random_integer_vectors_with_dimension_from_iterator;
///
/// assert_eq!(
///     prefix_to_string(
///         random_integer_vectors_with_dimension_from_iterator(
///             3,
///             uniform_random_integer_inclusive_range(
///                 EXAMPLE_SEED,
///                 Integer::from(-8),
///                 Integer::from(8)
///             )
///         ),
///         5
///     ),
///     "[(-1, 1, 0), (3, 6, 3), (-7, -1, -7), (-2, -2, -2), (-1, -3, 4), ...]"
/// );
/// ```
#[inline]
pub const fn random_integer_vectors_with_dimension_from_iterator<I: Iterator<Item = Integer>>(
    dimension: u64,
    xs: I,
) -> RandomIntegerVectors<RandomFixedLengthVecsFromSingle<I>> {
    RandomIntegerVectors(random_vecs_fixed_length_from_single(dimension, xs))
}

/// Generates random [`IntegerVector`]s.
///
/// The elements are sampled from [`random_integers`], with a mean bit count of `mean_bits_numerator
/// / mean_bits_denominator`. The dimensions are sampled from a geometric distribution with mean
/// `mean_length_numerator / mean_length_denominator`, so the 0-dimensional vector is generated with
/// the probability that that distribution gives to 0.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(nb)$
///
/// $M(i) = O(nb)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $n$ is the dimension
/// of the $i$th output, and $b$ is `mean_bits_numerator / mean_bits_denominator`.
///
/// # Panics
/// Panics if `mean_bits_numerator` or `mean_bits_denominator` are zero, if `mean_length_numerator`
/// or `mean_length_denominator` are zero, or if either ratio, after being reduced to lowest terms,
/// has a numerator and denominator whose sum is greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer_vector::random::random_integer_vectors;
///
/// assert_eq!(
///     prefix_to_string(random_integer_vectors(EXAMPLE_SEED, 4, 1, 2, 1), 5),
///     "[(-497, -1, 1, 19, 799, 799), (-1), (66, -10721, 334, 59, -119, -5, 0, -131), (-1), \
///     (0, 7, -1, 10, 11, -37408, 0, -1, -903, 28, 0, 0, -6, 2), ...]"
/// );
/// ```
#[inline]
pub fn random_integer_vectors(
    seed: Seed,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> RandomIntegerVectors<
    RandomVecs<Integer, GeometricRandomNaturalValues<u64>, RandomVectorElements>,
> {
    random_integer_vectors_from_iterator(
        seed,
        &|seed_2| random_integers(seed_2, mean_bits_numerator, mean_bits_denominator),
        mean_length_numerator,
        mean_length_denominator,
    )
}

/// Generates random [`IntegerVector`]s of a given dimension.
///
/// The elements are sampled from [`random_integers`], with a mean bit count of `mean_bits_numerator
/// / mean_bits_denominator`.
///
/// If `dimension` is 0, the output consists of the 0-dimensional vector, repeated.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(nb)$
///
/// $M(i) = O(nb)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $n$ is `dimension`,
/// and $b$ is `mean_bits_numerator / mean_bits_denominator`.
///
/// # Panics
/// Panics if `mean_bits_numerator` or `mean_bits_denominator` are zero, or if, after being reduced
/// to lowest terms, their sum is greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer_vector::random::random_integer_vectors_with_dimension;
///
/// assert_eq!(
///     prefix_to_string(random_integer_vectors_with_dimension(EXAMPLE_SEED, 3, 4, 1), 5),
///     "[(2, 152, 1), (0, -62, 5282), (0, 28, 4427344568), (79, -11, -7330), \
///     (-1, -5667523, 1618), ...]"
/// );
/// ```
#[inline]
pub fn random_integer_vectors_with_dimension(
    seed: Seed,
    dimension: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomIntegerVectors<RandomFixedLengthVecsFromSingle<RandomVectorElements>> {
    random_integer_vectors_with_dimension_from_iterator(
        dimension,
        random_integers(seed, mean_bits_numerator, mean_bits_denominator),
    )
}

/// Generates random [`IntegerVector`]s with striped elements.
///
/// The elements are sampled from [`striped_random_integers`]: each element's bit count has mean
/// `mean_bits_numerator / mean_bits_denominator`, and its bits come in runs whose mean length is
/// `mean_stripe_numerator / mean_stripe_denominator`. A striped element is one whose bits come in
/// long runs, which is what makes the carries and borrows of an arithmetic test interesting.
///
/// The dimensions are sampled from a geometric distribution with mean `mean_length_numerator /
/// mean_length_denominator`, so the 0-dimensional vector is generated with the probability that
/// that distribution gives to 0.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(nb)$
///
/// $M(i) = O(nb)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $n$ is the dimension
/// of the $i$th output, and $b$ is `mean_bits_numerator / mean_bits_denominator`.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if `mean_stripe_numerator <
/// mean_stripe_denominator`, if `mean_bits_numerator`, `mean_bits_denominator`,
/// `mean_length_numerator`, or `mean_length_denominator` are zero, or if either of the last two
/// ratios, after being reduced to lowest terms, has a numerator and denominator whose sum is
/// greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer_vector::random::striped_random_integer_vectors;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_integer_vectors(EXAMPLE_SEED, 16, 1, 4, 1, 2, 1),
///         5
///     ),
///     "[(-496, -1, 1, 16, 512, 543), (-1), (127, -10239, 383, 36, -127, -4, 0, -230), (-1), \
///     (0, 4, -1, 8, 15, -65024, 0, -1, -1023, 16, 0, 0, -7, 3), ...]"
/// );
/// ```
#[inline]
pub fn striped_random_integer_vectors(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> RandomIntegerVectors<
    RandomVecs<Integer, GeometricRandomNaturalValues<u64>, StripedRandomVectorElements>,
> {
    random_integer_vectors_from_iterator(
        seed,
        &|seed_2| {
            striped_random_integers(
                seed_2,
                mean_stripe_numerator,
                mean_stripe_denominator,
                mean_bits_numerator,
                mean_bits_denominator,
            )
        },
        mean_length_numerator,
        mean_length_denominator,
    )
}

/// Generates random [`IntegerVector`]s of a given dimension, with striped elements.
///
/// The elements are striped, as they are in [`striped_random_integer_vectors`].
///
/// If `dimension` is 0, the output consists of the 0-dimensional vector, repeated.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(nb)$
///
/// $M(i) = O(nb)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $n$ is `dimension`,
/// and $b$ is `mean_bits_numerator / mean_bits_denominator`.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if `mean_stripe_numerator <
/// mean_stripe_denominator`, if `mean_bits_numerator` or `mean_bits_denominator` are zero, or if,
/// after being reduced to lowest terms, their sum is greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer_vector::random::striped_random_integer_vectors_with_dimension;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_integer_vectors_with_dimension(EXAMPLE_SEED, 3, 16, 1, 4, 1),
///         5
///     ),
///     "[(2, 128, 1), (0, -32, 4096), (0, 31, 6442451424), (127, -8, -6145), \
///     (-1, -7864320, 1024), ...]"
/// );
/// ```
#[inline]
pub fn striped_random_integer_vectors_with_dimension(
    seed: Seed,
    dimension: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomIntegerVectors<RandomFixedLengthVecsFromSingle<StripedRandomVectorElements>> {
    random_integer_vectors_with_dimension_from_iterator(
        dimension,
        striped_random_integers(
            seed,
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator,
        ),
    )
}
