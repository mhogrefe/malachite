// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Equality of [`IntegerVector`](super::IntegerVector)s and
/// [`NaturalVector`](crate::natural_vector::NaturalVector)s.
///
/// # partial_eq
/// ```
/// use core::str::FromStr;
/// use malachite_nz::integer_vector::IntegerVector;
/// use malachite_nz::natural_vector::NaturalVector;
///
/// assert!(IntegerVector::from_str("()").unwrap() == NaturalVector::from_str("()").unwrap());
/// assert!(
///     IntegerVector::from_str("(3, 1000000000000000000000000)").unwrap()
///         == NaturalVector::from_str("(3, 1000000000000000000000000)").unwrap()
/// );
/// assert!(
///     IntegerVector::from_str("(3, -1)").unwrap() != NaturalVector::from_str("(3, 1)").unwrap()
/// );
/// assert!(IntegerVector::from_str("(1, 0)").unwrap() != NaturalVector::from_str("(1)").unwrap());
///
/// assert!(
///     NaturalVector::from_str("(1, 2)").unwrap() == IntegerVector::from_str("(1, 2)").unwrap()
/// );
/// assert!(
///     NaturalVector::from_str("(1, 2)").unwrap() != IntegerVector::from_str("(2, 1)").unwrap()
/// );
/// ```
pub mod partial_eq_natural_vector;
/// Equality of [`IntegerVector`](super::IntegerVector)s and
/// [`UnsignedVector`](malachite_base::unsigned_vector::UnsignedVector)s.
///
/// # partial_eq
/// ```
/// use core::str::FromStr;
/// use malachite_base::unsigned_vector::UnsignedVector;
/// use malachite_nz::integer_vector::IntegerVector;
///
/// assert!(
///     IntegerVector::from_str("()").unwrap() == UnsignedVector::<u8>::from_str("()").unwrap()
/// );
/// assert!(
///     IntegerVector::from_str("(3, 255)").unwrap()
///         == UnsignedVector::<u8>::from_str("(3, 255)").unwrap()
/// );
/// assert!(
///     IntegerVector::from_str("(3, 256)").unwrap()
///         != UnsignedVector::<u8>::from_str("(3, 0)").unwrap()
/// );
/// assert!(
///     IntegerVector::from_str("(3, -1)").unwrap()
///         != UnsignedVector::<u8>::from_str("(3, 255)").unwrap()
/// );
///
/// assert!(
///     UnsignedVector::<u16>::from_str("(1, 2)").unwrap()
///         == IntegerVector::from_str("(1, 2)").unwrap()
/// );
/// assert!(
///     UnsignedVector::<u128>::from_str("(1, 2)").unwrap()
///         != IntegerVector::from_str("(2, 1)").unwrap()
/// );
/// ```
pub mod partial_eq_unsigned_vector;
/// Implementations of [`Ord`] and [`PartialOrd`] for
/// [`ShortlexIntegerVector`](super::ShortlexIntegerVector) and
/// [`ShortlexIntegerVectorRef`](super::ShortlexIntegerVectorRef), comparing two vectors by
/// dimension and then lexicographically.
pub mod shortlex_cmp;
