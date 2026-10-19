// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::matrix::Matrix;
use crate::num::arithmetic::traits::ModIsReduced;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::num::random::geometric::{GeometricRandomNaturalValues, geometric_random_unsigneds};
use crate::num::random::striped::{StripedRandomUnsignedBitChunks, striped_random_unsigneds};
use crate::num::random::{
    RandomPrimitiveInts, VariableRangeGenerator, random_permutation_with_parity,
    random_primitive_ints,
};
use crate::random::Seed;
use crate::tuples::random::{RandomPairsFromSingle, random_pairs_from_single};
use crate::unsigned_matrix::{UnsignedMatrix, shape};
use alloc::vec::Vec;
use core::cmp::min;
use core::iter::{Repeat, repeat};

/// Generates random [`UnsignedMatrix`]s from an iterator of dimensions and an iterator of elements.
///
/// Each matrix takes its height and width from `dimensions` and then its elements, in row-major
/// order, from `xs`.
///
/// This `struct` is created by [`random_unsigned_matrices_from_iterator`],
/// [`random_unsigned_matrices_with_dimensions_from_iterator`], [`random_unsigned_matrices`],
/// [`random_unsigned_matrices_with_dimensions`], [`striped_random_unsigned_matrices`], and
/// [`striped_random_unsigned_matrices_with_dimensions`],
/// [`random_unsigned_square_matrices_from_iterator`], [`random_unsigned_square_matrices`], and
/// [`striped_random_unsigned_square_matrices`]; see their documentation for more.
#[derive(Clone, Debug)]
pub struct RandomUnsignedMatrices<
    T: PrimitiveUnsigned,
    D: Iterator<Item = (u64, u64)>,
    I: Iterator<Item = T>,
> {
    dimensions: D,
    xs: I,
}

impl<T: PrimitiveUnsigned, D: Iterator<Item = (u64, u64)>, I: Iterator<Item = T>> Iterator
    for RandomUnsignedMatrices<T, D, I>
{
    type Item = UnsignedMatrix<T>;

    fn next(&mut self) -> Option<UnsignedMatrix<T>> {
        let (height, width) = self.dimensions.next()?;
        Some(UnsignedMatrix::from_owned_elements(
            height,
            width,
            (&mut self.xs).take(shape(height, width).2).collect(),
        ))
    }
}

/// The dimensions that the [`UnsignedMatrix`] generators of random shape draw on.
pub type RandomMatrixDimensions = RandomPairsFromSingle<GeometricRandomNaturalValues<u64>>;

/// Generates random [`UnsignedMatrix`]s whose elements come from an iterator.
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
/// The elements here are from 0 to 8.
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::random::random_unsigned_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::unsigned_matrix::random::random_unsigned_matrices_from_iterator;
///
/// assert_eq!(
///     prefix_to_string(
///         random_unsigned_matrices_from_iterator(
///             EXAMPLE_SEED,
///             &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 8),
///             2,
///             1
///         ),
///         5
///     ),
///     "[[]_{0,1}, []_{0,11}, []_{2,0}, [4, 4; 5, 7; 1, 6; 1, 1], []_{6,0}, ...]"
/// );
/// ```
#[inline]
pub fn random_unsigned_matrices_from_iterator<T: PrimitiveUnsigned, I: Iterator<Item = T>>(
    seed: Seed,
    xs_gen: &dyn Fn(Seed) -> I,
    mean_dimension_numerator: u64,
    mean_dimension_denominator: u64,
) -> RandomUnsignedMatrices<T, RandomMatrixDimensions, I> {
    RandomUnsignedMatrices {
        dimensions: random_pairs_from_single(geometric_random_unsigneds(
            seed.fork("dimensions"),
            mean_dimension_numerator,
            mean_dimension_denominator,
        )),
        xs: xs_gen(seed.fork("elements")),
    }
}

/// Generates random [`UnsignedMatrix`]s with a given height and width whose elements come from an
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
/// The elements here are from 0 to 8.
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::random::random_unsigned_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::unsigned_matrix::random::*;
///
/// assert_eq!(
///     prefix_to_string(
///         random_unsigned_matrices_with_dimensions_from_iterator(
///             2,
///             2,
///             random_unsigned_inclusive_range::<u8>(EXAMPLE_SEED, 0, 8)
///         ),
///         5
///     ),
///     "[[1, 7; 5, 4], [6, 4; 2, 8], [1, 7; 5, 0], [2, 6; 3, 5], [2, 2; 5, 5], ...]"
/// );
/// ```
#[inline]
pub fn random_unsigned_matrices_with_dimensions_from_iterator<
    T: PrimitiveUnsigned,
    I: Iterator<Item = T>,
>(
    height: u64,
    width: u64,
    xs: I,
) -> RandomUnsignedMatrices<T, Repeat<(u64, u64)>, I> {
    shape(height, width);
    RandomUnsignedMatrices {
        dimensions: repeat((height, width)),
        xs,
    }
}

/// Generates random [`UnsignedMatrix`]s.
///
/// The elements are sampled from [`random_primitive_ints`], so each is uniform over its whole
/// range. The height and width are sampled independently from a geometric distribution with mean
/// `mean_dimension_numerator / mean_dimension_denominator`, so matrices with no rows or no columns
/// are generated with the probability that that distribution gives to 0.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(i) = O(n)$
///
/// $M(i) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, and $n$ is the number
/// of elements of the $i$th output.
///
/// # Panics
/// Panics if `mean_dimension_numerator` or `mean_dimension_denominator` are zero, or if, after
/// being reduced to lowest terms, their sum is greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::unsigned_matrix::random::random_unsigned_matrices;
///
/// assert_eq!(
///     prefix_to_string(random_unsigned_matrices::<u8>(EXAMPLE_SEED, 2, 1), 5),
///     "[[]_{0,1}, []_{0,11}, []_{2,0}, [164, 76; 94, 126; 28, 110; 175, 17], []_{6,0}, ...]"
/// );
/// ```
#[inline]
pub fn random_unsigned_matrices<T: PrimitiveUnsigned>(
    seed: Seed,
    mean_dimension_numerator: u64,
    mean_dimension_denominator: u64,
) -> RandomUnsignedMatrices<T, RandomMatrixDimensions, RandomPrimitiveInts<T>> {
    random_unsigned_matrices_from_iterator(
        seed,
        &random_primitive_ints,
        mean_dimension_numerator,
        mean_dimension_denominator,
    )
}

/// Generates random [`UnsignedMatrix`]s with a given height and width.
///
/// The elements are sampled from [`random_primitive_ints`], so each is uniform over its whole
/// range. If `height` or `width` is 0, the output consists of the matrix of that shape, which has
/// no elements, repeated.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(i) = O(n)$
///
/// $M(i) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, and $n$ is `height *
/// width`.
///
/// # Panics
/// Panics if `height * width` does not fit in a `usize`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::unsigned_matrix::random::random_unsigned_matrices_with_dimensions;
///
/// assert_eq!(
///     prefix_to_string(
///         random_unsigned_matrices_with_dimensions::<u8>(EXAMPLE_SEED, 2, 3),
///         5
///     ),
///     "[[113, 239, 69; 108, 228, 210], [168, 161, 87; 32, 110, 83], [188, 34, 89; 238, 93, 200], \
///     [149, 115, 189; 149, 217, 201], [117, 146, 31; 72, 151, 169], ...]"
/// );
/// ```
#[inline]
pub fn random_unsigned_matrices_with_dimensions<T: PrimitiveUnsigned>(
    seed: Seed,
    height: u64,
    width: u64,
) -> RandomUnsignedMatrices<T, Repeat<(u64, u64)>, RandomPrimitiveInts<T>> {
    random_unsigned_matrices_with_dimensions_from_iterator(
        height,
        width,
        random_primitive_ints(seed),
    )
}

/// Generates random [`UnsignedMatrix`]s with striped elements.
///
/// The elements are sampled from [`striped_random_unsigneds`], with a mean run length of
/// `mean_stripe_numerator / mean_stripe_denominator`. A striped element is one whose bits come in
/// long runs, which is what makes the carries and borrows of an arithmetic test interesting.
///
/// The height and width are sampled independently from a geometric distribution with mean
/// `mean_dimension_numerator / mean_dimension_denominator`, so matrices with no rows or no columns
/// are generated with the probability that that distribution gives to 0.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(i) = O(n)$
///
/// $M(i) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, and $n$ is the number
/// of elements of the $i$th output.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if `mean_stripe_numerator <
/// mean_stripe_denominator`, if `mean_dimension_numerator` or `mean_dimension_denominator` are
/// zero, or if, after being reduced to lowest terms, the sum of the last two is greater than or
/// equal to $2^{64}$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::unsigned_matrix::random::striped_random_unsigned_matrices;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_unsigned_matrices::<u8>(EXAMPLE_SEED, 4, 1, 2, 1),
///         5
///     ),
///     "[[]_{0,1}, []_{0,11}, []_{2,0}, [127, 128; 240, 255; 184, 224; 32, 3], []_{6,0}, ...]"
/// );
/// ```
#[inline]
pub fn striped_random_unsigned_matrices<T: PrimitiveUnsigned>(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_dimension_numerator: u64,
    mean_dimension_denominator: u64,
) -> RandomUnsignedMatrices<T, RandomMatrixDimensions, StripedRandomUnsignedBitChunks<T>> {
    random_unsigned_matrices_from_iterator(
        seed,
        &|seed_2| striped_random_unsigneds(seed_2, mean_stripe_numerator, mean_stripe_denominator),
        mean_dimension_numerator,
        mean_dimension_denominator,
    )
}

/// Generates random [`UnsignedMatrix`]s with a given height and width, with striped elements.
///
/// The elements are striped, as they are in [`striped_random_unsigned_matrices`]. If `height` or
/// `width` is 0, the output consists of the matrix of that shape, which has no elements, repeated.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(i) = O(n)$
///
/// $M(i) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, and $n$ is `height *
/// width`.
///
/// # Panics
/// Panics if `height * width` does not fit in a `usize`, if `mean_stripe_denominator` is zero, or
/// if `mean_stripe_numerator < mean_stripe_denominator`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::unsigned_matrix::random::striped_random_unsigned_matrices_with_dimensions;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_unsigned_matrices_with_dimensions::<u8>(EXAMPLE_SEED, 2, 3, 4, 1),
///         5
///     ),
///     "[[1, 76, 127; 195, 0, 128], [15, 118, 0; 248, 255, 253], [121, 0, 240; 3, 0, 127], [1, 0, \
///     211; 135, 7, 121], [176, 30, 127; 255, 244, 63], ...]"
/// );
/// ```
#[inline]
pub fn striped_random_unsigned_matrices_with_dimensions<T: PrimitiveUnsigned>(
    seed: Seed,
    height: u64,
    width: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
) -> RandomUnsignedMatrices<T, Repeat<(u64, u64)>, StripedRandomUnsignedBitChunks<T>> {
    random_unsigned_matrices_with_dimensions_from_iterator(
        height,
        width,
        striped_random_unsigneds(seed, mean_stripe_numerator, mean_stripe_denominator),
    )
}

/// Generates the dimensions of random square matrices: each output is $(n, n)$, with $n$ sampled
/// from a geometric distribution.
///
/// This `struct` is used by [`random_unsigned_square_matrices_from_iterator`],
/// [`random_unsigned_square_matrices`], and [`striped_random_unsigned_square_matrices`]; see their
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

/// Generates random square [`UnsignedMatrix`]s whose elements come from an iterator.
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
/// The elements here are from 0 to 8.
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::random::random_unsigned_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::unsigned_matrix::random::random_unsigned_square_matrices_from_iterator;
///
/// assert_eq!(
///     prefix_to_string(
///         random_unsigned_square_matrices_from_iterator(
///             EXAMPLE_SEED,
///             &|seed| random_unsigned_inclusive_range::<u8>(seed, 0, 8),
///             3,
///             2
///         ),
///         5
///     ),
///     "[[]_{0,0}, []_{0,0}, [4, 4, 5, 7, 1, 6; 1, 1, 0, 1, 4, 3; 1, 1, 4, 4, 0, 6; 7, 1, 1, 1, \
///     7, 5; 2, 2, 2, 4, 8, 5; 3, 3, 7, 0, 0, 8], []_{0,0}, [4, 0; 3, 7], ...]"
/// );
/// ```
#[inline]
pub fn random_unsigned_square_matrices_from_iterator<
    T: PrimitiveUnsigned,
    I: Iterator<Item = T>,
>(
    seed: Seed,
    xs_gen: &dyn Fn(Seed) -> I,
    mean_size_numerator: u64,
    mean_size_denominator: u64,
) -> RandomUnsignedMatrices<T, RandomSquareMatrixDimensions, I> {
    RandomUnsignedMatrices {
        dimensions: RandomSquareMatrixDimensions(geometric_random_unsigneds(
            seed.fork("dimensions"),
            mean_size_numerator,
            mean_size_denominator,
        )),
        xs: xs_gen(seed.fork("elements")),
    }
}

/// Generates random square [`UnsignedMatrix`]s.
///
/// The elements are sampled from [`random_primitive_ints`], so each is uniform over its whole
/// range. The size is sampled from a geometric distribution with mean `mean_size_numerator /
/// mean_size_denominator`, so the 0-by-0 matrix is generated with the probability that that
/// distribution gives to 0.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(i) = O(n)$
///
/// $M(i) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, and $n$ is the number
/// of elements of the $i$th output.
///
/// # Panics
/// Panics if `mean_size_numerator` or `mean_size_denominator` are zero, or if, after being reduced
/// to lowest terms, their sum is greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::unsigned_matrix::random::random_unsigned_square_matrices;
///
/// assert_eq!(
///     prefix_to_string(random_unsigned_square_matrices::<u8>(EXAMPLE_SEED, 3, 2), 5),
///     "[[]_{0,0}, []_{0,0}, [164, 76, 94, 126, 28, 110; 175, 17, 11, 238, 65, 62; 225, 188, 65, \
///     75, 96, 154; 151, 17, 31, 199, 37, 162; 235, 226, 74, 140, 91, 207; 254, 63, 63, 190, 7, \
///     128], []_{0,0}, [4, 115; 222, 72], ...]"
/// );
/// ```
#[inline]
pub fn random_unsigned_square_matrices<T: PrimitiveUnsigned>(
    seed: Seed,
    mean_size_numerator: u64,
    mean_size_denominator: u64,
) -> RandomUnsignedMatrices<T, RandomSquareMatrixDimensions, RandomPrimitiveInts<T>> {
    random_unsigned_square_matrices_from_iterator(
        seed,
        &random_primitive_ints,
        mean_size_numerator,
        mean_size_denominator,
    )
}

/// Generates random square [`UnsignedMatrix`]s with striped elements.
///
/// The elements are striped, as they are in [`striped_random_unsigned_matrices`]. The size is
/// sampled from a geometric distribution with mean `mean_size_numerator / mean_size_denominator`,
/// so the 0-by-0 matrix is generated with the probability that that distribution gives to 0.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(i) = O(n)$
///
/// $M(i) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, and $n$ is the number
/// of elements of the $i$th output.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if `mean_stripe_numerator <
/// mean_stripe_denominator`, if `mean_size_numerator` or `mean_size_denominator` are zero, or if,
/// after being reduced to lowest terms, the sum of the last two is greater than or equal to
/// $2^{64}$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::unsigned_matrix::random::striped_random_unsigned_square_matrices;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_unsigned_square_matrices::<u8>(EXAMPLE_SEED, 4, 1, 3, 2),
///         5
///     ),
///     "[[]_{0,0}, []_{0,0}, [127, 128, 240, 255, 184, 224; 32, 3, 224, 254, 112, 7; 194, 60, 1, \
///     128, 128, 15; 28, 127, 191, 253, 216, 200; 195, 239, 1, 3, 127, 126; 224, 62, 254, 240, \
///     120, 128], []_{0,0}, [28, 1; 31, 220], ...]"
/// );
/// ```
#[inline]
pub fn striped_random_unsigned_square_matrices<T: PrimitiveUnsigned>(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_size_numerator: u64,
    mean_size_denominator: u64,
) -> RandomUnsignedMatrices<T, RandomSquareMatrixDimensions, StripedRandomUnsignedBitChunks<T>> {
    random_unsigned_square_matrices_from_iterator(
        seed,
        &|seed_2| striped_random_unsigneds(seed_2, mean_stripe_numerator, mean_stripe_denominator),
        mean_size_numerator,
        mean_size_denominator,
    )
}

// Places `diagonal` at random positions in a zero matrix of the given shape: element `i` goes to
// row `rows[i]` and column `columns[i]`, for uniformly random permutations `rows` of the rows and
// `columns` of the columns. Returns the matrix and whether the two permutations together are odd.
fn place_permuted_diagonal<T: PrimitiveUnsigned>(
    ranges: &mut VariableRangeGenerator,
    height: u64,
    width: u64,
    diagonal: Vec<T>,
) -> (UnsignedMatrix<T>, bool) {
    let (h, w, _) = shape(height, width);
    let (rows, rows_odd) = random_permutation_with_parity(ranges, h);
    let (columns, columns_odd) = random_permutation_with_parity(ranges, w);
    let mut m = UnsignedMatrix::zero(height, width);
    for (i, x) in diagonal.into_iter().enumerate() {
        m[(rows[i], columns[i])] = x;
    }
    (m, rows_odd != columns_odd)
}

/// Generates random [`UnsignedMatrix`]s with a given diagonal, placed at random positions.
///
/// This `struct` is created by [`random_unsigned_matrices_with_permuted_diagonal`]; see its
/// documentation for more.
#[derive(Clone, Debug)]
pub struct RandomUnsignedMatricesWithPermutedDiagonal<T: PrimitiveUnsigned> {
    ranges: VariableRangeGenerator,
    height: u64,
    width: u64,
    diagonal: Vec<T>,
}

impl<T: PrimitiveUnsigned> Iterator for RandomUnsignedMatricesWithPermutedDiagonal<T> {
    type Item = (UnsignedMatrix<T>, bool);

    fn next(&mut self) -> Option<(UnsignedMatrix<T>, bool)> {
        Some(place_permuted_diagonal(
            &mut self.ranges,
            self.height,
            self.width,
            self.diagonal.clone(),
        ))
    }
}

/// Generates random [`UnsignedMatrix`]s of a given shape whose nonzero elements are a given
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
/// use malachite_base::unsigned_matrix::random::random_unsigned_matrices_with_permuted_diagonal;
///
/// assert_eq!(
///     prefix_to_string(
///         random_unsigned_matrices_with_permuted_diagonal(EXAMPLE_SEED, 3, 4, vec![2u8, 3, 5])
///             .map(|(m, odd)| format!("({m}, {odd})")),
///         3
///     ),
///     "[([0, 3, 0, 0; 0, 0, 0, 5; 2, 0, 0, 0], true), ([0, 2, 0, 0; 3, 0, 0, 0; 0, 0, 5, 0], \
///     true), ([0, 0, 5, 0; 0, 3, 0, 0; 0, 0, 0, 2], false), ...]"
/// );
/// ```
///
/// This is equivalent to `nmod_mat_randpermdiag` from `nmod_mat/randomisation.c`, FLINT 3.6.0.
pub fn random_unsigned_matrices_with_permuted_diagonal<T: PrimitiveUnsigned>(
    seed: Seed,
    height: u64,
    width: u64,
    diagonal: Vec<T>,
) -> RandomUnsignedMatricesWithPermutedDiagonal<T> {
    assert!(
        u64::exact_from(diagonal.len()) <= min(height, width),
        "a diagonal of {} elements does not fit in a {height}-by-{width} matrix",
        diagonal.len()
    );
    shape(height, width);
    RandomUnsignedMatricesWithPermutedDiagonal {
        ranges: VariableRangeGenerator::new(seed),
        height,
        width,
        diagonal,
    }
}

/// Generates random [`UnsignedMatrix`]s of a given rank modulo a number.
///
/// This `struct` is created by [`random_unsigned_matrices_with_rank_mod`]; see its documentation
/// for more.
#[derive(Clone, Debug)]
pub struct RandomUnsignedMatricesWithRankMod<T: PrimitiveUnsigned> {
    ranges: VariableRangeGenerator,
    height: u64,
    width: u64,
    rank: u64,
    m: T,
}

impl<T: PrimitiveUnsigned> Iterator for RandomUnsignedMatricesWithRankMod<T> {
    type Item = UnsignedMatrix<T>;

    fn next(&mut self) -> Option<UnsignedMatrix<T>> {
        let diagonal = (0..self.rank)
            .map(|_| self.ranges.next_in_range(T::ONE, self.m))
            .collect();
        Some(place_permuted_diagonal(&mut self.ranges, self.height, self.width, diagonal).0)
    }
}

/// Generates random sparse [`UnsignedMatrix`]s of a given shape whose elements are reduced modulo
/// `m`, with a given number of nonzero elements, no two in the same row or column.
///
/// Each nonzero element is uniformly distributed on $[1, m)$, and they are placed as
/// [`random_unsigned_matrices_with_permuted_diagonal`] places a diagonal. When $m$ is prime, the
/// rank of every output is `rank`. To turn the outputs into dense matrices of the same rank, pass
/// them to [`random_unsigned_matrices_from_elementary_operations_mod`].
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
/// use malachite_base::unsigned_matrix::random::random_unsigned_matrices_with_rank_mod;
///
/// assert_eq!(
///     prefix_to_string(
///         random_unsigned_matrices_with_rank_mod(EXAMPLE_SEED, 3, 4, 2, 7u8),
///         3
///     ),
///     "[[2, 0, 0, 0; 0, 0, 0, 0; 0, 0, 6, 0], [0, 0, 0, 1; 2, 0, 0, 0; 0, 0, 0, 0], [0, 0, 0, \
///     0; 6, 0, 0, 0; 0, 0, 0, 3], ...]"
/// );
/// ```
///
/// This is equivalent to `nmod_mat_randrank` from `nmod_mat/randomisation.c`, FLINT 3.6.0, except
/// that the modulus 1 is rejected for a positive rank rather than giving the zero matrix.
pub fn random_unsigned_matrices_with_rank_mod<T: PrimitiveUnsigned>(
    seed: Seed,
    height: u64,
    width: u64,
    rank: u64,
    m: T,
) -> RandomUnsignedMatricesWithRankMod<T> {
    assert!(
        rank <= min(height, width),
        "a {height}-by-{width} matrix cannot have rank {rank}"
    );
    assert!(!(m == T::ZERO), "the modulus must be positive");
    assert!(
        rank == 0 || !(m <= T::ONE),
        "there are no nonzero residues modulo {m}, so the rank must be 0"
    );
    shape(height, width);
    RandomUnsignedMatricesWithRankMod {
        ranges: VariableRangeGenerator::new(seed),
        height,
        width,
        rank,
        m,
    }
}

/// Generates random triangular [`UnsignedMatrix`]s modulo a number.
///
/// This `struct` is created by [`random_unsigned_lower_triangular_matrices_mod`] and
/// [`random_unsigned_upper_triangular_matrices_mod`]; see their documentation for more.
#[derive(Clone, Debug)]
pub struct RandomUnsignedTriangularMatricesMod<T: PrimitiveUnsigned> {
    ranges: VariableRangeGenerator,
    height: u64,
    width: u64,
    unit: bool,
    lower: bool,
    m: T,
}

impl<T: PrimitiveUnsigned> Iterator for RandomUnsignedTriangularMatricesMod<T> {
    type Item = UnsignedMatrix<T>;

    fn next(&mut self) -> Option<UnsignedMatrix<T>> {
        let mut elements = Vec::with_capacity(shape(self.height, self.width).2);
        for i in 0..self.height {
            for j in 0..self.width {
                elements.push(if i == j {
                    if self.unit {
                        T::ONE
                    } else {
                        self.ranges.next_in_range(T::ONE, self.m)
                    }
                } else if (j < i) == self.lower {
                    self.ranges.next_less_than(self.m)
                } else {
                    T::ZERO
                });
            }
        }
        Some(UnsignedMatrix::from_owned_elements(
            self.height,
            self.width,
            elements,
        ))
    }
}

/// Generates random lower triangular [`UnsignedMatrix`]s of a given shape whose elements are
/// reduced modulo `m`.
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
/// use malachite_base::unsigned_matrix::random::random_unsigned_lower_triangular_matrices_mod;
///
/// assert_eq!(
///     prefix_to_string(
///         random_unsigned_lower_triangular_matrices_mod(EXAMPLE_SEED, 3, 3, false, 7u8),
///         3
///     ),
///     "[[2, 0, 0; 6, 6, 0; 6, 3, 2], [3, 0, 0; 4, 6, 0; 1, 2, 6], [5, 0, 0; 6, 1, 0; 5, 6, 1], \
///     ...]"
/// );
/// ```
///
/// This is equivalent to `nmod_mat_randtril` from `nmod_mat/randomisation.c`, FLINT 3.6.0, except
/// that a random diagonal element is uniform on $[1, m)$, where FLINT replaces a 0 by 1, and that
/// the modulus 1 is rejected.
pub fn random_unsigned_lower_triangular_matrices_mod<T: PrimitiveUnsigned>(
    seed: Seed,
    height: u64,
    width: u64,
    unit: bool,
    m: T,
) -> RandomUnsignedTriangularMatricesMod<T> {
    assert!(!(m == T::ZERO), "the modulus must be positive");
    assert!(
        min(height, width) == 0 || !(m <= T::ONE),
        "no diagonal element can be reduced modulo {m}"
    );
    shape(height, width);
    RandomUnsignedTriangularMatricesMod {
        ranges: VariableRangeGenerator::new(seed),
        height,
        width,
        unit,
        lower: true,
        m,
    }
}

/// Generates random upper triangular [`UnsignedMatrix`]s of a given shape whose elements are
/// reduced modulo `m`.
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
/// use malachite_base::unsigned_matrix::random::random_unsigned_upper_triangular_matrices_mod;
///
/// assert_eq!(
///     prefix_to_string(
///         random_unsigned_upper_triangular_matrices_mod(EXAMPLE_SEED, 3, 3, false, 7u8),
///         3
///     ),
///     "[[2, 6, 5; 0, 4, 1; 0, 0, 3], [5, 5, 1; 0, 3, 6; 0, 0, 6], [5, 6, 0; 0, 6, 6; 0, 0, 1], \
///     ...]"
/// );
/// ```
///
/// This is equivalent to `nmod_mat_randtriu` from `nmod_mat/randomisation.c`, FLINT 3.6.0, except
/// that a random diagonal element is uniform on $[1, m)$, where FLINT replaces a 0 by 1, and that
/// the modulus 1 is rejected.
pub fn random_unsigned_upper_triangular_matrices_mod<T: PrimitiveUnsigned>(
    seed: Seed,
    height: u64,
    width: u64,
    unit: bool,
    m: T,
) -> RandomUnsignedTriangularMatricesMod<T> {
    assert!(!(m == T::ZERO), "the modulus must be positive");
    assert!(
        min(height, width) == 0 || !(m <= T::ONE),
        "no diagonal element can be reduced modulo {m}"
    );
    shape(height, width);
    RandomUnsignedTriangularMatricesMod {
        ranges: VariableRangeGenerator::new(seed),
        height,
        width,
        unit,
        lower: false,
        m,
    }
}

/// Applies random elementary row and column operations modulo a number to [`UnsignedMatrix`]s.
///
/// This `struct` is created by [`random_unsigned_matrices_from_elementary_operations_mod`]; see its
/// documentation for more.
#[derive(Clone, Debug)]
pub struct RandomUnsignedMatricesFromElementaryOperationsMod<
    T: PrimitiveUnsigned,
    I: Iterator<Item = UnsignedMatrix<T>>,
> {
    ranges: VariableRangeGenerator,
    xs: I,
    count: u64,
    m: T,
}

impl<T: PrimitiveUnsigned, I: Iterator<Item = UnsignedMatrix<T>>> Iterator
    for RandomUnsignedMatricesFromElementaryOperationsMod<T, I>
{
    type Item = UnsignedMatrix<T>;

    fn next(&mut self) -> Option<UnsignedMatrix<T>> {
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
                let row_i: Vec<T> = m.row_iter(i).copied().collect();
                if self.ranges.next_bool() {
                    for (y, x) in m.row_iter_mut(j).zip(row_i) {
                        y.mod_add_assign(x, self.m);
                    }
                } else {
                    for (y, x) in m.row_iter_mut(j).zip(row_i) {
                        y.mod_sub_assign(x, self.m);
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
                let column_i: Vec<T> = m.column_iter(i).copied().collect();
                if self.ranges.next_bool() {
                    for (y, x) in m.column_iter_mut(j).zip(column_i) {
                        y.mod_add_assign(x, self.m);
                    }
                } else {
                    for (y, x) in m.column_iter_mut(j).zip(column_i) {
                        y.mod_sub_assign(x, self.m);
                    }
                }
            }
        }
        Some(m)
    }
}

/// Applies `count` random elementary row and column operations modulo `m` to each
/// [`UnsignedMatrix`] that an iterator generates.
///
/// Each operation chooses, with equal probability, rows or columns, then two of them, $i$ and $j$,
/// uniformly and independently; if they are distinct, it adds row (or column) $i$ to row (or
/// column) $j$ modulo `m`, or subtracts it, with equal probability. An operation that picks the
/// same row or column twice is skipped, so at most `count` operations are applied. Such operations
/// preserve the rank modulo a prime, and for a square matrix the determinant, so this turns the
/// sparse outputs of [`random_unsigned_matrices_with_rank_mod`] into dense matrices with the same
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
/// use malachite_base::unsigned_matrix::random::{
///     random_unsigned_matrices_from_elementary_operations_mod,
///     random_unsigned_matrices_with_rank_mod,
/// };
///
/// assert_eq!(
///     prefix_to_string(
///         random_unsigned_matrices_from_elementary_operations_mod(
///             EXAMPLE_SEED,
///             random_unsigned_matrices_with_rank_mod(EXAMPLE_SEED.fork("rank"), 3, 3, 2, 7u8),
///             10,
///             7u8
///         ),
///         3
///     ),
///     "[[3, 3, 1; 6, 6, 2; 3, 3, 6], [5, 5, 0; 4, 0, 4; 0, 0, 0], [5, 4, 2; 0, 2, 0; 4, 5, 3], \
///     ...]"
/// );
/// ```
///
/// This is equivalent to `nmod_mat_randops` from `nmod_mat/randomisation.c`, FLINT 3.6.0.
pub fn random_unsigned_matrices_from_elementary_operations_mod<
    T: PrimitiveUnsigned,
    I: Iterator<Item = UnsignedMatrix<T>>,
>(
    seed: Seed,
    xs: I,
    count: u64,
    m: T,
) -> RandomUnsignedMatricesFromElementaryOperationsMod<T, I> {
    assert!(!(m == T::ZERO), "the modulus must be positive");
    RandomUnsignedMatricesFromElementaryOperationsMod {
        ranges: VariableRangeGenerator::new(seed),
        xs,
        count,
        m,
    }
}
