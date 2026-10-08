// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural::random::{
    RandomNaturals, StripedRandomNaturals, random_naturals, striped_random_naturals,
};
use crate::natural_vector::NaturalVector;
use alloc::vec::Vec;
use malachite_base::num::random::geometric::GeometricRandomNaturalValues;
use malachite_base::random::Seed;
use malachite_base::vecs::random::{
    RandomFixedLengthVecsFromSingle, RandomVecs, random_vecs, random_vecs_fixed_length_from_single,
};

/// Generates random [`NaturalVector`]s from an iterator of [`Vec`]s of [`Natural`]s.
///
/// This `struct` is created by [`random_natural_vectors_from_iterator`],
/// [`random_natural_vectors_with_dimension_from_iterator`], [`random_natural_vectors`],
/// [`random_natural_vectors_with_dimension`], [`striped_random_natural_vectors`], and
/// [`striped_random_natural_vectors_with_dimension`]; see their documentation for more.
#[derive(Clone, Debug)]
pub struct RandomNaturalVectors<I: Iterator<Item = Vec<Natural>>>(I);

impl<I: Iterator<Item = Vec<Natural>>> Iterator for RandomNaturalVectors<I> {
    type Item = NaturalVector;

    #[inline]
    fn next(&mut self) -> Option<NaturalVector> {
        self.0.next().map(|elements| NaturalVector { elements })
    }
}

/// The elements that the unstriped [`NaturalVector`] generators draw on.
pub type RandomVectorElements = RandomNaturals<GeometricRandomNaturalValues<u64>>;

/// The elements that the striped [`NaturalVector`] generators draw on.
pub type StripedRandomVectorElements = StripedRandomNaturals<GeometricRandomNaturalValues<u64>>;

/// Generates random [`NaturalVector`]s whose elements come from an iterator.
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
/// The elements here are [`Natural`]s from 0 to 15.
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural::random::uniform_random_natural_inclusive_range;
/// use malachite_nz::natural_vector::random::random_natural_vectors_from_iterator;
///
/// assert_eq!(
///     prefix_to_string(
///         random_natural_vectors_from_iterator(
///             EXAMPLE_SEED,
///             &|seed| uniform_random_natural_inclusive_range(
///                 seed,
///                 Natural::ZERO,
///                 Natural::from(15u32)
///             ),
///             2,
///             1
///         ),
///         5
///     ),
///     "[(5, 6, 14, 5, 10, 2), (13), (5, 10, 1, 9, 0, 14, 9, 13), (4), (4, 7, 1, 11, 9, 3, 12, \
///     9, 14, 10, 15, 14, 0, 3), ...]"
/// );
/// ```
#[inline]
pub fn random_natural_vectors_from_iterator<I: Iterator<Item = Natural>>(
    seed: Seed,
    xs_gen: &dyn Fn(Seed) -> I,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> RandomNaturalVectors<RandomVecs<Natural, GeometricRandomNaturalValues<u64>, I>> {
    RandomNaturalVectors(random_vecs(
        seed,
        xs_gen,
        mean_length_numerator,
        mean_length_denominator,
    ))
}

/// Generates random [`NaturalVector`]s of a given dimension whose elements come from an iterator.
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
/// The elements here are [`Natural`]s from 0 to 15.
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural::random::uniform_random_natural_inclusive_range;
/// use malachite_nz::natural_vector::random::random_natural_vectors_with_dimension_from_iterator;
///
/// assert_eq!(
///     prefix_to_string(
///         random_natural_vectors_with_dimension_from_iterator(
///             3,
///             uniform_random_natural_inclusive_range(
///                 EXAMPLE_SEED,
///                 Natural::ZERO,
///                 Natural::from(15u32)
///             )
///         ),
///         5
///     ),
///     "[(1, 7, 13), (5, 7, 9), (2, 8, 2), (11, 4, 11), (14, 13, 6), ...]"
/// );
/// ```
#[inline]
pub const fn random_natural_vectors_with_dimension_from_iterator<I: Iterator<Item = Natural>>(
    dimension: u64,
    xs: I,
) -> RandomNaturalVectors<RandomFixedLengthVecsFromSingle<I>> {
    RandomNaturalVectors(random_vecs_fixed_length_from_single(dimension, xs))
}

/// Generates random [`NaturalVector`]s.
///
/// The elements are sampled from [`random_naturals`], with a mean bit count of `mean_bits_numerator
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
/// use malachite_nz::natural_vector::random::random_natural_vectors;
///
/// assert_eq!(
///     prefix_to_string(random_natural_vectors(EXAMPLE_SEED, 4, 1, 2, 1), 5),
///     "[(3, 0, 13235, 1, 7, 1), (0), (15, 13, 2, 1, 0, 1, 0, 0), (1), \
///     (7, 1, 11, 1, 1523, 4, 2, 35, 2, 117, 0, 391, 12, 6), ...]"
/// );
/// ```
#[inline]
pub fn random_natural_vectors(
    seed: Seed,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> RandomNaturalVectors<
    RandomVecs<Natural, GeometricRandomNaturalValues<u64>, RandomVectorElements>,
> {
    random_natural_vectors_from_iterator(
        seed,
        &|seed_2| random_naturals(seed_2, mean_bits_numerator, mean_bits_denominator),
        mean_length_numerator,
        mean_length_denominator,
    )
}

/// Generates random [`NaturalVector`]s of a given dimension.
///
/// The elements are sampled from [`random_naturals`], with a mean bit count of `mean_bits_numerator
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
/// use malachite_nz::natural_vector::random::random_natural_vectors_with_dimension;
///
/// assert_eq!(
///     prefix_to_string(random_natural_vectors_with_dimension(EXAMPLE_SEED, 3, 4, 1), 5),
///     "[(14, 8, 10), (1, 1, 0), (1, 184, 15), (99, 1, 3), (3, 6, 1), ...]"
/// );
/// ```
#[inline]
pub fn random_natural_vectors_with_dimension(
    seed: Seed,
    dimension: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomNaturalVectors<RandomFixedLengthVecsFromSingle<RandomVectorElements>> {
    random_natural_vectors_with_dimension_from_iterator(
        dimension,
        random_naturals(seed, mean_bits_numerator, mean_bits_denominator),
    )
}

/// Generates random [`NaturalVector`]s with striped elements.
///
/// The elements are sampled from [`striped_random_naturals`]: each element's bit count has mean
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
/// use malachite_nz::natural_vector::random::striped_random_natural_vectors;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_natural_vectors(EXAMPLE_SEED, 16, 1, 4, 1, 2, 1),
///         5
///     ),
///     "[(2, 0, 16376, 1, 4, 1), (0), (15, 15, 3, 1, 0, 1, 0, 0), (1), \
///     (7, 1, 14, 1, 1024, 7, 2, 63, 2, 127, 0, 327, 8, 7), ...]"
/// );
/// ```
#[inline]
pub fn striped_random_natural_vectors(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> RandomNaturalVectors<
    RandomVecs<Natural, GeometricRandomNaturalValues<u64>, StripedRandomVectorElements>,
> {
    random_natural_vectors_from_iterator(
        seed,
        &|seed_2| {
            striped_random_naturals(
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

/// Generates random [`NaturalVector`]s of a given dimension, with striped elements.
///
/// The elements are striped, as they are in [`striped_random_natural_vectors`].
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
/// use malachite_nz::natural_vector::random::striped_random_natural_vectors_with_dimension;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_natural_vectors_with_dimension(EXAMPLE_SEED, 3, 16, 1, 4, 1),
///         5
///     ),
///     "[(8, 8, 8), (1, 1, 0), (1, 128, 15), (64, 1, 3), (2, 4, 1), ...]"
/// );
/// ```
#[inline]
pub fn striped_random_natural_vectors_with_dimension(
    seed: Seed,
    dimension: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomNaturalVectors<RandomFixedLengthVecsFromSingle<StripedRandomVectorElements>> {
    random_natural_vectors_with_dimension_from_iterator(
        dimension,
        striped_random_naturals(
            seed,
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator,
        ),
    )
}
