// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::num::exhaustive::{PrimitiveIntIncreasingRange, exhaustive_unsigneds};
use crate::num::iterators::{RulerSequence, ruler_sequence};
use crate::tuples::exhaustive::{
    ExhaustiveDependentPairs, ExhaustiveDependentPairsYsGenerator, ExhaustivePairs1Input,
    exhaustive_dependent_pairs, exhaustive_pairs_from_single,
};
use crate::unsigned_matrix::{UnsignedMatrix, element_count};
use crate::vecs::exhaustive::{
    ExhaustiveFixedLengthVecs1Input, exhaustive_vecs_fixed_length_from_single,
};
use alloc::vec::Vec;
use core::iter::{Repeat, Zip, repeat};

/// Generates [`UnsignedMatrix`]s from an iterator of dimensions paired with elements in row-major
/// order.
///
/// This `struct` is created by [`exhaustive_unsigned_matrices_from_iterator`],
/// [`exhaustive_unsigned_matrices`],
/// [`exhaustive_unsigned_matrices_with_dimensions_from_iterator`], and
/// [`exhaustive_unsigned_matrices_with_dimensions`]; see their documentation for more.
#[derive(Clone, Debug)]
pub struct ExhaustiveUnsignedMatrices<
    T: PrimitiveUnsigned,
    I: Iterator<Item = ((usize, usize), Vec<T>)>,
>(I);

impl<T: PrimitiveUnsigned, I: Iterator<Item = ((usize, usize), Vec<T>)>> Iterator
    for ExhaustiveUnsignedMatrices<T, I>
{
    type Item = UnsignedMatrix<T>;

    #[inline]
    fn next(&mut self) -> Option<UnsignedMatrix<T>> {
        self.0.next().map(|((height, width), elements)| {
            UnsignedMatrix::from_owned_elements(height, width, elements)
        })
    }
}

/// Generates, for a height and width, every way of filling a matrix of that shape with elements
/// from an iterator.
///
/// This `struct` is used by [`exhaustive_unsigned_matrices_from_iterator`] and
/// [`exhaustive_unsigned_matrices`]; see their documentation for more.
#[derive(Clone, Debug)]
pub struct ExhaustiveUnsignedMatrixElements<I: Clone + Iterator>(I);

impl<T: PrimitiveUnsigned, I: Clone + Iterator<Item = T>>
    ExhaustiveDependentPairsYsGenerator<(usize, usize), Vec<T>, ExhaustiveFixedLengthVecs1Input<I>>
    for ExhaustiveUnsignedMatrixElements<I>
{
    #[inline]
    fn get_ys(&self, &(height, width): &(usize, usize)) -> ExhaustiveFixedLengthVecs1Input<I> {
        exhaustive_vecs_fixed_length_from_single(
            u64::exact_from(element_count(height, width)),
            self.0.clone(),
        )
    }
}

/// Generates all [`UnsignedMatrix`]s whose elements come from an iterator.
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
/// use malachite_base::num::exhaustive::primitive_int_increasing_range;
/// use malachite_base::unsigned_matrix::exhaustive::exhaustive_unsigned_matrices_from_iterator;
///
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_unsigned_matrices_from_iterator(primitive_int_increasing_range(0u8, 2)),
///         20
///     ),
///     "[[]_{0,0}, []_{0,1}, []_{1,0}, []_{0,2}, [0], []_{0,3}, [1], [0, 0, 0], [0, 0], [0, 0, \
///     1], [0, 1], []_{2,0}, [1, 0], [0, 1, 0], [1, 1], []_{3,0}, [0, 1, 1], [0; 0], [1, 0, 0], \
///     [0; 0; 0], ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_unsigned_matrices_from_iterator<
    T: PrimitiveUnsigned,
    I: Clone + Iterator<Item = T>,
>(
    xs: I,
) -> ExhaustiveUnsignedMatrices<
    T,
    ExhaustiveDependentPairs<
        (usize, usize),
        Vec<T>,
        RulerSequence<usize>,
        ExhaustiveUnsignedMatrixElements<I>,
        ExhaustivePairs1Input<PrimitiveIntIncreasingRange<usize>>,
        ExhaustiveFixedLengthVecs1Input<I>,
    >,
> {
    ExhaustiveUnsignedMatrices(exhaustive_dependent_pairs(
        ruler_sequence(),
        exhaustive_pairs_from_single(exhaustive_unsigneds()),
        ExhaustiveUnsignedMatrixElements(xs),
    ))
}

/// Generates all [`UnsignedMatrix`]s.
///
/// Every matrix, of every shape, is generated once, the 0-by-0 matrix first. The matrices are
/// generated as they are by [`exhaustive_unsigned_matrices_from_iterator`], with
/// [`exhaustive_unsigneds`] as the elements.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(n)$
///
/// $M(i) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, and $n$ is the number
/// of elements of the $i$th output.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::unsigned_matrix::exhaustive::exhaustive_unsigned_matrices;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_unsigned_matrices::<u8>(), 20),
///     "[[]_{0,0}, []_{0,1}, []_{1,0}, []_{0,2}, [0], []_{0,3}, [1], [0, 0, 0], [2], [0, 0], [3], \
///     [0, 0, 1], [4], [0, 1], [5], [0; 0], [6], [1, 0], [7], [0, 1, 0], ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_unsigned_matrices<T: PrimitiveUnsigned>() -> ExhaustiveUnsignedMatrices<
    T,
    ExhaustiveDependentPairs<
        (usize, usize),
        Vec<T>,
        RulerSequence<usize>,
        ExhaustiveUnsignedMatrixElements<PrimitiveIntIncreasingRange<T>>,
        ExhaustivePairs1Input<PrimitiveIntIncreasingRange<usize>>,
        ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
    >,
> {
    exhaustive_unsigned_matrices_from_iterator(exhaustive_unsigneds())
}

/// Generates all [`UnsignedMatrix`]s with a given height and width whose elements come from an
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
/// use malachite_base::num::exhaustive::primitive_int_increasing_range;
/// use malachite_base::unsigned_matrix::exhaustive::*;
///
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_unsigned_matrices_with_dimensions_from_iterator(
///             2,
///             2,
///             primitive_int_increasing_range(0u8, 2)
///         ),
///         20
///     ),
///     "[[0, 0; 0, 0], [0, 0; 0, 1], [0, 0; 1, 0], [0, 0; 1, 1], [0, 1; 0, 0], [0, 1; 0, 1], [0, \
///     1; 1, 0], [0, 1; 1, 1], [1, 0; 0, 0], [1, 0; 0, 1], [1, 0; 1, 0], [1, 0; 1, 1], [1, 1; 0, \
///     0], [1, 1; 0, 1], [1, 1; 1, 0], [1, 1; 1, 1]]"
/// );
/// ```
#[inline]
pub fn exhaustive_unsigned_matrices_with_dimensions_from_iterator<
    T: PrimitiveUnsigned,
    I: Clone + Iterator<Item = T>,
>(
    height: usize,
    width: usize,
    xs: I,
) -> ExhaustiveUnsignedMatrices<T, Zip<Repeat<(usize, usize)>, ExhaustiveFixedLengthVecs1Input<I>>>
{
    ExhaustiveUnsignedMatrices(repeat((height, width)).zip(
        exhaustive_vecs_fixed_length_from_single(u64::exact_from(element_count(height, width)), xs),
    ))
}

/// Generates all [`UnsignedMatrix`]s with a given height and width.
///
/// If `height` or `width` is 0, the only output is the matrix of that shape, which has no elements;
/// otherwise the output length is infinite if `T` has more than one value, which it always does.
///
/// # Worst-case complexity per iteration
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
/// use malachite_base::unsigned_matrix::exhaustive::exhaustive_unsigned_matrices_with_dimensions;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_unsigned_matrices_with_dimensions::<u8>(0, 3), 10),
///     "[[]_{0,3}]"
/// );
/// assert_eq!(
///     prefix_to_string(exhaustive_unsigned_matrices_with_dimensions::<u8>(2, 2), 10),
///     "[[0, 0; 0, 0], [0, 0; 0, 1], [0, 0; 1, 0], [0, 0; 1, 1], [0, 1; 0, 0], [0, 1; 0, 1], [0, \
///     1; 1, 0], [0, 1; 1, 1], [1, 0; 0, 0], [1, 0; 0, 1], ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_unsigned_matrices_with_dimensions<T: PrimitiveUnsigned>(
    height: usize,
    width: usize,
) -> ExhaustiveUnsignedMatrices<
    T,
    Zip<Repeat<(usize, usize)>, ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>>,
> {
    exhaustive_unsigned_matrices_with_dimensions_from_iterator(
        height,
        width,
        exhaustive_unsigneds(),
    )
}

/// Generates square [`UnsignedMatrix`]s from an iterator of sizes paired with elements in row-major
/// order.
///
/// This `struct` is created by [`exhaustive_unsigned_square_matrices_from_iterator`] and
/// [`exhaustive_unsigned_square_matrices`]; see their documentation for more.
#[derive(Clone, Debug)]
pub struct ExhaustiveUnsignedSquareMatrices<
    T: PrimitiveUnsigned,
    I: Iterator<Item = (usize, Vec<T>)>,
>(I);

impl<T: PrimitiveUnsigned, I: Iterator<Item = (usize, Vec<T>)>> Iterator
    for ExhaustiveUnsignedSquareMatrices<T, I>
{
    type Item = UnsignedMatrix<T>;

    #[inline]
    fn next(&mut self) -> Option<UnsignedMatrix<T>> {
        self.0
            .next()
            .map(|(size, elements)| UnsignedMatrix::from_owned_elements(size, size, elements))
    }
}

/// Generates, for a size $n$, every way of filling an $n$-by-$n$ matrix with elements from an
/// iterator.
///
/// This `struct` is used by [`exhaustive_unsigned_square_matrices_from_iterator`] and
/// [`exhaustive_unsigned_square_matrices`]; see their documentation for more.
#[derive(Clone, Debug)]
pub struct ExhaustiveUnsignedSquareMatrixElements<I: Clone + Iterator>(I);

impl<T: PrimitiveUnsigned, I: Clone + Iterator<Item = T>>
    ExhaustiveDependentPairsYsGenerator<usize, Vec<T>, ExhaustiveFixedLengthVecs1Input<I>>
    for ExhaustiveUnsignedSquareMatrixElements<I>
{
    #[inline]
    fn get_ys(&self, &size: &usize) -> ExhaustiveFixedLengthVecs1Input<I> {
        exhaustive_vecs_fixed_length_from_single(
            u64::exact_from(element_count(size, size)),
            self.0.clone(),
        )
    }
}

/// Generates all square [`UnsignedMatrix`]s whose elements come from an iterator.
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
/// use malachite_base::num::exhaustive::primitive_int_increasing_range;
/// use malachite_base::unsigned_matrix::exhaustive::*;
///
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_unsigned_square_matrices_from_iterator(primitive_int_increasing_range(
///             0u8, 2
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
pub fn exhaustive_unsigned_square_matrices_from_iterator<
    T: PrimitiveUnsigned,
    I: Clone + Iterator<Item = T>,
>(
    xs: I,
) -> ExhaustiveUnsignedSquareMatrices<
    T,
    ExhaustiveDependentPairs<
        usize,
        Vec<T>,
        RulerSequence<usize>,
        ExhaustiveUnsignedSquareMatrixElements<I>,
        PrimitiveIntIncreasingRange<usize>,
        ExhaustiveFixedLengthVecs1Input<I>,
    >,
> {
    ExhaustiveUnsignedSquareMatrices(exhaustive_dependent_pairs(
        ruler_sequence(),
        exhaustive_unsigneds(),
        ExhaustiveUnsignedSquareMatrixElements(xs),
    ))
}

/// Generates all square [`UnsignedMatrix`]s.
///
/// Every square matrix, of every size, is generated once, the 0-by-0 matrix first. The matrices are
/// generated as they are by [`exhaustive_unsigned_square_matrices_from_iterator`], with
/// [`exhaustive_unsigneds`] as the elements.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(n)$
///
/// $M(i) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, and $n$ is the number
/// of elements of the $i$th output.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::unsigned_matrix::exhaustive::exhaustive_unsigned_square_matrices;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_unsigned_square_matrices::<u8>(), 20),
///     "[[]_{0,0}, [0], [1], [0, 0, 0; 0, 0, 0; 0, 0, 0], [2], [0, 0; 0, 0], [3], [0, 0, 0, 0; 0, \
///     0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0], [4], [0, 0; 0, 1], [5], [0, 0, 0; 0, 0, 0; 0, 0, 1], \
///     [6], [0, 0; 1, 0], [7], [0, 0, 0, 0, 0; 0, 0, 0, 0, 0; 0, 0, 0, 0, 0; 0, 0, 0, 0, 0; 0, 0, \
///     0, 0, 0], [8], [0, 0; 1, 1], [9], [0, 0, 0; 0, 0, 0; 0, 1, 0], ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_unsigned_square_matrices<T: PrimitiveUnsigned>()
-> ExhaustiveUnsignedSquareMatrices<
    T,
    ExhaustiveDependentPairs<
        usize,
        Vec<T>,
        RulerSequence<usize>,
        ExhaustiveUnsignedSquareMatrixElements<PrimitiveIntIncreasingRange<T>>,
        PrimitiveIntIncreasingRange<usize>,
        ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
    >,
> {
    exhaustive_unsigned_square_matrices_from_iterator(exhaustive_unsigneds())
}
