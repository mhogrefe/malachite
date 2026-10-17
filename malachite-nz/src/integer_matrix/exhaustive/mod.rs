// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer::exhaustive::{IntegerUpDown, exhaustive_integers};
use crate::integer_matrix::{IntegerMatrix, shape};
use alloc::vec::Vec;
use core::iter::{Chain, Once, Repeat, Zip, repeat};
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

/// Generates [`IntegerMatrix`]s from an iterator of dimensions paired with elements in row-major
/// order.
///
/// This `struct` is created by [`exhaustive_integer_matrices_from_iterator`],
/// [`exhaustive_integer_matrices`], [`exhaustive_integer_matrices_with_dimensions_from_iterator`],
/// and [`exhaustive_integer_matrices_with_dimensions`]; see their documentation for more.
#[derive(Clone, Debug)]
pub struct ExhaustiveIntegerMatrices<I: Iterator<Item = ((u64, u64), Vec<Integer>)>>(I);

impl<I: Iterator<Item = ((u64, u64), Vec<Integer>)>> Iterator for ExhaustiveIntegerMatrices<I> {
    type Item = IntegerMatrix;

    #[inline]
    fn next(&mut self) -> Option<IntegerMatrix> {
        self.0.next().map(|((height, width), elements)| {
            IntegerMatrix::from_owned_elements(height, width, elements)
        })
    }
}

/// Generates, for a height and width, every way of filling a matrix of that shape with elements
/// from an iterator.
///
/// This `struct` is used by [`exhaustive_integer_matrices_from_iterator`] and
/// [`exhaustive_integer_matrices`]; see their documentation for more.
#[derive(Clone, Debug)]
pub struct ExhaustiveIntegerMatrixElements<I: Clone + Iterator<Item = Integer>>(I);

impl<I: Clone + Iterator<Item = Integer>>
    ExhaustiveDependentPairsYsGenerator<
        (u64, u64),
        Vec<Integer>,
        ExhaustiveFixedLengthVecs1Input<I>,
    > for ExhaustiveIntegerMatrixElements<I>
{
    #[inline]
    fn get_ys(&self, &(height, width): &(u64, u64)) -> ExhaustiveFixedLengthVecs1Input<I> {
        exhaustive_vecs_fixed_length_from_single(
            u64::exact_from(shape(height, width).2),
            self.0.clone(),
        )
    }
}

/// Generates all [`IntegerMatrix`]s whose elements come from an iterator.
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
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer::exhaustive::integer_increasing_range;
/// use malachite_nz::integer_matrix::exhaustive::exhaustive_integer_matrices_from_iterator;
///
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_integer_matrices_from_iterator(integer_increasing_range(
///             Integer::ZERO,
///             Integer::TWO
///         )),
///         20
///     ),
///     "[[]_{0,0}, []_{0,1}, []_{1,0}, []_{0,2}, [0], []_{0,3}, [1], [0, 0, 0], [0, 0], [0, 0, \
///     1], [0, 1], []_{2,0}, [1, 0], [0, 1, 0], [1, 1], []_{3,0}, [0, 1, 1], [0; 0], [1, 0, 0], \
///     [0; 0; 0], ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_integer_matrices_from_iterator<I: Clone + Iterator<Item = Integer>>(
    xs: I,
) -> ExhaustiveIntegerMatrices<
    ExhaustiveDependentPairs<
        (u64, u64),
        Vec<Integer>,
        RulerSequence<usize>,
        ExhaustiveIntegerMatrixElements<I>,
        ExhaustivePairs1Input<PrimitiveIntIncreasingRange<u64>>,
        ExhaustiveFixedLengthVecs1Input<I>,
    >,
> {
    ExhaustiveIntegerMatrices(exhaustive_dependent_pairs(
        ruler_sequence(),
        exhaustive_pairs_from_single(exhaustive_unsigneds()),
        ExhaustiveIntegerMatrixElements(xs),
    ))
}

/// Generates all [`IntegerMatrix`]s.
///
/// Every matrix, of every shape, is generated once, the 0-by-0 matrix first. The matrices are
/// generated as they are by [`exhaustive_integer_matrices_from_iterator`], with
/// [`exhaustive_integers`] as the elements.
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
/// use malachite_nz::integer_matrix::exhaustive::exhaustive_integer_matrices;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_integer_matrices(), 20),
///     "[[]_{0,0}, []_{0,1}, []_{1,0}, []_{0,2}, [0], []_{0,3}, [1], [0, 0, 0], [-1], [0, 0], \
///     [2], [0, 0, 1], [-2], [0, 1], [3], [0; 0], [-3], [1, 0], [4], [0, 1, 0], ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_integer_matrices() -> ExhaustiveIntegerMatrices<
    ExhaustiveDependentPairs<
        (u64, u64),
        Vec<Integer>,
        RulerSequence<usize>,
        ExhaustiveIntegerMatrixElements<Chain<Once<Integer>, IntegerUpDown>>,
        ExhaustivePairs1Input<PrimitiveIntIncreasingRange<u64>>,
        ExhaustiveFixedLengthVecs1Input<Chain<Once<Integer>, IntegerUpDown>>,
    >,
> {
    exhaustive_integer_matrices_from_iterator(exhaustive_integers())
}

/// Generates all [`IntegerMatrix`]s with a given height and width whose elements come from an
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
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer::exhaustive::integer_increasing_range;
/// use malachite_nz::integer_matrix::exhaustive::*;
///
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_integer_matrices_with_dimensions_from_iterator(
///             2,
///             2,
///             integer_increasing_range(Integer::ZERO, Integer::TWO)
///         ),
///         20
///     ),
///     "[[0, 0; 0, 0], [0, 0; 0, 1], [0, 0; 1, 0], [0, 0; 1, 1], [0, 1; 0, 0], [0, 1; 0, 1], [0, \
///     1; 1, 0], [0, 1; 1, 1], [1, 0; 0, 0], [1, 0; 0, 1], [1, 0; 1, 0], [1, 0; 1, 1], [1, 1; 0, \
///     0], [1, 1; 0, 1], [1, 1; 1, 0], [1, 1; 1, 1]]"
/// );
/// ```
#[inline]
pub fn exhaustive_integer_matrices_with_dimensions_from_iterator<
    I: Clone + Iterator<Item = Integer>,
>(
    height: u64,
    width: u64,
    xs: I,
) -> ExhaustiveIntegerMatrices<Zip<Repeat<(u64, u64)>, ExhaustiveFixedLengthVecs1Input<I>>> {
    let count = shape(height, width).2;
    ExhaustiveIntegerMatrices(repeat((height, width)).zip(
        exhaustive_vecs_fixed_length_from_single(u64::exact_from(count), xs),
    ))
}

/// Generates all [`IntegerMatrix`]s with a given height and width.
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
/// use malachite_nz::integer_matrix::exhaustive::exhaustive_integer_matrices_with_dimensions;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_integer_matrices_with_dimensions(0, 3), 10),
///     "[[]_{0,3}]"
/// );
/// assert_eq!(
///     prefix_to_string(exhaustive_integer_matrices_with_dimensions(2, 2), 10),
///     "[[0, 0; 0, 0], [0, 0; 0, 1], [0, 0; 1, 0], [0, 0; 1, 1], [0, 1; 0, 0], [0, 1; 0, 1], [0, \
///     1; 1, 0], [0, 1; 1, 1], [1, 0; 0, 0], [1, 0; 0, 1], ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_integer_matrices_with_dimensions(
    height: u64,
    width: u64,
) -> ExhaustiveIntegerMatrices<
    Zip<Repeat<(u64, u64)>, ExhaustiveFixedLengthVecs1Input<Chain<Once<Integer>, IntegerUpDown>>>,
> {
    exhaustive_integer_matrices_with_dimensions_from_iterator(height, width, exhaustive_integers())
}

/// Generates square [`IntegerMatrix`]s from an iterator of sizes paired with elements in row-major
/// order.
///
/// This `struct` is created by [`exhaustive_integer_square_matrices_from_iterator`] and
/// [`exhaustive_integer_square_matrices`]; see their documentation for more.
#[derive(Clone, Debug)]
pub struct ExhaustiveIntegerSquareMatrices<I: Iterator<Item = (u64, Vec<Integer>)>>(I);

impl<I: Iterator<Item = (u64, Vec<Integer>)>> Iterator for ExhaustiveIntegerSquareMatrices<I> {
    type Item = IntegerMatrix;

    #[inline]
    fn next(&mut self) -> Option<IntegerMatrix> {
        self.0
            .next()
            .map(|(size, elements)| IntegerMatrix::from_owned_elements(size, size, elements))
    }
}

/// Generates, for a size $n$, every way of filling an $n$-by-$n$ matrix with elements from an
/// iterator.
///
/// This `struct` is used by [`exhaustive_integer_square_matrices_from_iterator`] and
/// [`exhaustive_integer_square_matrices`]; see their documentation for more.
#[derive(Clone, Debug)]
pub struct ExhaustiveIntegerSquareMatrixElements<I: Clone + Iterator<Item = Integer>>(I);

impl<I: Clone + Iterator<Item = Integer>>
    ExhaustiveDependentPairsYsGenerator<u64, Vec<Integer>, ExhaustiveFixedLengthVecs1Input<I>>
    for ExhaustiveIntegerSquareMatrixElements<I>
{
    #[inline]
    fn get_ys(&self, &size: &u64) -> ExhaustiveFixedLengthVecs1Input<I> {
        exhaustive_vecs_fixed_length_from_single(
            u64::exact_from(shape(size, size).2),
            self.0.clone(),
        )
    }
}

/// Generates all square [`IntegerMatrix`]s whose elements come from an iterator.
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
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer::exhaustive::integer_increasing_range;
/// use malachite_nz::integer_matrix::exhaustive::exhaustive_integer_square_matrices_from_iterator;
///
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_integer_square_matrices_from_iterator(integer_increasing_range(
///             Integer::ZERO,
///             Integer::TWO
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
pub fn exhaustive_integer_square_matrices_from_iterator<I: Clone + Iterator<Item = Integer>>(
    xs: I,
) -> ExhaustiveIntegerSquareMatrices<
    ExhaustiveDependentPairs<
        u64,
        Vec<Integer>,
        RulerSequence<usize>,
        ExhaustiveIntegerSquareMatrixElements<I>,
        PrimitiveIntIncreasingRange<u64>,
        ExhaustiveFixedLengthVecs1Input<I>,
    >,
> {
    ExhaustiveIntegerSquareMatrices(exhaustive_dependent_pairs(
        ruler_sequence(),
        exhaustive_unsigneds(),
        ExhaustiveIntegerSquareMatrixElements(xs),
    ))
}

/// Generates all square [`IntegerMatrix`]s.
///
/// Every square matrix, of every size, is generated once, the 0-by-0 matrix first. The matrices are
/// generated as they are by [`exhaustive_integer_square_matrices_from_iterator`], with
/// [`exhaustive_integers`] as the elements.
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
/// use malachite_nz::integer_matrix::exhaustive::exhaustive_integer_square_matrices;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_integer_square_matrices(), 20),
///     "[[]_{0,0}, [0], [1], [0, 0, 0; 0, 0, 0; 0, 0, 0], [-1], [0, 0; 0, 0], [2], [0, 0, 0, 0; \
///     0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0], [-2], [0, 0; 0, 1], [3], [0, 0, 0; 0, 0, 0; 0, 0, 1], \
///     [-3], [0, 0; 1, 0], [4], [0, 0, 0, 0, 0; 0, 0, 0, 0, 0; 0, 0, 0, 0, 0; 0, 0, 0, 0, 0; 0, \
///     0, 0, 0, 0], [-4], [0, 0; 1, 1], [5], [0, 0, 0; 0, 0, 0; 0, 1, 0], ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_integer_square_matrices() -> ExhaustiveIntegerSquareMatrices<
    ExhaustiveDependentPairs<
        u64,
        Vec<Integer>,
        RulerSequence<usize>,
        ExhaustiveIntegerSquareMatrixElements<Chain<Once<Integer>, IntegerUpDown>>,
        PrimitiveIntIncreasingRange<u64>,
        ExhaustiveFixedLengthVecs1Input<Chain<Once<Integer>, IntegerUpDown>>,
    >,
> {
    exhaustive_integer_square_matrices_from_iterator(exhaustive_integers())
}
