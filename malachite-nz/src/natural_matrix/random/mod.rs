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
use crate::natural_matrix::{NaturalMatrix, element_count};
use core::iter::{Repeat, repeat};
use malachite_base::num::random::geometric::{
    GeometricRandomNaturalValues, geometric_random_unsigneds,
};
use malachite_base::random::Seed;
use malachite_base::tuples::random::{RandomPairsFromSingle, random_pairs_from_single};

/// Generates random [`NaturalMatrix`]s from an iterator of dimensions and an iterator of elements.
///
/// Each matrix takes its height and width from `dimensions` and then its elements, in row-major
/// order, from `xs`.
///
/// This `struct` is created by [`random_natural_matrices_from_iterator`],
/// [`random_natural_matrices_with_dimensions_from_iterator`], [`random_natural_matrices`],
/// [`random_natural_matrices_with_dimensions`], [`striped_random_natural_matrices`], and
/// [`striped_random_natural_matrices_with_dimensions`],
/// [`random_natural_square_matrices_from_iterator`], [`random_natural_square_matrices`], and
/// [`striped_random_natural_square_matrices`]; see their documentation for more.
#[derive(Clone, Debug)]
pub struct RandomNaturalMatrices<D: Iterator<Item = (usize, usize)>, I: Iterator<Item = Natural>> {
    dimensions: D,
    xs: I,
}

impl<D: Iterator<Item = (usize, usize)>, I: Iterator<Item = Natural>> Iterator
    for RandomNaturalMatrices<D, I>
{
    type Item = NaturalMatrix;

    fn next(&mut self) -> Option<NaturalMatrix> {
        let (height, width) = self.dimensions.next()?;
        Some(NaturalMatrix::from_owned_elements(
            height,
            width,
            (&mut self.xs).take(element_count(height, width)).collect(),
        ))
    }
}

/// The dimensions that the [`NaturalMatrix`] generators of random shape draw on.
pub type RandomMatrixDimensions = RandomPairsFromSingle<GeometricRandomNaturalValues<usize>>;

/// The elements that the unstriped [`NaturalMatrix`] generators draw on.
pub type RandomMatrixElements = RandomNaturals<GeometricRandomNaturalValues<u64>>;

/// The elements that the striped [`NaturalMatrix`] generators draw on.
pub type StripedRandomMatrixElements = StripedRandomNaturals<GeometricRandomNaturalValues<u64>>;

/// Generates random [`NaturalMatrix`]s whose elements come from an iterator.
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
/// The elements here are [`Natural`]s from 0 to 8.
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural::random::uniform_random_natural_inclusive_range;
/// use malachite_nz::natural_matrix::random::random_natural_matrices_from_iterator;
///
/// assert_eq!(
///     prefix_to_string(
///         random_natural_matrices_from_iterator(
///             EXAMPLE_SEED,
///             &|seed| uniform_random_natural_inclusive_range(
///                 seed,
///                 Natural::ZERO,
///                 Natural::from(8u32)
///             ),
///             2,
///             1
///         ),
///         5
///     ),
///     "[[]_{0,1}, []_{0,11}, []_{2,0}, [4, 0; 5, 5; 1, 8; 4, 4], []_{6,0}, ...]"
/// );
/// ```
#[inline]
pub fn random_natural_matrices_from_iterator<I: Iterator<Item = Natural>>(
    seed: Seed,
    xs_gen: &dyn Fn(Seed) -> I,
    mean_dimension_numerator: u64,
    mean_dimension_denominator: u64,
) -> RandomNaturalMatrices<RandomMatrixDimensions, I> {
    RandomNaturalMatrices {
        dimensions: random_pairs_from_single(geometric_random_unsigneds(
            seed.fork("dimensions"),
            mean_dimension_numerator,
            mean_dimension_denominator,
        )),
        xs: xs_gen(seed.fork("elements")),
    }
}

/// Generates random [`NaturalMatrix`]s with a given height and width whose elements come from an
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
/// The elements here are [`Natural`]s from 0 to 8.
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural::random::uniform_random_natural_inclusive_range;
/// use malachite_nz::natural_matrix::random::random_natural_matrices_with_dimensions_from_iterator;
///
/// assert_eq!(
///     prefix_to_string(
///         random_natural_matrices_with_dimensions_from_iterator(
///             2,
///             2,
///             uniform_random_natural_inclusive_range(
///                 EXAMPLE_SEED,
///                 Natural::ZERO,
///                 Natural::from(8u32)
///             )
///         ),
///         5
///     ),
///     "[[1, 7; 5, 7], [2, 8; 2, 4], [6, 6; 1, 3], [7, 8; 1, 6], [6, 6; 7, 7], ...]"
/// );
/// ```
#[inline]
pub fn random_natural_matrices_with_dimensions_from_iterator<I: Iterator<Item = Natural>>(
    height: usize,
    width: usize,
    xs: I,
) -> RandomNaturalMatrices<Repeat<(usize, usize)>, I> {
    element_count(height, width);
    RandomNaturalMatrices {
        dimensions: repeat((height, width)),
        xs,
    }
}

/// Generates random [`NaturalMatrix`]s.
///
/// The elements are sampled from [`random_naturals`], with a mean bit count of `mean_bits_numerator
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
/// use malachite_nz::natural_matrix::random::random_natural_matrices;
///
/// assert_eq!(
///     prefix_to_string(random_natural_matrices(EXAMPLE_SEED, 4, 1, 2, 1), 5),
///     "[[]_{0,1}, []_{0,11}, []_{2,0}, [2, 7229; 18, 7; 0, 3; 0, 1], []_{6,0}, ...]"
/// );
/// ```
#[inline]
pub fn random_natural_matrices(
    seed: Seed,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_dimension_numerator: u64,
    mean_dimension_denominator: u64,
) -> RandomNaturalMatrices<RandomMatrixDimensions, RandomMatrixElements> {
    random_natural_matrices_from_iterator(
        seed,
        &|seed_2| random_naturals(seed_2, mean_bits_numerator, mean_bits_denominator),
        mean_dimension_numerator,
        mean_dimension_denominator,
    )
}

/// Generates random [`NaturalMatrix`]s with a given height and width.
///
/// The elements are sampled from [`random_naturals`], with a mean bit count of `mean_bits_numerator
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
/// use malachite_nz::natural_matrix::random::random_natural_matrices_with_dimensions;
///
/// assert_eq!(
///     prefix_to_string(
///         random_natural_matrices_with_dimensions(EXAMPLE_SEED, 2, 3, 4, 1),
///         5
///     ),
///     "[[14, 8, 10; 1, 1, 0], [1, 184, 15; 99, 1, 3], [3, 6, 1; 4, 494, 3], [13, 1, 1; 1757, \
///     1775, 3514], [2, 1799, 0; 5, 0, 7], ...]"
/// );
/// ```
#[inline]
pub fn random_natural_matrices_with_dimensions(
    seed: Seed,
    height: usize,
    width: usize,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomNaturalMatrices<Repeat<(usize, usize)>, RandomMatrixElements> {
    random_natural_matrices_with_dimensions_from_iterator(
        height,
        width,
        random_naturals(seed, mean_bits_numerator, mean_bits_denominator),
    )
}

/// Generates random [`NaturalMatrix`]s with striped elements.
///
/// The elements are sampled from [`striped_random_naturals`]: each element's bit count has mean
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
/// use malachite_nz::natural_matrix::random::striped_random_natural_matrices;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_natural_matrices(EXAMPLE_SEED, 16, 1, 4, 1, 2, 1),
///         5
///     ),
///     "[[]_{0,1}, []_{0,11}, []_{2,0}, [3, 7999; 24, 7; 0, 2; 0, 1], []_{6,0}, ...]"
/// );
/// ```
#[inline]
pub fn striped_random_natural_matrices(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_dimension_numerator: u64,
    mean_dimension_denominator: u64,
) -> RandomNaturalMatrices<RandomMatrixDimensions, StripedRandomMatrixElements> {
    random_natural_matrices_from_iterator(
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
        mean_dimension_numerator,
        mean_dimension_denominator,
    )
}

/// Generates random [`NaturalMatrix`]s with a given height and width, with striped elements.
///
/// The elements are striped, as they are in [`striped_random_natural_matrices`]. If `height` or
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
/// use malachite_nz::natural_matrix::random::striped_random_natural_matrices_with_dimensions;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_natural_matrices_with_dimensions(EXAMPLE_SEED, 2, 3, 16, 1, 4, 1),
///         5
///     ),
///     "[[8, 8, 8; 1, 1, 0], [1, 128, 15; 64, 1, 3], [2, 4, 1; 7, 376, 3], [8, 1, 1; 2047, 1920, \
///     3840], [3, 1984, 0; 4, 0, 7], ...]"
/// );
/// ```
#[inline]
pub fn striped_random_natural_matrices_with_dimensions(
    seed: Seed,
    height: usize,
    width: usize,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomNaturalMatrices<Repeat<(usize, usize)>, StripedRandomMatrixElements> {
    random_natural_matrices_with_dimensions_from_iterator(
        height,
        width,
        striped_random_naturals(
            seed,
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator,
        ),
    )
}

/// Generates the dimensions of random square matrices: each output is $(n, n)$, with $n$ sampled
/// from a geometric distribution.
///
/// This `struct` is used by [`random_natural_square_matrices_from_iterator`],
/// [`random_natural_square_matrices`], and [`striped_random_natural_square_matrices`]; see their
/// documentation for more.
#[derive(Clone, Debug)]
pub struct RandomSquareMatrixDimensions(GeometricRandomNaturalValues<usize>);

impl Iterator for RandomSquareMatrixDimensions {
    type Item = (usize, usize);

    #[inline]
    fn next(&mut self) -> Option<(usize, usize)> {
        self.0.next().map(|size| (size, size))
    }
}

/// Generates random square [`NaturalMatrix`]s whose elements come from an iterator.
///
/// `xs_gen` is given a seed derived from `seed` and produces the iterator of elements; each matrix
/// takes its elements, in row-major order, from that iterator. This allows any distribution of
/// elements, for example the residues modulo some $m$. The size is sampled from a geometric
/// distribution with mean `mean_size_numerator / mean_size_denominator`, so the 0-by-0 matrix is
/// generated with the probability that that distribution gives to 0.
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
/// Panics if `mean_size_numerator` or `mean_size_denominator` are zero, or if, after being reduced
/// to lowest terms, their sum is greater than or equal to $2^{64}$.
///
/// # Examples
/// The elements here are [`Natural`]s from 0 to 8.
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural::random::uniform_random_natural_inclusive_range;
/// use malachite_nz::natural_matrix::random::random_natural_square_matrices_from_iterator;
///
/// assert_eq!(
///     prefix_to_string(
///         random_natural_square_matrices_from_iterator(
///             EXAMPLE_SEED,
///             &|seed| uniform_random_natural_inclusive_range(
///                 seed,
///                 Natural::ZERO,
///                 Natural::from(8u32)
///             ),
///             3,
///             2
///         ),
///         5
///     ),
///     "[[]_{0,0}, []_{0,0}, [4, 0, 5, 5, 1, 8; 4, 4, 7, 2, 7, 4; 7, 1, 7, 0, 8, 7; 0, 6, 2, 7, \
///     2, 0; 6, 3, 1, 6, 2, 8; 3, 3, 5, 2, 4, 3], []_{0,0}, [3, 7; 3, 7], ...]"
/// );
/// ```
#[inline]
pub fn random_natural_square_matrices_from_iterator<I: Iterator<Item = Natural>>(
    seed: Seed,
    xs_gen: &dyn Fn(Seed) -> I,
    mean_size_numerator: u64,
    mean_size_denominator: u64,
) -> RandomNaturalMatrices<RandomSquareMatrixDimensions, I> {
    RandomNaturalMatrices {
        dimensions: RandomSquareMatrixDimensions(geometric_random_unsigneds(
            seed.fork("dimensions"),
            mean_size_numerator,
            mean_size_denominator,
        )),
        xs: xs_gen(seed.fork("elements")),
    }
}

/// Generates random square [`NaturalMatrix`]s.
///
/// The elements are sampled from [`random_naturals`], with a mean bit count of `mean_bits_numerator
/// / mean_bits_denominator`. The size is sampled from a geometric distribution with mean
/// `mean_size_numerator / mean_size_denominator`, so the 0-by-0 matrix is generated with the
/// probability that that distribution gives to 0.
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
/// Panics if `mean_bits_numerator` or `mean_bits_denominator` are zero, if `mean_size_numerator` or
/// `mean_size_denominator` are zero, or if either ratio, after being reduced to lowest terms, has a
/// numerator and denominator whose sum is greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::natural_matrix::random::random_natural_square_matrices;
///
/// assert_eq!(
///     prefix_to_string(random_natural_square_matrices(EXAMPLE_SEED, 4, 1, 3, 2), 5),
///     "[[]_{0,0}, []_{0,0}, [2, 7229, 18, 7, 0, 3; 0, 1, 1, 0, 0, 6; 4, 0, 129427, 1, 0, 1; 2, \
///     3, 30, 5, 259, 0; 104, 1, 6, 0, 1, 0; 137, 1556, 6, 740629, 5, 14], []_{0,0}, [4, 15208; \
///     5, 69], ...]"
/// );
/// ```
#[inline]
pub fn random_natural_square_matrices(
    seed: Seed,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_size_numerator: u64,
    mean_size_denominator: u64,
) -> RandomNaturalMatrices<RandomSquareMatrixDimensions, RandomMatrixElements> {
    random_natural_square_matrices_from_iterator(
        seed,
        &|seed_2| random_naturals(seed_2, mean_bits_numerator, mean_bits_denominator),
        mean_size_numerator,
        mean_size_denominator,
    )
}

/// Generates random square [`NaturalMatrix`]s with striped elements.
///
/// The elements are striped, as they are in [`striped_random_natural_matrices`]. The size is
/// sampled from a geometric distribution with mean `mean_size_numerator / mean_size_denominator`,
/// so the 0-by-0 matrix is generated with the probability that that distribution gives to 0.
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
/// `mean_size_numerator`, or `mean_size_denominator` are zero, or if either of the last two ratios,
/// after being reduced to lowest terms, has a numerator and denominator whose sum is greater than
/// or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::natural_matrix::random::striped_random_natural_square_matrices;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_natural_square_matrices(EXAMPLE_SEED, 16, 1, 4, 1, 3, 2),
///         5
///     ),
///     "[[]_{0,0}, []_{0,0}, [3, 7999, 24, 7, 0, 2; 0, 1, 1, 0, 0, 4; 4, 0, 65536, 1, 0, 1; 3, 3, \
///     31, 4, 256, 0; 127, 1, 7, 0, 1, 0; 128, 1024, 4, 524296, 5, 15], []_{0,0}, [4, 8192; 4, \
///     64], ...]"
/// );
/// ```
#[inline]
pub fn striped_random_natural_square_matrices(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_size_numerator: u64,
    mean_size_denominator: u64,
) -> RandomNaturalMatrices<RandomSquareMatrixDimensions, StripedRandomMatrixElements> {
    random_natural_square_matrices_from_iterator(
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
        mean_size_numerator,
        mean_size_denominator,
    )
}
