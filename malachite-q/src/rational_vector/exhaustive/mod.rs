// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational::exhaustive::{ExhaustiveNonzeroRationals, exhaustive_rationals};
use crate::rational_vector::RationalVector;
use alloc::vec::Vec;
use core::iter::{Chain, Once};
use malachite_base::iterators::bit_distributor::BitDistributorOutputType;
use malachite_base::num::exhaustive::PrimitiveIntIncreasingRange;
use malachite_base::num::iterators::{BitDistributorSequence, bit_distributor_sequence};
use malachite_base::vecs::exhaustive::{
    ExhaustiveFixedLengthVecs1Input, ExhaustiveVecs, exhaustive_vecs_fixed_length_from_single,
    exhaustive_vecs_with_index_generator,
};

/// Generates [`RationalVector`]s from an iterator of [`Vec`]s of [`Rational`]s.
///
/// This `struct` is created by [`exhaustive_rational_vectors`] and
/// [`exhaustive_rational_vectors_with_dimension`]; see their documentation for more.
#[derive(Clone, Debug)]
pub struct ExhaustiveRationalVectors<I: Iterator<Item = Vec<Rational>>>(I);

impl<I: Iterator<Item = Vec<Rational>>> Iterator for ExhaustiveRationalVectors<I> {
    type Item = RationalVector;

    #[inline]
    fn next(&mut self) -> Option<RationalVector> {
        self.0.next().map(|elements| RationalVector { elements })
    }
}

/// Generates all [`RationalVector`]s.
///
/// Every vector, of every dimension, is generated once, the 0-dimensional vector first.
///
/// The dimensions grow as the cube root of the iteration number, as the degrees do in
/// [`exhaustive_rational_polynomials`](
/// crate::rational_polynomial::exhaustive::exhaustive_rational_polynomials): dimension $d$ is first
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
/// use malachite_q::rational_vector::exhaustive::exhaustive_rational_vectors;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_rational_vectors(), 20),
///     "[(), (0), (1), (0, 0), (-1), (0, 1), (1/2), (1, 0), (0, 0, 0), (0, 0, 0, 0), (0, 0, 1), \
///     (0, 0, 0, 1), (0, 1, 0), (0, 0, 1, 0), (0, 1, 1), (0, 0, 1, 1), (-1/2), (1, 1), (2), \
///     (0, -1), ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_rational_vectors() -> ExhaustiveRationalVectors<
    ExhaustiveVecs<
        Rational,
        PrimitiveIntIncreasingRange<u64>,
        Chain<Once<Rational>, ExhaustiveNonzeroRationals>,
        BitDistributorSequence,
    >,
> {
    ExhaustiveRationalVectors(exhaustive_vecs_with_index_generator(
        exhaustive_rationals(),
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(2),
        ),
    ))
}

/// Generates all [`RationalVector`]s of a given dimension.
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
/// use malachite_q::rational_vector::exhaustive::exhaustive_rational_vectors_with_dimension;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_rational_vectors_with_dimension(0), 10),
///     "[()]"
/// );
/// assert_eq!(
///     prefix_to_string(exhaustive_rational_vectors_with_dimension(2), 10),
///     "[(0, 0), (0, 1), (1, 0), (1, 1), (0, -1), (0, 1/2), (1, -1), (1, 1/2), (-1, 0), \
///     (-1, 1), ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_rational_vectors_with_dimension(
    dimension: u64,
) -> ExhaustiveRationalVectors<
    ExhaustiveFixedLengthVecs1Input<Chain<Once<Rational>, ExhaustiveNonzeroRationals>>,
> {
    ExhaustiveRationalVectors(exhaustive_vecs_fixed_length_from_single(
        dimension,
        exhaustive_rationals(),
    ))
}
