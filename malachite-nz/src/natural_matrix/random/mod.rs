// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural::random::{
    RandomNaturals, StripedRandomNaturals, get_random_natural_less_than, random_naturals,
    striped_random_naturals,
};
use crate::natural_matrix::{NaturalMatrix, shape};
use alloc::vec::Vec;
use core::cmp::min;
use core::iter::{Repeat, repeat};
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModAddAssign, ModIsReduced, ModSubAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::random::geometric::{
    GeometricRandomNaturalValues, geometric_random_unsigneds,
};
use malachite_base::num::random::{
    RandomPrimitiveInts, VariableRangeGenerator, random_permutation_with_parity,
    random_primitive_ints,
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
pub struct RandomNaturalMatrices<D: Iterator<Item = (u64, u64)>, I: Iterator<Item = Natural>> {
    dimensions: D,
    xs: I,
}

impl<D: Iterator<Item = (u64, u64)>, I: Iterator<Item = Natural>> Iterator
    for RandomNaturalMatrices<D, I>
{
    type Item = NaturalMatrix;

    fn next(&mut self) -> Option<NaturalMatrix> {
        let (height, width) = self.dimensions.next()?;
        Some(NaturalMatrix::from_owned_elements(
            height,
            width,
            (&mut self.xs).take(shape(height, width).2).collect(),
        ))
    }
}

/// The dimensions that the [`NaturalMatrix`] generators of random shape draw on.
pub type RandomMatrixDimensions = RandomPairsFromSingle<GeometricRandomNaturalValues<u64>>;

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
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_base::random::EXAMPLE_SEED;
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
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_base::random::EXAMPLE_SEED;
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
    height: u64,
    width: u64,
    xs: I,
) -> RandomNaturalMatrices<Repeat<(u64, u64)>, I> {
    shape(height, width);
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
    height: u64,
    width: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomNaturalMatrices<Repeat<(u64, u64)>, RandomMatrixElements> {
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
    height: u64,
    width: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomNaturalMatrices<Repeat<(u64, u64)>, StripedRandomMatrixElements> {
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
pub struct RandomSquareMatrixDimensions(GeometricRandomNaturalValues<u64>);

impl Iterator for RandomSquareMatrixDimensions {
    type Item = (u64, u64);

    #[inline]
    fn next(&mut self) -> Option<(u64, u64)> {
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

// Places `diagonal` at random positions in a zero matrix of the given shape: element `i` goes to
// row `rows[i]` and column `columns[i]`, for uniformly random permutations `rows` of the rows and
// `columns` of the columns. Returns the matrix and whether the two permutations together are odd.
fn place_permuted_diagonal(
    ranges: &mut VariableRangeGenerator,
    height: u64,
    width: u64,
    diagonal: Vec<Natural>,
) -> (NaturalMatrix, bool) {
    let (h, w, _) = shape(height, width);
    let (rows, rows_odd) = random_permutation_with_parity(ranges, h);
    let (columns, columns_odd) = random_permutation_with_parity(ranges, w);
    let mut m = NaturalMatrix::zero(height, width);
    for (i, x) in diagonal.into_iter().enumerate() {
        m[(rows[i], columns[i])] = x;
    }
    (m, rows_odd != columns_odd)
}

/// Generates random [`NaturalMatrix`]s with a given diagonal, placed at random positions.
///
/// This `struct` is created by [`random_natural_matrices_with_permuted_diagonal`]; see its
/// documentation for more.
#[derive(Clone, Debug)]
pub struct RandomNaturalMatricesWithPermutedDiagonal {
    ranges: VariableRangeGenerator,
    height: u64,
    width: u64,
    diagonal: Vec<Natural>,
}

impl Iterator for RandomNaturalMatricesWithPermutedDiagonal {
    type Item = (NaturalMatrix, bool);

    fn next(&mut self) -> Option<(NaturalMatrix, bool)> {
        Some(place_permuted_diagonal(
            &mut self.ranges,
            self.height,
            self.width,
            self.diagonal.clone(),
        ))
    }
}

/// Generates random [`NaturalMatrix`]s of a given shape whose nonzero elements are a given
/// diagonal, with its rows and columns permuted at random, together with the parity of the
/// permutation.
///
/// Each output starts as the zero matrix with `diagonal[i]` in row $i$ and column $i$, and its rows
/// and columns are then permuted by independent uniformly random permutations. The `bool` is `true`
/// when the two permutations together are odd. For a square matrix whose diagonal has an element
/// for every row, the determinant is then the product of the diagonal elements, negated if the
/// `bool` is `true`.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `height * width`.
///
/// # Panics
/// Panics if `diagonal` has more elements than the smaller of `height` and `width`, or if the
/// number of elements does not fit in a [`usize`].
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural_matrix::random::random_natural_matrices_with_permuted_diagonal;
///
/// assert_eq!(
///     prefix_to_string(
///         random_natural_matrices_with_permuted_diagonal(
///             EXAMPLE_SEED,
///             3,
///             4,
///             vec![Natural::from(2u32), Natural::from(3u32), Natural::from(5u32)]
///         )
///         .map(|(m, odd)| format!("({m}, {odd})")),
///         3
///     ),
///     "[([0, 3, 0, 0; 0, 0, 0, 5; 2, 0, 0, 0], true), ([0, 2, 0, 0; 3, 0, 0, 0; 0, 0, 5, 0], \
///     true), ([0, 0, 5, 0; 0, 3, 0, 0; 0, 0, 0, 2], false), ...]"
/// );
/// ```
pub fn random_natural_matrices_with_permuted_diagonal(
    seed: Seed,
    height: u64,
    width: u64,
    diagonal: Vec<Natural>,
) -> RandomNaturalMatricesWithPermutedDiagonal {
    assert!(
        u64::exact_from(diagonal.len()) <= min(height, width),
        "a diagonal of {} elements does not fit in a {height}-by-{width} matrix",
        diagonal.len()
    );
    shape(height, width);
    RandomNaturalMatricesWithPermutedDiagonal {
        ranges: VariableRangeGenerator::new(seed),
        height,
        width,
        diagonal,
    }
}

/// Generates random [`NaturalMatrix`]s of a given rank modulo a number.
///
/// This `struct` is created by [`random_natural_matrices_with_rank_mod`]; see its documentation for
/// more.
#[derive(Clone, Debug)]
pub struct RandomNaturalMatricesWithRankMod {
    ranges: VariableRangeGenerator,
    limbs: RandomPrimitiveInts<u64>,
    height: u64,
    width: u64,
    rank: u64,
    m_minus_1: Natural,
}

impl Iterator for RandomNaturalMatricesWithRankMod {
    type Item = NaturalMatrix;

    fn next(&mut self) -> Option<NaturalMatrix> {
        let diagonal = (0..self.rank)
            .map(|_| get_random_natural_less_than(&mut self.limbs, &self.m_minus_1) + Natural::ONE)
            .collect();
        Some(place_permuted_diagonal(&mut self.ranges, self.height, self.width, diagonal).0)
    }
}

/// Generates random sparse [`NaturalMatrix`]s of a given shape whose elements are reduced modulo
/// `m`, with a given number of nonzero elements, no two in the same row or column.
///
/// Each nonzero element is uniformly distributed on $[1, m)$, and they are placed as
/// [`random_natural_matrices_with_permuted_diagonal`] places a diagonal. When $m$ is prime, the
/// rank of every output is `rank`. To turn the outputs into dense matrices of the same rank, pass
/// them to [`random_natural_matrices_from_elementary_operations_mod`].
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `height * width`.
///
/// # Panics
/// Panics if `rank` is greater than the smaller of `height` and `width`, if `m` is zero, if `rank`
/// is positive and `m` is 1, since then there are no nonzero residues, or if the number of elements
/// does not fit in a [`usize`].
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural_matrix::random::random_natural_matrices_with_rank_mod;
///
/// assert_eq!(
///     prefix_to_string(
///         random_natural_matrices_with_rank_mod(EXAMPLE_SEED, 3, 4, 2, Natural::from(7u32)),
///         3
///     ),
///     "[[0, 0, 0, 0; 3, 0, 0, 0; 0, 0, 0, 1], [3, 0, 0, 0; 0, 0, 0, 0; 0, 0, 5, 0], [0, 0, 0, \
///     0; 0, 4, 0, 0; 0, 0, 0, 1], ...]"
/// );
/// ```
///
/// This is equivalent to `fmpz_mod_mat_randrank` from `fmpz_mod_mat/randrank.c`, FLINT 3.6.0,
/// except that the modulus 1 is rejected for a positive rank rather than giving the zero matrix.
pub fn random_natural_matrices_with_rank_mod(
    seed: Seed,
    height: u64,
    width: u64,
    rank: u64,
    m: Natural,
) -> RandomNaturalMatricesWithRankMod {
    assert!(
        rank <= min(height, width),
        "a {height}-by-{width} matrix cannot have rank {rank}"
    );
    assert!(!(m == 0u32), "the modulus must be positive");
    assert!(
        rank == 0 || !(m <= 1u32),
        "there are no nonzero residues modulo {m}, so the rank must be 0"
    );
    shape(height, width);
    RandomNaturalMatricesWithRankMod {
        ranges: VariableRangeGenerator::new(seed.fork("ranges")),
        limbs: random_primitive_ints(seed.fork("limbs")),
        height,
        width,
        rank,
        m_minus_1: m - Natural::ONE,
    }
}

/// Generates random triangular [`NaturalMatrix`]s modulo a number.
///
/// This `struct` is created by [`random_natural_lower_triangular_matrices_mod`] and
/// [`random_natural_upper_triangular_matrices_mod`]; see their documentation for more.
#[derive(Clone, Debug)]
pub struct RandomNaturalTriangularMatricesMod {
    limbs: RandomPrimitiveInts<u64>,
    height: u64,
    width: u64,
    unit: bool,
    lower: bool,
    m: Natural,
    m_minus_1: Natural,
}

impl Iterator for RandomNaturalTriangularMatricesMod {
    type Item = NaturalMatrix;

    fn next(&mut self) -> Option<NaturalMatrix> {
        let mut elements = Vec::with_capacity(shape(self.height, self.width).2);
        for i in 0..self.height {
            for j in 0..self.width {
                elements.push(if i == j {
                    if self.unit {
                        Natural::ONE
                    } else {
                        get_random_natural_less_than(&mut self.limbs, &self.m_minus_1)
                            + Natural::ONE
                    }
                } else if (j < i) == self.lower {
                    get_random_natural_less_than(&mut self.limbs, &self.m)
                } else {
                    Natural::ZERO
                });
            }
        }
        Some(NaturalMatrix::from_owned_elements(
            self.height,
            self.width,
            elements,
        ))
    }
}

/// Generates random lower triangular [`NaturalMatrix`]s of a given shape whose elements are reduced
/// modulo `m`.
///
/// The elements below the diagonal are uniformly distributed on $[0, m)$, and those above it are 0.
/// If `unit` is `true`, every diagonal element is 1; otherwise each is uniformly distributed on
/// $[1, m)$, so it is nonzero. The matrix need not be square.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `height * width`.
///
/// # Panics
/// Panics if `m` is zero, if `m` is 1 and the matrix has a diagonal element, since then neither 1
/// nor a nonzero residue is reduced, or if the number of elements does not fit in a [`usize`].
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural_matrix::random::random_natural_lower_triangular_matrices_mod;
///
/// assert_eq!(
///     prefix_to_string(
///         random_natural_lower_triangular_matrices_mod(
///             EXAMPLE_SEED,
///             3,
///             3,
///             false,
///             Natural::from(7u32)
///         ),
///         3
///     ),
///     "[[2, 0, 0; 5, 6, 0; 1, 2, 1], [3, 0, 0; 3, 5, 0; 3, 6, 6], [4, 0, 0; 1, 4, 0; 5, 1, 1], \
///     ...]"
/// );
/// ```
///
/// This is equivalent to `fmpz_mod_mat_randtril` from `fmpz_mod_mat/randtril.c`, FLINT 3.6.0,
/// except that a random diagonal element is uniform on $[1, m)$, where FLINT replaces a 0 by 1, and
/// that the modulus 1 is rejected.
pub fn random_natural_lower_triangular_matrices_mod(
    seed: Seed,
    height: u64,
    width: u64,
    unit: bool,
    m: Natural,
) -> RandomNaturalTriangularMatricesMod {
    assert!(!(m == 0u32), "the modulus must be positive");
    assert!(
        min(height, width) == 0 || !(m <= 1u32),
        "no diagonal element can be reduced modulo {m}"
    );
    shape(height, width);
    RandomNaturalTriangularMatricesMod {
        limbs: random_primitive_ints(seed),
        height,
        width,
        unit,
        lower: true,
        m_minus_1: &m - Natural::ONE,
        m,
    }
}

/// Generates random upper triangular [`NaturalMatrix`]s of a given shape whose elements are reduced
/// modulo `m`.
///
/// The elements above the diagonal are uniformly distributed on $[0, m)$, and those below it are 0.
/// If `unit` is `true`, every diagonal element is 1; otherwise each is uniformly distributed on
/// $[1, m)$, so it is nonzero. The matrix need not be square.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `height * width`.
///
/// # Panics
/// Panics if `m` is zero, if `m` is 1 and the matrix has a diagonal element, since then neither 1
/// nor a nonzero residue is reduced, or if the number of elements does not fit in a [`usize`].
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural_matrix::random::random_natural_upper_triangular_matrices_mod;
///
/// assert_eq!(
///     prefix_to_string(
///         random_natural_upper_triangular_matrices_mod(
///             EXAMPLE_SEED,
///             3,
///             3,
///             false,
///             Natural::from(7u32)
///         ),
///         3
///     ),
///     "[[2, 5, 5; 0, 2, 2; 0, 0, 1], [3, 3, 4; 0, 4, 6; 0, 0, 6], [4, 1, 3; 0, 6, 1; 0, 0, 1], \
///     ...]"
/// );
/// ```
///
/// This is equivalent to `fmpz_mod_mat_randtriu` from `fmpz_mod_mat/randtriu.c`, FLINT 3.6.0,
/// except that a random diagonal element is uniform on $[1, m)$, where FLINT replaces a 0 by 1, and
/// that the modulus 1 is rejected.
pub fn random_natural_upper_triangular_matrices_mod(
    seed: Seed,
    height: u64,
    width: u64,
    unit: bool,
    m: Natural,
) -> RandomNaturalTriangularMatricesMod {
    assert!(!(m == 0u32), "the modulus must be positive");
    assert!(
        min(height, width) == 0 || !(m <= 1u32),
        "no diagonal element can be reduced modulo {m}"
    );
    shape(height, width);
    RandomNaturalTriangularMatricesMod {
        limbs: random_primitive_ints(seed),
        height,
        width,
        unit,
        lower: false,
        m_minus_1: &m - Natural::ONE,
        m,
    }
}

/// Applies random elementary row and column operations modulo a number to [`NaturalMatrix`]s.
///
/// This `struct` is created by [`random_natural_matrices_from_elementary_operations_mod`]; see its
/// documentation for more.
#[derive(Clone, Debug)]
pub struct RandomNaturalMatricesFromElementaryOperationsMod<I: Iterator<Item = NaturalMatrix>> {
    ranges: VariableRangeGenerator,
    xs: I,
    count: u64,
    m: Natural,
}

impl<I: Iterator<Item = NaturalMatrix>> Iterator
    for RandomNaturalMatricesFromElementaryOperationsMod<I>
{
    type Item = NaturalMatrix;

    fn next(&mut self) -> Option<NaturalMatrix> {
        let mut m = self.xs.next()?;
        assert!(
            m.mod_is_reduced(&self.m),
            "every element of the matrix must be reduced modulo {}",
            self.m
        );
        let (height, width) = (m.height(), m.width());
        if height == 0 || width == 0 {
            return Some(m);
        }
        for _ in 0..self.count {
            if self.ranges.next_bool() {
                let (i, j) = (
                    self.ranges.next_less_than(height),
                    self.ranges.next_less_than(height),
                );
                if i == j {
                    continue;
                }
                let row_i: Vec<Natural> = m.row_iter(i).cloned().collect();
                if self.ranges.next_bool() {
                    for (y, x) in m.row_iter_mut(j).zip(row_i) {
                        y.mod_add_assign(x, &self.m);
                    }
                } else {
                    for (y, x) in m.row_iter_mut(j).zip(row_i) {
                        y.mod_sub_assign(x, &self.m);
                    }
                }
            } else {
                let (i, j) = (
                    self.ranges.next_less_than(width),
                    self.ranges.next_less_than(width),
                );
                if i == j {
                    continue;
                }
                let column_i: Vec<Natural> = m.column_iter(i).cloned().collect();
                if self.ranges.next_bool() {
                    for (y, x) in m.column_iter_mut(j).zip(column_i) {
                        y.mod_add_assign(x, &self.m);
                    }
                } else {
                    for (y, x) in m.column_iter_mut(j).zip(column_i) {
                        y.mod_sub_assign(x, &self.m);
                    }
                }
            }
        }
        Some(m)
    }
}

/// Applies `count` random elementary row and column operations modulo `m` to each [`NaturalMatrix`]
/// that an iterator generates.
///
/// Each operation chooses, with equal probability, rows or columns, then two of them, $i$ and $j$,
/// uniformly and independently; if they are distinct, it adds row (or column) $i$ to row (or
/// column) $j$ modulo `m`, or subtracts it, with equal probability. An operation that picks the
/// same row or column twice is skipped, so at most `count` operations are applied. Such operations
/// preserve the rank modulo a prime, and for a square matrix the determinant, so this turns the
/// sparse outputs of [`random_natural_matrices_with_rank_mod`] into dense matrices with the same
/// rank. Matrices with no elements are passed through unchanged.
///
/// The output has the same length as `xs`.
///
/// # Worst-case complexity per iteration
/// $T(n, c) = O(n + cl)$
///
/// $M(n, c) = O(n + cl)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the $i$th output,
/// $c$ is `count`, and $l$ is the larger of its height and width.
///
/// # Panics
/// Panics if `m` is zero, and, when an output is generated, if any element of the matrix that `xs`
/// generated is greater than or equal to `m`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural_matrix::random::{
///     random_natural_matrices_from_elementary_operations_mod,
///     random_natural_matrices_with_rank_mod,
/// };
///
/// assert_eq!(
///     prefix_to_string(
///         random_natural_matrices_from_elementary_operations_mod(
///             EXAMPLE_SEED,
///             random_natural_matrices_with_rank_mod(
///                 EXAMPLE_SEED.fork("rank"),
///                 3,
///                 3,
///                 2,
///                 Natural::from(7u32)
///             ),
///             10,
///             Natural::from(7u32)
///         ),
///         3
///     ),
///     "[[0, 0, 0; 4, 0, 3; 5, 5, 4], [0, 0, 0; 5, 5, 4; 0, 0, 3], [0, 4, 6; 0, 0, 6; 0, 6, 6], \
///     ...]"
/// );
/// ```
///
/// This is equivalent to `fmpz_mod_mat_randops` from `fmpz_mod_mat/randops.c`, FLINT 3.6.0.
pub fn random_natural_matrices_from_elementary_operations_mod<I: Iterator<Item = NaturalMatrix>>(
    seed: Seed,
    xs: I,
    count: u64,
    m: Natural,
) -> RandomNaturalMatricesFromElementaryOperationsMod<I> {
    assert!(!(m == 0u32), "the modulus must be positive");
    RandomNaturalMatricesFromElementaryOperationsMod {
        ranges: VariableRangeGenerator::new(seed),
        xs,
        count,
        m,
    }
}
