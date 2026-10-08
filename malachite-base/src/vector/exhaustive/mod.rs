// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::iterators::bit_distributor::BitDistributorOutputType;
use crate::num::exhaustive::PrimitiveIntIncreasingRange;
use crate::num::iterators::{BitDistributorSequence, bit_distributor_sequence};
use crate::vecs::exhaustive::{
    ExhaustiveFixedLengthVecs1Input, ExhaustiveVecs, exhaustive_vecs_fixed_length_from_single,
    exhaustive_vecs_with_index_generator,
};
use crate::vector::Vector;
use alloc::vec::Vec;

/// Generates [`Vector`]s from an iterator of [`Vec`]s.
///
/// This `struct` is created by [`exhaustive_vectors`] and [`exhaustive_vectors_with_dimension`];
/// see their documentation for more.
#[derive(Clone, Debug)]
pub struct ExhaustiveVectors<I>(I);

impl<T, I: Iterator<Item = Vec<T>>> Iterator for ExhaustiveVectors<I> {
    type Item = Vector<T>;

    #[inline]
    fn next(&mut self) -> Option<Vector<T>> {
        self.0.next().map(|elements| Vector { elements })
    }
}

/// Generates all [`Vector`]s with elements from a given iterator.
///
/// Every vector, of every dimension, is generated once, the 0-dimensional vector first. If `xs` is
/// empty, the 0-dimensional vector is the only output; otherwise the output length is infinite.
///
/// The dimensions grow as the cube root of the iteration number, as the degrees of the exhaustive
/// polynomial generators do: dimension $d$ is first reached after $O(d^3)$ outputs, which is slow
/// enough to leave room for the elements and fast enough that low-dimensional vectors are not most
/// of what comes out.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(n + T^\prime(i))$
///
/// $M(i) = O(n + M^\prime(i))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of `xs`, and $n$ is the dimension of the $i$th
/// output.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::exhaustive::exhaustive_unsigneds;
/// use malachite_base::vector::exhaustive::exhaustive_vectors;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_vectors(exhaustive_unsigneds::<u8>()), 20),
///     "[(), (0), (1), (0, 0), (2), (0, 1), (3), (1, 0), (0, 0, 0), (0, 0, 0, 0), (0, 0, 1), \
///     (0, 0, 0, 1), (0, 1, 0), (0, 0, 1, 0), (0, 1, 1), (0, 0, 1, 1), (4), (1, 1), (5), \
///     (0, 2), ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_vectors<I: Clone + Iterator>(
    xs: I,
) -> ExhaustiveVectors<
    ExhaustiveVecs<I::Item, PrimitiveIntIncreasingRange<u64>, I, BitDistributorSequence>,
>
where
    I::Item: Clone,
{
    ExhaustiveVectors(exhaustive_vecs_with_index_generator(
        xs,
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(2),
        ),
    ))
}

/// Generates all [`Vector`]s of a given dimension with elements from a given iterator.
///
/// If `dimension` is 0, the only output is the 0-dimensional vector. Otherwise the output length
/// is the length of `xs` raised to the power `dimension`, which is infinite if `xs` is.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(n + T^\prime(i))$
///
/// $M(i) = O(n + M^\prime(i))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of `xs`, and $n$ is `dimension`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::exhaustive::exhaustive_unsigneds;
/// use malachite_base::vector::exhaustive::exhaustive_vectors_with_dimension;
///
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_vectors_with_dimension(0, exhaustive_unsigneds::<u8>()),
///         10
///     ),
///     "[()]"
/// );
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_vectors_with_dimension(2, exhaustive_unsigneds::<u8>()),
///         10
///     ),
///     "[(0, 0), (0, 1), (1, 0), (1, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 0), (2, 1), ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_vectors_with_dimension<I: Clone + Iterator>(
    dimension: u64,
    xs: I,
) -> ExhaustiveVectors<ExhaustiveFixedLengthVecs1Input<I>>
where
    I::Item: Clone,
{
    ExhaustiveVectors(exhaustive_vecs_fixed_length_from_single(dimension, xs))
}
