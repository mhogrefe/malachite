// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Equality of [`RationalMatrix`](super::RationalMatrix)s and
/// [`IntegerMatrix`](malachite_nz::integer_matrix::IntegerMatrix)s.
///
/// # partial_eq
/// ```
/// use core::str::FromStr;
/// use malachite_nz::integer_matrix::IntegerMatrix;
/// use malachite_q::rational_matrix::RationalMatrix;
///
/// assert!(
///     RationalMatrix::from_str("[1, -2; 3, 4]").unwrap()
///         == IntegerMatrix::from_str("[1, -2; 3, 4]").unwrap()
/// );
/// assert!(
///     RationalMatrix::from_str("[1/2, 2]").unwrap() != IntegerMatrix::from_str("[0, 2]").unwrap()
/// );
/// assert!(
///     RationalMatrix::from_str("[4/2, 2]").unwrap() != IntegerMatrix::from_str("[2, 1]").unwrap()
/// );
/// // Matrices with no elements are equal only when their shapes match.
/// assert!(
///     RationalMatrix::from_str("[]_{0,3}").unwrap()
///         != IntegerMatrix::from_str("[]_{3,0}").unwrap()
/// );
///
/// assert!(
///     IntegerMatrix::from_str("[]_{2,0}").unwrap()
///         == RationalMatrix::from_str("[]_{2,0}").unwrap()
/// );
/// ```
pub mod partial_eq_integer_matrix;
/// Equality of [`RationalMatrix`](super::RationalMatrix)s and
/// [`NaturalMatrix`](malachite_nz::natural_matrix::NaturalMatrix)s.
///
/// # partial_eq
/// ```
/// use core::str::FromStr;
/// use malachite_nz::natural_matrix::NaturalMatrix;
/// use malachite_q::rational_matrix::RationalMatrix;
///
/// assert!(
///     RationalMatrix::from_str("[1, 2; 3, 4]").unwrap()
///         == NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap()
/// );
/// assert!(
///     RationalMatrix::from_str("[-1, 2]").unwrap() != NaturalMatrix::from_str("[1, 2]").unwrap()
/// );
/// assert!(RationalMatrix::from_str("[1/2]").unwrap() != NaturalMatrix::from_str("[0]").unwrap());
/// // Matrices with no elements are equal only when their shapes match.
/// assert!(
///     RationalMatrix::from_str("[]_{0,3}").unwrap()
///         != NaturalMatrix::from_str("[]_{3,0}").unwrap()
/// );
///
/// assert!(
///     NaturalMatrix::from_str("[]_{2,0}").unwrap()
///         == RationalMatrix::from_str("[]_{2,0}").unwrap()
/// );
/// ```
pub mod partial_eq_natural_matrix;
/// Equality of [`RationalMatrix`](super::RationalMatrix)s and
/// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)s.
///
/// # partial_eq
/// ```
/// use core::str::FromStr;
/// use malachite_base::unsigned_matrix::UnsignedMatrix;
/// use malachite_q::rational_matrix::RationalMatrix;
///
/// assert!(
///     RationalMatrix::from_str("[1, 2; 3, 255]").unwrap()
///         == UnsignedMatrix::<u8>::from_str("[1, 2; 3, 255]").unwrap()
/// );
/// assert!(
///     RationalMatrix::from_str("[3, 256]").unwrap()
///         != UnsignedMatrix::<u8>::from_str("[3, 0]").unwrap()
/// );
/// assert!(
///     RationalMatrix::from_str("[-1]").unwrap()
///         != UnsignedMatrix::<u8>::from_str("[255]").unwrap()
/// );
/// assert!(
///     RationalMatrix::from_str("[1/2]").unwrap()
///         != UnsignedMatrix::<u8>::from_str("[0]").unwrap()
/// );
/// // Matrices with no elements are equal only when their shapes match.
/// assert!(
///     RationalMatrix::from_str("[]_{0,3}").unwrap()
///         != UnsignedMatrix::<u8>::from_str("[]_{3,0}").unwrap()
/// );
///
/// assert!(
///     UnsignedMatrix::<u8>::from_str("[]_{2,0}").unwrap()
///         == RationalMatrix::from_str("[]_{2,0}").unwrap()
/// );
/// ```
pub mod partial_eq_unsigned_matrix;

/// Implementations of [`Ord`] and [`PartialOrd`] for
/// [`ShortlexRationalMatrix`](super::ShortlexRationalMatrix) and
/// [`ShortlexRationalMatrixRef`](super::ShortlexRationalMatrixRef), comparing two matrices by
/// number of elements, then height, then width, and then lexicographically.
pub mod shortlex_cmp;
