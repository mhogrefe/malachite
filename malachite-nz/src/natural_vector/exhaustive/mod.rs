// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural::exhaustive::{ExhaustiveNaturalRangeToInfinity, exhaustive_naturals};
use malachite_base::num::exhaustive::PrimitiveIntIncreasingRange;
use malachite_base::num::iterators::BitDistributorSequence;
use malachite_base::vecs::exhaustive::{ExhaustiveFixedLengthVecs1Input, ExhaustiveVecs};
use malachite_base::vector::exhaustive::{
    ExhaustiveVectors, exhaustive_vectors, exhaustive_vectors_with_dimension,
};

/// Generates all [`NaturalVector`](super::NaturalVector)s.
///
/// Every vector, of every dimension, is generated once, the 0-dimensional vector first.
///
/// The dimensions grow as the cube root of the iteration number, as the degrees do in
/// [`exhaustive_natural_polynomials`](
/// crate::natural_polynomial::exhaustive::exhaustive_natural_polynomials): dimension $d$ is first
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
/// use malachite_nz::natural_vector::exhaustive::exhaustive_natural_vectors;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_natural_vectors(), 20),
///     "[(), (0), (1), (0, 0), (2), (0, 1), (3), (1, 0), (0, 0, 0), (0, 0, 0, 0), (0, 0, 1), \
///     (0, 0, 0, 1), (0, 1, 0), (0, 0, 1, 0), (0, 1, 1), (0, 0, 1, 1), (4), (1, 1), (5), \
///     (0, 2), ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_natural_vectors() -> ExhaustiveVectors<
    ExhaustiveVecs<
        Natural,
        PrimitiveIntIncreasingRange<u64>,
        ExhaustiveNaturalRangeToInfinity,
        BitDistributorSequence,
    >,
> {
    exhaustive_vectors(exhaustive_naturals())
}

/// Generates all [`NaturalVector`](super::NaturalVector)s of a given dimension.
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
) -> ExhaustiveVectors<ExhaustiveFixedLengthVecs1Input<ExhaustiveNaturalRangeToInfinity>> {
    exhaustive_vectors_with_dimension(dimension, exhaustive_naturals())
}
