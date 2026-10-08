// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer::exhaustive::{IntegerUpDown, exhaustive_integers};
use crate::integer_vector::IntegerVector;
use alloc::vec::Vec;
use core::iter::{Chain, Once};
use malachite_base::iterators::bit_distributor::BitDistributorOutputType;
use malachite_base::num::exhaustive::PrimitiveIntIncreasingRange;
use malachite_base::num::iterators::{BitDistributorSequence, bit_distributor_sequence};
use malachite_base::vecs::exhaustive::{
    ExhaustiveFixedLengthVecs1Input, ExhaustiveVecs, exhaustive_vecs_fixed_length_from_single,
    exhaustive_vecs_with_index_generator,
};

/// Generates [`IntegerVector`]s from an iterator of [`Vec`]s of [`Integer`]s.
///
/// This `struct` is created by [`exhaustive_integer_vectors`] and
/// [`exhaustive_integer_vectors_with_dimension`]; see their documentation for more.
#[derive(Clone, Debug)]
pub struct ExhaustiveIntegerVectors<I: Iterator<Item = Vec<Integer>>>(I);

impl<I: Iterator<Item = Vec<Integer>>> Iterator for ExhaustiveIntegerVectors<I> {
    type Item = IntegerVector;

    #[inline]
    fn next(&mut self) -> Option<IntegerVector> {
        self.0.next().map(|elements| IntegerVector { elements })
    }
}

/// Generates all [`IntegerVector`]s.
///
/// Every vector, of every dimension, is generated once, the 0-dimensional vector first.
///
/// The dimensions grow as the cube root of the iteration number, as the degrees do in
/// [`exhaustive_integer_polynomials`](
/// crate::integer_polynomial::exhaustive::exhaustive_integer_polynomials): dimension $d$ is first
/// reached after $O(d^3)$ outputs, which is slow enough to leave room for the elements and fast
/// enough that low-dimensional vectors are not most of what comes out.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(n\ell)$
///
/// $M(i) = O(n\ell)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $n$ is the dimension
/// of the $i$th output, and $\ell$ is the number of significant bits of its largest element.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_nz::integer_vector::exhaustive::exhaustive_integer_vectors;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_integer_vectors(), 20),
///     "[(), (0), (1), (0, 0), (-1), (0, 1), (2), (1, 0), (0, 0, 0), (0, 0, 0, 0), (0, 0, 1), \
///     (0, 0, 0, 1), (0, 1, 0), (0, 0, 1, 0), (0, 1, 1), (0, 0, 1, 1), (-2), (1, 1), (3), \
///     (0, -1), ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_integer_vectors() -> ExhaustiveIntegerVectors<
    ExhaustiveVecs<
        Integer,
        PrimitiveIntIncreasingRange<u64>,
        Chain<Once<Integer>, IntegerUpDown>,
        BitDistributorSequence,
    >,
> {
    ExhaustiveIntegerVectors(exhaustive_vecs_with_index_generator(
        exhaustive_integers(),
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(2),
        ),
    ))
}

/// Generates all [`IntegerVector`]s of a given dimension.
///
/// If `dimension` is 0, the only output is the 0-dimensional vector; otherwise the output length is
/// infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(n\ell)$
///
/// $M(i) = O(n\ell)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $n$ is `dimension`,
/// and $\ell$ is the number of significant bits of the $i$th output's largest element.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_nz::integer_vector::exhaustive::exhaustive_integer_vectors_with_dimension;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_integer_vectors_with_dimension(0), 10),
///     "[()]"
/// );
/// assert_eq!(
///     prefix_to_string(exhaustive_integer_vectors_with_dimension(2), 10),
///     "[(0, 0), (0, 1), (1, 0), (1, 1), (0, -1), (0, 2), (1, -1), (1, 2), (-1, 0), (-1, 1), ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_integer_vectors_with_dimension(
    dimension: u64,
) -> ExhaustiveIntegerVectors<ExhaustiveFixedLengthVecs1Input<Chain<Once<Integer>, IntegerUpDown>>>
{
    ExhaustiveIntegerVectors(exhaustive_vecs_fixed_length_from_single(
        dimension,
        exhaustive_integers(),
    ))
}
