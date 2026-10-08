// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Equality of [`RationalVector`](super::RationalVector)s and
/// [`IntegerVector`](malachite_nz::integer_vector::IntegerVector)s.
///
/// # partial_eq
/// ```
/// use core::str::FromStr;
/// use malachite_nz::integer_vector::IntegerVector;
/// use malachite_q::rational_vector::RationalVector;
///
/// assert!(RationalVector::from_str("()").unwrap() == IntegerVector::from_str("()").unwrap());
/// assert!(
///     RationalVector::from_str("(-4/2, 3)").unwrap()
///         == IntegerVector::from_str("(-2, 3)").unwrap()
/// );
/// assert!(RationalVector::from_str("(1/2)").unwrap() != IntegerVector::from_str("(0)").unwrap());
/// assert!(RationalVector::from_str("(1, 0)").unwrap() != IntegerVector::from_str("(1)").unwrap());
/// assert!(
///     IntegerVector::from_str("(1, -2)").unwrap() == RationalVector::from_str("(1, -2)").unwrap()
/// );
/// assert!(
///     IntegerVector::from_str("(1, 2)").unwrap() != RationalVector::from_str("(2, 1)").unwrap()
/// );
/// ```
pub mod partial_eq_integer_vector;
/// Equality of [`RationalVector`](super::RationalVector)s and
/// [`NaturalVector`](malachite_nz::natural_vector::NaturalVector)s.
///
/// # partial_eq
/// ```
/// use core::str::FromStr;
/// use malachite_nz::natural_vector::NaturalVector;
/// use malachite_q::rational_vector::RationalVector;
///
/// assert!(RationalVector::from_str("()").unwrap() == NaturalVector::from_str("()").unwrap());
/// assert!(
///     RationalVector::from_str("(4/2, 3)").unwrap() == NaturalVector::from_str("(2, 3)").unwrap()
/// );
/// assert!(RationalVector::from_str("(-1)").unwrap() != NaturalVector::from_str("(1)").unwrap());
/// assert!(RationalVector::from_str("(1/2)").unwrap() != NaturalVector::from_str("(0)").unwrap());
/// assert!(
///     NaturalVector::from_str("(1, 2)").unwrap() == RationalVector::from_str("(1, 2)").unwrap()
/// );
/// assert!(
///     NaturalVector::from_str("(1, 2)").unwrap() != RationalVector::from_str("(2, 1)").unwrap()
/// );
/// ```
pub mod partial_eq_natural_vector;
/// Equality of [`RationalVector`](super::RationalVector)s and
/// [`UnsignedVector`](malachite_base::unsigned_vector::UnsignedVector)s.
///
/// # partial_eq
/// ```
/// use core::str::FromStr;
/// use malachite_base::unsigned_vector::UnsignedVector;
/// use malachite_q::rational_vector::RationalVector;
///
/// assert!(
///     RationalVector::from_str("()").unwrap() == UnsignedVector::<u8>::from_str("()").unwrap()
/// );
/// assert!(
///     RationalVector::from_str("(3, 510/2)").unwrap()
///         == UnsignedVector::<u8>::from_str("(3, 255)").unwrap()
/// );
/// assert!(
///     RationalVector::from_str("(3, 256)").unwrap()
///         != UnsignedVector::<u8>::from_str("(3, 0)").unwrap()
/// );
/// assert!(
///     RationalVector::from_str("(3, -1)").unwrap()
///         != UnsignedVector::<u8>::from_str("(3, 255)").unwrap()
/// );
/// assert!(
///     RationalVector::from_str("(1/2)").unwrap()
///         != UnsignedVector::<u8>::from_str("(0)").unwrap()
/// );
/// assert!(
///     UnsignedVector::<u16>::from_str("(1, 2)").unwrap()
///         == RationalVector::from_str("(1, 2)").unwrap()
/// );
/// assert!(
///     UnsignedVector::<u128>::from_str("(1, 2)").unwrap()
///         != RationalVector::from_str("(2, 1)").unwrap()
/// );
/// ```
pub mod partial_eq_unsigned_vector;
/// Implementations of [`Ord`] and [`PartialOrd`] for
/// [`ShortlexRationalVector`](super::ShortlexRationalVector) and
/// [`ShortlexRationalVectorRef`](super::ShortlexRationalVectorRef), comparing two vectors by
/// dimension and then lexicographically.
pub mod shortlex_cmp;
