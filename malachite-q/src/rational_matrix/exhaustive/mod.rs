// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational::exhaustive::{ExhaustiveNonzeroRationals, exhaustive_rationals};
use crate::rational_matrix::{RationalMatrix, element_count};
use alloc::vec::Vec;
use core::iter::{Chain, Once, Repeat, Zip, repeat};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::exhaustive::{PrimitiveIntIncreasingRange, exhaustive_unsigneds};
use malachite_base::num::iterators::{RulerSequence, ruler_sequence};
use malachite_base::tuples::exhaustive::{
    ExhaustiveDependentPairs, ExhaustiveDependentPairsYsGenerator, ExhaustivePairs1Input,
    exhaustive_dependent_pairs, exhaustive_pairs_from_single,
};
use malachite_base::vecs::exhaustive::{
    ExhaustiveFixedLengthVecs1Input, exhaustive_vecs_fixed_length_from_single,
};

/// Generates [`RationalMatrix`]s from an iterator of dimensions paired with elements in row-major
/// order.
///
/// This `struct` is created by [`exhaustive_rational_matrices_from_iterator`],
/// [`exhaustive_rational_matrices`],
/// [`exhaustive_rational_matrices_with_dimensions_from_iterator`], and
/// [`exhaustive_rational_matrices_with_dimensions`]; see their documentation for more.
#[derive(Clone, Debug)]
pub struct ExhaustiveRationalMatrices<I: Iterator<Item = ((usize, usize), Vec<Rational>)>>(I);

impl<I: Iterator<Item = ((usize, usize), Vec<Rational>)>> Iterator
    for ExhaustiveRationalMatrices<I>
{
    type Item = RationalMatrix;

    #[inline]
    fn next(&mut self) -> Option<RationalMatrix> {
        self.0.next().map(|((height, width), elements)| {
            RationalMatrix::from_owned_elements(height, width, elements)
        })
    }
}

/// Generates, for a height and width, every way of filling a matrix of that shape with elements
/// from an iterator.
///
/// This `struct` is used by [`exhaustive_rational_matrices_from_iterator`] and
/// [`exhaustive_rational_matrices`]; see their documentation for more.
#[derive(Clone, Debug)]
pub struct ExhaustiveRationalMatrixElements<I: Clone + Iterator<Item = Rational>>(I);

impl<I: Clone + Iterator<Item = Rational>>
    ExhaustiveDependentPairsYsGenerator<
        (usize, usize),
        Vec<Rational>,
        ExhaustiveFixedLengthVecs1Input<I>,
    > for ExhaustiveRationalMatrixElements<I>
{
    #[inline]
    fn get_ys(&self, &(height, width): &(usize, usize)) -> ExhaustiveFixedLengthVecs1Input<I> {
        exhaustive_vecs_fixed_length_from_single(
            u64::exact_from(element_count(height, width)),
            self.0.clone(),
        )
    }
}

/// Generates all [`RationalMatrix`]s whose elements come from an iterator.
///
/// Every shape, including those with no rows or no columns, is generated, and so is every way of
/// filling it with elements of `xs`; each matrix is generated once if `xs` generates each element
/// once. This allows any set of elements, for example the residues modulo some $m$.
///
/// The shapes are reached in the order in which [`exhaustive_pairs_from_single`] generates $(h, w)$
/// pairs, and each shape's matrices are interleaved with the others' so that small matrices of
/// every shape come early. The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(n T^\prime(i))$
///
/// $M(i) = O(n M^\prime(i))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of `xs`, and $n$ is the number of elements of the
/// $i$th output.
///
/// # Examples
/// The elements here are 0 and 1/2.
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::basic::traits::{OneHalf, Zero};
/// use malachite_q::Rational;
/// use malachite_q::rational_matrix::exhaustive::exhaustive_rational_matrices_from_iterator;
///
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_rational_matrices_from_iterator(
///             [Rational::ZERO, Rational::ONE_HALF].into_iter()
///         ),
///         20
///     ),
///     "[[]_{0,0}, []_{0,1}, []_{1,0}, []_{0,2}, [0], []_{0,3}, [1/2], [0, 0, 0], [0, 0], [0, 0, \
///     1/2], [0, 1/2], []_{2,0}, [1/2, 0], [0, 1/2, 0], [1/2, 1/2], []_{3,0}, [0, 1/2, 1/2], [0; \
///     0], [1/2, 0, 0], [0; 0; 0], ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_rational_matrices_from_iterator<I: Clone + Iterator<Item = Rational>>(
    xs: I,
) -> ExhaustiveRationalMatrices<
    ExhaustiveDependentPairs<
        (usize, usize),
        Vec<Rational>,
        RulerSequence<usize>,
        ExhaustiveRationalMatrixElements<I>,
        ExhaustivePairs1Input<PrimitiveIntIncreasingRange<usize>>,
        ExhaustiveFixedLengthVecs1Input<I>,
    >,
> {
    ExhaustiveRationalMatrices(exhaustive_dependent_pairs(
        ruler_sequence(),
        exhaustive_pairs_from_single(exhaustive_unsigneds()),
        ExhaustiveRationalMatrixElements(xs),
    ))
}

/// Generates all [`RationalMatrix`]s.
///
/// Every matrix, of every shape, is generated once, the 0-by-0 matrix first. The matrices are
/// generated as they are by [`exhaustive_rational_matrices_from_iterator`], with
/// [`exhaustive_rationals`] as the elements.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(n\ell)$
///
/// $M(i) = O(n\ell)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $n$ is the number of
/// elements of the $i$th output, and $\ell$ is the number of significant bits of its largest
/// element.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_q::rational_matrix::exhaustive::exhaustive_rational_matrices;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_rational_matrices(), 20),
///     "[[]_{0,0}, []_{0,1}, []_{1,0}, []_{0,2}, [0], []_{0,3}, [1], [0, 0, 0], [-1], [0, 0], \
///     [1/2], [0, 0, 1], [-1/2], [0, 1], [2], [0; 0], [-2], [1, 0], [1/3], [0, 1, 0], ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_rational_matrices() -> ExhaustiveRationalMatrices<
    ExhaustiveDependentPairs<
        (usize, usize),
        Vec<Rational>,
        RulerSequence<usize>,
        ExhaustiveRationalMatrixElements<Chain<Once<Rational>, ExhaustiveNonzeroRationals>>,
        ExhaustivePairs1Input<PrimitiveIntIncreasingRange<usize>>,
        ExhaustiveFixedLengthVecs1Input<Chain<Once<Rational>, ExhaustiveNonzeroRationals>>,
    >,
> {
    exhaustive_rational_matrices_from_iterator(exhaustive_rationals())
}

/// Generates all [`RationalMatrix`]s with a given height and width whose elements come from an
/// iterator.
///
/// Each way of filling a `height`-by-`width` matrix with elements of `xs` is generated once if `xs`
/// generates each element once. If `height` or `width` is 0, the only output is the matrix of that
/// shape, which has no elements. Otherwise the output is finite if and only if `xs` is.
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
/// The elements here are 0 and 1/2.
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::basic::traits::{OneHalf, Zero};
/// use malachite_q::Rational;
/// use malachite_q::rational_matrix::exhaustive::*;
///
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_rational_matrices_with_dimensions_from_iterator(
///             2,
///             2,
///             [Rational::ZERO, Rational::ONE_HALF].into_iter()
///         ),
///         20
///     ),
///     "[[0, 0; 0, 0], [0, 0; 0, 1/2], [0, 0; 1/2, 0], [0, 0; 1/2, 1/2], [0, 1/2; 0, 0], [0, 1/2; \
///     0, 1/2], [0, 1/2; 1/2, 0], [0, 1/2; 1/2, 1/2], [1/2, 0; 0, 0], [1/2, 0; 0, 1/2], [1/2, 0; \
///     1/2, 0], [1/2, 0; 1/2, 1/2], [1/2, 1/2; 0, 0], [1/2, 1/2; 0, 1/2], [1/2, 1/2; 1/2, 0], \
///     [1/2, 1/2; 1/2, 1/2]]"
/// );
/// ```
#[inline]
pub fn exhaustive_rational_matrices_with_dimensions_from_iterator<
    I: Clone + Iterator<Item = Rational>,
>(
    height: usize,
    width: usize,
    xs: I,
) -> ExhaustiveRationalMatrices<Zip<Repeat<(usize, usize)>, ExhaustiveFixedLengthVecs1Input<I>>> {
    ExhaustiveRationalMatrices(repeat((height, width)).zip(
        exhaustive_vecs_fixed_length_from_single(u64::exact_from(element_count(height, width)), xs),
    ))
}

/// Generates all [`RationalMatrix`]s with a given height and width.
///
/// If `height` or `width` is 0, the only output is the matrix of that shape, which has no elements;
/// otherwise the output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(n\ell)$
///
/// $M(i) = O(n\ell)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $n$ is `height *
/// width`, and $\ell$ is the number of significant bits of the $i$th output's largest element.
///
/// # Panics
/// Panics if `height * width` does not fit in a `usize`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_q::rational_matrix::exhaustive::exhaustive_rational_matrices_with_dimensions;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_rational_matrices_with_dimensions(0, 3), 10),
///     "[[]_{0,3}]"
/// );
/// assert_eq!(
///     prefix_to_string(exhaustive_rational_matrices_with_dimensions(2, 2), 10),
///     "[[0, 0; 0, 0], [0, 0; 0, 1], [0, 0; 1, 0], [0, 0; 1, 1], [0, 1; 0, 0], [0, 1; 0, 1], [0, \
///     1; 1, 0], [0, 1; 1, 1], [1, 0; 0, 0], [1, 0; 0, 1], ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_rational_matrices_with_dimensions(
    height: usize,
    width: usize,
) -> ExhaustiveRationalMatrices<
    Zip<
        Repeat<(usize, usize)>,
        ExhaustiveFixedLengthVecs1Input<Chain<Once<Rational>, ExhaustiveNonzeroRationals>>,
    >,
> {
    exhaustive_rational_matrices_with_dimensions_from_iterator(
        height,
        width,
        exhaustive_rationals(),
    )
}
