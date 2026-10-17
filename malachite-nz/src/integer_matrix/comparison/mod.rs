// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Equality of [`IntegerMatrix`](super::IntegerMatrix)s and
/// [`NaturalMatrix`](crate::natural_matrix::NaturalMatrix)s.
///
/// # partial_eq
/// ```
/// use core::str::FromStr;
/// use malachite_nz::integer_matrix::IntegerMatrix;
/// use malachite_nz::natural_matrix::NaturalMatrix;
///
/// assert!(
///     IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap()
///         == NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap()
/// );
/// assert!(
///     IntegerMatrix::from_str("[-1, 2]").unwrap() != NaturalMatrix::from_str("[1, 2]").unwrap()
/// );
/// // The same elements in a different shape are not equal.
/// assert!(
///     IntegerMatrix::from_str("[1, 2, 3, 4]").unwrap()
///         != NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap()
/// );
/// assert!(
///     IntegerMatrix::from_str("[]_{0,3}").unwrap()
///         != NaturalMatrix::from_str("[]_{3,0}").unwrap()
/// );
///
/// assert!(
///     NaturalMatrix::from_str("[]_{2,0}").unwrap()
///         == IntegerMatrix::from_str("[]_{2,0}").unwrap()
/// );
/// assert!(
///     NaturalMatrix::from_str("[1; 2]").unwrap() != IntegerMatrix::from_str("[2; 1]").unwrap()
/// );
/// ```
pub mod partial_eq_natural_matrix;
/// Equality of [`IntegerMatrix`](super::IntegerMatrix)s and
/// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)s.
///
/// # partial_eq
/// ```
/// use core::str::FromStr;
/// use malachite_base::unsigned_matrix::UnsignedMatrix;
/// use malachite_nz::integer_matrix::IntegerMatrix;
///
/// assert!(
///     IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap()
///         == UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap()
/// );
/// assert!(
///     IntegerMatrix::from_str("[3, 256]").unwrap()
///         != UnsignedMatrix::<u8>::from_str("[3, 0]").unwrap()
/// );
/// assert!(
///     IntegerMatrix::from_str("[-1]").unwrap()
///         != UnsignedMatrix::<u8>::from_str("[255]").unwrap()
/// );
/// assert!(
///     IntegerMatrix::from_str("[]_{0,3}").unwrap()
///         != UnsignedMatrix::<u32>::from_str("[]_{3,0}").unwrap()
/// );
///
/// assert!(
///     UnsignedMatrix::<u16>::from_str("[]_{2,0}").unwrap()
///         == IntegerMatrix::from_str("[]_{2,0}").unwrap()
/// );
/// assert!(
///     UnsignedMatrix::<u128>::from_str("[1; 2]").unwrap()
///         != IntegerMatrix::from_str("[2; 1]").unwrap()
/// );
/// ```
pub mod partial_eq_unsigned_matrix;

/// Implementations of [`Ord`] and [`PartialOrd`] for
/// [`ShortlexIntegerMatrix`](super::ShortlexIntegerMatrix) and
/// [`ShortlexIntegerMatrixRef`](super::ShortlexIntegerMatrixRef), comparing two matrices by number
/// of elements, then height, then width, and then lexicographically.
pub mod shortlex_cmp;
