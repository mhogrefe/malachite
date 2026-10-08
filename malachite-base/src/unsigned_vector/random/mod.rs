// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::random::geometric::GeometricRandomNaturalValues;
use crate::num::random::striped::{StripedRandomUnsignedBitChunks, striped_random_unsigneds};
use crate::num::random::{RandomPrimitiveInts, random_primitive_ints};
use crate::random::Seed;
use crate::unsigned_vector::UnsignedVector;
use crate::vecs::random::{
    RandomFixedLengthVecsFromSingle, RandomVecs, random_vecs, random_vecs_fixed_length_from_single,
};
use alloc::vec::Vec;

/// Generates random [`UnsignedVector`]s from an iterator of [`Vec`]s.
///
/// This `struct` is created by [`random_unsigned_vectors`],
/// [`random_unsigned_vectors_with_dimension`], [`striped_random_unsigned_vectors`], and
/// [`striped_random_unsigned_vectors_with_dimension`]; see their documentation for more.
#[derive(Clone, Debug)]
pub struct RandomUnsignedVectors<I>(I);

impl<T: PrimitiveUnsigned, I: Iterator<Item = Vec<T>>> Iterator for RandomUnsignedVectors<I> {
    type Item = UnsignedVector<T>;

    #[inline]
    fn next(&mut self) -> Option<UnsignedVector<T>> {
        self.0.next().map(|elements| UnsignedVector { elements })
    }
}

/// Generates random [`UnsignedVector`]s.
///
/// The elements are sampled from [`random_primitive_ints`], so each is uniform over its whole
/// range. The dimensions are sampled from a geometric distribution with mean `mean_length_numerator
/// / mean_length_denominator`, so the 0-dimensional vector is generated with the probability that
/// that distribution gives to 0.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(n)$
///
/// $M(i) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, and $n$ is the
/// dimension of the $i$th output.
///
/// # Panics
/// Panics if `mean_length_numerator` or `mean_length_denominator` are zero, or if, after being
/// reduced to lowest terms, their sum is greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::unsigned_vector::random::random_unsigned_vectors;
///
/// assert_eq!(
///     prefix_to_string(random_unsigned_vectors::<u8>(EXAMPLE_SEED, 2, 1), 5),
///     "[(85, 11, 136, 200, 235, 134), (203), (223, 38, 235, 217, 177, 162, 32, 166), (234), \
///     (30, 218, 90, 106, 9, 216, 204, 151, 213, 97, 253, 78, 91, 39), ...]"
/// );
/// ```
#[inline]
pub fn random_unsigned_vectors<T: PrimitiveUnsigned>(
    seed: Seed,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> RandomUnsignedVectors<RandomVecs<T, GeometricRandomNaturalValues<u64>, RandomPrimitiveInts<T>>>
{
    RandomUnsignedVectors(random_vecs(
        seed,
        &random_primitive_ints,
        mean_length_numerator,
        mean_length_denominator,
    ))
}

/// Generates random [`UnsignedVector`]s of a given dimension.
///
/// The elements are sampled from [`random_primitive_ints`], so each is uniform over its whole
/// range.
///
/// If `dimension` is 0, the output consists of the 0-dimensional vector, repeated.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(n)$
///
/// $M(i) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, and $n$ is
/// `dimension`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::unsigned_vector::random::random_unsigned_vectors_with_dimension;
///
/// assert_eq!(
///     prefix_to_string(
///         random_unsigned_vectors_with_dimension::<u8>(EXAMPLE_SEED, 3),
///         5
///     ),
///     "[(113, 239, 69), (108, 228, 210), (168, 161, 87), (32, 110, 83), (188, 34, 89), ...]"
/// );
/// ```
#[inline]
pub fn random_unsigned_vectors_with_dimension<T: PrimitiveUnsigned>(
    seed: Seed,
    dimension: u64,
) -> RandomUnsignedVectors<RandomFixedLengthVecsFromSingle<RandomPrimitiveInts<T>>> {
    RandomUnsignedVectors(random_vecs_fixed_length_from_single(
        dimension,
        random_primitive_ints(seed),
    ))
}

/// Generates random [`UnsignedVector`]s with striped elements.
///
/// The elements are sampled from [`striped_random_unsigneds`], with a mean run length of
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
/// $T(i) = O(n)$
///
/// $M(i) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, and $n$ is the
/// dimension of the $i$th output.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if `mean_stripe_numerator <
/// mean_stripe_denominator`, if `mean_length_numerator` or `mean_length_denominator` are zero, or
/// if, after being reduced to lowest terms, the sum of those two is greater than or equal to
/// $2^{64}$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::unsigned_vector::random::striped_random_unsigned_vectors;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_unsigned_vectors::<u8>(EXAMPLE_SEED, 4, 1, 2, 1),
///         5
///     ),
///     "[(255, 130, 152, 6, 62, 119), (1), (128, 203, 3, 30, 7, 0, 24, 63), (192), \
///     (30, 0, 240, 195, 248, 56, 231, 7, 193, 194, 96, 126, 253, 119), ...]"
/// );
/// ```
#[inline]
pub fn striped_random_unsigned_vectors<T: PrimitiveUnsigned>(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> RandomUnsignedVectors<
    RandomVecs<T, GeometricRandomNaturalValues<u64>, StripedRandomUnsignedBitChunks<T>>,
> {
    RandomUnsignedVectors(random_vecs(
        seed,
        &|seed_2| striped_random_unsigneds(seed_2, mean_stripe_numerator, mean_stripe_denominator),
        mean_length_numerator,
        mean_length_denominator,
    ))
}

/// Generates random [`UnsignedVector`]s of a given dimension, with striped elements.
///
/// The elements are striped, as they are in [`striped_random_unsigned_vectors`].
///
/// If `dimension` is 0, the output consists of the 0-dimensional vector, repeated.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(n)$
///
/// $M(i) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, and $n$ is
/// `dimension`.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero or if `mean_stripe_numerator <
/// mean_stripe_denominator`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::unsigned_vector::random::striped_random_unsigned_vectors_with_dimension;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_unsigned_vectors_with_dimension::<u8>(EXAMPLE_SEED, 3, 4, 1),
///         5
///     ),
///     "[(1, 76, 127), (195, 0, 128), (15, 118, 0), (248, 255, 253), (121, 0, 240), ...]"
/// );
/// ```
#[inline]
pub fn striped_random_unsigned_vectors_with_dimension<T: PrimitiveUnsigned>(
    seed: Seed,
    dimension: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
) -> RandomUnsignedVectors<RandomFixedLengthVecsFromSingle<StripedRandomUnsignedBitChunks<T>>> {
    RandomUnsignedVectors(random_vecs_fixed_length_from_single(
        dimension,
        striped_random_unsigneds(seed, mean_stripe_numerator, mean_stripe_denominator),
    ))
}
