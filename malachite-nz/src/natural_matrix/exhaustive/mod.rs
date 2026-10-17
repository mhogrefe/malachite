// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural::exhaustive::{ExhaustiveNaturalRangeToInfinity, exhaustive_naturals};
use crate::natural_matrix::{NaturalMatrix, shape};
use alloc::vec::Vec;
use core::iter::{Repeat, Zip, repeat};
use malachite_base::matrix::Matrix;
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

/// Generates [`NaturalMatrix`]s from an iterator of dimensions paired with elements in row-major
/// order.
///
/// This `struct` is created by [`exhaustive_natural_matrices_from_iterator`],
/// [`exhaustive_natural_matrices`], [`exhaustive_natural_matrices_with_dimensions_from_iterator`],
/// and [`exhaustive_natural_matrices_with_dimensions`]; see their documentation for more.
#[derive(Clone, Debug)]
pub struct ExhaustiveNaturalMatrices<I: Iterator<Item = ((u64, u64), Vec<Natural>)>>(I);

impl<I: Iterator<Item = ((u64, u64), Vec<Natural>)>> Iterator for ExhaustiveNaturalMatrices<I> {
    type Item = NaturalMatrix;

    #[inline]
    fn next(&mut self) -> Option<NaturalMatrix> {
        self.0.next().map(|((height, width), elements)| {
            NaturalMatrix::from_owned_elements(height, width, elements)
        })
    }
}

/// Generates, for a height and width, every way of filling a matrix of that shape with elements
/// from an iterator.
///
/// This `struct` is used by [`exhaustive_natural_matrices_from_iterator`] and
/// [`exhaustive_natural_matrices`]; see their documentation for more.
#[derive(Clone, Debug)]
pub struct ExhaustiveNaturalMatrixElements<I: Clone + Iterator<Item = Natural>>(I);

impl<I: Clone + Iterator<Item = Natural>>
    ExhaustiveDependentPairsYsGenerator<
        (u64, u64),
        Vec<Natural>,
        ExhaustiveFixedLengthVecs1Input<I>,
    > for ExhaustiveNaturalMatrixElements<I>
{
    #[inline]
    fn get_ys(&self, &(height, width): &(u64, u64)) -> ExhaustiveFixedLengthVecs1Input<I> {
        exhaustive_vecs_fixed_length_from_single(
            u64::exact_from(shape(height, width).2),
            self.0.clone(),
        )
    }
}

/// Generates all [`NaturalMatrix`]s whose elements come from an iterator.
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
/// The elements here are 0 and 1.
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::basic::traits::{Two, Zero};
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural::exhaustive::exhaustive_natural_range;
/// use malachite_nz::natural_matrix::exhaustive::exhaustive_natural_matrices_from_iterator;
///
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_natural_matrices_from_iterator(exhaustive_natural_range(
///             Natural::ZERO,
///             Natural::TWO
///         )),
///         20
///     ),
///     "[[]_{0,0}, []_{0,1}, []_{1,0}, []_{0,2}, [0], []_{0,3}, [1], [0, 0, 0], [0, 0], [0, 0, \
///     1], [0, 1], []_{2,0}, [1, 0], [0, 1, 0], [1, 1], []_{3,0}, [0, 1, 1], [0; 0], [1, 0, 0], \
///     [0; 0; 0], ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_natural_matrices_from_iterator<I: Clone + Iterator<Item = Natural>>(
    xs: I,
) -> ExhaustiveNaturalMatrices<
    ExhaustiveDependentPairs<
        (u64, u64),
        Vec<Natural>,
        RulerSequence<usize>,
        ExhaustiveNaturalMatrixElements<I>,
        ExhaustivePairs1Input<PrimitiveIntIncreasingRange<u64>>,
        ExhaustiveFixedLengthVecs1Input<I>,
    >,
> {
    ExhaustiveNaturalMatrices(exhaustive_dependent_pairs(
        ruler_sequence(),
        exhaustive_pairs_from_single(exhaustive_unsigneds()),
        ExhaustiveNaturalMatrixElements(xs),
    ))
}

/// Generates all [`NaturalMatrix`]s.
///
/// Every matrix, of every shape, is generated once, the 0-by-0 matrix first. The matrices are
/// generated as they are by [`exhaustive_natural_matrices_from_iterator`], with
/// [`exhaustive_naturals`] as the elements.
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
/// use malachite_nz::natural_matrix::exhaustive::exhaustive_natural_matrices;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_natural_matrices(), 20),
///     "[[]_{0,0}, []_{0,1}, []_{1,0}, []_{0,2}, [0], []_{0,3}, [1], [0, 0, 0], [2], [0, 0], [3], \
///     [0, 0, 1], [4], [0, 1], [5], [0; 0], [6], [1, 0], [7], [0, 1, 0], ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_natural_matrices() -> ExhaustiveNaturalMatrices<
    ExhaustiveDependentPairs<
        (u64, u64),
        Vec<Natural>,
        RulerSequence<usize>,
        ExhaustiveNaturalMatrixElements<ExhaustiveNaturalRangeToInfinity>,
        ExhaustivePairs1Input<PrimitiveIntIncreasingRange<u64>>,
        ExhaustiveFixedLengthVecs1Input<ExhaustiveNaturalRangeToInfinity>,
    >,
> {
    exhaustive_natural_matrices_from_iterator(exhaustive_naturals())
}

/// Generates all [`NaturalMatrix`]s with a given height and width whose elements come from an
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
/// The elements here are 0 and 1.
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::basic::traits::{Two, Zero};
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural::exhaustive::exhaustive_natural_range;
/// use malachite_nz::natural_matrix::exhaustive::*;
///
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_natural_matrices_with_dimensions_from_iterator(
///             2,
///             2,
///             exhaustive_natural_range(Natural::ZERO, Natural::TWO)
///         ),
///         20
///     ),
///     "[[0, 0; 0, 0], [0, 0; 0, 1], [0, 0; 1, 0], [0, 0; 1, 1], [0, 1; 0, 0], [0, 1; 0, 1], [0, \
///     1; 1, 0], [0, 1; 1, 1], [1, 0; 0, 0], [1, 0; 0, 1], [1, 0; 1, 0], [1, 0; 1, 1], [1, 1; 0, \
///     0], [1, 1; 0, 1], [1, 1; 1, 0], [1, 1; 1, 1]]"
/// );
/// ```
#[inline]
pub fn exhaustive_natural_matrices_with_dimensions_from_iterator<
    I: Clone + Iterator<Item = Natural>,
>(
    height: u64,
    width: u64,
    xs: I,
) -> ExhaustiveNaturalMatrices<Zip<Repeat<(u64, u64)>, ExhaustiveFixedLengthVecs1Input<I>>> {
    let count = shape(height, width).2;
    ExhaustiveNaturalMatrices(repeat((height, width)).zip(
        exhaustive_vecs_fixed_length_from_single(u64::exact_from(count), xs),
    ))
}

/// Generates all [`NaturalMatrix`]s with a given height and width.
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
/// use malachite_nz::natural_matrix::exhaustive::exhaustive_natural_matrices_with_dimensions;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_natural_matrices_with_dimensions(0, 3), 10),
///     "[[]_{0,3}]"
/// );
/// assert_eq!(
///     prefix_to_string(exhaustive_natural_matrices_with_dimensions(2, 2), 10),
///     "[[0, 0; 0, 0], [0, 0; 0, 1], [0, 0; 1, 0], [0, 0; 1, 1], [0, 1; 0, 0], [0, 1; 0, 1], [0, \
///     1; 1, 0], [0, 1; 1, 1], [1, 0; 0, 0], [1, 0; 0, 1], ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_natural_matrices_with_dimensions(
    height: u64,
    width: u64,
) -> ExhaustiveNaturalMatrices<
    Zip<Repeat<(u64, u64)>, ExhaustiveFixedLengthVecs1Input<ExhaustiveNaturalRangeToInfinity>>,
> {
    exhaustive_natural_matrices_with_dimensions_from_iterator(height, width, exhaustive_naturals())
}

/// Generates square [`NaturalMatrix`]s from an iterator of sizes paired with elements in row-major
/// order.
///
/// This `struct` is created by [`exhaustive_natural_square_matrices_from_iterator`] and
/// [`exhaustive_natural_square_matrices`]; see their documentation for more.
#[derive(Clone, Debug)]
pub struct ExhaustiveNaturalSquareMatrices<I: Iterator<Item = (u64, Vec<Natural>)>>(I);

impl<I: Iterator<Item = (u64, Vec<Natural>)>> Iterator for ExhaustiveNaturalSquareMatrices<I> {
    type Item = NaturalMatrix;

    #[inline]
    fn next(&mut self) -> Option<NaturalMatrix> {
        self.0
            .next()
            .map(|(size, elements)| NaturalMatrix::from_owned_elements(size, size, elements))
    }
}

/// Generates, for a size $n$, every way of filling an $n$-by-$n$ matrix with elements from an
/// iterator.
///
/// This `struct` is used by [`exhaustive_natural_square_matrices_from_iterator`] and
/// [`exhaustive_natural_square_matrices`]; see their documentation for more.
#[derive(Clone, Debug)]
pub struct ExhaustiveNaturalSquareMatrixElements<I: Clone + Iterator<Item = Natural>>(I);

impl<I: Clone + Iterator<Item = Natural>>
    ExhaustiveDependentPairsYsGenerator<u64, Vec<Natural>, ExhaustiveFixedLengthVecs1Input<I>>
    for ExhaustiveNaturalSquareMatrixElements<I>
{
    #[inline]
    fn get_ys(&self, &size: &u64) -> ExhaustiveFixedLengthVecs1Input<I> {
        exhaustive_vecs_fixed_length_from_single(
            u64::exact_from(shape(size, size).2),
            self.0.clone(),
        )
    }
}

/// Generates all square [`NaturalMatrix`]s whose elements come from an iterator.
///
/// Every size, including 0, is generated, and so is every way of filling an $n$-by-$n$ matrix with
/// elements of `xs`; each matrix is generated once if `xs` generates each element once. This allows
/// any set of elements, for example the residues modulo some $m$. The sizes are interleaved so that
/// small matrices of every size come early.
///
/// The output length is infinite.
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
/// The elements here are 0 and 1.
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::basic::traits::{Two, Zero};
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural::exhaustive::exhaustive_natural_range;
/// use malachite_nz::natural_matrix::exhaustive::exhaustive_natural_square_matrices_from_iterator;
///
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_natural_square_matrices_from_iterator(exhaustive_natural_range(
///             Natural::ZERO,
///             Natural::TWO
///         )),
///         20
///     ),
///     "[[]_{0,0}, [0], [1], [0, 0, 0; 0, 0, 0; 0, 0, 0], [0, 0; 0, 0], [0, 0, 0; 0, 0, 0; 0, 0, \
///     1], [0, 0; 0, 1], [0, 0, 0, 0, 0; 0, 0, 0, 0, 0; 0, 0, 0, 0, 0; 0, 0, 0, 0, 0; 0, 0, 0, 0, \
///     0], [0, 0; 1, 0], [0, 0, 0; 0, 0, 0; 0, 1, 0], [0, 0; 1, 1], [0, 0, 0, 0; 0, 0, 0, 0; 0, \
///     0, 0, 0; 0, 0, 0, 0], [0, 1; 0, 0], [0, 0, 0; 0, 0, 0; 0, 1, 1], [0, 1; 0, 1], [0, 0, 0, \
///     0, 0, 0; 0, 0, 0, 0, 0, 0; 0, 0, 0, 0, 0, 0; 0, 0, 0, 0, 0, 0; 0, 0, 0, 0, 0, 0; 0, 0, 0, \
///     0, 0, 0], [0, 1; 1, 0], [0, 0, 0; 0, 0, 0; 1, 0, 0], [0, 1; 1, 1], [0, 0, 0, 0; 0, 0, 0, \
///     0; 0, 0, 0, 0; 0, 0, 0, 1], ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_natural_square_matrices_from_iterator<I: Clone + Iterator<Item = Natural>>(
    xs: I,
) -> ExhaustiveNaturalSquareMatrices<
    ExhaustiveDependentPairs<
        u64,
        Vec<Natural>,
        RulerSequence<usize>,
        ExhaustiveNaturalSquareMatrixElements<I>,
        PrimitiveIntIncreasingRange<u64>,
        ExhaustiveFixedLengthVecs1Input<I>,
    >,
> {
    ExhaustiveNaturalSquareMatrices(exhaustive_dependent_pairs(
        ruler_sequence(),
        exhaustive_unsigneds(),
        ExhaustiveNaturalSquareMatrixElements(xs),
    ))
}

/// Generates all square [`NaturalMatrix`]s.
///
/// Every square matrix, of every size, is generated once, the 0-by-0 matrix first. The matrices are
/// generated as they are by [`exhaustive_natural_square_matrices_from_iterator`], with
/// [`exhaustive_naturals`] as the elements.
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
/// use malachite_nz::natural_matrix::exhaustive::exhaustive_natural_square_matrices;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_natural_square_matrices(), 20),
///     "[[]_{0,0}, [0], [1], [0, 0, 0; 0, 0, 0; 0, 0, 0], [2], [0, 0; 0, 0], [3], [0, 0, 0, 0; 0, \
///     0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0], [4], [0, 0; 0, 1], [5], [0, 0, 0; 0, 0, 0; 0, 0, 1], \
///     [6], [0, 0; 1, 0], [7], [0, 0, 0, 0, 0; 0, 0, 0, 0, 0; 0, 0, 0, 0, 0; 0, 0, 0, 0, 0; 0, 0, \
///     0, 0, 0], [8], [0, 0; 1, 1], [9], [0, 0, 0; 0, 0, 0; 0, 1, 0], ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_natural_square_matrices() -> ExhaustiveNaturalSquareMatrices<
    ExhaustiveDependentPairs<
        u64,
        Vec<Natural>,
        RulerSequence<usize>,
        ExhaustiveNaturalSquareMatrixElements<ExhaustiveNaturalRangeToInfinity>,
        PrimitiveIntIncreasingRange<u64>,
        ExhaustiveFixedLengthVecs1Input<ExhaustiveNaturalRangeToInfinity>,
    >,
> {
    exhaustive_natural_square_matrices_from_iterator(exhaustive_naturals())
}
