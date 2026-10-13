// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::exhaustive::{PrimitiveIntIncreasingRange, exhaustive_unsigneds};
use crate::unsigned_vector::UnsignedVector;
use crate::vecs::exhaustive::{
    ExhaustiveFixedLengthVecs1Input, ExhaustiveOrderedUniqueCollections, ExhaustiveVecs,
    exhaustive_ordered_unique_vecs_fixed_length, exhaustive_vecs,
    exhaustive_vecs_fixed_length_from_single,
};
use alloc::vec::Vec;

/// Generates [`UnsignedVector`]s from an iterator of [`Vec`]s.
///
/// This `struct` is created by [`exhaustive_unsigned_vectors`],
/// [`exhaustive_unsigned_vectors_with_dimension`], and
/// [`exhaustive_ordered_unique_unsigned_vectors_with_dimension`]; see their documentation for more.
#[derive(Clone, Debug)]
pub struct ExhaustiveUnsignedVectors<I>(I);

impl<T: PrimitiveUnsigned, I: Iterator<Item = Vec<T>>> Iterator for ExhaustiveUnsignedVectors<I> {
    type Item = UnsignedVector<T>;

    #[inline]
    fn next(&mut self) -> Option<UnsignedVector<T>> {
        self.0.next().map(|elements| UnsignedVector { elements })
    }
}

/// Generates all [`UnsignedVector`]s.
///
/// Every vector, of every dimension, is generated once, the 0-dimensional vector first.
///
/// The dimensions grow logarithmically, as the degrees do in [`exhaustive_unsigned_polynomials`](
/// crate::unsigned_polynomial::exhaustive::exhaustive_unsigned_polynomials): dimension $d$ is first
/// reached after $O(2^d)$ outputs.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(n)$
///
/// $M(i) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, and $n$ is the
/// dimension of the $i$th output.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::unsigned_vector::exhaustive::exhaustive_unsigned_vectors;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_unsigned_vectors::<u8>(), 20),
///     "[(), (0), (1), (0, 0, 0), (2), (0, 0), (3), (0, 0, 0, 0), (4), (0, 1), (5), (0, 0, 1), \
///     (6), (1, 0), (7), (0, 0, 0, 0, 0), (8), (1, 1), (9), (0, 1, 0), ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_unsigned_vectors<T: PrimitiveUnsigned>() -> ExhaustiveUnsignedVectors<
    ExhaustiveVecs<T, PrimitiveIntIncreasingRange<u64>, PrimitiveIntIncreasingRange<T>>,
> {
    ExhaustiveUnsignedVectors(exhaustive_vecs(exhaustive_unsigneds()))
}

/// Generates all [`UnsignedVector`]s of a given dimension.
///
/// If `dimension` is 0, the only output is the 0-dimensional vector. Otherwise the output length is
/// $2^{Wn}$, where $W$ is `T::WIDTH` and $n$ is `dimension`.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(n)$
///
/// $M(i) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, and $n$ is
/// `dimension`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::unsigned_vector::exhaustive::exhaustive_unsigned_vectors_with_dimension;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_unsigned_vectors_with_dimension::<u8>(0), 10),
///     "[()]"
/// );
/// assert_eq!(
///     prefix_to_string(exhaustive_unsigned_vectors_with_dimension::<u8>(2), 10),
///     "[(0, 0), (0, 1), (1, 0), (1, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 0), (2, 1), ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_unsigned_vectors_with_dimension<T: PrimitiveUnsigned>(
    dimension: u64,
) -> ExhaustiveUnsignedVectors<ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>> {
    ExhaustiveUnsignedVectors(exhaustive_vecs_fixed_length_from_single(
        dimension,
        exhaustive_unsigneds(),
    ))
}

/// Generates all [`UnsignedVector`]s of a given dimension whose elements are distinct and in
/// ascending order.
///
/// If `dimension` is 0, the only output is the 0-dimensional vector. Otherwise the output length is
/// $\binom{2^W}{n}$, where $W$ is `T::WIDTH` and $n$ is `dimension`; in particular, there is no
/// output if $n > 2^W$.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(n)$
///
/// $M(i) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, and $n$ is
/// `dimension`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::unsigned_vector::exhaustive::*;
///
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_ordered_unique_unsigned_vectors_with_dimension::<u8>(0),
///         10
///     ),
///     "[()]"
/// );
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_ordered_unique_unsigned_vectors_with_dimension::<u8>(2),
///         10
///     ),
///     "[(0, 1), (0, 2), (1, 2), (0, 3), (1, 3), (2, 3), (0, 4), (1, 4), (2, 4), (3, 4), ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_ordered_unique_unsigned_vectors_with_dimension<T: PrimitiveUnsigned>(
    dimension: u64,
) -> ExhaustiveUnsignedVectors<
    ExhaustiveOrderedUniqueCollections<PrimitiveIntIncreasingRange<T>, Vec<T>>,
> {
    ExhaustiveUnsignedVectors(exhaustive_ordered_unique_vecs_fixed_length(
        dimension,
        exhaustive_unsigneds(),
    ))
}
