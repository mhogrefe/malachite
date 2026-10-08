// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational::random::{
    RandomRationalsFromDoubleAndSign, random_rationals, striped_random_rationals,
};
use crate::rational_vector::RationalVector;
use alloc::vec::Vec;
use malachite_base::num::random::geometric::GeometricRandomNaturalValues;
use malachite_base::random::Seed;
use malachite_base::vecs::random::{
    RandomFixedLengthVecsFromSingle, RandomVecs, random_vecs, random_vecs_fixed_length_from_single,
};
use malachite_nz::natural::random::{RandomNaturals, StripedRandomNaturals};

/// Generates random [`RationalVector`]s from an iterator of [`Vec`]s of [`Rational`]s.
///
/// This `struct` is created by [`random_rational_vectors_from_iterator`],
/// [`random_rational_vectors_with_dimension_from_iterator`], [`random_rational_vectors`],
/// [`random_rational_vectors_with_dimension`], [`striped_random_rational_vectors`], and
/// [`striped_random_rational_vectors_with_dimension`]; see their documentation for more.
#[derive(Clone, Debug)]
pub struct RandomRationalVectors<I: Iterator<Item = Vec<Rational>>>(I);

impl<I: Iterator<Item = Vec<Rational>>> Iterator for RandomRationalVectors<I> {
    type Item = RationalVector;

    #[inline]
    fn next(&mut self) -> Option<RationalVector> {
        self.0.next().map(|elements| RationalVector { elements })
    }
}

/// The elements that the unstriped [`RationalVector`] generators draw on.
pub type RandomVectorElements = RandomRationalsFromDoubleAndSign<
    RandomNaturals<GeometricRandomNaturalValues<u64>>,
    RandomNaturals<GeometricRandomNaturalValues<u64>>,
>;

/// The elements that the striped [`RationalVector`] generators draw on.
pub type StripedRandomVectorElements = RandomRationalsFromDoubleAndSign<
    StripedRandomNaturals<GeometricRandomNaturalValues<u64>>,
    StripedRandomNaturals<GeometricRandomNaturalValues<u64>>,
>;

/// Generates random [`RationalVector`]s whose elements come from an iterator.
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
/// The elements here are [`Rational`]s in $[-1, 1)$.
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_q::Rational;
/// use malachite_q::rational::random::random_rational_range;
/// use malachite_q::rational_vector::random::random_rational_vectors_from_iterator;
///
/// assert_eq!(
///     prefix_to_string(
///         random_rational_vectors_from_iterator(
///             EXAMPLE_SEED,
///             &|seed| random_rational_range(
///                 seed,
///                 Rational::from(-1),
///                 Rational::from(1),
///                 3,
///                 1,
///                 3,
///                 1
///             ),
///             2,
///             1
///         ),
///         5
///     ),
///     "[(1/3, -3/4, 0, -1/6, 1/10, 6/7), (0), (-5/9, 1/3, 5/6, 1/8, 1/2, 1/2, -1/3, -1), \
///     (-4/5), (0, -2/7, -1/10, -1, 1/2, 0, -1, 0, -1/3, 1/3, 0, 1/2, 0, -1/5), ...]"
/// );
/// ```
#[inline]
pub fn random_rational_vectors_from_iterator<I: Iterator<Item = Rational>>(
    seed: Seed,
    xs_gen: &dyn Fn(Seed) -> I,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> RandomRationalVectors<RandomVecs<Rational, GeometricRandomNaturalValues<u64>, I>> {
    RandomRationalVectors(random_vecs(
        seed,
        xs_gen,
        mean_length_numerator,
        mean_length_denominator,
    ))
}

/// Generates random [`RationalVector`]s of a given dimension whose elements come from an iterator.
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
/// The elements here are [`Rational`]s in $[-1, 1)$.
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_q::Rational;
/// use malachite_q::rational::random::random_rational_range;
/// use malachite_q::rational_vector::random::random_rational_vectors_with_dimension_from_iterator;
///
/// assert_eq!(
///     prefix_to_string(
///         random_rational_vectors_with_dimension_from_iterator(
///             3,
///             random_rational_range(
///                 EXAMPLE_SEED,
///                 Rational::from(-1),
///                 Rational::from(1),
///                 3,
///                 1,
///                 3,
///                 1
///             )
///         ),
///         5
///     ),
///     "[(1/2, 0, 0), (-1/2, 4/11, -1), (-1, -1, -1/2), (1/8, -9/11, -5/9), (1/2, 1/8, -2/5), \
///     ...]"
/// );
/// ```
#[inline]
pub const fn random_rational_vectors_with_dimension_from_iterator<I: Iterator<Item = Rational>>(
    dimension: u64,
    xs: I,
) -> RandomRationalVectors<RandomFixedLengthVecsFromSingle<I>> {
    RandomRationalVectors(random_vecs_fixed_length_from_single(dimension, xs))
}

/// Generates random [`RationalVector`]s.
///
/// The elements are sampled from [`random_rationals`], with a mean bit count of
/// `mean_bits_numerator / mean_bits_denominator`. The dimensions are sampled from a geometric
/// distribution with mean `mean_length_numerator / mean_length_denominator`, so the 0-dimensional
/// vector is generated with the probability that that distribution gives to 0.
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
/// Panics if `mean_bits_numerator` or `mean_bits_denominator` are zero, if `mean_bits_numerator <=
/// mean_bits_denominator`, if `mean_length_numerator` or `mean_length_denominator` are zero, or if,
/// after being reduced to lowest terms, their sum is greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_q::rational_vector::random::random_rational_vectors;
///
/// assert_eq!(
///     prefix_to_string(random_rational_vectors(EXAMPLE_SEED, 4, 1, 2, 1), 5),
///     "[(0, 1, -1/55, 0, -5/166, 7/118), (-3/194), (-1/6, 265/42, -1/6, 1, 0, 0, 53/15, 1/14), \
///     (5/4), \
///     (714121/22, -196, 0, -1, -3372766/3, -10, -8, 0, 0, 3699/7, -6, 36, 59/2, -47/31566), ...]"
/// );
/// ```
#[inline]
pub fn random_rational_vectors(
    seed: Seed,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> RandomRationalVectors<
    RandomVecs<Rational, GeometricRandomNaturalValues<u64>, RandomVectorElements>,
> {
    random_rational_vectors_from_iterator(
        seed,
        &|seed_2| random_rationals(seed_2, mean_bits_numerator, mean_bits_denominator),
        mean_length_numerator,
        mean_length_denominator,
    )
}

/// Generates random [`RationalVector`]s of a given dimension.
///
/// The elements are sampled from [`random_rationals`], with a mean bit count of
/// `mean_bits_numerator / mean_bits_denominator`.
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
/// Panics if `mean_bits_numerator` or `mean_bits_denominator` are zero, or if `mean_bits_numerator
/// <= mean_bits_denominator`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_q::rational_vector::random::random_rational_vectors_with_dimension;
///
/// assert_eq!(
///     prefix_to_string(random_rational_vectors_with_dimension(EXAMPLE_SEED, 3, 4, 1), 5),
///     "[(0, -1/3, 1), (4/323, 0, 7/12), (1/17, 7, 1/97), (15/19, 0, -79/21), \
///     (7/6, -9912911/5, 0), ...]"
/// );
/// ```
#[inline]
pub fn random_rational_vectors_with_dimension(
    seed: Seed,
    dimension: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomRationalVectors<RandomFixedLengthVecsFromSingle<RandomVectorElements>> {
    random_rational_vectors_with_dimension_from_iterator(
        dimension,
        random_rationals(seed, mean_bits_numerator, mean_bits_denominator),
    )
}

/// Generates random [`RationalVector`]s with striped elements.
///
/// The elements are sampled from [`striped_random_rationals`]: each element's bit count has mean
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
/// mean_stripe_denominator`, if `mean_bits_numerator` or `mean_bits_denominator` are zero, if
/// `mean_bits_numerator <= mean_bits_denominator`, if `mean_length_numerator` or
/// `mean_length_denominator` are zero, or if, after being reduced to lowest terms, the sum of those
/// two is greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_q::rational_vector::random::striped_random_rational_vectors;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_rational_vectors(EXAMPLE_SEED, 16, 1, 4, 1, 2, 1),
///         5
///     ),
///     "[(0, 1, -3/127, 0, -7/128, 1/16), (-5/341), (-1/4, 363/85, -1/7, 1, 0, 0, 4, 1/14), (1), \
///     (1048323/28, -128, 0, -15/8, -2162672/3, -15, -8, 0, 0, 4095/4, -79/16, 62, 63/2, \
///     -32/16639), ...]"
/// );
/// ```
#[inline]
pub fn striped_random_rational_vectors(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> RandomRationalVectors<
    RandomVecs<Rational, GeometricRandomNaturalValues<u64>, StripedRandomVectorElements>,
> {
    random_rational_vectors_from_iterator(
        seed,
        &|seed_2| {
            striped_random_rationals(
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

/// Generates random [`RationalVector`]s of a given dimension, with striped elements.
///
/// The elements are striped, as they are in [`striped_random_rational_vectors`].
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
/// mean_stripe_denominator`, if `mean_bits_numerator` or `mean_bits_denominator` are zero, or if
/// `mean_bits_numerator <= mean_bits_denominator`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_q::rational_vector::random::striped_random_rational_vectors_with_dimension;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_rational_vectors_with_dimension(EXAMPLE_SEED, 3, 16, 1, 4, 1),
///         5
///     ),
///     "[(0, -1/3, 1), (1/64, 0, 2/7), (1/16, 16/3, 1/64), (31/32, 0, -127/23), \
///     (1, -2321847, 0), ...]"
/// );
/// ```
#[inline]
pub fn striped_random_rational_vectors_with_dimension(
    seed: Seed,
    dimension: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomRationalVectors<RandomFixedLengthVecsFromSingle<StripedRandomVectorElements>> {
    random_rational_vectors_with_dimension_from_iterator(
        dimension,
        striped_random_rationals(
            seed,
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator,
        ),
    )
}
