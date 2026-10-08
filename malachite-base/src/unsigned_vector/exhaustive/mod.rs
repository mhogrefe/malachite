// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::iterators::bit_distributor::BitDistributorOutputType;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::exhaustive::{PrimitiveIntIncreasingRange, exhaustive_unsigneds};
use crate::num::iterators::{BitDistributorSequence, bit_distributor_sequence};
use crate::unsigned_vector::UnsignedVector;
use crate::vecs::exhaustive::{
    ExhaustiveFixedLengthVecs1Input, ExhaustiveVecs, exhaustive_vecs_fixed_length_from_single,
    exhaustive_vecs_with_index_generator,
};
use alloc::vec::Vec;

/// Generates [`UnsignedVector`]s from an iterator of [`Vec`]s.
///
/// This `struct` is created by [`exhaustive_unsigned_vectors`] and
/// [`exhaustive_unsigned_vectors_with_dimension`]; see their documentation for more.
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
/// The dimensions grow as the cube root of the iteration number, as the degrees do in
/// [`exhaustive_unsigned_polynomials`](
/// crate::unsigned_polynomial::exhaustive::exhaustive_unsigned_polynomials): dimension $d$ is first
/// reached after $O(d^3)$ outputs, which is slow enough to leave room for the elements and fast
/// enough that low-dimensional vectors are not most of what comes out.
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
///     "[(), (0), (1), (0, 0), (2), (0, 1), (3), (1, 0), (0, 0, 0), (0, 0, 0, 0), (0, 0, 1), \
///     (0, 0, 0, 1), (0, 1, 0), (0, 0, 1, 0), (0, 1, 1), (0, 0, 1, 1), (4), (1, 1), (5), \
///     (0, 2), ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_unsigned_vectors<T: PrimitiveUnsigned>() -> ExhaustiveUnsignedVectors<
    ExhaustiveVecs<
        T,
        PrimitiveIntIncreasingRange<u64>,
        PrimitiveIntIncreasingRange<T>,
        BitDistributorSequence,
    >,
> {
    ExhaustiveUnsignedVectors(exhaustive_vecs_with_index_generator(
        exhaustive_unsigneds(),
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(2),
        ),
    ))
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
