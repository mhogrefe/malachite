// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural::exhaustive::{ExhaustiveNaturalRangeToInfinity, exhaustive_naturals};
use crate::natural_vector::NaturalVector;
use alloc::vec::Vec;
use malachite_base::num::exhaustive::PrimitiveIntIncreasingRange;
use malachite_base::vecs::exhaustive::{
    ExhaustiveFixedLengthVecs1Input, ExhaustiveOrderedUniqueCollections, ExhaustiveVecs,
    exhaustive_ordered_unique_vecs_fixed_length, exhaustive_vecs,
    exhaustive_vecs_fixed_length_from_single,
};

/// Generates [`NaturalVector`]s from an iterator of [`Vec`]s of [`Natural`]s.
///
/// This `struct` is created by [`exhaustive_natural_vectors`],
/// [`exhaustive_natural_vectors_with_dimension`], and
/// [`exhaustive_ordered_unique_natural_vectors_with_dimension`]; see their documentation for more.
#[derive(Clone, Debug)]
pub struct ExhaustiveNaturalVectors<I: Iterator<Item = Vec<Natural>>>(I);

impl<I: Iterator<Item = Vec<Natural>>> Iterator for ExhaustiveNaturalVectors<I> {
    type Item = NaturalVector;

    #[inline]
    fn next(&mut self) -> Option<NaturalVector> {
        self.0.next().map(|elements| NaturalVector { elements })
    }
}

/// Generates all [`NaturalVector`]s.
///
/// Every vector, of every dimension, is generated once, the 0-dimensional vector first.
///
/// The dimensions grow logarithmically, as the degrees do in [`exhaustive_natural_polynomials`](
/// crate::natural_polynomial::exhaustive::exhaustive_natural_polynomials): dimension $d$ is first
/// reached after $O(2^d)$ outputs.
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
/// use malachite_nz::natural_vector::exhaustive::exhaustive_natural_vectors;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_natural_vectors(), 20),
///     "[(), (0), (1), (0, 0, 0), (2), (0, 0), (3), (0, 0, 0, 0), (4), (0, 1), (5), (0, 0, 1), \
///     (6), (1, 0), (7), (0, 0, 0, 0, 0), (8), (1, 1), (9), (0, 1, 0), ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_natural_vectors() -> ExhaustiveNaturalVectors<
    ExhaustiveVecs<Natural, PrimitiveIntIncreasingRange<u64>, ExhaustiveNaturalRangeToInfinity>,
> {
    ExhaustiveNaturalVectors(exhaustive_vecs(exhaustive_naturals()))
}

/// Generates all [`NaturalVector`]s of a given dimension.
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
/// use malachite_nz::natural_vector::exhaustive::exhaustive_natural_vectors_with_dimension;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_natural_vectors_with_dimension(0), 10),
///     "[()]"
/// );
/// assert_eq!(
///     prefix_to_string(exhaustive_natural_vectors_with_dimension(2), 10),
///     "[(0, 0), (0, 1), (1, 0), (1, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 0), (2, 1), ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_natural_vectors_with_dimension(
    dimension: u64,
) -> ExhaustiveNaturalVectors<ExhaustiveFixedLengthVecs1Input<ExhaustiveNaturalRangeToInfinity>> {
    ExhaustiveNaturalVectors(exhaustive_vecs_fixed_length_from_single(
        dimension,
        exhaustive_naturals(),
    ))
}

/// Generates all [`NaturalVector`]s of a given dimension whose elements are distinct and in
/// ascending order.
///
/// If `dimension` is 0, the only output is the 0-dimensional vector; otherwise the output length is
/// infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(n \log i)$
///
/// $M(i) = O(n \log i)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, and $n$ is
/// `dimension`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_nz::natural_vector::exhaustive::*;
///
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_ordered_unique_natural_vectors_with_dimension(0),
///         10
///     ),
///     "[()]"
/// );
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_ordered_unique_natural_vectors_with_dimension(2),
///         10
///     ),
///     "[(0, 1), (0, 2), (1, 2), (0, 3), (1, 3), (2, 3), (0, 4), (1, 4), (2, 4), (3, 4), ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_ordered_unique_natural_vectors_with_dimension(
    dimension: u64,
) -> ExhaustiveNaturalVectors<
    ExhaustiveOrderedUniqueCollections<ExhaustiveNaturalRangeToInfinity, Vec<Natural>>,
> {
    ExhaustiveNaturalVectors(exhaustive_ordered_unique_vecs_fixed_length(
        dimension,
        exhaustive_naturals(),
    ))
}
