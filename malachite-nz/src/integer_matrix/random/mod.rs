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
use crate::integer_matrix::{IntegerMatrix, element_count};
use core::iter::{Repeat, repeat};
use malachite_base::num::random::geometric::{
    GeometricRandomNaturalValues, GeometricRandomSigneds, geometric_random_unsigneds,
};
use malachite_base::random::Seed;
use malachite_base::tuples::random::{RandomPairsFromSingle, random_pairs_from_single};

/// Generates random [`IntegerMatrix`]s from an iterator of dimensions and an iterator of elements.
///
/// Each matrix takes its height and width from `dimensions` and then its elements, in row-major
/// order, from `xs`.
///
/// This `struct` is created by [`random_integer_matrices_from_iterator`],
/// [`random_integer_matrices_with_dimensions_from_iterator`], [`random_integer_matrices`],
/// [`random_integer_matrices_with_dimensions`], [`striped_random_integer_matrices`], and
/// [`striped_random_integer_matrices_with_dimensions`]; see their documentation for more.
#[derive(Clone, Debug)]
pub struct RandomIntegerMatrices<D: Iterator<Item = (usize, usize)>, I: Iterator<Item = Integer>> {
    dimensions: D,
    xs: I,
}

impl<D: Iterator<Item = (usize, usize)>, I: Iterator<Item = Integer>> Iterator
    for RandomIntegerMatrices<D, I>
{
    type Item = IntegerMatrix;

    fn next(&mut self) -> Option<IntegerMatrix> {
        let (height, width) = self.dimensions.next()?;
        Some(IntegerMatrix::from_owned_elements(
            height,
            width,
            (&mut self.xs).take(element_count(height, width)).collect(),
        ))
    }
}

/// The dimensions that the [`IntegerMatrix`] generators of random shape draw on.
pub type RandomMatrixDimensions = RandomPairsFromSingle<GeometricRandomNaturalValues<usize>>;

/// The elements that the unstriped [`IntegerMatrix`] generators draw on.
pub type RandomMatrixElements = RandomIntegers<GeometricRandomSigneds<i64>>;

/// The elements that the striped [`IntegerMatrix`] generators draw on.
pub type StripedRandomMatrixElements = StripedRandomIntegers<GeometricRandomSigneds<i64>>;

/// Generates random [`IntegerMatrix`]s whose elements come from an iterator.
///
/// `xs_gen` is given a seed derived from `seed` and produces the iterator of elements; each matrix
/// takes its elements, in row-major order, from that iterator. This allows any distribution of
/// elements, for example the residues modulo some $m$. The height and width are sampled
/// independently from a geometric distribution with mean `mean_dimension_numerator /
/// mean_dimension_denominator`, so matrices with no rows or no columns are generated with the
/// probability that that distribution gives to 0.
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
/// the number of elements of the $i$th output.
///
/// # Panics
/// Panics if `mean_dimension_numerator` or `mean_dimension_denominator` are zero, or if, after
/// being reduced to lowest terms, their sum is greater than or equal to $2^{64}$.
///
/// # Examples
/// The elements here are [`Integer`]s from $-8$ to $8$.
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer::random::uniform_random_integer_inclusive_range;
/// use malachite_nz::integer_matrix::random::random_integer_matrices_from_iterator;
///
/// assert_eq!(
///     prefix_to_string(
///         random_integer_matrices_from_iterator(
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
///     "[[]_{0,1}, []_{0,11}, []_{2,0}, [-4, 3; -8, 3; 5, 1; 5, 7], []_{6,0}, ...]"
/// );
/// ```
#[inline]
pub fn random_integer_matrices_from_iterator<I: Iterator<Item = Integer>>(
    seed: Seed,
    xs_gen: &dyn Fn(Seed) -> I,
    mean_dimension_numerator: u64,
    mean_dimension_denominator: u64,
) -> RandomIntegerMatrices<RandomMatrixDimensions, I> {
    RandomIntegerMatrices {
        dimensions: random_pairs_from_single(geometric_random_unsigneds(
            seed.fork("dimensions"),
            mean_dimension_numerator,
            mean_dimension_denominator,
        )),
        xs: xs_gen(seed.fork("elements")),
    }
}

/// Generates random [`IntegerMatrix`]s with a given height and width whose elements come from an
/// iterator.
///
/// Each matrix takes its `height * width` elements, in row-major order, from `xs`. This allows any
/// distribution of elements, for example the residues modulo some $m$. If `height` or `width` is 0,
/// the output consists of the matrix of that shape, which has no elements, repeated.
///
/// `xs` must be infinite, and so is the output.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(n T^\prime(i))$
///
/// $M(i) = O(n M^\prime(i))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of `xs`, and $n$ is `height * width`.
///
/// # Panics
/// Panics if `height * width` does not fit in a `usize`.
///
/// # Examples
/// The elements here are [`Integer`]s from $-8$ to $8$.
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer::random::uniform_random_integer_inclusive_range;
/// use malachite_nz::integer_matrix::random::random_integer_matrices_with_dimensions_from_iterator;
///
/// assert_eq!(
///     prefix_to_string(
///         random_integer_matrices_with_dimensions_from_iterator(
///             2,
///             2,
///             uniform_random_integer_inclusive_range(
///                 EXAMPLE_SEED,
///                 Integer::from(-8),
///                 Integer::from(8)
///             )
///         ),
///         5
///     ),
///     "[[-1, 1; 0, 3], [6, 3; -7, -1], [-7, -2; -2, -2], [-1, -3; 4, -1], [7, -1; -1, -3], ...]"
/// );
/// ```
#[inline]
pub fn random_integer_matrices_with_dimensions_from_iterator<I: Iterator<Item = Integer>>(
    height: usize,
    width: usize,
    xs: I,
) -> RandomIntegerMatrices<Repeat<(usize, usize)>, I> {
    element_count(height, width);
    RandomIntegerMatrices {
        dimensions: repeat((height, width)),
        xs,
    }
}

/// Generates random [`IntegerMatrix`]s.
///
/// The elements are sampled from [`random_integers`], with a mean bit count of `mean_bits_numerator
/// / mean_bits_denominator`. The height and width are sampled independently from a geometric
/// distribution with mean `mean_dimension_numerator / mean_dimension_denominator`, so matrices with
/// no rows or no columns are generated with the probability that that distribution gives to 0.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(nb)$
///
/// $M(i) = O(nb)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $n$ is the number of
/// elements of the $i$th output, and $b$ is `mean_bits_numerator / mean_bits_denominator`.
///
/// # Panics
/// Panics if `mean_bits_numerator` or `mean_bits_denominator` are zero, if
/// `mean_dimension_numerator` or `mean_dimension_denominator` are zero, or if either ratio, after
/// being reduced to lowest terms, has a numerator and denominator whose sum is greater than or
/// equal to $2^{64}$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer_matrix::random::random_integer_matrices;
///
/// assert_eq!(
///     prefix_to_string(random_integer_matrices(EXAMPLE_SEED, 4, 1, 2, 1), 5),
///     "[[]_{0,1}, []_{0,11}, []_{2,0}, [0, 0; -1, 3; -1, -7; 0, 35], []_{6,0}, ...]"
/// );
/// ```
#[inline]
pub fn random_integer_matrices(
    seed: Seed,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_dimension_numerator: u64,
    mean_dimension_denominator: u64,
) -> RandomIntegerMatrices<RandomMatrixDimensions, RandomMatrixElements> {
    random_integer_matrices_from_iterator(
        seed,
        &|seed_2| random_integers(seed_2, mean_bits_numerator, mean_bits_denominator),
        mean_dimension_numerator,
        mean_dimension_denominator,
    )
}

/// Generates random [`IntegerMatrix`]s with a given height and width.
///
/// The elements are sampled from [`random_integers`], with a mean bit count of `mean_bits_numerator
/// / mean_bits_denominator`. If `height` or `width` is 0, the output consists of the matrix of that
/// shape, which has no elements, repeated.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(nb)$
///
/// $M(i) = O(nb)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $n$ is `height *
/// width`, and $b$ is `mean_bits_numerator / mean_bits_denominator`.
///
/// # Panics
/// Panics if `height * width` does not fit in a `usize`, if `mean_bits_numerator` or
/// `mean_bits_denominator` are zero, or if, after being reduced to lowest terms, their sum is
/// greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer_matrix::random::random_integer_matrices_with_dimensions;
///
/// assert_eq!(
///     prefix_to_string(
///         random_integer_matrices_with_dimensions(EXAMPLE_SEED, 2, 3, 4, 1),
///         5
///     ),
///     "[[2, 152, 1; 0, -62, 5282], [0, 28, 4427344568; 79, -11, -7330], [-1, -5667523, 1618; \
///     120, -1, -1006], [-7, -45, -1686; -206, 3, -1], [-1, 2516, 775; 5, 95, 23], ...]"
/// );
/// ```
#[inline]
pub fn random_integer_matrices_with_dimensions(
    seed: Seed,
    height: usize,
    width: usize,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomIntegerMatrices<Repeat<(usize, usize)>, RandomMatrixElements> {
    random_integer_matrices_with_dimensions_from_iterator(
        height,
        width,
        random_integers(seed, mean_bits_numerator, mean_bits_denominator),
    )
}

/// Generates random [`IntegerMatrix`]s with striped elements.
///
/// The elements are sampled from [`striped_random_integers`]: each element's bit count has mean
/// `mean_bits_numerator / mean_bits_denominator`, and its bits come in runs whose mean length is
/// `mean_stripe_numerator / mean_stripe_denominator`. A striped element is one whose bits come in
/// long runs, which is what makes the carries and borrows of an arithmetic test interesting.
///
/// The height and width are sampled independently from a geometric distribution with mean
/// `mean_dimension_numerator / mean_dimension_denominator`, so matrices with no rows or no columns
/// are generated with the probability that that distribution gives to 0.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(nb)$
///
/// $M(i) = O(nb)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $n$ is the number of
/// elements of the $i$th output, and $b$ is `mean_bits_numerator / mean_bits_denominator`.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if `mean_stripe_numerator <
/// mean_stripe_denominator`, if `mean_bits_numerator`, `mean_bits_denominator`,
/// `mean_dimension_numerator`, or `mean_dimension_denominator` are zero, or if either of the last
/// two ratios, after being reduced to lowest terms, has a numerator and denominator whose sum is
/// greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer_matrix::random::striped_random_integer_matrices;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_integer_matrices(EXAMPLE_SEED, 16, 1, 4, 1, 2, 1),
///         5
///     ),
///     "[[]_{0,1}, []_{0,11}, []_{2,0}, [0, 0; -1, 3; -1, -7; 0, 48], []_{6,0}, ...]"
/// );
/// ```
#[inline]
pub fn striped_random_integer_matrices(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_dimension_numerator: u64,
    mean_dimension_denominator: u64,
) -> RandomIntegerMatrices<RandomMatrixDimensions, StripedRandomMatrixElements> {
    random_integer_matrices_from_iterator(
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
        mean_dimension_numerator,
        mean_dimension_denominator,
    )
}

/// Generates random [`IntegerMatrix`]s with a given height and width, with striped elements.
///
/// The elements are striped, as they are in [`striped_random_integer_matrices`]. If `height` or
/// `width` is 0, the output consists of the matrix of that shape, which has no elements, repeated.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(nb)$
///
/// $M(i) = O(nb)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $n$ is `height *
/// width`, and $b$ is `mean_bits_numerator / mean_bits_denominator`.
///
/// # Panics
/// Panics if `height * width` does not fit in a `usize`, if `mean_stripe_denominator` is zero, if
/// `mean_stripe_numerator < mean_stripe_denominator`, if `mean_bits_numerator` or
/// `mean_bits_denominator` are zero, or if, after being reduced to lowest terms, their sum is
/// greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer_matrix::random::striped_random_integer_matrices_with_dimensions;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_integer_matrices_with_dimensions(EXAMPLE_SEED, 2, 3, 16, 1, 4, 1),
///         5
///     ),
///     "[[2, 128, 1; 0, -32, 4096], [0, 31, 6442451424; 127, -8, -6145], [-1, -7864320, 1024; 64, \
///     -1, -512], [-7, -32, -1024; -135, 3, -1], [-1, 4095, 512; 6, 127, 19], ...]"
/// );
/// ```
#[inline]
pub fn striped_random_integer_matrices_with_dimensions(
    seed: Seed,
    height: usize,
    width: usize,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomIntegerMatrices<Repeat<(usize, usize)>, StripedRandomMatrixElements> {
    random_integer_matrices_with_dimensions_from_iterator(
        height,
        width,
        striped_random_integers(
            seed,
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator,
        ),
    )
}
