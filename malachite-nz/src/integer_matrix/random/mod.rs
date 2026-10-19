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
use crate::integer_matrix::{IntegerMatrix, shape};
use crate::natural::random::get_random_natural_with_bits;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::min;
use core::iter::{Repeat, repeat};
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::factorization::traits::Factor;
use malachite_base::num::random::geometric::{
    GeometricRandomNaturalValues, GeometricRandomSigneds, geometric_random_unsigneds,
};
use malachite_base::num::random::{
    RandomPrimitiveInts, VariableRangeGenerator, random_permutation_with_parity,
    random_primitive_ints,
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
/// [`striped_random_integer_matrices_with_dimensions`],
/// [`random_integer_square_matrices_from_iterator`], [`random_integer_square_matrices`], and
/// [`striped_random_integer_square_matrices`]; see their documentation for more.
#[derive(Clone, Debug)]
pub struct RandomIntegerMatrices<D: Iterator<Item = (u64, u64)>, I: Iterator<Item = Integer>> {
    dimensions: D,
    xs: I,
}

impl<D: Iterator<Item = (u64, u64)>, I: Iterator<Item = Integer>> Iterator
    for RandomIntegerMatrices<D, I>
{
    type Item = IntegerMatrix;

    fn next(&mut self) -> Option<IntegerMatrix> {
        let (height, width) = self.dimensions.next()?;
        Some(IntegerMatrix::from_owned_elements(
            height,
            width,
            (&mut self.xs).take(shape(height, width).2).collect(),
        ))
    }
}

/// The dimensions that the [`IntegerMatrix`] generators of random shape draw on.
pub type RandomMatrixDimensions = RandomPairsFromSingle<GeometricRandomNaturalValues<u64>>;

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
    height: u64,
    width: u64,
    xs: I,
) -> RandomIntegerMatrices<Repeat<(u64, u64)>, I> {
    shape(height, width);
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
    height: u64,
    width: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomIntegerMatrices<Repeat<(u64, u64)>, RandomMatrixElements> {
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
    height: u64,
    width: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomIntegerMatrices<Repeat<(u64, u64)>, StripedRandomMatrixElements> {
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

/// Generates the dimensions of random square matrices: each output is $(n, n)$, with $n$ sampled
/// from a geometric distribution.
///
/// This `struct` is used by [`random_integer_square_matrices_from_iterator`],
/// [`random_integer_square_matrices`], and [`striped_random_integer_square_matrices`]; see their
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

/// Generates random square [`IntegerMatrix`]s whose elements come from an iterator.
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
/// The elements here are [`Integer`]s from $-8$ to $8$.
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer::random::uniform_random_integer_inclusive_range;
/// use malachite_nz::integer_matrix::random::random_integer_square_matrices_from_iterator;
///
/// assert_eq!(
///     prefix_to_string(
///         random_integer_square_matrices_from_iterator(
///             EXAMPLE_SEED,
///             &|seed| uniform_random_integer_inclusive_range(
///                 seed,
///                 Integer::from(-8),
///                 Integer::from(8)
///             ),
///             3,
///             2
///         ),
///         5
///     ),
///     "[[]_{0,0}, []_{0,0}, [-4, 3, -8, 3, 5, 1; 5, 7, -7, 6, 4, 2; -6, 5, -4, -1, 2, 1; -1, 3, \
///     -8, 0, -1, 8; 1, 8, -2, 2, -5, -7; 7, -6, 1, 0, 1, 6], []_{0,0}, [-4, -5; 1, 1], ...]"
/// );
/// ```
#[inline]
pub fn random_integer_square_matrices_from_iterator<I: Iterator<Item = Integer>>(
    seed: Seed,
    xs_gen: &dyn Fn(Seed) -> I,
    mean_size_numerator: u64,
    mean_size_denominator: u64,
) -> RandomIntegerMatrices<RandomSquareMatrixDimensions, I> {
    RandomIntegerMatrices {
        dimensions: RandomSquareMatrixDimensions(geometric_random_unsigneds(
            seed.fork("dimensions"),
            mean_size_numerator,
            mean_size_denominator,
        )),
        xs: xs_gen(seed.fork("elements")),
    }
}

/// Generates random square [`IntegerMatrix`]s.
///
/// The elements are sampled from [`random_integers`], with a mean bit count of `mean_bits_numerator
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
/// use malachite_nz::integer_matrix::random::random_integer_square_matrices;
///
/// assert_eq!(
///     prefix_to_string(random_integer_square_matrices(EXAMPLE_SEED, 4, 1, 3, 2), 5),
///     "[[]_{0,0}, []_{0,0}, [0, 0, -1, 3, -1, -7; 0, 35, 1955, 2354, -1, -34592; 915, -4, -1, \
///     -26, -9, 2; 3, -131, 680, -4, 1, -1; 41, -12, -3598, 3, -3, 0; 0, 30, -4, -40, 1, 0], \
///     []_{0,0}, [3, 743; 60, -1], ...]"
/// );
/// ```
#[inline]
pub fn random_integer_square_matrices(
    seed: Seed,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_size_numerator: u64,
    mean_size_denominator: u64,
) -> RandomIntegerMatrices<RandomSquareMatrixDimensions, RandomMatrixElements> {
    random_integer_square_matrices_from_iterator(
        seed,
        &|seed_2| random_integers(seed_2, mean_bits_numerator, mean_bits_denominator),
        mean_size_numerator,
        mean_size_denominator,
    )
}

/// Generates random square [`IntegerMatrix`]s with striped elements.
///
/// The elements are striped, as they are in [`striped_random_integer_matrices`]. The size is
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
/// use malachite_nz::integer_matrix::random::striped_random_integer_square_matrices;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_integer_square_matrices(EXAMPLE_SEED, 16, 1, 4, 1, 3, 2),
///         5
///     ),
///     "[[]_{0,0}, []_{0,0}, [0, 0, -1, 3, -1, -7; 0, 48, 1793, 4095, -1, -61440; 512, -7, -1, \
///     -31, -15, 3; 2, -128, 1023, -4, 1, -1; 46, -8, -2048, 2, -3, 0; 0, 31, -4, -32, 1, 0], \
///     []_{0,0}, [2, 512; 32, -1], ...]"
/// );
/// ```
#[inline]
pub fn striped_random_integer_square_matrices(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_size_numerator: u64,
    mean_size_denominator: u64,
) -> RandomIntegerMatrices<RandomSquareMatrixDimensions, StripedRandomMatrixElements> {
    random_integer_square_matrices_from_iterator(
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
    diagonal: Vec<Integer>,
) -> (IntegerMatrix, bool) {
    let (h, w, _) = shape(height, width);
    let (rows, rows_odd) = random_permutation_with_parity(ranges, h);
    let (columns, columns_odd) = random_permutation_with_parity(ranges, w);
    let mut m = IntegerMatrix::zero(height, width);
    for (i, x) in diagonal.into_iter().enumerate() {
        m[(rows[i], columns[i])] = x;
    }
    (m, rows_odd != columns_odd)
}

/// Generates random [`IntegerMatrix`]s with a given diagonal, placed at random positions.
///
/// This `struct` is created by [`random_integer_matrices_with_permuted_diagonal`]; see its
/// documentation for more.
#[derive(Clone, Debug)]
pub struct RandomIntegerMatricesWithPermutedDiagonal {
    ranges: VariableRangeGenerator,
    height: u64,
    width: u64,
    diagonal: Vec<Integer>,
}

impl Iterator for RandomIntegerMatricesWithPermutedDiagonal {
    type Item = (IntegerMatrix, bool);

    fn next(&mut self) -> Option<(IntegerMatrix, bool)> {
        Some(place_permuted_diagonal(
            &mut self.ranges,
            self.height,
            self.width,
            self.diagonal.clone(),
        ))
    }
}

/// Generates random [`IntegerMatrix`]s of a given shape whose nonzero elements are a given
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
/// $T(n, m) = O(n + m)$
///
/// $M(n, m) = O(n + m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `height * width`, and $m$ is the total
/// number of bits of the elements of `diagonal`.
///
/// # Panics
/// Panics if `diagonal` has more elements than the smaller of `height` and `width`, or if the
/// number of elements does not fit in a [`usize`].
///
/// # Examples
/// ```
/// use core::str::FromStr;
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer_matrix::random::random_integer_matrices_with_permuted_diagonal;
///
/// let diagonal = vec![
///     Integer::from_str("2").unwrap(),
///     Integer::from_str("-3").unwrap(),
///     Integer::from_str("5").unwrap(),
/// ];
/// assert_eq!(
///     prefix_to_string(
///         random_integer_matrices_with_permuted_diagonal(EXAMPLE_SEED, 3, 4, diagonal)
///             .map(|(m, odd)| format!("({m}, {odd})")),
///         3
///     ),
///     "[([0, -3, 0, 0; 0, 0, 0, 5; 2, 0, 0, 0], true), ([0, 2, 0, 0; -3, 0, 0, 0; 0, 0, 5, 0], \
///     true), ([0, 0, 5, 0; 0, -3, 0, 0; 0, 0, 0, 2], false), ...]"
/// );
/// ```
///
/// This is equivalent to `fmpz_mat_randpermdiag` from `fmpz_mat/randpermdiag.c`, FLINT 3.6.0.
pub fn random_integer_matrices_with_permuted_diagonal(
    seed: Seed,
    height: u64,
    width: u64,
    diagonal: Vec<Integer>,
) -> RandomIntegerMatricesWithPermutedDiagonal {
    assert!(
        u64::exact_from(diagonal.len()) <= min(height, width),
        "a diagonal of {} elements does not fit in a {height}-by-{width} matrix",
        diagonal.len()
    );
    shape(height, width);
    RandomIntegerMatricesWithPermutedDiagonal {
        ranges: VariableRangeGenerator::new(seed),
        height,
        width,
        diagonal,
    }
}

/// Generates random [`IntegerMatrix`]s of a given rank.
///
/// This `struct` is created by [`random_integer_matrices_with_rank`]; see its documentation for
/// more.
#[derive(Clone, Debug)]
pub struct RandomIntegerMatricesWithRank {
    ranges: VariableRangeGenerator,
    limbs: RandomPrimitiveInts<u64>,
    height: u64,
    width: u64,
    rank: u64,
    bits: u64,
}

impl Iterator for RandomIntegerMatricesWithRank {
    type Item = IntegerMatrix;

    fn next(&mut self) -> Option<IntegerMatrix> {
        let diagonal = (0..self.rank)
            .map(|_| {
                let abs = get_random_natural_with_bits(&mut self.limbs, self.bits);
                Integer::from_sign_and_abs(self.ranges.next_bool(), abs)
            })
            .collect();
        Some(place_permuted_diagonal(&mut self.ranges, self.height, self.width, diagonal).0)
    }
}

/// Generates random sparse [`IntegerMatrix`]s of a given shape and rank.
///
/// Each output has exactly `rank` nonzero elements, no two in the same row or column, so its rank
/// is `rank`. Each nonzero element has a random sign and an absolute value with exactly `bits`
/// bits, and they are placed as [`random_integer_matrices_with_permuted_diagonal`] places a
/// diagonal. To turn the outputs into dense matrices of the same rank, pass them to
/// [`random_integer_matrices_from_elementary_operations`].
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(n, r, b) = O(n + rb)$
///
/// $M(n, r, b) = O(n + rb)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `height * width`, $r$ is `rank`, and $b$ is
/// `bits`.
///
/// # Panics
/// Panics if `rank` is greater than the smaller of `height` and `width`, if `rank` is positive and
/// `bits` is zero, or if the number of elements does not fit in a [`usize`].
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer_matrix::random::random_integer_matrices_with_rank;
///
/// assert_eq!(
///     prefix_to_string(
///         random_integer_matrices_with_rank(EXAMPLE_SEED, 3, 4, 2, 3),
///         3
///     ),
///     "[[0, 0, 0, 0; 0, 0, -6, 0; 0, 0, 0, 4], [0, 0, 0, 0; 0, 0, -6, 0; 0, 6, 0, 0], [0, 6, 0, \
///     0; 0, 0, 0, 4; 0, 0, 0, 0], ...]"
/// );
/// ```
///
/// This is equivalent to `fmpz_mat_randrank` from `fmpz_mat/randrank.c`, FLINT 3.6.0, except that
/// the nonzero elements have exactly `bits` bits rather than at most `bits`.
pub fn random_integer_matrices_with_rank(
    seed: Seed,
    height: u64,
    width: u64,
    rank: u64,
    bits: u64,
) -> RandomIntegerMatricesWithRank {
    assert!(
        rank <= min(height, width),
        "a {height}-by-{width} matrix cannot have rank {rank}"
    );
    assert!(
        rank == 0 || bits != 0,
        "the nonzero elements of a matrix of positive rank need a positive number of bits"
    );
    shape(height, width);
    RandomIntegerMatricesWithRank {
        ranges: VariableRangeGenerator::new(seed.fork("ranges")),
        limbs: random_primitive_ints(seed.fork("limbs")),
        height,
        width,
        rank,
        bits,
    }
}

/// Generates random [`IntegerMatrix`]s with a given determinant.
///
/// This `struct` is created by [`random_integer_matrices_with_determinant`]; see its documentation
/// for more.
#[derive(Clone, Debug)]
pub struct RandomIntegerMatricesWithDeterminant {
    ranges: VariableRangeGenerator,
    size: u64,
    factors: Vec<(u64, u8)>,
    negative: bool,
    zero: bool,
}

impl Iterator for RandomIntegerMatricesWithDeterminant {
    type Item = IntegerMatrix;

    fn next(&mut self) -> Option<IntegerMatrix> {
        let n = usize::exact_from(self.size);
        if self.zero || n == 0 {
            return Some(IntegerMatrix::zero(self.size, self.size));
        }
        // Spread the prime factors over the diagonal at random.
        let mut diagonal = vec![1u64; n];
        for &(p, e) in &self.factors {
            for _ in 0..e {
                diagonal[self.ranges.next_less_than(n)] *= p;
            }
        }
        let mut diagonal: Vec<Integer> = diagonal.into_iter().map(Integer::from).collect();
        // An even number of random sign changes leaves the product positive.
        for _ in 0..n << 1 {
            diagonal[self.ranges.next_less_than(n)].neg_assign();
        }
        if self.negative {
            diagonal[0].neg_assign();
        }
        let (mut m, odd) =
            place_permuted_diagonal(&mut self.ranges, self.size, self.size, diagonal);
        // An odd permutation negates the determinant, so negate one nonzero element to undo it.
        if odd {
            m.elements
                .iter_mut()
                .find(|x| **x != 0u32)
                .unwrap()
                .neg_assign();
        }
        Some(m)
    }
}

/// Generates random sparse square [`IntegerMatrix`]s of a given size and determinant.
///
/// Each output is a diagonal matrix with its rows and columns permuted at random, as
/// [`random_integer_matrices_with_permuted_diagonal`] places a diagonal. The prime factors of
/// `determinant`, counted with multiplicity, are distributed at random among the diagonal elements,
/// the signs are chosen at random so that their product has the sign of `determinant`, and one sign
/// is changed when the permutation is odd. The determinant of the output is therefore
/// `determinant`. A zero determinant gives the zero matrix. To turn the outputs into dense matrices
/// with the same determinant, pass them to [`random_integer_matrices_from_elementary_operations`].
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(n) = O(n^2)$
///
/// $M(n) = O(n^2)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `size`.
///
/// # Panics
/// Panics if `size` is zero and `determinant` is not 1, since the 0-by-0 matrix has determinant 1;
/// if the absolute value of `determinant` is greater than $2^{64} - 1$, since Malachite cannot yet
/// factor larger integers; or if the number of elements does not fit in a [`usize`].
///
/// # Examples
/// ```
/// use core::str::FromStr;
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer_matrix::random::random_integer_matrices_with_determinant;
///
/// assert_eq!(
///     prefix_to_string(
///         random_integer_matrices_with_determinant(
///             EXAMPLE_SEED,
///             3,
///             &Integer::from_str("-60").unwrap()
///         ),
///         3
///     ),
///     "[[6, 0, 0; 0, 0, -5; 0, -2, 0], [-5, 0, 0; 0, -3, 0; 0, 0, -4], [0, 2, 0; 0, 0, 5; -6, \
///     0, 0], ...]"
/// );
/// ```
///
/// This is equivalent to `fmpz_mat_randdet` from `fmpz_mat/randdet.c`, FLINT 3.6.0, for
/// determinants whose absolute value fits in a `u64`.
pub fn random_integer_matrices_with_determinant(
    seed: Seed,
    size: u64,
    determinant: &Integer,
) -> RandomIntegerMatricesWithDeterminant {
    assert!(
        size != 0 || *determinant == 1u32,
        "the 0-by-0 matrix has determinant 1, not {determinant}"
    );
    shape(size, size);
    let abs = u64::try_from(determinant.unsigned_abs_ref()).unwrap_or_else(|_| {
        panic!("cannot yet factor the determinant {determinant}, which exceeds u64::MAX")
    });
    RandomIntegerMatricesWithDeterminant {
        ranges: VariableRangeGenerator::new(seed),
        size,
        factors: if abs == 0 {
            Vec::new()
        } else {
            abs.factor().into_iter().collect()
        },
        negative: *determinant < 0u32,
        zero: abs == 0,
    }
}

/// Applies random elementary row and column operations to [`IntegerMatrix`]s.
///
/// This `struct` is created by [`random_integer_matrices_from_elementary_operations`]; see its
/// documentation for more.
#[derive(Clone, Debug)]
pub struct RandomIntegerMatricesFromElementaryOperations<I: Iterator<Item = IntegerMatrix>> {
    ranges: VariableRangeGenerator,
    xs: I,
    count: u64,
}

impl<I: Iterator<Item = IntegerMatrix>> Iterator
    for RandomIntegerMatricesFromElementaryOperations<I>
{
    type Item = IntegerMatrix;

    fn next(&mut self) -> Option<IntegerMatrix> {
        let mut m = self.xs.next()?;
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
                let row_i: Vec<Integer> = m.row_iter(i).cloned().collect();
                if self.ranges.next_bool() {
                    for (y, x) in m.row_iter_mut(j).zip(row_i) {
                        *y += x;
                    }
                } else {
                    for (y, x) in m.row_iter_mut(j).zip(row_i) {
                        *y -= x;
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
                let column_i: Vec<Integer> = m.column_iter(i).cloned().collect();
                if self.ranges.next_bool() {
                    for (y, x) in m.column_iter_mut(j).zip(column_i) {
                        *y += x;
                    }
                } else {
                    for (y, x) in m.column_iter_mut(j).zip(column_i) {
                        *y -= x;
                    }
                }
            }
        }
        Some(m)
    }
}

/// Applies `count` random elementary row and column operations to each [`IntegerMatrix`] that an
/// iterator generates.
///
/// Each operation chooses, with equal probability, rows or columns, then two of them, $i$ and $j$,
/// uniformly and independently; if they are distinct, it adds row (or column) $i$ to row (or
/// column) $j$, or subtracts it, with equal probability. An operation that picks the same row or
/// column twice is skipped, so at most `count` operations are applied. Such operations preserve the
/// rank, and for a square matrix the determinant, so this turns the sparse outputs of
/// [`random_integer_matrices_with_rank`] and [`random_integer_matrices_with_determinant`] into
/// dense matrices with the same rank or determinant. Matrices with no elements are passed through
/// unchanged.
///
/// The output has the same length as `xs`.
///
/// # Worst-case complexity per iteration
/// $T(n, c) = O(n + cm)$
///
/// $M(n, c) = O(n + cm)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the elements of
/// the $i$th output, $c$ is `count`, and $m$ is the total number of bits of its largest row or
/// column.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer_matrix::random::{
///     random_integer_matrices_from_elementary_operations, random_integer_matrices_with_rank,
/// };
///
/// assert_eq!(
///     prefix_to_string(
///         random_integer_matrices_from_elementary_operations(
///             EXAMPLE_SEED,
///             random_integer_matrices_with_rank(EXAMPLE_SEED.fork("rank"), 3, 3, 2, 2),
///             10
///         ),
///         3
///     ),
///     "[[-2, 0, 2; -7, -3, 10; -2, 0, 2], [-2, 0, -2; 3, 3, 0; -3, -3, 0], [-2, 11, 2; -2, 2, \
///     2; -2, 5, 2], ...]"
/// );
/// ```
///
/// This is equivalent to `fmpz_mat_randops` from `fmpz_mat/randops.c`, FLINT 3.6.0.
pub fn random_integer_matrices_from_elementary_operations<I: Iterator<Item = IntegerMatrix>>(
    seed: Seed,
    xs: I,
    count: u64,
) -> RandomIntegerMatricesFromElementaryOperations<I> {
    RandomIntegerMatricesFromElementaryOperations {
        ranges: VariableRangeGenerator::new(seed),
        xs,
        count,
    }
}
